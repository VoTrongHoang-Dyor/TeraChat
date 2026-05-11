# Phase 5 — Governance, Private Data & Secure Enclave

```yaml
id: "TERA-PHASE-5"
title: "Governance Enforcement, AI Enclave & Private Search"
duration: "5 days (Day 26–30)"
economic_phase: "Phase 3 — 'Moat and Ecosystem'"
priority: 🟠 HIGH — Governance and AI are the long-term moat
teams: [Governance & Ecosystem Team, Private AI & Enclave Team]
debt_targets: [GAP-E, GAP-G, GAP-F]
exit_criteria:
  - SCIM, audit, legal hold, DataGrant, AI redaction all work without conflict
  - Clear deploy model for On-Prem, Air-Gapped, Hybrid
  - AI never sees un-redacted PII
  - ZK Memory Agent functional with local indexing budgets
```

## System Design: Phase 5 Connections

```
┌──────────────────────────────────────────────────┐
│  PHASE 5 BUILDS                                   │
│                                                   │
│  Phase 4 (DataGrant) ──────────────┐              │
│  Phase 2 (Sync) ───────────────────┤              │
│                                    ↓               │
│  Governance Layer                                     │
│  ├─ OPA Policy Engine (bundle lifecycle)             │
│  ├─ SCIM 2.0 Offboarding (<30s SLA)                  │
│  ├─ Signed Audit Trail (Ed25519 append)              │
│  ├─ Legal Hold Gates                                 │
│  └─ RBAC → ABAC migration                            │
│                                    ↓               │
│  AI Enclave Layer                                     │
│  ├─ SanitizedPrompt (PII redaction)                  │
│  ├─ DomainPiiPolicy (tenant-specific)                │
│  ├─ ZKMemoryIndex (local encrypted search)           │
│  ├─ On-Prem AI Appliance (Mac mini + RTX)            │
│  └─ Private Search Limits                            │
│                                    ↓               │
│  Deployment Models                                    │
│  ├─ On-Prem (single Mac mini)                        │
│  ├─ Air-Gapped (Shamir 3-of-5, physical ceremony)    │
│  └─ Hybrid (cloud relay + local AI)                  │
│                                    ↓               │
│                              Phase 6 (Release)        │
└──────────────────────────────────────────────────┘
```

---

## Task Box 5.1 — OPA Policy Engine & Signed Audit Trail

```yaml
task_id: "PH5-T01"
name: "Implement Policy Enforcement at Every Boundary"
status: pending
priority: 🔴 CRITICAL
index: IDX-11-GOV-DATAGRANT (continued)
platforms: [macOS, Windows, Linux, iPhone, Android, Oppo, Huawei]
estimated_hours: 10
```

### Code Tasks

| # | Task | Source Path | Platform |
|---|------|-------------|----------|
| 1 | OPA bundle lifecycle: load → validate → activate → enforce at boundary | `source/core/tc-tapp/src/opa.rs` | All |
| 2 | `host_opa_check()` ABI function: .tapp calls policy check before data access | `source/core/tc-tapp/src/abi.rs` | All |
| 3 | Signed audit trail: every governance decision → Ed25519 signed → append-only log | `source/core/tc-tapp/src/audit.rs` | All |
| 4 | Audit log immutability: hash chain linking entries, Merkle root periodically published | `source/core/tc-tapp/src/audit_chain.rs` | All |
| 5 | RBAC → ABAC migration path: existing roles mapped to OPA attribute policies | `source/core/tc-tapp/src/rbac_abac.rs` | All |
| 6 | Policy hot-reload: OPA bundle update without daemon restart | `source/core/tc-tapp/src/opa.rs` | All |

### Testing

| # | Test | Tool | Platform | Chaos Ref |
|---|------|------|----------|-----------|
| 1 | Policy denies data access → host_opa_check returns DENY → .tapp blocked | `cargo test --test opa_deny` | All | — |
| 2 | Audit log entry → Ed25519 signature verifiable with admin pubkey | `cargo test --test audit_signature` | All | — |
| 3 | Audit log tampered (hash chain break) → detected on next integrity check | `cargo test --test audit_tamper` | All | — |
| 4 | OPA bundle hot-reloaded → new policy active < 5s without daemon restart | `cargo test --test opa_hot_reload` | All | — |
| 5 | RBAC role "finance_admin" → ABAC attribute policy → same effective permissions | `cargo test --test rbac_abac_map` | All | — |

### Deployment

OPA engine embedded in Rust Core (via `opa-wasm`). Policy bundles distributed via Relay.

### Request / Review

| Department | Reviewer | Gate |
|------------|----------|------|
| Governance & Compliance | CISO | Policy enforcement at all boundaries |
| Governance & Compliance | Security Architect | Audit log immutability |
| Architecture & Leadership | Compliance Officer | RBAC→ABAC migration completeness |

