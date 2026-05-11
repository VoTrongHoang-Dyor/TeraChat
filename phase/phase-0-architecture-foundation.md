# Phase 0 — Architecture & Design Foundation

```yaml
id: "TERA-PHASE-0"
title: "Architecture Lock & Delivery Skeleton"
duration: "5 days (Day 1–5)"
economic_phase: "Pre-Pilot Foundation"
priority: 🔴 CRITICAL — All subsequent phases depend on locked contracts
teams: [Architecture & Leadership, Core Mesh & Cryptography, Infra Ops & Quality, Client UX]
exit_criteria:
  - ADRs for daemon, gRPC, dual-sync, DataGrant, .tapp runtime
  - Every workstream has spec refs + file scope + acceptance test + owner
  - CI gates active for all incoming PRs
  - Design system signal catalog frozen
```

---

## Task Box 0.1 — Domain Map & Dependency Graph

```yaml
task_id: "PH0-T01"
name: "Lock Domain Boundaries & Dependency Graph"
status: pending
priority: 🔴 CRITICAL
estimated_hours: 8
```

### Code Tasks

| # | Task | Source Path | Output |
|---|------|-------------|--------|
| 1 | Freeze crate boundaries in `source/core/` workspace | `source/core/tc-crypto/`, `tc-mesh/`, `tc-crdt-sync/`, `tc-store/`, `tc-tapp/` | Cargo.toml workspace manifest with `pub use` limited exports |
| 2 | Define `ffi_boundary!` macro contract signature | `source/core/tc-crypto/src/ffi.rs` | Macro that wraps all `extern "C"` + `catch_unwind` |
| 3 | Lock dependency direction: CORE → SYNC → RUNTIME → CLIENT → ECO | All crates | Dependency graph CI lint (banned reverse deps) |
| 4 | Create task index IDX-01 through IDX-14 with scope per crate | `phase/` | Context packs for every AI agent workstream |

### Testing

- [ ] `cargo check --workspace` passes with locked toolchain `1.75.0`
- [ ] Dependency graph CI lint: no reverse deps, no circular deps
- [ ] `ffi_boundary!` macro compiles on all 4 targets (x86_64-linux, aarch64-linux, x86_64-darwin, aarch64-darwin)

### Deployment

- N/A (toolchain lock only)

### Request / Review

| Reviewer | Scope | Gate |
|----------|-------|------|
| System Architect | Crate boundary approval | Block merge if any crate exposes internals |
| Tech Lead | Task index sign-off | All 14 IDX must have spec refs + owners |
| CISO | FFI boundary compliance | `ffi_boundary!` must cover all `extern "C"` |

### Design Requirements

- Each crate has exactly ONE public API module
- Internal modules gated with `pub(crate)` visibility
- No `pub` on any crypto key type

### Reference Documentation

- `TERA-CORE §12.1` — FFI boundary specification
- `docs/wiki/concepts/terachat-architecture-overview.md` — Dependency flow diagram
- `docs/raw/MD/Note.md §1` — Toolchain lock specification

### System Design Connection

- **Input from:** Project charter, TERA-INTRO
- **Output to:** All Phase 1–6 task boxes
- **Blocks:** Everything — this is the foundation

---

## Task Box 0.2 — gRPC/Protobuf Contract Lock

```yaml
task_id: "PH0-T02"
name: "Freeze gRPC Contracts & ABI Versioning"
status: pending
priority: 🔴 CRITICAL
estimated_hours: 8
```

### Code Tasks

| # | Task | Source Path | Output |
|---|------|-------------|--------|
| 1 | Scaffold `proto/` directory with core service definitions | `source/core/proto/` | `terachat.proto` — CoreService, SyncService, MeshService |
| 2 | Define ABI `schema_version` header for all MessagePack payloads | `source/core/tc-tapp/src/abi.rs` | Version enum + negotiation handshake |
| 3 | Generate Rust stubs via `prost` + `tonic` | `source/core/` | Build script output: `terachat.rs` |
| 4 | Define error code enum covering all 7 domains | `source/core/proto/errors.proto` | Standard error codes: CRYPTO_*, SYNC_*, RUNTIME_*, GOV_*, ENCLAVE_*, ECO_*, CLIENT_* |

### Testing

- [ ] Protobuf backward compatibility check (buf breaking)
- [ ] `cargo build` succeeds with generated stubs
- [ ] Error code uniqueness: no duplicate numeric codes across domains

