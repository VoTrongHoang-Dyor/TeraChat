# Spec-Wasm-Tapp-Runtime.md — TeraChat Enterprise Platform

```yaml
# DOCUMENT IDENTITY
id: "TERA-RUNTIME"
title: "TeraChat — WASM .tapp Runtime & SDK Specification"
version: "1.0.0"
status: "ACTIVE"
audience: "Rust Core Dev, .tapp Developer, Frontend Engineer"
purpose: "Đặc tả môi trường thực thi WASM dual-engine, Host ABI, Local Event Bus, Background Execution, Encrypted App Storage cho các mini-app doanh nghiệp. Đây là ranh giới bộ nhớ và giới hạn RAM mà Developer viết .tapp cần biết."
depends_on: ["TERA-CORE", "TERA-SYNC"]
constraints_global:
  - "iOS: wasm3 interpreter only — JIT tuyệt đối bị cấm (W^X)"
  - "Mọi crypto op trong WASM phải delegate về Rust Core qua Host ABI"
  - "WASM sandbox strip wasi-sockets — không truy cập TCP/UDP trực tiếp"
  - "Egress_Outbox giới hạn cứng 2MB — vượt = sealed + terminate"
  - "Inbound Webhook tách rời biệt lập với Egress_Outbox (quota & pathway)"
  - "Background task WASM: tối đa 10MB RAM khi OS suspend"
  - ".tapp quyền phải khai báo đầy đủ trong Manifest — không xin runtime"

```

> **Status:** `ACTIVE — Implementation Reference`
> **Audience:** .tapp Developer · Rust Core Dev · Frontend Engineer
> **Last Updated:** 2026-03-29
> **Depends On:** → TERA-CORE · → TERA-SYNC
> **Consumed By:** → TERA-CLIENT · → TERA-ECO

---

## §1 — EXECUTIVE SUMMARY & TRUST BOUNDARIES

### 1.1 Mục tiêu & Trách nhiệm

File này **chịu trách nhiệm** cho:

- WASM Dual-Engine constraints (wasm3 iOS / wasmtime Android+Desktop)
- Host ABI — cách WASM gọi ngược Rust Core
- Local Event Bus — cách .tapp A kích hoạt .tapp B
- Background Execution — ủy quyền cronjob cho Rust Core
- Encrypted App-Local Storage (SQLite Virtual Tables)
- Egress_Outbox data-diode architecture
- .tapp Manifest & permission declaration

File này **KHÔNG chịu trách nhiệm** cho:

- Crypto primitives → `TERA-CORE`
- Storage & sync backend → `TERA-SYNC`
- AI inference → `TERA-ENCLAVE`
- App signing & distribution → `TERA-ECO`
- RBAC & OPA policy → `TERA-GOV`

### 1.2 Trust Model

| Boundary | Bên trong tin tưởng | Bên ngoài không tin tưởng |
|---|---|---|
| WASM sandbox | .tapp business logic | Host network (raw TCP/UDP) |
| Host ABI | Rust Core crypto ops | WASM linear memory |
| Egress_Outbox | Write-only .tapp output | Raw Internet (filtered by OPA) |
| Local Event Bus | Typed, signed inter-app events | Spoofed event sources |
| SQLite Virtual Tables | .tapp data with row-level encryption | Other .tapp instances |

---

## §2 — SYSTEM ARCHITECTURE

### 2.1 WASM Runtime Dual-Engine Strategy

> ⚠️ iOS áp đặt chính sách W^X nghiêm cấm JIT Runtime. Lõi Rust **PHẢI** tự động phát hiện Platform và chọn Engine phù hợp.

| Platform | WASM Engine | JIT | RAM limit (active) | RAM limit (background) |
|---|---|---|---|---|
| 📱 iOS | `wasm3` pure interpreter | ❌ (W^X) | 50MB | 10MB |
| 📱 Android | `wasmtime` JIT (Cranelift) | ✅ | 50MB | 10MB |
| 📱 HarmonyOS | `wasmtime` JIT (Cranelift) | ✅ | 50MB | 10MB |
| 💻🖥️ Desktop/Server | `wasmtime` JIT (Cranelift) | ✅ | 128MB | N/A |

