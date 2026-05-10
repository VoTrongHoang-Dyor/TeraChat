---
type: source
created: 2026-05-10
tags: [terachat, identity, governance, rbac, opa, did, scim, audit]
sources: [raw/MD/Spec-Identity-And-Governance.md]
depends_on: [tera-core-spec, tera-sync-spec]
---

# Identity, RBAC & Governance (TERA-GOV)

Source: `raw/MD/Spec-Identity-And-Governance.md` — v1.0.0, 2026-03-29.

## What It Covers

Enterprise governance: role-based access control, OPA policy enforcement, identity federation (OIDC/SAML/SCIM), immutable audit trail, and tenant isolation. This is the first file a customer CISO audits.

## Key Constraints

- Ed25519 signed, append-only Audit Log — cannot delete/modify
- OPA Policies must pass Z3 formal verification before deploy
- SCIM offboarding: departed employee must be revoked in < 30s
- .tapp capability permissions declared in Manifest, not runtime
- Legal Hold flag mandatory before vacuuming related tombstones
- Approval action requires Ed25519 signature — not just a database flag

## Consumed By

TERA-RUNTIME, TERA-ENCLAVE, TERA-ECO, TERA-CLIENT

## Related Concepts

- [[Enterprise Identity & Governance]]
- [[Zero-Knowledge Architecture]]
- [[Data Sovereignty & Export]]
