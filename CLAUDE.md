# CLAUDE.md — TeraChat Engineering Guardrails

```yaml
id: "TERA-CLAUDE"
version: "2.0.0"
date: "2026-05-15"
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
Slice 3: "Relay + mesh failover" (6 tuần) → shippable
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
// Interior: ThermalMonitor, InferenceScheduler, PiiGate — all hidden

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

Những điều sau là **không thể thương lượng**. Mọi PR vi phạm = auto-reject.

1. **Rust Core is domain owner** — UI (Flutter/SwiftUI/Tauri) is passive renderer only. No business logic in UI layer.
2. **ZeroizeOnDrop on ALL key material types** — Mọi struct chứa `Session_Key`, `Company_Key`, `DeviceIdentityKey`, `MLS_PrivateKey` PHẢI derive `ZeroizeOnDrop`.
3. **No raw pointer in `pub extern "C"`** — Mọi FFI function phải dùng Token Protocol (opaque `u64` token, không raw `*const u8`).
4. **Dual-plane sync** — CRDT DAG cho chat (hot_dag.db), Relational cho Finance/HR (cold_state.db). KHÔNG ép Finance data vào CRDT.
5. **AI only after SanitizedPrompt** — Mọi prompt gửi đến AI model PHẢI qua PII redaction. Không embedding egress.
6. **Headless daemon + gRPC before UI expansion** — Rust Core chạy độc lập với UI process.
7. **Test never trails** — SC-34 đến SC-40 là deployment blockers.
8. **1 subsystem per phase** — Không build 2 subsystem chính cùng lúc.
9. **iOS election_weight = 0** — iPhone không bao giờ làm mesh coordinator.

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
```

## Model Tiers

AI model assignment based on device capability:

| Tier | Model | RAM | Device |
|------|-------|-----|--------|
| **Tiny** | Qwen2.5-0.5B (~600MB) | 4GB+ | iPhone 12+ |
| **Small** | Qwen2.5-1.5B (~1.8GB) | 8GB+ | iPhone 15 Pro+ |
| **Medium** | Qwen2.5-7B (~5GB) | 32GB+ | Mac mini M2 |
| **Large** | Qwen2.5-14B (~10GB) | 48GB+ | Mac mini M4 Pro |
| **XLarge** | Qwen2.5-32B (~24GB) | 64GB+ | Mac mini M4 Max |

Model selection is automatic via `InferenceScheduler` based on `ThermalMonitor` + available RAM + network state.

## iOS Mesh Constraints

- **Buffer limit:** 32MB (iPhone 12) to 64MB (iPhone 15 Pro)
- **Content:** Text only on Minimal tier, text + voice on Standard
- **Election weight:** Always 0 (never mesh coordinator)
- **Thermal:** Critical throttling suspends mesh sync entirely
- See `docs/wiki/concepts/ios-mesh-storage-tiers.md` for full spec

## Context Files

Mỗi Claude Code session cho TeraChat PHẢI có những file này trong context:

```
AGENT_CONTEXT.md                                # Agent entry point — đọc đầu tiên
CLAUDE.md                                        # Engineering guardrails (file này)
docs/wiki/ubiquitous-language.md                  # Shared vocabulary
docs/wiki/invariants.md                           # Detailed invariants
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