Switch condition at compile-time: `#[cfg(target_os = "ios")]`

**Latency penalty (wasm3 interpreter):** +15–20ms/call vs wasmtime JIT. **Chấp nhận được** cho Enterprise UX.

**WasmParity CI Gate (EXPANDED):**

- Cùng test vector phải chạy identical trên `wasm3` và `wasmtime`.
- Fail → block merge. Latency delta ≤ 20ms. Output semantic **phải identical**.
- **Float Detection Gate (HIGH-3):** LLVM IR analysis phải explicitly flag và reject bất kỳ `f32`/`f64` instruction trong `.tapp` mang manifest `arithmetic_mode: fixed_point`. CI block merge nếu float detected. Không phải soft warning.
- **Fuel Limit Gate (TD-003):** Mọi `.tapp` chạy trên `wasm3` và `wasmtime` phải demonstrably complete trong `instruction_fuel` budget. CI phải simulate worst-case execution path và verify fuel không bị exhausted trước khi hoàn thành core logic.

### 2.2 .tapp Sandbox Architecture

```text
┌─────────────────────────────────────────────────────────────┐
│                    .tapp WASM SANDBOX                        │
│                                                              │
│  ┌─────────────┐  ┌─────────────┐  ┌──────────────────────┐│
│  │ UI Schema   │  │ Business    │  │ SQLite Virtual       ││
│  │ (JSON/DSL)  │  │ Logic       │  │ Tables (Encrypted)   ││
│  └──────┬──────┘  └──────┬──────┘  └──────────────────────┘│
│         │                │                                   │
│         └──────┬──────────┘                                  │
│                │ Host ABI calls only                         │
│                ▼                                             │
│  ┌─────────────────────────────────────────────────────────┐│
│  │              RUST CORE HOST ABI                         ││
│  │  crypto / storage / network / event bus                 ││
│  └──────────────────────┬──────────────────────────────────┘│
│                         │                                    │
│          ┌──────────────┴─────────────────┐                 │
│          ▼                                ▼                  │
│  [Encrypted App Data]           [Egress_Outbox]              │
│  cold_state.db (TERA-SYNC)      (write-only, 2MB limit)      │
└─────────────────────────────────────────────────────────────┘
                                   │
                         OPA DLP Check + BLAKE3 verify
                                   ▼
                           [Internet / Partner API]
```

### 2.3 Local Event Bus Architecture

```text
.tapp A (HRM) ──publishes── EventBus ──subscribes── .tapp B (Notification)
                               │
                         Rust Core routes
                         (signed, typed events)
                               │
                         .tapp C (Task)
```

---

## §3 — DATA MODEL & ENCRYPTION STATE

### 3.1 .tapp Manifest Schema

```yaml
# Mandatory .tapp Manifest
id: "com.company.hr-approvals"
name: "HR Approvals"
version: "1.2.0"
wasm_sha256: "sha256:abc123..."  # Verified before load
signature: "ed25519:xyz..."     # Signed by Developer Key (TERA-ECO)

# Capability Declarations (ALL must be declared upfront — no runtime requests)
capabilities:
  read_message_context: false     # Cannot read chat history
  write_egress_outbox: true       # Can send data out (OPA filtered)
  app_local_storage_mb: 50        # Quota for SQLite Virtual Tables (Hard ceiling)
  event_bus_publish: ["approval.submitted", "approval.rejected"]
  event_bus_subscribe: ["user.role_changed"]
  background_tick_interval_s: 300 # 5min background wakeup

# Math Context (Rule 3: Financial/Approval Data Never Uses Float)
arithmetic_mode: "fixed_point"    # Requires i64, blocks f32/f64 usage in core logic

# UI Declaration
ui_schema: "react-schema"  # Compiled to TeraChat UI schema, not raw HTML
```

### 3.1.2 Three-Tier .tapp Governance Model

