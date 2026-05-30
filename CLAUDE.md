# CLAUDE.md — TeraChat Engineering Guardrails

```yaml
id: "TERA-CLAUDE"
version: "2.1.0"
date: "2026-05-16"
purpose: "AI-assisted development guardrails — prevent AI from introducing tech debt and security vulnerabilities"
philosophy: "Vertical Slice + Deep Modules + Multi-Agent Harness"
```

## Quick Reference — Core Agent Files

Every agent MUST read these files before writing code:

| Order | File | Purpose |
|-------|------|---------|
| 1 | `AGENT_CONTEXT.md` | Project overview, reading order, current priority |
| 2 | `docs/wiki/ubiquitous-language.md` | Shared vocabulary (EN + VI) |
| 3 | `docs/wiki/invariants.md` | Detailed invariants with code examples |
| 4 | `phase/README.md` | Current slice + timeline |
| 5 | `docs/wiki/concepts/platform-architecture.md` | License tiers, BSL boundary, module diagram |
| 6 | `docs/wiki/concepts/threat-model.md` | STRIDE for 3 attack vectors |
| 7 | `docs/wiki/concepts/hardware-specification.md` | Compute/Storage/AI node hardware tiers |
| 8 | `docs/wiki/concepts/teralink-fallback-network.md` | TeraLink 3-tier fallback network |

---

## Development Philosophy

### Vertical Slice > Horizontal Layer

```
❌ WRONG (Horizontal):
[Crypto Layer] → [Sync Layer] → [Runtime Layer] → [Client Layer] → [AI Layer]
Result: 2 years, nothing runnable

✅ RIGHT (Vertical Slice):
Slice 1: "E2EE message giữa 2 Mac" (6 tuần) → shippable
Slice 2: "iPhone → Mac E2EE" (6 tuần) → shippable
Slice 3: "Relay + TeraLink fallback" (6 tuần) → shippable
Slice 4: ".tapp đầu tiên" (8 tuần) → shippable
Slice 5: "AI summarize" (8 tuần) → shippable
```

Every slice is **shippable** — can demo, can charge, can get feedback.

### Deep Module Principle (Matt Pocock)

Modules with **simple interfaces** (≤ 5 public items) and **complex interiors** (hidden implementation).

```rust
// ✅ CORRECT: Deep module
pub trait InferenceGateway: Send + Sync {
    async fn complete(&self, req: InferenceRequest) -> Result<InferenceResponse>;
    async fn stream(&self, req: InferenceRequest) -> Result<InferenceStream>;
    fn health(&self) -> GatewayHealth;
}
// Interior: ThermalMonitor, InferenceScheduler — all hidden

// ❌ WRONG: Shallow module (exposes internals)
pub struct InferenceGateway {
    pub scheduler: InferenceScheduler,  // internal — should be hidden
    pub thermal: Arc<ThermalMonitor>,    // internal — should be hidden
}
```

**CI enforcement:** Public items > 7 per module triggers refactor warning.

### Multi-Agent Harness

```
Human (Strategic Architect) — architecture + review + customer dev
       │
LangGraph Orchestrator — grooming → TDD → implement → invariant check → security
       │
  ┌────┼────┬────────┐
  │    │    │        │
Rust  Test  Security Doc
Agent Agent Agent    Agent
```

Each agent has **strict file scope** (see AGENT_CONTEXT.md). Agents implement within clear boundaries with test contracts written first. Human reviews architecture + security decisions.

---

## Architectural Invariants (NEVER Violate)

Những điều sau là **không thể thương lượng**. Mỗi invariant được enforce bởi type system, CI gate, hoặc integration test — không phải convention hay comment. Mọi PR vi phạm = auto-reject.

