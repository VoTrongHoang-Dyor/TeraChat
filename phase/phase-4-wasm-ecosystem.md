# Phase 4 — WASM Runtime & .tapp Ecosystem

```yaml
id: "TERA-PHASE-4"
title: "WASM Runtime, .tapp Contracts & Ecosystem Safety"
duration: "5 days (Day 21–25)"
economic_phase: "Phase 2 — 'Enough to Renew and Upsell'"
priority: 🟠 HIGH — Runtime must be deterministic before .tapp marketplace opens
teams: [Governance & Ecosystem Team, Client Bridge Team]
debt_targets: [TD-001, TD-003, TD-004]
exit_criteria:
  - .tapp runtime has deterministic execution (fuel metering, not wall-clock)
  - Registry/MDM/kill-switch path has complete trust chain
  - DataGrant prefetch, revocation gossip, Gov-tier quorum all functional
```

## System Design: Phase 4 Connections

```
┌──────────────────────────────────────────────────┐
│  PHASE 4 BUILDS                                   │
│                                                   │
│  Phase 2 (Sync) ─────────────────┐                │
│  Phase 3 (gRPC Bridge) ──────────┤                │
│                                    ↓               │
│  tc-tapp (WASM Runtime)                              │
│  ├─ MessagePack schema_version                      │
│  ├─ Host ABI (cursor, error codes)                  │
│  ├─ Fuel Metering (instruction_fuel)                │
│  ├─ Circuit Breaker (per-endpoint)                  │
│  └─ wasmtime (Desktop) / wasm3 (iOS)                │
│                                    ↓               │
│  Ecosystem                                            │
│  ├─ Registry + Review Pipeline                      │
│  ├─ App Signing PKI (Ed25519)                       │
│  ├─ MDM Distribution                                 │
│  ├─ Kill-Switch Path                                │
│  └─ Transparency Log                                │
│                                    ↓               │
│  Governance                                           │
│  ├─ DataGrant Quorum (Hash_Frontier gossip)         │
│  ├─ Revocation Gossip                               │
│  └─ License Validator                               │
│                                    ↓               │
│                              Phase 5 (AI Enclave)    │
└──────────────────────────────────────────────────┘
```

---

## Task Box 4.1 — Host ABI Versioning + Cursor Protocol

```yaml
task_id: "PH4-T01"
name: "Versioned MessagePack ABI & Cursor Protocol"
status: pending
priority: 🟠 HIGH
debt: TD-001
index: IDX-09-RUNTIME-ABI
platforms: [macOS, Windows, Linux, iPhone, Android, Oppo, Huawei]
estimated_hours: 10
```

### Code Tasks

| # | Task | Source Path | Platform |
|---|------|-------------|----------|
| 1 | Add `schema_version` field to all MessagePack IPC payloads | `source/core/tc-tapp/src/abi.rs` | All |
| 2 | Version negotiation: Host advertises supported versions → .tapp selects | `source/core/tc-tapp/src/version.rs` | All |
| 3 | Auto-reject mismatched versions at FFI boundary | `source/core/tc-tapp/src/abi.rs` | All |
| 4 | Host ABI cursor protocol: paginated data access (limit, cursor, total) | `source/core/tc-tapp/src/cursor.rs` | All |
| 5 | Standard ABI error codes: `ABI_OK`, `ABI_VERSION_MISMATCH`, `ABI_QUOTA_EXCEEDED`, `ABI_INVALID_CURSOR` | `source/core/tc-tapp/src/errors.rs` | All |
| 6 | First-party exemption path: `first_party: true` flag skips version negotiation | `source/core/tc-tapp/src/manifest.rs` | All |

### Testing

