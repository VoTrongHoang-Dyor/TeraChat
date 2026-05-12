# Task Index — AI Agent Workstream Context Packs

```yaml
id: "TERA-IDX"
version: "1.0.0"
date: "2026-05-12"
purpose: "Every AI agent workstream receives exactly one context pack with spec refs, file scope, acceptance test, and owner"
```

---

## IDX-01 — Cryptographic Core (tc-crypto)

| Thuộc tính | Giá trị |
|------------|--------|
| **Crate** | `tc-crypto` |
| **Path** | `source/core/tc-crypto/` |
| **Spec** | TERA-CORE (Spec-Core-Cryptography-And-Mesh.md) |
| **Owner** | Applied Cryptographer |
| **Acceptance** | `cargo miri test --test ffi_boundary_zeroize` pass; ZeroizeOnDrop verified on all key types |
| **Blocked by** | None |
| **Blocks** | IDX-02, IDX-03, IDX-04, IDX-05 |

---

## IDX-02 — Mesh Networking (tc-mesh)

| Thuộc tính | Giá trị |
|------------|--------|
| **Crate** | `tc-mesh` |
| **Path** | `source/core/tc-mesh/` |
| **Spec** | TERA-CORE §6-11 (Survival Mesh, BLE, EMDP) |
| **Owner** | Distributed Systems Engineer |
| **Acceptance** | SC-38: BLE 100kbps cap + 250ms RTT → P0 delivered < 2s |
| **Blocked by** | IDX-01 (tc-crypto) |
| **Blocks** | None (leaf crate) |

---

## IDX-03 — CRDT DAG Sync (tc-crdt-sync)

| Thuộc tính | Giá trị |
|------------|--------|
| **Crate** | `tc-crdt-sync` |
| **Path** | `source/core/tc-crdt-sync/` |
| **Spec** | TERA-SYNC (Spec-Dual-Sync-And-Local-Storage.md) |
| **Owner** | Distributed Systems Engineer |
| **Acceptance** | SC-01: Internet partition 30min → full recovery < 120s, 0 data loss |
| **Blocked by** | IDX-01 (tc-crypto) |
| **Blocks** | IDX-04 (tc-store handles dual-plane storage) |

---

## IDX-04 — Storage Layer (tc-store)

| Thuộc tính | Giá trị |
|------------|--------|
| **Crate** | `tc-store` |
| **Path** | `source/core/tc-store/` |
| **Spec** | TERA-SYNC §3-8 (SQLite WAL, SQLCipher, CAS VFS) |
| **Owner** | Systems Engineer |
| **Acceptance** | `PRAGMA integrity_check` pass on both databases after crash recovery |
| **Blocked by** | IDX-01 (tc-crypto) |
| **Blocks** | IDX-05 (tc-tapp stores transient state) |

---

## IDX-05 — WASM .tapp Runtime (tc-tapp)

| Thuộc tính | Giá trị |
|------------|--------|
| **Crate** | `tc-tapp` |
| **Path** | `source/core/tc-tapp/` |
| **Spec** | TERA-RUNTIME (Spec-Wasm-Tapp-Runtime.md) |
| **Owner** | WASM/Systems Engineer |
| **Acceptance** | WasmParity CI: wasm3 ≡ wasmtime, delta ≤ 20ms; fuel metering deterministic |
| **Blocked by** | IDX-01 (tc-crypto), IDX-04 (tc-store) |
| **Blocks** | IDX-09 (.tapp marketplace) |

---

## IDX-06 — Protobuf & gRPC Contracts (tc-proto)

| Thuộc tính | Giá trị |
|------------|--------|
| **Crate** | `tc-proto` |
| **Path** | `source/core/tc-proto/`, `source/core/proto/` |
| **Spec** | TERA-CLIENT §12.3 (gRPC migration), TERA-RUNTIME §11.4 (Host ABI) |
| **Owner** | Systems Architect |
| **Acceptance** | `buf lint` + `buf breaking` pass; all 7 service domains defined |
| **Blocked by** | None |
| **Blocks** | IDX-07 (IPC bridge), IDX-08 (client integration) |

---

## IDX-07 — FFI & IPC Bridge (Bindings)

| Thuộc tính | Giá trị |
|------------|--------|
| **Crate** | `tc-bindings` (future: `source/bindings/`) |
| **Path** | `source/bindings/uniffi-apple/`, `source/bindings/uniffi-kotlin/` |
| **Spec** | TERA-CLIENT (Spec-Client-IPC-And-UI-Bridge.md) |
| **Owner** | Client Bridge Team |
| **Acceptance** | `ffi_boundary!` covers all `extern "C"`; Token Protocol used for all handles |
| **Blocked by** | IDX-06 (proto contracts) |
| **Blocks** | IDX-08 (UI can receive signals) |

---

