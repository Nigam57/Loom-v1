# Corporate Authorization Registry

This document explains the maintenance rules for corporate contribution authorizations.

The machine-readable source file for authorizations is located at:

- [`corporate-authorizations.json`](./corporate-authorizations.json)

## Rules

- It is recommended to use the `CCA-YYYY-NNN` format for the authorization reference, e.g., `CCA-2026-001`.
- A new record should be added to `corporate-authorizations.json` only after the maintainers have confirmed receipt of a valid `CCLA` or equivalent written authorization.
- If a PR declares a "contribution on behalf of an organization", the existing authorization reference must be provided.
- `authorizedGitHubUsernames` is used to declare the GitHub usernames covered by this authorization.
- `authorizedEmails` is used to declare the co-author emails covered by this authorization; if a PR contains `Co-authored-by:`, these emails must also be covered.
- It is recommended to use clear status values such as `Active`, `Expired`, `Revoked` for `status`; only `Active` will be passed by the workflow.
- Update the status and notes promptly when an authorization expires, is revoked, or its scope changes.

## JSON Structure Example

```json
{
  "version": 1,
  "authorizations": [
    {
      "authorizationReference": "CCA-2026-001",
      "organization": "Example Corp",
      "authorizedGitHubUsernames": ["alice", "bob"],
      "authorizedEmails": ["alice@example.com", "bob@example.com"],
      "effectiveDate": "2026-03-21",
      "expirationDate": "2027-03-21",
      "status": "Active",
      "notes": "Optional notes"
    }
  ]
}
```