### Design Requirements

- OPA policies: REGO language, bundled + signed, hot-reloadable
- `host_opa_check()`: synchronous, < 5ms response time
- Audit log: append-only, hash-chained, Ed25519 signed per entry
- No governance decision without audit entry

### Reference Documentation

- `TERA-GOV` — OPA Policy Engine specification
- `TERA-ECO §8.3–8.7` — DataGrant and governance integration
- `docs/raw/MD/Tech_Debt.md` — GAP-E

### System Design Connection

- **Input from:** Phase 4 (DataGrant, registry PKI), Phase 2 (audit log storage)
- **Output to:** Phase 6 (compliance audit pack)
- **Connects:** OPA Engine → Every boundary → Audit Log → PostgreSQL (server)

---

## Task Box 5.2 — SCIM Offboarding & Legal Hold

```yaml
task_id: "PH5-T02"
name: "Implement Enterprise Identity Lifecycle (<30s SLA)"
status: pending
priority: 🔴 CRITICAL
platforms: [macOS, Windows, Linux, iPhone, Android, Oppo, Huawei]
estimated_hours: 8
```

### Code Tasks

| # | Task | Source Path | Platform |
|---|------|-------------|----------|
| 1 | SCIM 2.0 offboarding: user deprovisioned → all keys revoked < 30s | `source/core/tc-tapp/src/scim.rs` | All |
| 2 | OIDC/SAML integration: map Azure AD / Google Workspace attributes → TeraChat roles | `source/core/tc-tapp/src/oidc_saml.rs` | All |
| 3 | Legal hold: freeze all data for user/workspace → prevent deletion, vacuum skip | `source/core/tc-store/src/legal_hold.rs` | All |
| 4 | Approval signatures: sensitive operations require multi-party Ed25519 signatures | `source/core/tc-tapp/src/approval.rs` | All |
| 5 | Remote wipe: Admin triggers → crypto-shred all local keys → force logout all sessions | `source/core/tc-tapp/src/remote_wipe.rs` | All |
| 6 | Duress PIN: alternate PIN → silently triggers remote wipe notification (TD-012 mitigation) | `source/core/tc-crypto/src/duress.rs` | All |

### Testing

| # | Test | Tool | Platform | Chaos Ref |
|---|------|------|----------|-----------|
| 1 | SCIM offboard user → keys revoked, sessions terminated < 30s | `cargo test --test scim_offboard` | All | — |
| 2 | SAML role mapping: "finance_dept" → TeraChat role "FinanceUser" | `cargo test --test saml_role_map` | All | — |
| 3 | Legal hold on workspace → delete blocked, vacuum skips held rows | `cargo test --test legal_hold_delete` | All | — |
| 4 | Remote wipe triggered → local keys shredded → all sessions dead | `cargo test --test remote_wipe` | All | — |
| 5 | Duress PIN entered → silent alert sent → no visible difference to attacker | `cargo test --test duress_pin` | All | — |

### Deployment

SCIM endpoint on TeraRelay server. OIDC/SAML via Keycloak/Dex bridge.

### Request / Review

| Department | Reviewer | Gate |
|------------|----------|------|
| Governance & Compliance | CISO | SCIM SLA < 30s verified |
| Governance & Compliance | IT Admin Representative | SAML role mapping correctness |
| Architecture & Leadership | Security Architect | Remote wipe non-reversible |

### Design Requirements

- SCIM offboarding: < 30s from API call to key revocation
- SAML attribute → role mapping: explicit, auditable, configurable per tenant
- Legal hold: absolute — no operation can delete held data
- Remote wipe: crypto-shred keys, NOT just revoke tokens
- Duress PIN: silent alert, exponential backoff tarpit (TD-012 mitigation)

### Reference Documentation

- `TERA-GOV` — SCIM and OIDC/SAML specification
- `TERA-CLIENT §12.2` — Duress PIN and Hard Wipe mitigation
- `docs/raw/MD/Tech_Debt.md` — TD-012, GAP-F

### System Design Connection

- **Input from:** Task Box 5.1 (OPA engine), Phase 4 (DataGrant revocation)
- **Output to:** Phase 6 (compliance audit, SCIM chaos tests)
- **Connects:** SCIM → OPA → Key Revocation → Audit Log → Legal Hold

---

## Task Box 5.3 — SanitizedPrompt & PII Redaction Pipeline

```yaml
task_id: "PH5-T03"
name: "Implement AI Input Sanitization & Tenant PII Redaction"
status: pending
priority: 🔴 CRITICAL
index: IDX-13-ENCLAVE-AI
platforms: [macOS, Windows, Linux, iPhone, Android, Oppo, Huawei]
estimated_hours: 10
```