- **TeraChat Native Apps:** Developed by TeraChat Inc. (e.g., HR, Finance tools). Signed by TeraChat Root CA. Exempt from ABI negotiation (can use `"first_party": true`). Pre-installed globally.
- **Community/ISV Apps:** Developed by third parties. Submitted to TeraChat App Directory. Scanned & reviewed. Signed by Marketplace CA. IT Admin must explicitly enable.
- **Enterprise Private Apps:** Custom apps built by customer IT. Signed by Enterprise CA. No Marketplace review. Operates strictly within the customer's workspace boundary.

### 3.2 SQLite Virtual Tables (Encrypted App Storage)

**Host-Provided SQLite Virtual Tables — thay thế `sled` KV-store:**

```sql
-- .tapp thấy standard SQL interface:
CREATE VIRTUAL TABLE IF NOT EXISTS approvals USING tera_encrypted(
    id TEXT PRIMARY KEY,
    requester_id TEXT,
    status TEXT,
    payload BLOB,
    created_at INTEGER
);

-- Rust Core: tự động mã hóa từng row bằng DEK riêng (row-level encryption)
-- .tapp không thấy encryption key bao giờ
```

**Schema Migration (Rule 7: Yêu cầu Migration Contract cứng):**

```sql
-- .tapp bắt buộc khai báo mapping migration rõ ràng:
schema_version: 3
migrations:
  - from: 2
    to: 3
    sql: "ALTER TABLE approvals ADD COLUMN priority INTEGER DEFAULT 0"
```

Rust Core tự apply migration khi .tapp version bump. Nếu schema_version mới hơn nhưng không có chain mapping, Rust Core từ chối khởi động .tapp (bắn `CoreSignal::ComponentFault`) để tránh data loss âm thầm.

### 3.3 Egress_Outbox Object

| Property | Value |
|---|---|
| Type | Write-only queue (từ .tapp), Read-only (từ Egress Daemon) |
| Size limit | **2MB cứng** — vượt = sealed + terminate .tapp |
| Access pattern | .tapp writes JSON payload; daemon reads + sends |
| Security | BLAKE3 DLP Hash Chain (Rust Core ký trước push; Daemon tái tính trước send) |
| Network | URLSession (iOS) / Cronet (Android) / HTTP (Desktop) |

### 3.4 Local Event Bus Objects

```rust
// Event published to Local Event Bus
pub struct TappEvent {
    pub event_type: String,       // e.g., "approval.submitted"
    pub source_tapp_id: String,   // Publisher identity
    pub payload: Vec<u8>,         // Serialized, typed payload
    pub signature: Ed25519Sig,    // Signed by publisher's app key
    pub hlc: HLCTimestamp,
}
```

---

## §4 — PROTOCOL & EXECUTION CONTRACT

### 4.1 Host ABI (WASM → Rust Core)

> 💡 **Mệnh đề ABI (Rule 1: Typed Contract, Not a Pipe):** Host function input/output phải sử dụng MessagePack-serialized với schema version field. Mọi Host function mới đều phải có test WasmParity và ErrorCode. Không serialize raw JSON hay untyped bytes qua biên này.