| # | Test | Tool | Platform | Chaos Ref |
|---|------|------|----------|-----------|
| 1 | Old .tapp (v1) connects to new Host (v2) → rejected with ABI_VERSION_MISMATCH | `cargo test --test abi_version` | All | — |
| 2 | Cursor pagination: 10k records → cursor walk returns all without duplication | `cargo test --test cursor_protocol` | All | — |
| 3 | First-party .tapp skips version check → connects successfully on latest ABI | `cargo test --test first_party_abi` | All | — |
| 4 | WasmParity: same .tapp on wasm3 (iOS) vs wasmtime (Desktop) → identical output | `cargo test --test wasm_parity` | All | — |

### Deployment

ABI version lock in Rust Core. .tapp publishers get deprecation warning at 12 months.

### Request / Review

| Department | Reviewer | Gate |
|------------|----------|------|
| Governance & Ecosystem | Systems/WASM Runtime Lead | ABI completeness |
| Architecture & Leadership | Tech Lead | First-party exemption justified |
| Governance & Ecosystem | Review Lead | Backward compatibility plan |

### Design Requirements

- `schema_version`: u32, increment on breaking ABI change
- Cursor protocol: `limit` max 1000, opaque `cursor` token, `total` for progress
- First-party exemption: only for TeraChat official .tapps, CI verifies consistency
- Deprecation window: 12 months for third-party, immediate for security patches

### Reference Documentation

- `TERA-RUNTIME §11.4` — MessagePack schema version
- `TERA-RUNTIME §11.6` — Cursor protocol
- `docs/raw/MD/Tech_Debt.md` — TD-001, TD-004

### System Design Connection

- **Input from:** Phase 0 (ABI contracts), Phase 2 (data access via cursors)
- **Output to:** Phase 5 (governance .tapp), Phase 6 (WasmParity CI)
- **Connects:** Host ABI → .tapp WASM → gRPC → tc-store (data)

---

## Task Box 4.2 — Fuel Metering & Deterministic Execution

```yaml
task_id: "PH4-T02"
name: "Implement Gas/Fuel Metering for Deterministic WASM"
status: pending
priority: 🟠 HIGH
debt: TD-003
index: IDX-10-RUNTIME-FUEL
platforms: [macOS, Windows, Linux, iPhone, Android, Oppo, Huawei]
estimated_hours: 10
```

### Code Tasks

| # | Task | Source Path | Platform |
|---|------|-------------|----------|
| 1 | `instruction_fuel` metering: fixed fuel budget per .tapp execution | `source/core/tc-tapp/src/fuel.rs` | All |
| 2 | Fuel accounting: every WASM instruction decrements fuel counter | `source/core/tc-tapp/src/fuel.rs` | All |
| 3 | Fuel exhaustion → graceful termination with `ABI_FUEL_EXHAUSTED` | `source/core/tc-tapp/src/fuel.rs` | All |
| 4 | Fixed-point arithmetic enforcement: no `f32`/`f64` in financial .tapp | `source/core/tc-tapp/src/float_detect.rs` | All |
| 5 | Per-endpoint circuit breaker: max concurrent calls, timeout, quota | `source/core/tc-tapp/src/circuit_breaker.rs` | All |
| 6 | Webhook ACK contract: 10s timeout, exponential backoff retry | `source/core/tc-tapp/src/webhook.rs` | All |

### Testing

| # | Test | Tool | Platform | Chaos Ref |
|---|------|------|----------|-----------|
| 1 | .tapp exhausts fuel → terminated, not timed out | `cargo test --test fuel_exhaust` | All | — |
| 2 | Same .tapp on wasm3 (slow) vs wasmtime (fast) → same fuel exhaustion point | `cargo test --test fuel_parity` | All | — |
| 3 | Finance .tapp uses `f64` → LLVM IR analysis catches → merge blocked | CI gate | All | GAP-H |
| 4 | Circuit breaker: 1000 concurrent calls → 101st rejected with quota error | `cargo test --test circuit_breaker` | All | — |
| 5 | Webhook times out → ACK not sent → retry with backoff → eventual delivery | `cargo test --test webhook_ack` | All | — |

### Deployment

Fuel metering bundled in WASM runtime. Circuit breaker thresholds configurable per deployment tier.

### Request / Review