1. **Rust Core is domain owner** — UI (Flutter/SwiftUI/Tauri) is passive renderer only. No business logic in UI layer. **Enforce:** CI gate `business-logic-in-ui` — scans `source/apps/` for crypto/routing/key imports.
2. **ZeroizeOnDrop on ALL key material types** — Mọi struct chứa `SessionKey`, `CompanyKey`, `DeviceIdentityKey`, `MLSPrivateKey` PHẢI derive `ZeroizeOnDrop`. **Enforce:** CI lint `zeroize-verify` — blocks `mem::forget`, `ManuallyDrop`, `Arc<KeyMaterial>`.
3. **No raw pointer in `pub extern "C"`** — Mọi FFI function phải dùng Token Protocol (opaque `u64` token, không raw `*const u8`). **Enforce:** CI lint `ffi-token-protocol`.
4. **Dual-plane sync** — CRDT DAG cho chat (hot_dag.db), Relational cho Finance/HR (cold_state.db). KHÔNG ép Finance data vào CRDT. **Enforce:** CI lint `sync-boundary` — blocks CRDT types in `tc-store` schema.
5. **AI model selection via InferenceScheduler** — Model tier automatically selected based on `ThermalMonitor` + available RAM + network state. **Enforce:** `InferenceScheduler::decide()` returns `ModelTier` enum — no hardcoded model paths.
6. **Headless daemon + gRPC before UI expansion** — Rust Core chạy độc lập với UI process. **Enforce:** Integration test `daemon_independence` — kills UI, verifies Core continues serving.
7. **Test never trails** — SC-34 đến SC-40 là deployment blockers. **Enforce:** CI gate `cargo test --workspace` must pass.
8. **1 subsystem per phase** — Không build 2 subsystem chính cùng lúc. **Enforce:** `phase/README.md` scope; multi-crate PR triggers architect CODEOWNERS review.
9. **iOS election_weight = 0** — iPhone không bao giờ làm Floor Gateway coordinator. **Enforce:** CI gate `ios-election-zero` — integration test verifies `ElectionWeight::zero()` for iOS.
10. **NAS ECC is sole storage authority** — Mac mini (non-ECC RAM) KHÔNG bao giờ là primary DB writer. Chỉ NAS với ECC RAM mới có `StorageAuthority`. **Enforce:** Type system — `StorageAuthority` enum = `NasEcc`; `tc-store` write path chỉ compile cho `aarch64-linux`, không compile cho `aarch64-darwin`.
11. **BLE text only, ≤ 500 bytes** — TeraLink T3 BLE không relay file/media/attachment. **Enforce:** Type system — `BlePayload` là `[u8; 500]` fixed array.
12. **.tapp no external network egress** — Capability `network:external` bị chặn vĩnh viễn tại Host ABI. Không .tapp nào có thể negotiate để có network. **Enforce:** Host ABI permanent block; CI test `tapp-network-sandbox`.
13. **BSL boundary immutable after publish** — LICENSE file được hash trong git tag release. Không thể thay đổi BSL boundary sau khi publish. **Enforce:** CI gate `bsl-boundary-hash` — mỗi `git tag` CI hash `LICENSE` so với `.github/BSL_BOUNDARY.sha256`.

## Forbidden Patterns

Những pattern sau bị CẤM trong toàn bộ codebase:

```rust
// ❌ CẤM: impl Drop manually cho key structs
impl Drop for SessionKey {
    fn drop(&mut self) { ... }  // Dùng ZeroizeOnDrop derive thay vì
}

// ❌ CẤM: unwrap() trong pub functions
pub fn encrypt(key: &Key, data: &[u8]) -> Vec<u8> {
    let secret = key.as_bytes().unwrap();  // Dùng ? hoặc Result
}

// ❌ CẤM: println!() với key material
println!("Session key: {:?}", session_key);  // Tuyệt đối không

// ❌ CẤM: SystemTime::now() cho TTL logic
let ttl_expiry = SystemTime::now() + Duration::from_secs(3600);  // Dùng MonotonicTimeSource

// ❌ CẤM: Raw pointer trong pub extern "C"
pub extern "C" fn encrypt(key: *const u8, len: usize) -> *mut u8;  // Dùng Token Protocol

// ❌ CẤM: Global CAS hash (cross-workspace dedup)
let cas_hash = blake3::hash(chunk);  // Phải dùng workspace_id || salt || chunk

// ❌ CẤM: Wall-clock timeout cho WASM
fn run_tapp(tapp: &Tapp, timeout_ms: u64);  // Dùng fuel metering (instruction count)

// ❌ CẤM: f32/f64 trong financial .tapp code
fn calculate_balance() -> f64;  // Block bởi float-detection CI gate

// ❌ CẤM: Shallow module (> 7 public items)
// Phải refactor thành sub-modules với deep interfaces

// ❌ CẤM: BLE payload > 500 bytes
fn ble_send(payload: Vec<u8>)  // Phải dùng [u8; 500] fixed array

// ❌ CẤM: .tapp manifest khai báo network:external capability
// Host ABI phải chặn vĩnh viễn — không thể negotiate

// ❌ CẤM: Mac mini compute node direct DB write không qua NAS ECC
// tc-store write path chỉ compile cho NAS target (aarch64-linux)
```