```rust
// Crypto operations — .tapp KHÔNG tự chạy crypto
extern "C" {
    fn host_blake3_hash(data_ptr: *const u8, data_len: usize, out_ptr: *mut u8) -> i32;
    fn host_ed25519_sign(key_id: u64, msg_ptr: *const u8, msg_len: usize, sig_out: *mut u8) -> i32;
    fn host_aes256gcm_encrypt(
        key_id: u64, nonce_ptr: *const u8,
        plaintext_ptr: *const u8, plaintext_len: usize,
        ciphertext_out: *mut u8
    ) -> i32;
}

// Storage — SQLite Virtual Tables (Cấm materialized queries cho list data)
// Thay vì host_app_state_query trả về nguyên mảng lớn gây nguy cơ OOM,
// yêu cầu sử dụng Cursor Protocol (PAGE_SIZE = 500).
extern "C" {
    fn host_db_query(
        tapp_did_ptr: *const u8, did_len: usize,
        sql_ptr: *const u8, sql_len: usize,
        params_msgpack: *const u8, params_len: usize,
    ) -> u64; // Trả về cursor_id (u64) hoặc 0 nếu lỗi

    fn host_db_cursor_next(
        cursor_id: u64,
        out_row_msgpack: *mut u8,
        out_max: usize,
    ) -> i32; // Returns bytes_written, 0 = exhausted, -1 = error

    fn host_db_cursor_close(cursor_id: u64) -> i32;

    // Enforcement: lỗi Storage Quota Exceeded trả về -5 synchronous lúc write.
    fn host_app_state_write(patch_ptr: *const u8, patch_len: usize) -> i32; 
}

// Event Bus
extern "C" {
    fn host_event_publish(event_ptr: *const u8, event_len: usize) -> i32;
    fn host_event_subscribe(event_type_ptr: *const u8) -> i32;
}

// Egress
extern "C" {
    // Egress Outbox (4KB - 2MB limit constraint tùy endpoint, tách mạch với Webhook Ingress)
    fn host_egress_write(payload_ptr: *const u8, payload_len: usize) -> i32;
}
```

### 4.1.2 Slash Command Routing Protocol

To emulate Slack/Teams functionality securely on-device:

- Users typing `/command` directly trigger the Rust Core syntax parser.
- The Core looks up the `.tapp` registered to the command prefix in `cold_state.db`.
- Command execution resolves through the Local Event Bus (or direct IPC `__tera_on_slash_command(...)`). Output is rendered natively as an ephemeral or permanent UI element without leaving the device. NO external webhooks are called directly by the client.

### 4.2 I/O Delegation (Control Plane vs. Data Plane)

To enforce the 64MB `.tapp` memory limit while handling gigabytes of enterprise data securely, the architecture strictly segregates the execution into two layers:

**Data Plane (Host Rust Core):**
Responsible for heavy I/O operations including raw bytes network fetching, hardware-accelerated Ed25519 cryptography, Large Blob transfers, and encrypted SQLite transactions. The data plane resides outside the sandbox and processes data natively.

**Control Plane (.tapp WASM System):**
The WASM sandbox acts solely as the system's brain. It sequences API operations, coordinates application logic, handles User Interface states, and routes business flows. It never holds large byte arrays directly in its limited RAM.

*Workflow Example:*
Instead of the `.tapp` pulling a 10MB PDF over the network into memory to decrypt and store it, the `.tapp` sends a declarative command to the Rust Core: `Download(URL) -> Decrypt(Key) -> SaveToDB(Table)`. The Rust Core performs this natively. The `.tapp` only receives a fast acknowledgment: `Success(FileID)`.

### 4.3 Background Execution Delegation

**Vấn đề:** iOS Jetsam kill app nếu .tapp chạy ngầm với RAM/CPU cao.

**Giải pháp — Rust Core Delegation:**

```text
[.tapp registers background_tick_interval_s = 300 in Manifest]
     │
[OS Background Task fires]
     │
[Rust Core wakes up (NOT .tapp)]
     │ Only 10MB RAM budget
     │
[Rust Core calls: __tera_on_background_tick(app_id)]
     │ in WASM module with 10MB limit
     │
[.tapp runs minimal sync/check logic]
     │
[Results written to cold_state.db]
     │
[Rust Core suspends WASM; returns to OS]
```

**Constraints:**

- Background WASM: **max 10MB RAM**, **max 30s execution time**.
- No network I/O in background WASM — queue in `Egress_Outbox`, Egress Daemon sends.
- No UI operations in background WASM.

### 4.4 Local Event Bus Protocol

**Publish:**

1. .tapp calls `host_event_publish(event)`.
2. Rust Core: verify event_type is in .tapp's `event_bus_publish` capability list.
3. Rust Core: sign event with app key, add HLC timestamp.
4. Rust Core: route to all subscribed .tapp instances.

**Subscribe:**

1. .tapp calls `host_event_subscribe("approval.submitted")`.
2. Rust Core: verify event_type is in .tapp's `event_bus_subscribe` capability list.
3. On event received: Rust Core calls `__tera_on_event(event_ptr)` in subscriber WASM.