### Deployment

- `buf lint` and `buf breaking` in CI pipeline

### Request / Review

| Reviewer | Scope | Gate |
|----------|-------|------|
| Systems Runtime Lead | Proto service definitions | Must cover all 7 spec domains |
| Client Bridge Team | gRPC vs FFI migration path | Confirm non-breaking for existing IPC |
| Security Architect | Error code surface | No information leakage in error messages |

### Design Requirements

- Proto service names match domain: `CoreService`, `SyncService`, `MeshService`, `RuntimeService`, `GovService`, `EcoService`, `EnclaveService`
- All RPCs carry `TraceId` + `WorkspaceId` in metadata
- Error responses MUST NOT include key material paths

### Reference Documentation

- `TERA-CLIENT §12.3` — gRPC migration spec
- `TERA-RUNTIME §11.4` — Host ABI MessagePack contract
- `docs/raw/MD/Tech_Debt.md` — TD-001 (versioned ABI)

### System Design Connection

- **Input from:** Domain spec map (7 TERA specs)
- **Output to:** Phase 1 (daemon gRPC), Phase 3 (client bridge), Phase 4 (WASM ABI)
- **Connects:** Rust Core → All Clients via gRPC

---

## Task Box 0.3 — CI Baseline Infrastructure

```yaml
task_id: "PH0-T03"
name: "Deploy CI Gates & Automated Build Pipeline"
status: pending
priority: 🔴 CRITICAL
estimated_hours: 8
```

### Code Tasks

| # | Task | Source Path | Output |
|---|------|-------------|--------|
| 1 | CI workflow: `cargo fmt --check`, `cargo clippy --all-features` | `.github/workflows/ci.yml` | Blocker gate on every PR |
| 2 | Security scan gates: `cargo audit`, `gitleaks`, `trivy` | `.github/workflows/security.yml` | Blocker — deny warnings, exit-code 1 |
| 3 | WasmParity CI: `wasm3` vs `wasmtime` semantic equivalence | `.github/workflows/wasm-parity.yml` | Non-blocker initially, blocker by Phase 4 |
| 4 | SBOM generation: `cargo cyclonedx` + `cosign sign-blob` | `.github/workflows/sbom.yml` | CycloneDX 1.5 SBOM per build |
| 5 | Custom lint: `tera_ffi_raw_pointer` clippy plugin | `source/core/tc-crypto/src/lints.rs` | Deny `pub extern "C"` with raw pointers |

### Testing

- [ ] CI catches: fmt violation, clippy warning, audit CVE, hardcoded secret
- [ ] WasmParity: `delta ≤ 20ms`, `mem ≤ 5MB` across wasm3/wasmtime
- [ ] SBOM artifact produced on every merge to main

### Deployment

- GitHub Actions runners: ubuntu-latest, macos-latest, windows-latest
- Self-hosted Mac mini runner for EV signing (Phase 6)

### Request / Review

| Reviewer | Scope | Gate |
|----------|-------|------|
| SRE + SecOps | CI pipeline design | All gates must fail CLOSED (fail secure) |
| Infra Lead | Runner infrastructure | Self-hosted runner for signing |
| Review Agent | Custom lint rules | `tera_ffi_raw_pointer` catches all FFI violations |

### Design Requirements

- CI must complete < 15 min for PR feedback
- Security scan runs on every push, not just PR
- SBOM archived as build artifact (90-day retention)

### Reference Documentation

- `docs/raw/MD/Note.md §1` — Dependencies and toolchain
- `docs/raw/MD/Note.md §9` — Linter and static analysis tools
- `docs/raw/MD/Tech_Debt.md §5.6` — Hermetic Build Forge

### System Design Connection

- **Input from:** Rust toolchain spec (Note.md §1)
- **Output to:** Every phase — gates all code merges
- **Connects:** `.github/workflows/` → every `source/` crate

---

## Task Box 0.4 — Design System & Signal Catalog

```yaml
task_id: "PH0-T04"
name: "Freeze Design System & IPC Signal-to-UI Map"
status: pending
priority: 🟠 HIGH
estimated_hours: 8
```

### Code Tasks