## Model Tiers

AI model assignment based on device capability:

| Tier | Model | RAM | Node | Device |
|------|-------|-----|------|--------|
| **Tiny** | Qwen2.5-0.5B (~600MB) | 4GB+ | Compute Node | iPhone 12+ |
| **Small** | Qwen2.5-1.5B (~1.8GB) | 8GB+ | Compute Node | iPhone 15 Pro+ |
| **Medium** | Qwen2.5-7B (~5GB) | 32GB+ | Compute or AI Node | Mac mini M2 |
| **Large** | Qwen2.5-14B (~10GB) | 48GB+ | AI Node | Mac mini M4 Pro |
| **XLarge** | Qwen2.5-32B (~24GB) | 64GB+ | AI Node | Mac mini M4 Max |

Model selection is automatic via `InferenceScheduler` based on `ThermalMonitor` + available RAM + network state.

## iOS TeraLink Constraints

- **Buffer limit:** 32MB (iPhone 12) to 64MB (iPhone 15 Pro)
- **Content:** Text only on Minimal tier, text + voice on Standard
- **Election weight:** Always 0 (never Floor Gateway coordinator)
- **T2 Discovery:** MultipeerConnectivity Framework (not Wi-Fi Direct)
- **Background:** iOS không relay khi màn hình tắt (background BLE restriction)
- **Thermal:** Critical throttling suspends TeraLink sync entirely
- See `docs/wiki/concepts/ios-mesh-storage-tiers.md` for full spec
- See `docs/wiki/concepts/teralink-fallback-network.md` for TeraLink architecture

## Context Files

Mỗi Claude Code session cho TeraChat PHẢI có những file này trong context:

```
AGENT_CONTEXT.md                                # Agent entry point — đọc đầu tiên
CLAUDE.md                                        # Engineering guardrails (file này)
docs/wiki/ubiquitous-language.md                  # Shared vocabulary
docs/wiki/invariants.md                           # Detailed invariants
docs/wiki/concepts/platform-architecture.md       # License tiers + BSL boundary
docs/wiki/concepts/threat-model.md                 # STRIDE for 3 attack vectors
docs/wiki/concepts/hardware-specification.md       # Compute/Storage/AI node hardware
docs/wiki/concepts/teralink-fallback-network.md    # TeraLink 3-tier fallback
docs/raw/MD/Tech_Debt.md                          # Tech debt registry
docs/raw/MD/Spec-Core-Cryptography-And-Mesh.md    # Crypto spec
docs/raw/MD/Spec-Dual-Sync-And-Local-Storage.md    # Sync spec
docs/raw/MD/Spec-Wasm-Tapp-Runtime.md              # WASM runtime spec
docs/wiki/syntheses/gap-resolution-tracker.md      # GAP status
phase/README.md                                    # Slice plan
```

## AI Compatibility Matrix