### 4.5 Data Diode (Egress_Outbox) Protocol

```text
[Lõi Rust - Crypto Core] ──Push Masked Data──▶ [WASM Sandbox .tapp]
                                                        │
                                            (không có callback ngược)
                                                        │
                                              Write ──▶ [Egress_Outbox]
                                                        │
                           [tera_egress_daemon / OS Background Service]
                              (Separate process, không share memory với Lõi Rust)
                                                        │
                                       OPA DLP Check + BLAKE3 Hash Verify
                                                        │
                           URLSession (iOS) / Cronet (Android) / HTTP (Desktop)
                                               ▼
                                         Internet / Partner API
```

### 4.6 macOS XPC Process Isolation

- macOS Hardened Runtime tách `wasmtime` JIT ra `terachat-wasm-worker` (child process).
- Main process không có `allow-jit`. Giao tiếp qua XPC Service (Mach port).
- XPC crash → `XpcTransactionJournal` recovery (PENDING → rollback).

---

## §5 — STATE MACHINE

### 5.1 .tapp Lifecycle State

```text
[NOT_INSTALLED]
     │ IT Admin installs via OPA-approved manifest
     ▼
[INSTALLED]
     │ User opens .tapp
     ▼
[LOADING] ── WASM module verified (sha256 + Ed25519 sig)
     │
     ├── sig invalid → [BLOCKED]
     │
     ▼
[RUNNING]
     │ RAM > 50MB
     ├──> [OOM_TERMINATED] ── Jetsam / SIGSEGV
     │
     │ Network output needed
     ├──> [EGRESS_PENDING] ── Egress Daemon picks up
     │
     │ OS suspends
     ▼
[BACKGROUNDED] ── max 10MB RAM
     │ background_tick fires
     ├──> [BACKGROUND_TICK_RUNNING] → [BACKGROUNDED]
     │
     │ User returns
     ▼
[RUNNING]
```

### 5.1.2 Thermal-Aware Execution State

Mobile hardware strictly constrains continuous loads during background WASM execution, E2EE, and Mesh operations.
System operates a `ThermalStateMonitor`:

- `thermalState < .serious`: Operations continue normally (nominal/fair states may throttle tick frequencies).
- `thermalState >= .serious`: WASM sandbox is **immediately suspended**, background ticks are halted, and `CoreSignal::ThermalThrottling { level: Critical }` is emitted. Only Rust Core text messaging and BLE control plane stay active to prevent device shutdown.

### 5.2 OOM Prevention & Memory Lifecycle (Tombstoning vs. Hydration)

Given the aggregate memory cap (e.g., maximum 64MB combined limit across all running instances), the system implements an aggressive memory lifecycle manager.

- **RAM Watchdog:** A background routine continuously polling heap sizes. If aggregate memory footprints threaten to exceed limits, idle or backgrounded `.tapps` are prioritized for suspension.
- **Tombstoning:** The runtime serializes the specific internal state variables of the `.tapp` out to encrypted local storage (`cold_state.db`) and deallocates the WASM instance from RAM, leaving behind a stub ("Tombstone").
- **Hydration:** Upon User interaction or incoming `event_bus` triggers, the runtime instantly reloads the WASM binary, decrypts the Tombstone state, and injects it back, achieving sub-100ms resumability. This process is entirely transparent to the user.

### 5.3 Egress_Outbox State

```text
[EMPTY]
     │ .tapp writes payload
     ▼
[PENDING (size < 2MB)]
     │ Egress Daemon reads
     ├── OPA reject → [BLOCKED_DLP]
     ├── BLAKE3 mismatch → [INTEGRITY_FAIL]
     ▼
[SENT]
```

---

## §6 — API / IPC / EVENT BUS

### 6.1 Signals (Runtime Domain)