| # | Task | Source Path | Output |
|---|------|-------------|--------|
| 1 | Define `CoreSignal` enum: all IPC signals Rust → UI | `source/core/proto/signals.proto` | Complete signal catalog with priority levels |
| 2 | Define `UICommand` enum: all UI → Rust requests | `source/core/proto/commands.proto` | Pull-based command set |
| 3 | Map each signal to widget state: `PENDING_SECURE_CHANNEL`, `MEMORY_PURGE`, `FCP`, `E2EE_INDICATOR` | `docs/wiki/concepts/glassmorphism-design-system.md` | Signal-to-state lookup table |
| 4 | Define Glassmorphism modes: Light, Dark, High Contrast, Security Overlay | Design system spec | CSS/token variables for all 4 modes |
| 5 | Define GPU tier ladder: Tier A (full) → Tier C (fallback) | Design system spec | `ui_emergency_mode` integration |

### Testing

- [ ] Every `CoreSignal` variant has exactly one widget state mapping
- [ ] Every widget state has valid UI render for all 4 visual modes
- [ ] GPU tier downgrade path: Tier A → B → C without visual breakage

### Deployment

- Design tokens as JSON → consumed by Flutter Theme + Tauri CSS variables

### Request / Review

| Reviewer | Scope | Gate |
|----------|-------|------|
| Product UI Lead | Signal catalog completeness | All 7 domains must have UI signals |
| Design Lead | Visual mode consistency | Glassmorphism fidelity across modes |
| Security Architect | Security overlay accuracy | UI must not LIE about security state |

### Design Requirements

- UI is passive renderer — NEVER owns business logic
- Security state indicators must be non-spoofable (data from Rust Core, not UI)
- Glassmorphism blur/transparency: GPU Tier A = 20px blur, Tier B = 10px, Tier C = solid fallback

### Reference Documentation

- `TERA-CLIENT §11.3–11.5` — Widget states and security priority channel
- `docs/HTML/Design.html` — Glassmorphism design contract
- `docs/wiki/concepts/glassmorphism-design-system.md`

### System Design Connection

- **Input from:** TERA-CLIENT spec
- **Output to:** Phase 3 (security-visible UI), Phase 4 (UX rendering)
- **Connects:** Rust Core → IPC Signals → Flutter/Tauri Renderer

---

## Task Box 0.5 — ADR Freeze & Phase Exit Checklist

```yaml
task_id: "PH0-T05"
name: "Publish Architecture Decision Records & Phase 1 Entry Gate"
status: pending
priority: 🔴 CRITICAL
estimated_hours: 4
```

### Code Tasks

| # | Task | Source Path | Output |
|---|------|-------------|--------|
| 1 | Write ADR-001: Headless Daemon Architecture | `docs/wiki/concepts/` | Decision: Rust Core as daemon, UI as client |
| 2 | Write ADR-002: Dual-Plane Sync (CRDT + Relational) | `docs/wiki/concepts/` | Decision: Chat → CRDT DAG, Finance/HR → Relational |
| 3 | Write ADR-003: gRPC over FFI for IPC | `docs/wiki/concepts/` | Decision: Unified gRPC bridge replaces fragmented FFI |
| 4 | Write ADR-004: WASM Dual-Engine (wasmtime + wasm3) | `docs/wiki/concepts/` | Decision: wasmtime for desktop, wasm3 for iOS W^X |
| 5 | Write ADR-005: DataGrant Quorum Protocol | `docs/wiki/concepts/` | Decision: Majority quorum for Gov-tier DataGrant activation |
| 6 | Write branch strategy: agent worktrees, merge gates | `.agents/rules/` | AI agent branching contract |

### Testing

- N/A (documentation)

### Deployment

- ADRs committed to `docs/wiki/concepts/` for LLM context embedding

### Request / Review

| Reviewer | Scope | Gate |
|----------|-------|------|
| System Architect | All 5 ADRs | Sign-off each ADR |
| Engineering Manager | Branch strategy | Agent worktree contract |
| CISO | ADR-005 (DataGrant) | Veto if quorum undefined |

### Design Requirements

- ADR format: Title, Status, Context, Decision, Consequences
- Branch strategy: 1 agent = 1 worktree = 1 bounded scope
- Merge gate: Review Agent sign-off + CI green + spec compliance check

### Reference Documentation

- All 7 TERA domain specs
- `docs/raw/MD/Tech_Debt.md` — All CRITICAL debt items
- `phase/terachat-ai-agent-phase-plan.md` — Original 35-day plan

### System Design Connection

- **Input from:** Phase 0 task boxes 0.1–0.4
- **Output to:** Phase 1 entry gate
- **Blocks:** Phase 1 start (no Phase 1 work before ADRs frozen)