| Department | Reviewer | Gate |
|------------|----------|------|
| Governance & Ecosystem | Systems/WASM Runtime Lead | Deterministic execution proof |
| Governance & Ecosystem | Review Lead | Financial .tapp float detection |
| Architecture & Leadership | Security Architect | No resource exhaustion vectors |

### Design Requirements

- Fuel budget: per .tapp, defined in manifest, enforced by runtime
- Wall clock independence: fuel exhausted = stopped, regardless of hardware speed
- Financial .tapp: MUST use fixed-point (i64), `f32`/`f64` = CI blocker
- Circuit breaker: per-endpoint, configurable `max_concurrent` and `timeout_ms`

### Reference Documentation

- `TERA-RUNTIME §11.3` — Fuel/gas metering
- `TERA-RUNTIME §11.5` — Fixed-point enforcement
- `TERA-RUNTIME §11.8` — Webhook contract
- `TERA-ECO §4.1` — Float detection checklist
- `docs/raw/MD/Tech_Debt.md` — TD-003, GAP-H

### System Design Connection

- **Input from:** Task Box 4.1 (ABI), Task Box 1.5 (thermal → fuel reduction)
- **Output to:** Phase 5 (secure WASM for governance .tapps)
- **Connects:** tc-tapp → wasmtime/wasm3 → fuel counter → circuit breaker

---

## Task Box 4.3 — Registry, Signing PKI & Kill-Switch

```yaml
task_id: "PH4-T03"
name: "Implement Ecosystem Trust Chain & Supply Chain Safety"
status: pending
priority: 🟠 HIGH
index: IDX-12-ECO-TRUST
platforms: [macOS, Windows, Linux, iPhone, Android, Oppo, Huawei]
estimated_hours: 10
```

### Code Tasks

| # | Task | Source Path | Platform |
|---|------|-------------|----------|
| 1 | App signing PKI: Ed25519 publisher key → TeraChat Root CA → signature verification | `source/core/tc-tapp/src/signing.rs` | All |
| 2 | Registry: .tapp manifest, version, BLAKE3 hash, publisher identity | `source/core/tc-tapp/src/registry.rs` | All |
| 3 | Review pipeline: security scan, capability declaration, float detection, OPA policy check | `.agents/commands/review.md` | All |
| 4 | Transparency log: append-only Merkle tree of all published .tapp versions | `source/core/tc-tapp/src/transparency.rs` | All |
| 5 | Kill-switch: emergency revoke by Admin → all instances suspended within 60s | `source/core/tc-tapp/src/kill_switch.rs` | All |
| 6 | EV Code Signing pathway for Windows .tapp binaries | `source/infra/` | Windows |
| 7 | MDM Distribution: enterprise push .tapp to fleet devices | `source/core/tc-tapp/src/mdm.rs` | All |

### Testing

| # | Test | Tool | Platform | Chaos Ref |
|---|------|------|----------|-----------|
| 1 | Unsigned .tapp → rejected at install | `cargo test --test signing_check` | All | — |
| 2 | Publisher key revoked → installed .tapp suspended on next policy check | `cargo test --test key_revocation` | All | — |
| 3 | Kill-switch triggered → all instances suspended < 60s | `cargo test --test kill_switch` | All | — |
| 4 | Transparency log: verify Merkle consistency proof | `cargo test --test transparency_log` | All | — |
| 5 | Binary transparency gossip signed → unsigned gossip rejected | `cargo test --test gossip_sign` | All | GAP-I |

### Deployment

Registry hosted on TeraRelay server. PKI keys in HSM. MDM via enterprise provisioning profiles.

### Request / Review

| Department | Reviewer | Gate |
|------------|----------|------|
| Governance & Ecosystem | Platform Engineer | Registry + PKI completeness |
| Architecture & Leadership | CISO | Kill-switch security review |
| Governance & Ecosystem | Review Lead | Signed gossip verification |

### Design Requirements