| Signal | Trigger | Consumer |
|---|---|---|
| `WasmSandboxTerminated(reason)` | OOM / crash / sig violation | TERA-CLIENT (show error) + TERA-GOV (audit) |
| `EgressSchemaViolation(plugin_id)` | DLP check fail | TERA-GOV (audit + block) |
| `WasmOomKill` | RAM > limit | TERA-CLIENT (refresh notification) |
| `TappEventPublished(event_type)` | Event Bus publish | Subscribed .tapp instances |
| `BackgroundTickCompleted(app_id, duration_ms)` | Background tick done | Telemetry |

### 6.2 .tapp SDK Entrypoints

```rust
// Rust Core calls into .tapp WASM at these entry points:
// (All exported from .tapp WASM module)

#[no_mangle]
pub extern "C" fn __tera_on_start() { /* .tapp initialization */ }

#[no_mangle]
pub extern "C" fn __tera_on_background_tick() { /* max 30s, 10MB */ }

#[no_mangle]
pub extern "C" fn __tera_on_event(event_ptr: *const u8, event_len: usize) { /* handle event */ }

#[no_mangle]
pub extern "C" fn __tera_on_ui_action(action_ptr: *const u8, action_len: usize) { /* UI interaction */ }

// Yêu cầu xác nhận (ACK) Webhook Ingress trong 500ms
#[no_mangle]
pub extern "C" fn on_webhook_received(
    endpoint_id: u64, 
    payload: *const u8, payload_len: usize, 
    delivery_id: u64
) -> i32 { 
    // Returns 0 = ACK, 1 = NACK (retry), 2 = REJECT (no retry)
    0 
}
```

---

## §7 — PLATFORM MATRIX & CONSTRAINTS

| Constraint | 📱 iOS | 📱 Android | 💻🖥️ Desktop |
|---|---|---|---|
| WASM Engine | wasm3 (interpreter) | wasmtime JIT | wasmtime JIT |
| JIT allowed | ❌ W^X | ✅ | ✅ |
| Active RAM limit | 50MB | 50MB | 128MB |
| Background RAM limit | 10MB | 10MB | N/A |
| Background execution | BGProcessingTask ≤ 30s | Background Service | Always-on |
| Network from WASM | ❌ (Egress_Outbox only) | ❌ (Egress_Outbox only) | ❌ (Egress_Outbox only) |
| XPC isolation | ✅ (XPC Service) | ✅ (isolated process) | ✅ (terachat-wasm-worker) |
| SQLite Virtual Tables | via Host ABI | via Host ABI | via Host ABI |
| SIMD in WASM | ❌ disabled | ❌ disabled | ❌ disabled |

---

## §8 — NON-FUNCTIONAL REQUIREMENTS (NFR)

| Requirement | Target | Notes |
|---|---|---|
| WASM startup latency (wasm3/iOS) | < 200ms | Cold start |
| Host ABI crypto call overhead | < 20ms | Per call |
| Background tick total time | < 30s | Hard limit |
| Egress_Outbox write throughput | > 1MB/s | .tapp write speed |
| Event Bus routing latency | < 5ms per event | Local only |
| SQLite Virtual Table query | < 50ms | Simple SELECT |
| .tapp RAM limit enforcement | Hard kill at 50MB | Instant |
| WasmParity CI gate | 100% test pass | Both engines |

---

## §9 — SECURITY & THREAT MODEL

| Attack | Vector | Mitigation |
|---|---|---|
| Sandbox escape via raw syscall | WASM calls OS directly | wasi-sockets stripped; Host ABI only |
| Egress data exfiltration | .tapp writes PII to Egress_Outbox | OPA DLP check + BLAKE3 hash verify |
| Inter-.tapp data leakage | .tapp A reads .tapp B's Virtual Tables | SQLite Virtual Tables isolated per app_id |
| Privilege escalation via event bus | Malicious .tapp publishes fake event type | Rust Core verifies event_type vs Manifest capability list |
| Memory spray into Rust Core | WASM linear memory overflow | Linear Memory Isolation: each instance isolated, overflow → SIGSEGV |
| JIT spraying (iOS) | Attacker triggers JIT code injection | wasm3 interpreter only on iOS; no JIT pages |
| Manifest tampering | Attacker modifies sha256/capabilities | Ed25519 signature from Developer Key verified before load (TERA-ECO) |
| Background RAM abuse | .tapp allocates >10MB in background | Hard kill by Rust Core memory monitor |