## IDX-08 — macOS/iOS Client (Apple)

| Thuộc tính | Giá trị |
|------------|--------|
| **Crate** | N/A (SwiftUI + Flutter) |
| **Path** | `source/apps/Laptop/macOS/`, `source/apps/Phone/Iphone/` |
| **Spec** | TERA-CLIENT, Design.html |
| **Owner** | Product UI Lead |
| **Acceptance** | All CoreSignals render correctly in all 4 visual modes; GPU tier fallback works |
| **Blocked by** | IDX-07 (IPC bridge) |
| **Blocks** | None (first client platforms) |

---

## IDX-09 — .tapp Marketplace & Trust Chain

| Thuộc tính | Giá trị |
|------------|--------|
| **Crate** | N/A (web service + tc-tapp integration) |
| **Path** | `terachat.io/web/marketplace/` |
| **Spec** | TERA-ECO (Spec-Ecosystem-And-Trust-Chain.md) |
| **Owner** | Ecosystem Lead |
| **Acceptance** | ≥ 5 vetted .tapps; self-service deploy < 10min |
| **Blocked by** | IDX-05 (tc-tapp) |
| **Blocks** | None |

---

## IDX-10 — Enterprise Identity & Governance

| Thuộc tính | Giá trị |
|------------|--------|
| **Crate** | N/A (integrated into TeraRelay + Admin Console) |
| **Path** | `source/server/` |
| **Spec** | TERA-GOV (Spec-Identity-And-Governance.md) |
| **Owner** | CISO / Governance Lead |
| **Acceptance** | SCIM offboarding < 30s; OPA policy evaluation < 5ms |
| **Blocked by** | IDX-06 (proto contracts) |
| **Blocks** | IDX-13 (Gov/Military certification) |

---

## IDX-11 — AI Enclave & PII Redaction

| Thuộc tính | Giá trị |
|------------|--------|
| **Crate** | Future: `tc-enclave` |
| **Path** | `source/core/tc-enclave/` (Phase 2D) |
| **Spec** | TERA-ENCLAVE (Spec-Enterprise-Secure-Enclave.md) |
| **Owner** | ML/Enclave Lead |
| **Acceptance** | PII redaction: 0 false negatives; inference < 2s with prompt ≤ 2000 tokens |
| **Blocked by** | IDX-05 (tc-tapp for AI Host ABI) |
| **Blocks** | IDX-12 (Open AI Framework) |

---

## IDX-12 — Open AI Framework

| Thuộc tính | Giá trị |
|------------|--------|
| **Crate** | Future: extends `tc-enclave` |
| **Path** | `source/core/tc-enclave/src/open_framework/` (Phase 3A) |
| **Spec** | TERA-ENCLAVE §8-12 |
| **Owner** | ML/Enclave Lead |
| **Acceptance** | ≥ 5 enterprise custom models registered; model load < 5s |
| **Blocked by** | IDX-11 (AI Enclave baseline) |
| **Blocks** | None |

---

## IDX-13 — Gov/Military Certification

| Thuộc tính | Giá trị |
|------------|--------|
| **Crate** | N/A (compliance + air-gapped deployment) |
| **Path** | `compliance/iso27001/`, `deploy/air-gapped/` |
| **Spec** | TERA-GOV §9-14, TERA-CORE §12 |
| **Owner** | CISO |
| **Acceptance** | ISO 27001 certified; ≥ 1 Gov contract signed; all SC-01..40 pass |
| **Blocked by** | IDX-10 (governance baseline) |
| **Blocks** | None (final phase) |

---

## IDX-14 — CI/CD & Infra Quality

| Thuộc tính | Giá trị |
|------------|--------|
| **Crate** | N/A |
| **Path** | `.github/workflows/`, `source/core/rust-toolchain.toml` |
| **Spec** | Note.md, Tech_Debt.md §5.6 |
| **Owner** | SRE + SecOps |
| **Acceptance** | All CI gates pass; SBOM generated; hermetic build verified |
| **Blocked by** | None |
| **Blocks** | Everything (gates all merges) |

---

## Dependency Flow

```
IDX-01 (tc-crypto) ─┬─→ IDX-02 (tc-mesh)
                    ├─→ IDX-03 (tc-crdt-sync)
                    ├─→ IDX-04 (tc-store) ──→ IDX-05 (tc-tapp) ──→ IDX-09 (marketplace)
                    │                                       ──→ IDX-11 (AI enclave) ──→ IDX-12 (Open AI)
                    └─→ IDX-06 (tc-proto) ──→ IDX-07 (IPC bridge) ──→ IDX-08 (Apple clients)

IDX-14 (CI/CD) ──→ Gates ALL merges

IDX-10 (Governance) ──→ IDX-13 (Gov/Military)
```

---

*TERA-IDX v1.0.0 · 2026-05-12 · Created during Phase 0 execution*
