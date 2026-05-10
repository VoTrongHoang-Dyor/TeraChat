---
type: concept
created: 2026-05-10
tags: [terachat, identity, governance, did, opa, rbac, scim, audit]
sources: [tera-gov-spec, tera-core-spec, tera-eco-spec]
---

# Enterprise Identity & Governance

TeraChat's identity and access control system: Decentralized Identifiers (DID), OPA Policy Engine, RBAC, SCIM federation, and immutable audit trail. This is what replaces centralized "Admin = God" models from Slack/Teams.

## Core Components

### DID (Decentralized Identifiers)
Every user has a DID anchored to their device's hardware key. No central identity provider holds user credentials. DIDs are portable across devices via secure enrollment.

### OPA (Open Policy Agent) Policy Engine
Policies are code (Rego), evaluated at the device edge — not just at the server. Every action (read message, send file, install .tapp) passes through OPA:
```
User Action → OPA Policy Evaluation (local) → Allow/Deny
```
Policies must pass **Z3 formal verification** before deployment to ensure no logical contradictions.

### RBAC with Cryptographic Enforcement
- Roles: Organization Admin, IT Admin, Member, Guest
- Each role maps to cryptographic capabilities (DataGrants)
- Approval actions require Ed25519 signatures — not just database flags
- SCIM offboarding: departed employee revoked in < 30s across all devices

### Immutable Audit Trail
- Ed25519 signed, append-only log
- Cannot be deleted or modified — even by TeraChat Inc.
- Legal Hold flag prevents tombstone vacuum for relevant records

## 🧠 Design Decisions (Q&A)

- **Why OPA at the device instead of just server?** → Zero-Trust principle: even if TeraRelay is compromised, the device still enforces policy locally. A compromised relay cannot grant access the device would deny. Trade-off: policy bundles must be distributed and verified on every device.
- **Why Z3 formal verification for policies?** → OPA Rego policies can have logical contradictions (e.g., two rules granting and denying the same action). Z3 proves the policy set is consistent before deployment. Trade-off: adds latency to policy changes, requires writing verifiable policies.
- **Why Ed25519 signatures on approvals instead of DB flag?** → Database flags can be tampered with by a compromised admin or database breach. Cryptographic signatures prove who approved what, non-repudiably. Trade-off: approval workflow is more complex — admin must have key material available.