---

## §10 — FAILURE MODEL & RECOVERY

| Failure | Detection | Recovery |
|---|---|---|
| WASM crash during execution | SIGSEGV / OOM signal | `WasmSandboxTerminated` signal; journal rollback; .tapp state preserved in cold_state.db |
| XPC child crash (macOS) | XPC connection died | `XpcTransactionJournal` replay; PENDING → rollback if no COMMITTED |
| Egress_Outbox size overflow (>2MB) | Size check before write | Outbox sealed; .tapp terminated; admin notified |
| Background tick timeout | Timer expired | Rust Core force-kills WASM; logs `BackgroundTickTimeout` |
| Host ABI crypto failure | Error code returned | .tapp receives error code; must handle gracefully (no crash) |
| SQLite Virtual Table migration fail | Version mismatch | Rust Core: rollback to previous schema; .tapp blocked until migration fixed |
| Event Bus subscriber crash | WASM crash during `__tera_on_event` | Event dropped; retry policy per event_type (configured in manifest) |

## §11 — ARCHITECTURAL INVARIANTS & AUDIT RESOLUTIONS (WASM RUNTIME)

### 11.1 WASM Heap vs. App Suite Data Volume

**Constraint:** The WASM linear memory poses a hard `<64MB` ceiling. A single Finance tapp loading 50,000 encrypted records will exhaust the heap.
**Resolution:** Tapp Data Namespace I/O must strictly be separated from the WASM heap. All database access must utilize the `host_db_query` function via a **streaming cursor protocol**, enforcing a `PAGE_SIZE = 500` contract at the ABI level. Complete result sets must never be materialized entirely into memory.

### 11.2 Inbound Webhook + Egress Circuit Breaker

**Constraint:** A global circuit breaker penalizes a tapp globally, making independent channels vulnerable to single-point lockouts.
**Resolution:** Circuit breakers must be scoped **per egress endpoint declaration**. Inbound webhook handling (`host_webhook_deliver`) must execute on an isolated code path with an independent budget, preventing egress violations from crippling inbound delivery.

### 11.3 WasmParity Gate vs. App Suite Float Logic

**Constraint:** Platform floating-point semantics diverge (e.g., `wasm3` vs. `wasmtime`), leading to compliance-breaking drift in financial calculations over long life-cycles.
**Resolution:** All financial math must utilize **fixed-point arithmetic** (`i64` with up to 4 decimal places). Floating-point math is restricted solely to non-critical logic (e.g., probability scoring). The tapp manifest must explicitly declare `"arithmetic_mode": "fixed_point"`.

> **Enforcement (HIGH-3 Fix):** Float detection phải là **required CI gate**, không chỉ là documentation recommendation. LLVM IR analysis pipeline (TERA-ECO §4.1) phải explicit scan và block merge nếu `f32`/`f64` xuất hiện trong financial `.tapp` (manifest `arithmetic_mode: fixed_point`). Không phải soft warning.

### 11.4 Host ABI Gaps (Cursor Protocol)

**Constraint:** Existing definitions lack clear signatures for `host_db_query` cursor flows, risking OOM and boundary leaks.
**Resolution:** Defined streaming API:

- `host_db_query(...) -> cursor_id: u64`

- `host_db_cursor_next(cursor_id, ...) -> i32`
- `host_db_cursor_close(cursor_id) -> i32`
Params must be serialized as `MessagePack`. Error codes must align with `host_egress_write`.

### 11.5 Inbound Webhook Delivery Guarantee

**Constraint:** Webhooks arriving while the tapp is suspended lack defined retry and acknowledgement semantics.
**Resolution:** Introduced `on_webhook_received(endpoint_id, payload_ptr, len, delivery_id) -> i32`. Webhook payloads (governed by `max_payload_bytes` up to 64KB) are buffered in `hot_dag.db` with a 5-minute TTL during tapp suspension. The tapp must ACK within 500ms or trigger an automatic retry sequence (up to 3 attempts).

