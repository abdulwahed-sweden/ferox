# Shopify Partner GraphQL IDOR Testing

## Hypothesis
The Partners GraphQL API may fail to validate tenant authorization on store-scoped queries, allowing cross-tenant data access.

## Prerequisites

### 1. Create Two Shopify Partner Accounts

**Account A (Attacker):**
- Email: Use your HackerOne-registered email
- URL: https://partners.shopify.com/signup

**Account B (Victim - simulated):**
- Email: Use a different email you control
- URL: https://partners.shopify.com/signup

### 2. Create Development Stores

For each partner account:
1. Log in to partners.shopify.com
2. Go to Stores → Add store → Create development store
3. Add sample data (products, test orders)
4. Note the store's internal ID from the URL or API responses

### 3. Tools Required
- Browser with DevTools (Network tab)
- OR Burp Suite / mitmproxy for request interception
- Python 3.x for automated testing scripts

## Testing Workflow

```
Account A                     Account B
    │                             │
    ├── Create dev store ──────► Note Store ID: 123456
    │                             │
    ├── Note own Store ID: 789012 │
    │                             │
    ▼                             │
Intercept GraphQL request         │
    │                             │
    ├── Modify storeId: 789012 → 123456
    │                             │
    ▼                             │
Check response for Account B data │
```

## Scope Reminder

- ONLY test on development stores YOU created
- Do NOT access real merchant data
- Do NOT enumerate store IDs beyond test accounts
- Report immediately if vulnerability confirmed
