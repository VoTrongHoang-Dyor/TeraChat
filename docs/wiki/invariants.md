---
type: concept
created: 2026-05-15
updated: 2026-05-18
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

**Enforcement:** CI gate `business-logic-in-ui` — scans all `source/apps/` files for `package:pointycastle`, `CryptoKit`, or `openmls` imports. Auto-reject on match.

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

**Enforcement:** CI gate `zeroize-verify` — custom lint validates every struct containing `KeyMaterial` derives `ZeroizeOnDrop`. Also blocks `mem::forget`, `ManuallyDrop`, `Arc<KeyMaterial>` usage on key types.

## 3. No Raw Pointer in `pub extern "C"`

All FFI functions must use Token Protocol (opaque `u64`), never raw `*const u8`.

```rust
// ✅ CORRECT
pub extern "C" fn encrypt(key_token: u64, data: *const u8, len: usize) -> FfiResult;

// ❌ FORBIDDEN
pub extern "C" fn encrypt(key: *const u8, len: usize) -> *mut u8;
```

**Enforcement:** CI gate `ffi-token-protocol` — lint forbids `*const u8` and `*mut u8` in any `pub extern "C"` function signature. Only `u64` tokens allowed.

## 4. Dual-Plane Sync

```
hot_dag.db   → CRDT DAG for chat (append-only)
cold_state.db → Relational for Finance/HR
```

NEVER force Finance data into CRDT. CRDT is for chat only.

**Enforcement:** CI gate `sync-boundary` — forbids `INSERT`/`UPDATE`/`DELETE` on `hot_dag.db` CRDT table from any code outside `tc-crdt-sync`. Also forbids CRDT types in `tc-store` relational schema.

## 5. AI Only After SanitizedPrompt

Every prompt sent to an AI model MUST pass through PII redaction. No embedding egress without redaction.

```rust
// ✅ CORRECT
let sanitized = pii_gate.redact(raw_prompt)?;
let response = inference.complete(sanitized).await?;

// ❌ FORBIDDEN
let response = inference.complete(raw_prompt).await?;
```

**Enforcement:** Type system — `InferenceGateway::complete()` and `stream()` accept ONLY `SanitizedPrompt`, never `String` or `&str`. `SanitizedPrompt` has a private `inner: String` field — cannot be constructed outside `tc-ai` module. Compile-time guarantee.

## 6. Headless Daemon + gRPC Before UI Expansion

Rust Core runs as an independent process. UI connects via gRPC over UDS. No UI-specific code in Core.

**Enforcement:** Integration test `daemon_independence` — kills UI process and verifies Core continues serving gRPC. Must pass CI. Core binary must compile and run without any UI crate dependency.

## 7. Test Never Trails

SC-34 through SC-40 are deployment blockers. No phase ships with failing tests.

**Enforcement:** CI gate — `cargo test --workspace` must pass. SC-34 through SC-40 marked `#[ignore]` only if explicitly approved by human architect via CODEOWNERS.

## 8. 1 Subsystem per Phase (Vertical Slice)

Progressive complexity — one vertical slice at a time. Never build two major subsystems concurrently.

**Enforcement:** Process — `phase/README.md` + `CLAUDE.md` define explicit scope per slice. PR that touches >1 major crate outside current slice scope triggers architect review in CODEOWNERS.

---

## 9. iOS election_weight = 0

iPhone không bao giờ làm mesh coordinator. iOS devices have zero voting power in Floor Gateway election.

**Enforcement:** CI gate `ios-election-zero` — integration test verifies iOS target always returns `ElectionWeight::zero()`. Cannot be overridden at runtime. Hardcoded in `tc-mesh` platform layer.

---

## 10. NAS ECC is Sole Storage Authority

Mac mini (non-ECC RAM) KHÔNG bao giờ là primary database writer. Chỉ NAS với ECC RAM mới có `StorageAuthority`.

```
Compute Node (Mac mini)  →  gRPC write request  →  NAS ECC Storage Node
                                                       ↓
                                                  SQLite WAL write
                                                  (authoritative)
```

**Why:** Non-ECC RAM có thể gây silent data corruption — bit flip trong WAL journal trước khi flush ra disk sẽ phá hủy toàn bộ database. NAS ECC RAM phát hiện và sửa single-bit errors.

**Enforcement:** Type system — `StorageAuthority` enum có exactly one variant `NasEcc`. `tc-store` write path (`WALWriter`, `JournalAppender`) chỉ compile cho `aarch64-linux` (NAS target), không compile cho `aarch64-darwin` (Mac mini). Compile-time guarantee.

---

## 11. BLE Text Only — Maximum 500 Bytes

TeraLink Fallback Network T3 (BLE Emergency) chỉ relay text. Không file, không media, không attachment. Mỗi message tối đa 500 bytes.

**Why:** BLE 5.0 Coded PHY effective payload ~0.5 Mbps. 500 bytes limit ngăn broadcast storm trên BLE subnet 50+ thiết bị. File/media qua BLE sẽ làm nghẽn toàn bộ Floor Subnet.

**Enforcement:** Type system — `BlePayload` là `[u8; 500]` fixed-size array, không phải `Vec<u8>`. Host ABI từ chối `send_media()` syscall khi transport = BLE. Compile-time guarantee.

---

## 12. .tapp No External Network Egress

WASM sandbox không có quyền truy cập mạng ra ngoài tổ chức. Capability `network:external` bị chặn vĩnh viễn tại Host ABI — không phải policy, mà là kỹ thuật.

**Why:** Zero Data Egress guarantee cho enterprise compliance. Một .tapp độc hại không thể exfiltrate data ra internet, ngay cả khi có bug trong permission system.

**Enforcement:** WASM Host ABI — capability `network:external` bị xóa khỏi `HostAbiPermissions` struct. Không capability negotiation nào có thể enable nó. CI test `tapp-network-sandbox` xác nhận mọi WASM call đến `host_network_send()` luôn trả về `Err(PermissionDenied)`.

---

## 13. BSL Boundary Immutable After Publish

Một khi module được publish dưới BSL hoặc MIT, không được upgrade lên license restrictive hơn. LICENSE file được hash và ghi vào git tag release.

**Why:** Bài học HashiCorp 2023 — license bait-and-switch phá hủy community trust. TeraChat commit: module MIT hôm nay sẽ mãi mãi là MIT.

**Enforcement:** CI gate `bsl-boundary-hash` — mỗi `git tag` release, CI hash `LICENSE` file và so sánh với `.github/BSL_BOUNDARY.sha256`. Mismatch = tag creation bị block. Hash file chỉ có thể sửa bởi human architect qua CODEOWNERS.

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

// ❌ FORBIDDEN: BLE payload > 500 bytes
fn ble_send(payload: Vec<u8>)  // Must use [u8; 500] fixed array

// ❌ FORBIDDEN: InferenceGateway::complete() with raw String
async fn complete(&self, prompt: String) -> Result<InferenceResponse>;  // Must accept SanitizedPrompt only

// ❌ FORBIDDEN: .tapp manifest declares network:external capability
// Host ABI must permanently block this capability — no negotiation possible

// ❌ FORBIDDEN: Mac mini direct database write bypassing NAS ECC
// tc-store write path only compiles for NAS (aarch64-linux), NOT Mac mini (aarch64-darwin)
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