### Code Tasks

| # | Task | Source Path | Platform |
|---|------|-------------|----------|
| 1 | `SanitizedPrompt` struct: prompt after PII redaction, with audit trail of removals | `source/core/tc-tapp/src/sanitize.rs` | All |
| 2 | `DomainPiiPolicy`: tenant-specific PII rules (SSN for US, NIF for EU, MyNumber for JP) | `source/core/tc-tapp/src/pii_policy.rs` | All |
| 3 | Micro-NER ONNX model (≤ 1MB, 8MB RAM ceiling) for PII detection | `source/core/tc-tapp/src/ner.rs` | All |
| 4 | ONNX Runtime integration for Desktop/Android, CoreML for iOS/macOS, HiAI for Huawei | `source/bindings/*/` | Per-platform |
| 5 | Redaction pipeline: PII detect → replace with type token → log redaction → forward to AI | `source/core/tc-tapp/src/redact.rs` | All |
| 6 | NO embedding egress: enforce that AI output never contains raw embeddings | `source/core/tc-tapp/src/egress_guard.rs` | All |

### Testing

| # | Test | Tool | Platform | Chaos Ref |
|---|------|------|----------|-----------|
| 1 | Prompt with SSN → NER detects → SanitizedPrompt returns "[SSN_REDACTED]" | `cargo test --test pii_redaction` | All | — |
| 2 | Tenant policy: US tenant → SSN redacted; EU tenant → NIF redacted | `cargo test --test domain_pii` | All | — |
| 3 | ONNX model BLAKE3 mismatch → AI worker terminated → ModelIntegrityViolation audit | `cargo test --test model_integrity` | All | SC-20 |
| 4 | AI output contains raw embedding → egress guard blocks → sanitized output only | `cargo test --test egress_guard` | All | — |
| 5 | ONNX model load: peak RAM < 8MB, inference < 50ms per prompt | `cargo bench --bench ner_perf` | Mobile | — |

### Deployment

ONNX models bundled in app. CoreML models in `.mlmodelc` for Apple. HiAI `.om` for Huawei.

### Request / Review

| Department | Reviewer | Gate |
|------------|----------|------|
| Private AI & Enclave | AI/ML Enclave Lead | PII detection accuracy |
| Governance & Compliance | Compliance Officer | Domain-specific PII completeness |
| Architecture & Leadership | CISO | Veto if embedding egress possible |

### Design Requirements

- PII redaction: BEFORE any data reaches AI model
- NER model: ≤ 1MB ONNX, ≤ 8MB RAM at peak
- Domain policies: per-tenant, configurable, OPA-enforced
- Model integrity: BLAKE3 hash verified on every load
- NO raw embeddings in AI output — egress guard is absolute

### Reference Documentation

- `TERA-ENCLAVE §1.1` — SanitizedPrompt specification
- `TERA-SYNC §3.4` — ZK Memory integration
- `TERA-SYNC §4.5` — PII redaction pipeline
- `TERA-SYNC §8.6` — No embedding egress rule

### System Design Connection

- **Input from:** Task Box 5.1 (OPA for PII policies), Phase 2 (secure storage)
- **Output to:** Task Box 5.4 (ZK Memory Agent)
- **Connects:** SanitizedPrompt → NER Model → PII Policy → AI Model (sanitized only)

---

## Task Box 5.4 — ZK Memory Agent & Private Search

```yaml
task_id: "PH5-T04"
name: "Implement Zero-Knowledge Memory Index & Encrypted Search"
status: pending
priority: 🟠 HIGH
platforms: [macOS, Windows, Linux, iPhone, Android, Oppo, Huawei]
estimated_hours: 8
```

### Code Tasks

| # | Task | Source Path | Platform |
|---|------|-------------|----------|
| 1 | `ZKMemoryIndex`: encrypted local index of user data for private search | `source/core/tc-tapp/src/zk_memory.rs` | All |
| 2 | Private search: query encrypted index → decrypt results locally → display | `source/core/tc-tapp/src/private_search.rs` | All |
| 3 | Local indexing budget: max 50MB per user, max 500K documents | `source/core/tc-tapp/src/index_budget.rs` | Mobile |
| 4 | Index consolidation: merge fragmented indexes online without blocking queries | `source/core/tc-tapp/src/consolidation.rs` | All |
| 5 | ZK Memory cleanup: purge expired indexes per retention policy | `source/core/tc-tapp/src/zk_cleanup.rs` | All |
| 6 | Search limits: max 100 results per query, 10s timeout, no cross-tenant search | `source/core/tc-tapp/src/search_limits.rs` | All |

### Testing