| Category | AI Can Do | AI Needs Review | Human Required |
|----------|-----------|-----------------|----------------|
| Protobuf schemas | ✅ Generate | Review field numbers | — |
| Rust structs + derive | ✅ Generate | Review trait bounds | — |
| Flutter widget trees | ✅ Generate | Review state binding | — |
| CI/CD YAML | ✅ Generate | Review secrets handling | — |
| SQLite migrations | ✅ Scaffold | Review schema | Complex migrations |
| CRDT merge logic | — | ⚠️ Review edge cases | Conflict resolution |
| OPA Rego policies | — | ⚠️ Review coverage | Policy design |
| gRPC service impl | — | ⚠️ Review error handling | Streaming logic |
| Dart FFI bindings | — | ⚠️ Review memory | FFI boundary |
| WASM Host ABI | — | ⚠️ Review versioning | ABI design |
| **MLS RFC 9420 impl** | ❌ | — | **Cryptographer required** |
| **ZeroizeOnDrop verify** | ❌ | — | **Rust security eng required** |
| **EMDP Protocol** | ❌ | — | **Distributed systems eng required** |
| **iOS NSE + flock()** | ❌ | — | **iOS specialist required** |
| **Fuel metering** | ❌ | — | **WASM specialist required** |
| **CRDT-Finance saga** | ❌ | — | **Distributed systems eng required** |
| **Key material handling** | ❌ | — | **Cryptographer required** |
| **Thermal monitoring** | ⚠️ Generate | ⚠️ Review thresholds | Platform-specific tuning |
| **Inference scheduling** | ⚠️ Generate | ⚠️ Review decision tree | Model tier assignment |
| **Raft consensus** | — | ⚠️ Review edge cases | Leader election + WAL safety |
| **.tapp SDK design** | ⚠️ Generate | ⚠️ Review API surface | Trait design + ABI stability |
| **BSL boundary change** | ❌ | — | **Human architect required** |

## .tapp SDK Community Model

- **Simple interface:** `Tapp` trait with 3 methods (`on_start`, `on_action`, `on_tick`)
- **Deep interior:** WASM engine, fuel metering, sandbox — all hidden from developer
- **Validation gate:** `terachat-tapp validate ./my-tapp.wasm` before submission
- **TDD required:** Contributors must write tests before implementation
- See `CONTRIBUTING.md` for full developer guide

## Code Review Checklist

Mỗi PR phải pass những check sau trước khi merge:

- [ ] `cargo clippy -- -D warnings` — zero warnings
- [ ] `cargo fmt --check` — formatted
- [ ] `cargo audit --deny warnings` — no vulnerable dependencies
- [ ] `gitleaks detect` — no secrets in code
- [ ] `buf lint && buf breaking` — protobuf compatibility
- [ ] Không có `unwrap()` trong `pub fn`
- [ ] Không có `println!()` với key material
- [ ] Không có `SystemTime::now()` cho TTL
- [ ] Không có raw pointer trong `pub extern "C"`
- [ ] Mọi key struct có `ZeroizeOnDrop`
- [ ] UI chỉ render — không có business logic
- [ ] CRDT chỉ cho chat data — không cho Finance
- [ ] Module depth ≤ 7 public items
- [ ] Ubiquitous language terms used correctly
- [ ] Không có BLE payload > 500 bytes (I-11)
- [ ] .tapp manifest không chứa `network:external` capability (I-12)
- [ ] Không có Mac mini direct DB write — chỉ NAS ECC (I-10)
- [ ] LICENSE hash matches `.github/BSL_BOUNDARY.sha256` — release only (I-13)

## Crypto Code — Additional Requirements

Mọi PR chạm vào `tc-crypto/` phải có:

1. **External review** bởi applied cryptographer (sau Phase 1)
2. **`cargo miri test --test ffi_boundary_zeroize`** pass
3. **Property-based test** (`proptest`) cho mọi crypto primitive mới
4. **No self-implemented crypto** — chỉ dùng `ring` crate hoặc `RustCrypto` audited
5. **Constant-time comparison** cho mọi sensitive comparison
6. **ZeroizeOnDrop verified** — không bypass qua `mem::forget`, `ManuallyDrop`, `Arc`

## Pre-Commit Hook Setup

```bash
#!/bin/bash
# .git/hooks/pre-commit
cargo fmt --all -- --check || exit 1
cargo clippy --all-targets -- -D warnings || exit 1
gitleaks detect --source . || exit 1
```

## Dependency Policy

- **Add:** Phải qua `cargo audit` + manual review của Rust lead
- **Remove:** Clean `Cargo.toml` + `Cargo.lock` update
- **Update:** Chạy `cargo test` + `cargo miri test --test ffi_boundary_zeroize`
- **Forbidden:** Không package từ personal GitHub, không unmaintained crates (> 6 tháng không update)