### 11.6 Tapp Storage Quota Enforcement Timing

**Constraint:** Asynchronous quota enforcement enables mid-transaction overflow vulnerabilities.
**Resolution:** Enforcement must be **synchronous at the write path** (`host_db_query`). A hard ceiling (`max_storage_mb: 256`) is enforced prior to SQL transaction commit. Violations immediately return `ERR_STORAGE_QUOTA_EXCEEDED` (-5). Tapps must handle their own eviction strategies; the host will never auto-evict data.

> **Deadlock vector — HIGH-6 (triggers GAP-A):** Nếu `.tapp` background tick bị block trên quota và 30s timeout giết WASM instance mid-transaction, Saga Journal sẽ thấy `CrdtCommitted` entry không có corresponding cold_state write — trigger GAP-A (TERA-SYNC §8.4). `CoreBootSequence::SagaRecoveryGuard` sẽ detect và recover tại next startup. UI phải hiển thị "Chờ đồng bộ" thay vì "Đã duyệt" cho đến khi Saga resolved.

### 11.7 Strict Engineering Guardrails (Runtime Contracts)

- **Rule 1 (Typed FFI Boundary):** The WASM boundary is a strict typed contract. Do not pass unstructured bytes. Use MessagePack with declared schema versions. New host functions must be defined in the ABI specification accompanied by WasmParity test vectors.
- **Rule 3 (No Float in Finance):** `f32/f64` primitives are categorically forbidden for financial parameters. Fixed-point is mandatory. **Enforced via LLVM IR CI gate — not advisory.**
- **Rule 7 (Migration Contract):** Every schema update necessitates a defined migration path within the manifest (`schema_version`, `migrations`). Unmigrated tapps will fail to launch, emitting `CoreSignal::ComponentFault`.

### 11.8 Gas/Fuel Metering — Deterministic Execution (TD-003 / XPLAT-01)

**Constraint:** Timeout theo giây thiên vị phần cứng mạnh. `.tapp` chạy ổn trên Desktop (`wasmtime` JIT) có thể vượt 30s timeout trên iOS (`wasm3` interpreter) — không biết trước khi deploy.
**Resolution:** Host ABI áp đặt Gas/Fuel Metering. Mỗi `.tapp` được cấp `instruction_fuel: u64` cố định thay vì timeout theo giây. Khi hết Fuel, `.tapp` bị buộc dừng — Deterministic tuyệt đối trên cả 2 engine.

```yaml
# .tapp Manifest additions:
computation:
  instruction_fuel: 50_000_000   # Deterministic regardless of engine/hardware
  fuel_per_background_tick: 5_000_000
```

```rust

// Rust Core host side:
let mut store = Store::new(&engine, ());
store.add_fuel(tapp_manifest.computation.instruction_fuel)?;
// WASM instances được monitor deterministically trên cả wasm3 và wasmtime
```

### 11.9 Burner Agent TTL + EMDP Epoch Freeze Intersection (HIGH-4 / GAP-G)

**Constraint:** Burner Agent (FUNC-13, TTL=60min) join MLS group như một member. Khi TTL expire, phải bị remove, triggering MLS Epoch Rotation. Trong Mesh Mode đang có EMDP Epoch Freeze active, removal được queued nhưng Epoch không advance. Burner Agent trở thành **"zombie member"** — removed logically nhưng derived keys vẫn valid past TTL.
**Resolution:**

- Khi EMDP Epoch Freeze active và Burner Agent TTL expire: removal được đánh dấu `removal_pending_freeze: true` trong Saga Journal.
- Epoch Ratchet advance được scheduled ngay sau `EmdpSessionTerminated` signal — trước khi sync bất kỳ message nào.
- Burner Agent CRDT events trong freeze window mang flag `emdp_forced: true` — đi vào review queue.
- **TestMatrix SC-36** phải implement scenario này (xem TERA-TEST).