- Publisher key: Ed25519, registered with TeraChat Root CA
- Registry: append-only, immutable versions (no overwrite)
- Kill-switch: Admin-initiated, 60s propagation deadline, queue pending ops
- Transparency log: every version published → Merkle leaf
- Gossip messages: MUST be signed (GAP-I fix)

### Reference Documentation

- `TERA-ECO §2–4` — App signing, registry, kill-switch
- `TERA-ECO §8.8–8.9` — EV signing, transparency log
- `docs/raw/MD/Tech_Debt.md` — GAP-I

### System Design Connection

- **Input from:** Task Box 4.2 (deterministic runtime for review), Phase 0 (CI gates)
- **Output to:** Phase 5 (governance .tapp enforcement)
- **Connects:** Registry → Signing PKI → Transparency Log → MDM → Kill-Switch

---

## Task Box 4.4 — DataGrant Quorum & Revocation Gossip

```yaml
task_id: "PH4-T04"
name: "Implement DataGrant Orchestration with Gov-Tier Quorum"
status: pending
priority: 🔴 CRITICAL
debt: GAP-E
index: IDX-11-GOV-DATAGRANT
platforms: [macOS, Windows, Linux, iPhone, Android, Oppo, Huawei]
estimated_hours: 8
```

### Code Tasks

| # | Task | Source Path | Platform |
|---|------|-------------|----------|
| 1 | DataGrant `generation` counter: distinguish "never seen" (gen 0) vs "revoked" (gen > 0) | `source/core/tc-tapp/src/data_grant.rs` | All |
| 2 | Gov-tier quorum protocol: majority of `election_weight > 0` nodes confirm via `Hash_Frontier` gossip | `source/core/tc-tapp/src/quorum.rs` | All |
| 3 | Unconfirmed grants: return `PENDING_QUORUM`, do not serve data | `source/core/tc-tapp/src/data_grant.rs` | All |
| 4 | Revocation gossip: propagate `DataGrantRevoked` via Hash_Frontier to all peers | `source/core/tc-tapp/src/revocation.rs` | All |
| 5 | DataGrant prefetch: anticipate grant needs based on user context | `source/core/tc-tapp/src/prefetch.rs` | All |

### Testing

| # | Test | Tool | Platform | Chaos Ref |
|---|------|------|----------|-----------|
| 1 | Grant issued while node offline → gen 0 → PENDING_QUORUM → no data served | `cargo test --test grant_offline` | All | — |
| 2 | Quorum achieved → grant activated → data accessible | `cargo test --test quorum_activate` | All | — |
| 3 | Revocation propagated during partition → grant revoked on rejoin | `cargo test --test revoke_partition` | All | — |
| 4 | Gov-tier: 3 of 5 quorum → grant active; 2 of 5 → PENDING_QUORUM | `cargo test --test gov_quorum` | All | — |

### Deployment

DataGrant logic in Rust Core. Quorum parameters configurable per deployment tier.

### Request / Review

| Department | Reviewer | Gate |
|------------|----------|------|
| Governance & Ecosystem | CISO | Quorum protocol security |
| Arch & Leadership | Security Architect | No data served without quorum |
| Governance & Ecosystem | Platform Engineer | Revocation propagation latency |

### Design Requirements

- Grant `generation` counter: monotonic, per-workspace
- Gov-tier quorum: majority of weighted nodes (election_weight > 0)
- Unconfirmed = `PENDING_QUORUM` = NO data served
- Revocation propagation: gossip to all peers, immediate local enforcement

### Reference Documentation

- `TERA-ECO §8.3` — DataGrant quorum protocol
- `TERA-ECO §2.5` — generation counter
- `TERA-GOV` — OPA integration
- `docs/raw/MD/Tech_Debt.md` — GAP-E

### System Design Connection

- **Input from:** Task Box 4.3 (registry PKI for peer identity), Phase 2 (sync for gossip)
- **Output to:** Phase 5 (OPA policy enforcement), Phase 6 (partition chaos tests)
- **Connects:** DataGrant → Quorum → Hash_Frontier → tc-mesh (gossip transport)