| # | Test | Tool | Platform | Chaos Ref |
|---|------|------|----------|-----------|
| 1 | Search encrypted index → decrypt results locally → verify correctness | `cargo test --test zk_search` | All | — |
| 2 | Index budget exceeded → old indexes evicted by LRU → new data indexed | `cargo test --test index_budget` | Mobile | — |
| 3 | Concurrent query + consolidation → no query blocked > 100ms | `cargo bench --bench zk_consolidation` | All | — |
| 4 | Cross-tenant search attempt → blocked → audit log entry | `cargo test --test cross_tenant_block` | All | — |

### Deployment

ZK Memory Agent runs locally in daemon. No server-side index — all data stays on device.

### Request / Review

| Department | Reviewer | Gate |
|------------|----------|------|
| Private AI & Enclave | Private Search Lead | ZK index security proof |
| Architecture & Leadership | CISO | No cross-tenant search possible |
| Private AI & Enclave | DBA | Index budget enforcement |

### Design Requirements

- Index: encrypted with workspace key, stored in tc-store
- Search: decrypt results client-side ONLY
- Budget: 50MB mobile, 500MB desktop, per user
- Cross-tenant search: blocked at OPA boundary
- Consolidation: online, non-blocking, merge-in-place

### Reference Documentation

- `TERA-ENCLAVE` — Secure Enclave AI specification
- `TERA-SYNC §3.4` — ZK Memory Agent integration
- `TERA-SYNC §4.5` — Private search architecture

### System Design Connection

- **Input from:** Task Box 5.3 (SanitizedPrompt), Phase 2 (tc-store for index)
- **Output to:** Phase 6 (search chaos tests)
- **Connects:** ZKMemoryIndex → tc-store → Private Search API → UI (results)

---

## Task Box 5.5 — On-Prem & Air-Gapped Deployment Models

```yaml
task_id: "PH5-T05"
name: "Define Physical Deployment Topologies"
status: pending
priority: 🟡 MEDIUM
platforms: [Mac Server, Physical Server]
estimated_hours: 6
```

### Code Tasks

| # | Task | Source Path | Platform |
|---|------|-------------|----------|
| 1 | On-Prem single Mac mini: TeraRelay + PostgreSQL + MinIO on bare metal | `server/Mac/` | Mac Server |
| 2 | Air-Gapped topology: Shamir 3-of-5 key ceremony, physical YubiKey HSM | `server/Physical Server/` | Physical Server |
| 3 | Mac mini HA cluster: 2-node Active-Passive, shared NAS, WAL replication, failover | `server/Mac/` | Mac Server |
| 4 | EV signing runner: self-hosted Mac mini + YubiKey FIPS USB for Windows .exe signing | `source/infra/mac-mini/` | Mac Server |
| 5 | Deploy runbook: step-by-step for IT admin, no DevOps background required | `docs/wiki/concepts/` | All |
| 6 | Admin Console: minimal UI for SCIM, audit export, remote wipe, license management | `source/clients/desktop/` | Desktop |

### Testing

| # | Test | Tool | Platform | Chaos Ref |
|---|------|------|----------|-----------|
| 1 | Mac mini HA: kill primary → secondary takes over < 30s → no data loss | Integration test | Mac Server | — |
| 2 | Air-gapped: Shamir key ceremony → 3 of 5 shards → decrypt root key | Manual ceremony test | Physical Server | — |
| 3 | EV signing: trigger build → YubiKey PIN prompt → signed .exe produced | CI pipeline test | Mac Server | — |

### Deployment

Physical hardware deployment. Air-gapped requires on-site ceremony.

### Request / Review

| Department | Reviewer | Gate |
|------------|----------|------|
| Infra, Ops & Quality | SRE | HA failover verified |
| Governance & Compliance | CISO | Air-gapped ceremony protocol |
| Architecture & Leadership | System Architect | Deploy runbook clarity |

### Design Requirements

- On-Prem: single Mac mini, 30-minute deploy target
- Air-Gapped: Shamir 3-of-5, physical ceremony, no network egress
- HA: 2-node Active-Passive, < 30s failover, shared NAS
- Admin Console: SCIM, audit export, remote wipe, license — minimal but complete

### Reference Documentation

- `TERA-CORE §2.4` — Deployment topologies
- `TERA-ENCLAVE` — On-prem AI appliance
- `docs/wiki/concepts/enterprise-license-model.md`
- `docs/raw/MD/Tech_Debt.md` — XPLAT-06, XPLAT-09

### System Design Connection

- **Input from:** Task Box 5.1 (governance), Task Box 5.3 (AI enclave), Phase 1 (daemon)
- **Output to:** Phase 6 (deployment verification, reproducible build)
- **Connects:** Server infra → Relay daemon → Client apps → HSM → Audit
