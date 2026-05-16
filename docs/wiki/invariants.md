---
type: concept
created: 2026-05-15
updated: 2026-05-15
tags: [invariants, security, agent-context]
sources: [CLAUDE.md]
---

# Architectural Invariants

These are **non-negotiable.** Any PR violating them = auto-reject.

## 1. Rust Core is Domain Owner

```
UI (Flutter/SwiftUI/Tauri) → Passive renderer only
Rust Core                → ALL business logic, crypto, sync, AI
```

UI layer must NEVER contain:
- Crypto operations
- Message routing logic
- Sync decisions
- AI prompt construction
- Key management

## 2. ZeroizeOnDrop on ALL Key Material Types

Every struct containing `SessionKey`, `CompanyKey`, `DeviceIdentityKey`, `MLSPrivateKey` MUST derive `ZeroizeOnDrop`.

```rust
// ✅ CORRECT
#[derive(ZeroizeOnDrop)]
pub struct SessionKey { ... }

// ❌ FORBIDDEN: Manual Drop impl
impl Drop for SessionKey {
    fn drop(&mut self) { ... }
}
```

No bypass via `mem::forget`, `ManuallyDrop`, or `Arc`.

## 3. No Raw Pointer in `pub extern "C"`

All FFI functions must use Token Protocol (opaque `u64`), never raw `*const u8`.

```rust
// ✅ CORRECT
pub extern "C" fn encrypt(key_token: u64, data: *const u8, len: usize) -> FfiResult;

// ❌ FORBIDDEN
pub extern "C" fn encrypt(key: *const u8, len: usize) -> *mut u8;
```

## 4. Dual-Plane Sync

```
hot_dag.db   → CRDT DAG for chat (append-only)
cold_state.db → Relational for Finance/HR
```

NEVER force Finance data into CRDT. CRDT is for chat only.

## 5. AI Only After SanitizedPrompt

Every prompt sent to an AI model MUST pass through PII redaction. No embedding egress without redaction.

```rust
// ✅ CORRECT
let sanitized = pii_gate.redact(raw_prompt)?;
let response = inference.complete(sanitized).await?;

// ❌ FORBIDDEN
let response = inference.complete(raw_prompt).await?;
```

## 6. Headless Daemon + gRPC Before UI Expansion

Rust Core runs as an independent process. UI connects via gRPC over UDS. No UI-specific code in Core.

## 7. Test Never Trails

SC-34 through SC-40 are deployment blockers. No phase ships with failing tests.

## 8. 1 Subsystem per Phase (Vertical Slice)

Progressive complexity — one vertical slice at a time. Never build two major subsystems concurrently.

---

## Forbidden Patterns

```rust
// ❌ FORBIDDEN: unwrap() in pub functions
pub fn encrypt(key: &Key, data: &[u8]) -> Vec<u8> {
    let secret = key.as_bytes().unwrap();  // Use ? or Result propagation
}

// ❌ FORBIDDEN: println!() with key material
println!("Session key: {:?}", session_key);   // Never log sensitive data
tracing::info!("Key: {:?}", key_bytes);       // Even tracing is forbidden

// ❌ FORBIDDEN: SystemTime::now() for TTL logic
let ttl_expiry = SystemTime::now() + Duration::from_secs(3600);  // Use monotonic clock

// ❌ FORBIDDEN: Raw pointer in pub extern "C"
pub extern "C" fn encrypt(key: *const u8, len: usize) -> *mut u8;  // Use Token Protocol

// ❌ FORBIDDEN: Global CAS hash (cross-workspace dedup)
let cas_hash = blake3::hash(chunk);  // Must use workspace_id || salt || chunk

// ❌ FORBIDDEN: Wall-clock timeout for WASM
fn run_tapp(tapp: &Tapp, timeout_ms: u64);  // Use fuel metering (instruction count)

// ❌ FORBIDDEN: f32/f64 in financial .tapp code
fn calculate_balance() -> f64;  // Blocked by CI float-detection gate
```

---

## Crypto Stack (Decided — Do Not Re-litigate)

| Function | Crate | Status |
|----------|-------|--------|
| E2EE Messaging | `openmls` (MLS RFC 9420) | Decided |
| Symmetric Encryption | `ring::aead::AES_256_GCM` | Decided |
| Digital Signatures | `ring::signature::Ed25519` | Decided |
| Hashing | `blake3` | Decided |
| Post-Quantum KEM | `ml-kem` (NIST FIPS 203) | Phase 2 |
| Signal Protocol | NOT used | Decided — MLS is the path |

No self-implemented crypto. Only `ring` crate or `RustCrypto` audited implementations.

---

## Deep Module Principle (Matt Pocock)

Every module must follow:
- **Simple interface:** ≤ 5 public functions/types
- **Complex interior:** Implementation details hidden
- **CI enforced:** `pub` items > 7 triggers refactor warning

```rust
// ✅ CORRECT: Deep module — simple interface, complex interior
pub trait InferenceGateway: Send + Sync {
    async fn complete(&self, request: InferenceRequest) -> Result<InferenceResponse>;
    async fn stream(&self, request: InferenceRequest) -> Result<InferenceStream>;
    fn health(&self) -> GatewayHealth;
}

// Interior: ThermalMonitor, InferenceScheduler, PiiRedactionGate — all hidden

// ❌ WRONG: Shallow module — exposes internals
pub struct InferenceGateway {
    pub scheduler: InferenceScheduler,  // exposed — bad
    pub thermal: Arc<ThermalMonitor>,    // exposed — bad
    pub config: InferenceConfig,        // exposed — bad
}
```

---

## Dependency Policy

- **Add:** Requires `cargo audit` + manual review
- **Remove:** Clean `Cargo.toml` + `Cargo.lock` update
- **Update:** Run `cargo test` + `cargo miri test --test ffi_boundary_zeroize`
- **Forbidden:** Packages from personal GitHub, unmaintained crates (> 6 months without updates)
