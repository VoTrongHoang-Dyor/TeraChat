# Spec-Client-IPC-And-UI-Bridge.md — TeraChat Enterprise Platform

```yaml
# DOCUMENT IDENTITY
id: "TERA-CLIENT"
title: "TeraChat — Client IPC & UI Bridge Specification"
version: "1.0.0"
status: "ACTIVE"
audience: "Frontend Engineer, Flutter/Tauri Developer, Mobile Engineer"
purpose: "Hợp đồng giao tiếp (contract) giữa Rust Core và các Frontend Native (Flutter/Tauri/Swift). Frontend dev chỉ cần đọc file này để biết cách gọi Core — không cần hiểu MLS hay Crypto. Đặc tả FFI Token Protocol, SharedArrayBuffer Data Plane, UICommands, CoreSignals, và Streaming Decryption Local Proxy."
depends_on: ["TERA-CORE"]
constraints_global:
  - "UI Layer tuyệt đối không lưu state — chỉ render data qua FFI Token"
  - "UI tuyệt đối không port Crypto/Business Logic lên Dart/JS Thread"
  - "Mọi FFI endpoint KHÔNG trả raw ptr — dùng Token Protocol"
  - "Unidirectional: Core push StateChanged signal → UI pull snapshot"
  - "Streaming Decryption Proxy: stream tới 127.0.0.1 loopback — không ghi plaintext ra disk"

```

> **Status:** `ACTIVE — Implementation Reference`
> **Audience:** Frontend Engineer · Flutter/Tauri Developer · Mobile Engineer
> **Last Updated:** 2026-03-29
> **Depends On:** → TERA-CORE
> **Consumed By:** _(leaf node — no other spec consumes this)_

---

## §1 — EXECUTIVE SUMMARY & TRUST BOUNDARIES

### 1.1 Mục tiêu & Trách nhiệm

File này **chịu trách nhiệm** cho:

- FFI Token Protocol (Zero-pointer cross-boundary)
- SharedArrayBuffer / Dart FFI Data Plane
- UICommands (UI → Core) & CoreSignals (Core → UI)
- Streaming Decryption Local Proxy (video 10GB E2EE → native player)
- State synchronization model (unidirectional)
- Platform-specific IPC mechanisms (iOS XPC, Android Binder, Tauri IPC)

File này **KHÔNG chịu trách nhiệm** cho:

- Crypto primitives → `TERA-CORE`
- Storage & sync → `TERA-SYNC`
- WASM .tapp SDK → `TERA-RUNTIME`
- AI inference → `TERA-ENCLAVE`
- RBAC & OPA → `TERA-GOV`

### 1.2 Core Rule cho Frontend Dev

> **"UI là pure renderer. Rust Core là domain owner."**

- ❌ Không lưu state trong UI layer (Dart, JS, Swift)
- ❌ Không implement bất kỳ business logic trong UI
- ❌ Không truyền raw pointer qua FFI
- ✅ Chỉ render data từ snapshots pulled từ Rust Core
- ✅ Chỉ gửi UICommands lên Core khi user action

### 1.3 Trust Boundaries

| Boundary | Bên trong tin tưởng | Bên ngoài không tin tưởng |
|---|---|---|
| FFI Token Protocol | Opaque token handle | Raw data pointer |
| Control Plane (Protobuf) | Signed command structs | Oversized commands (>1KB) |
| Data Plane (SharedArrayBuffer) | Zero-copy snapshot | Mutable shared state |
| Local Proxy (127.0.0.1) | Streaming decrypted bytes | Plaintext on disk |

---

## §2 — SYSTEM ARCHITECTURE

### 2.1 IPC Architecture — Tách Control/Data Plane

```text
┌─────────────────────────────────────────────────────────────┐
│                     RUST CORE                                │
│                                                              │
│  Business Logic · Crypto · State Machine · CRDT Sync        │
│                                                              │
│  ┌─────────────────────────┐  ┌───────────────────────────┐│
│  │   CONTROL PLANE         │  │    DATA PLANE             ││
│  │   (Protobuf / JSI)      │  │    (Zero-Copy Buffer)     ││
│  │   UICommand → Core      │  │    Snapshot → UI          ││
│  │   < 1KB per message     │  │    ~400-500 MB/s          ││
│  └──────────┬──────────────┘  └──────────────┬────────────┘│
│             │ FFI/JSI                          │ SharedArrayBuffer│
└─────────────┼──────────────────────────────────┼────────────┘
              │                                  │
┌─────────────┼──────────────────────────────────┼────────────┐
│  Flutter (Dart FFI) / Tauri (JS) / Swift (XPC)  │            │
│                                                             │
│  ┌──────────────────────┐  ┌──────────────────────────────┐│
│  │  UI Components       │  │  State Snapshot Renderer     ││
│  │  (Pure Renderer)     │  │  (Pull on StateChanged sig)  ││
│  └──────────────────────┘  └──────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

### 2.1.1 Mesh IPC CasHash Control (Payload Offloading)

Do Control Plane bị giới hạn `< 1KB`, khi User gửi MessageTextPayload quá lớn hoặc đẩy file/ảnh, hệ thống áp dụng cơ chế **Mesh IPC CasHash Offload**:

- **Ngưỡng giới hạn WebRTC/Mesh:** Mọi payload Push qua Mesh Network (WebRTC) không được vượt quá `10KB` để giữ độ trễ thấp và tránh stall DataChannel.
- **Offload Mechanism:**
  - Nếu Payload UICommand > 1KB (ví dụ: raw text rất dài), Frontend phải gọi `tera_buf_write` vào Data Plane. Core sẽ cấp 1 `CasHash` reference. Gửi `CasHash` qua Control Plane.
  - Khi Rust Core đẩy dữ liệu qua Mesh (Client-to-Client), Core tự động mã hoá Chunk, upload lên Edge Transit Node/NAS, và đẩy mẩu tin báo hiệu chứa `CasHash` (SHA3 UUID) qua WebRTC.
  - Node nhận tự động fetch `CasHash` từ Transit Node xuống. Mọi việc trong mờ với UI.

### 2.2 Streaming Decryption Local Proxy

**Problem:** File video 10GB E2EE không thể load vào RAM. AVPlayer/ExoPlayer cần HTTP URL.

**Solution — Local Decryption Proxy:**

```text
┌────────────────────────────────────────────────────────────┐
│                    CLIENT DEVICE                            │
│                                                             │
│  [Rust Core: Streaming Decryption Proxy]                   │
│  Lắng nghe: http://127.0.0.1:{dynamic_port}/blob/{token}   │
│                  │                                          │
│   ┌──────────────┴───────────────────────────────────┐     │
│   │ Chunk fetch loop:                                 │     │
│   │   1. Fetch encrypted chunk from NAS/MinIO         │     │
│   │   2. Decrypt with ChunkKey (AES-256-GCM)          │     │
│   │   3. Stream decrypted bytes to HTTP response      │     │
│   │   4. ZeroizeOnDrop ChunkKey after each chunk      │     │
│   └───────────────────────────────────────────────────┘     │
│                  │                                          │
│  ┌───────────────▼──────────────────────────────────────┐  │
│  │  Native Media Player:                                 │  │
│  │  AVPlayer (iOS) / ExoPlayer (Android) / HTML5 (Tauri) │  │
│  │  URL: http://127.0.0.1:PORT/blob/{one_time_token}     │  │
│  └───────────────────────────────────────────────────────┘  │
└────────────────────────────────────────────────────────────┘
```

**Security constraints:**

- Port assigned dynamically (random ephemeral port).
- Token is one-time use, TTL 60s.
- Proxy only serves localhost interface — no external access.
- Plaintext bytes stream directly to player buffer — NEVER written to disk.
- Proxy process shares memory isolation with Rust Core — no WASM access.

---

## §3 — DATA MODEL & ENCRYPTION STATE

### 3.1 FFI Token Objects

| Object | Type | Storage | Lifetime | Notes |
|---|---|---|---|---|
| `TeraBufToken` | Opaque u64 handle | Rust Core memory | Until `tera_buf_release()` called | References ZeroizeOnDrop buffer in Core |
| `UIRenderSnapshot` | Protobuf serialized view state | SharedArrayBuffer (read-only for UI) | Duration of render cycle | Never persisted; refreshed on StateChanged |
| `UICommand` | Protobuf message | Control Plane buffer | Single dispatch | < 1KB limit enforced |
| `CoreSignal` | Protobuf message | Control Plane buffer | Single event | Rust Core → UI only |

### 3.2 Local Proxy Objects

| Object | Type | Storage | Lifetime | Notes |
|---|---|---|---|---|
| `ProxySession` | `{token, cas_hash, port, expires_at}` | RAM (Rust Core) | TTL 60s | One-time token |
| `StreamChunkBuffer` | Decrypted chunk bytes | RAM streaming buffer | Per chunk (~2MB) | ZeroizeOnDrop after stream |

### 3.3 Platform IPC Objects

| Platform | Control Plane | Data Plane |
|---|---|---|
| 📱 iOS (Flutter) | Dart FFI + Protobuf | Dart FFI TypedData (read-only) |
| 📱 Android (Flutter) | Dart FFI + Protobuf | Dart FFI TypedData (read-only) |
| 💻 macOS (Tauri) | Tauri IPC (JSON-RPC) | SharedArrayBuffer |
| 💻 Windows (Tauri) | Tauri IPC (JSON-RPC) | SharedArrayBuffer |
| 🖥️ Linux (Tauri) | Tauri IPC (JSON-RPC) | SharedArrayBuffer |

---

## §4 — PROTOCOL & EXECUTION CONTRACT

### 4.1 FFI Token Protocol

```rust
// Rust Core exposes these C-compatible functions to UI layer:

/// Acquire a buffer token — UI receives opaque handle, not raw pointer
#[no_mangle]
pub extern "C" fn tera_buf_acquire(
    table: u32,      // Which data table to snapshot
    version: u64,   // Request specific version
) -> u64;           // Returns TeraBufToken; 0 = error

/// Read from token into UI-side buffer (zero-copy)
#[no_mangle]
pub extern "C" fn tera_buf_read(
    token: u64,
    offset: usize,
    out_buf: *mut u8,
    len: usize
) -> i32;           // Bytes read; -1 = error

/// Release token — Rust Core ZeroizeOnDrop buffer
#[no_mangle]
pub extern "C" fn tera_buf_release(token: u64) -> i32;

/// Send UICommand to Core (Control Plane)
#[no_mangle]
pub extern "C" fn tera_ui_command(
    cmd_ptr: *const u8,
    cmd_len: usize
) -> i32;           // 0 = success; error code otherwise
```

### 4.2 UICommands (UI → Core)

```protobuf
// UI sends these to Rust Core via Control Plane
enum UICommandType {
    SEND_MESSAGE = 1;
    MARK_READ = 2;
    OPEN_CHAT = 3;
    REQUEST_BLOB_STREAM = 4;  // Triggers Local Proxy session
    TRIGGER_AI = 5;
    APPROVE_ACTION = 6;       // Triggers ApprovalSignature in TERA-GOV
    LOGOUT = 7;
}

message UICommand {
    UICommandType type = 1;
    bytes payload = 2;  // Protobuf-encoded command-specific data
    uint64 request_id = 3;
}
```

### 4.3 CoreSignals (Core → UI)

```protobuf
// Rust Core pushes these to UI via Control Plane
enum CoreSignalType {
    STATE_CHANGED = 1;          // New data available — UI must pull snapshot
    NETWORK_STATUS_CHANGED = 2;
    AI_RESPONSE_READY = 3;
    BLOB_STREAM_READY = 4;      // Local proxy URL ready
    ERROR = 5;
    SECURITY_ALERT = 6;         // DMA intrusion, attestation fail, etc.
}

message CoreSignal {
    CoreSignalType type = 1;
    bytes payload = 2;
    uint64 correlation_id = 3;  // Links to UICommand.request_id
}
```

### 4.4 Unidirectional State Sync Protocol

```text
[Any state change in Rust Core]
     │ (CRDT event merged, message received, etc.)
     │
[Rust Core emits: StateChanged(table="messages", version=1042)]
     │
[UI receives CoreSignal via Control Plane listener]
     │
[UI: tera_buf_acquire(table=MESSAGES, version=1042)]
     │ returns TeraBufToken
     │
[UI: tera_buf_read(token, offset, buf, len) in chunks]
     │ reads Protobuf-serialized snapshot
     │
[UI: render from snapshot data]
     │
[UI: tera_buf_release(token)]
     │ Core: ZeroizeOnDrop snapshot buffer
```

Key properties:

- **No polling.** UI only pulls when `StateChanged` signal received.
- **No push JSON.** Large data transferred via zero-copy Data Plane.
- **Version-gated.** UI specifies version; Core returns that exact snapshot.

### 4.5 Streaming Decryption Proxy Protocol

```text
[User taps video file in chat]
     │
[UI sends: UICommand(REQUEST_BLOB_STREAM, {cas_hash: "blake3:..."})]
     │
[Rust Core:]
     ├── Creates ProxySession (one-time token, dynamic port, TTL 60s)
     ├── Opens localhost HTTP listener on random port
     └── Emits: CoreSignal(BLOB_STREAM_READY, {url: "http://127.0.0.1:PORT/blob/TOKEN"})
     │
[UI: passes URL to AVPlayer/ExoPlayer]
     │
[Player sends HTTP request to 127.0.0.1]
     │
[Rust Core Proxy:]
     ├── Validate TOKEN (one-time, TTL check)
     ├── Fetch encrypted chunk 1 from NAS/MinIO
     ├── Decrypt: AES-256-GCM with ChunkKey
     ├── Stream bytes to HTTP response
     ├── ZeroizeOnDrop ChunkKey
     ├── Fetch chunk 2... repeat
     └── Close HTTP response when all chunks done
```

---

## §5 — STATE MACHINE

### 5.1 UI Render Lifecycle

```text
[APP_LAUNCH]
     │
[RUST_CORE_INIT] ← wait for Rust Core ready signal
     │
[IDLE] ← UI shows cached snapshot (if any)
     │
[StateChanged received]
     │
[PULLING_SNAPSHOT]
     │ tera_buf_acquire()
     │ tera_buf_read()
     │ tera_buf_release()
     ▼
[RENDERING] — Phân tuyến Render Boundary
     │
     ├── Nếu là tĩnh: [TeraChat Message Boundary] 
     │   → Map Markdown string thẳng ra Native Text Widget.
     │
     └── Nếu là tương tác: [Workspace Widget Loader]
         │ (Snapshot cung cấp `WidgetDataState` theo DataGrant)
         ├── State: NeverLoaded (Skeleton tĩnh)
         ├── State: Restoring (Slide Up content)
         ├── State: StaleServing (Render cũ + Amber Dot indicator)
         └── State: Fresh (Render nội dung đầy đủ)
     │
     ▼
[IDLE]
```

### 5.2 Local Proxy Lifecycle

```text
[REQUEST_BLOB_STREAM command]
     │
[PROXY_CREATING] ← Rust Core opens localhost HTTP
     │ port allocated
     ▼
[PROXY_READY] ── CoreSignal(BLOB_STREAM_READY, url)
     │ player requests bytes
     ▼
[STREAMING] ── chunk fetch + decrypt loop
     │ all chunks done OR player closes connection
     ▼
[PROXY_CLOSED] ── ZeroizeOnDrop all ChunkKeys
     │ token invalidated
     ▼
[IDLE]
```

---

## §6 — API / IPC / EVENT BUS

### 6.1 Dart FFI Bindings (Flutter)

```dart
// Flutter side bindings
class TeraCore {
  static final DynamicLibrary _lib = Platform.isIOS
      ? DynamicLibrary.process()
      : DynamicLibrary.open('libterachat_core.so');

  // FFI Token Protocol
  static final _bufAcquire = _lib.lookupFunction<
      Uint64 Function(Uint32, Uint64),
      int Function(int, int)>('tera_buf_acquire');

  static final _bufRelease = _lib.lookupFunction<
      Int32 Function(Uint64),
      int Function(int)>('tera_buf_release');

  // Signal listener (Core → UI)
  static void onCoreSignal(CoreSignal signal) {
    switch (signal.type) {
      case CoreSignalType.STATE_CHANGED:
        _pullAndRender(signal.payload);
        break;
      case CoreSignalType.BLOB_STREAM_READY:
        _openMediaPlayer(signal.payload.url);
        break;
      // ...
    }
  }
}
```

### 6.2 CoreSignals Reference

| Signal | Payload | UI Action |
|---|---|---|
| `STATE_CHANGED(table, version)` | Table identifier + version | Pull snapshot via tera_buf_acquire |
| `NETWORK_STATUS_CHANGED(status)` | `{protocol, latency_ms}` | Update network indicator |
| `AI_RESPONSE_READY(session_id)` | Encrypted response token | Pull + decrypt response |
| `BLOB_STREAM_READY(url, token)` | Local proxy URL | Open native media player |
| `SECURITY_ALERT(type, severity)` | Alert details | Show security modal |
| `ERROR(code, message)` | Error details | Show error toast |

---

## §7 — PLATFORM MATRIX & CONSTRAINTS

| Feature | 📱 iOS | 📱 Android | 💻 macOS | 💻 Windows | 🖥️ Linux |
|---|---|---|---|---|---|
| IPC mechanism | Dart FFI + XPC | Dart FFI + Binder | Tauri IPC | Tauri IPC | Tauri IPC |
| Data Plane | Dart FFI TypedData | Dart FFI TypedData | SharedArrayBuffer | SharedArrayBuffer | SharedArrayBuffer |
| Local Proxy | ✅ (127.0.0.1 only) | ✅ (127.0.0.1 only) | ✅ (127.0.0.1 only) | ✅ (127.0.0.1 only) | ✅ (127.0.0.1 only) |
| Control Plane max size | 1KB | 1KB | 1KB | 1KB | 1KB |
| StateChanged polling | ❌ (signal-driven) | ❌ (signal-driven) | ❌ (signal-driven) | ❌ (signal-driven) | ❌ (signal-driven) |
| UI state storage | ❌ (pure renderer) | ❌ (pure renderer) | ❌ (pure renderer) | ❌ (pure renderer) | ❌ (pure renderer) |
| Crypto in UI thread | ❌ (forbidden) | ❌ (forbidden) | ❌ (forbidden) | ❌ (forbidden) | ❌ (forbidden) |

---

## §8 — NON-FUNCTIONAL REQUIREMENTS (NFR)

| Requirement | Target | Notes |
|---|---|---|
| StateChanged → render latency | < 16ms | 60fps target |
| tera_buf_acquire throughput | > 400MB/s | Dart FFI TypedData |
| Local Proxy first-byte latency | < 200ms | From UICommand to player start |
| UICommand dispatch latency | < 5ms | Control Plane round-trip |
| Control Plane message size | < 1KB | Hard limit, reject larger |
| Local Proxy concurrent sessions | Max 3 | One per active media player |
| Snapshot buffer ZeroizeOnDrop | < 1ms | After tera_buf_release |

---

## §9 — SECURITY & THREAT MODEL

| Attack | Vector | Mitigation |
|---|---|---|
| State extraction via snapshot | UI caches snapshot to disk | Snapshot is `ZeroizeOnDrop`; called after `tera_buf_release`; UI must NOT write to disk |
| Proxy SSRF | Player requests non-localhost URL via proxy redirect | Proxy binds to 127.0.0.1 ONLY; no redirect follow |
| Token replay | Reuse one-time proxy token | Token invalidated immediately after first use; TTL 60s |
| Control Plane flooding | UI sends huge command | 1KB hard limit; reject + log |
| Crypto in UI thread | Dev accidentally calls crypto in Dart | No crypto function exported via FFI; Dart has no access to ring/RustCrypto |
| Shared data plane mutation | UI writes to SharedArrayBuffer | UI has read-only view of Data Plane buffer |
| XPC process injection (macOS) | Injected code in terachat-wasm-worker | XPC Service uses hardened runtime; signature verification on launch |

---

## §10 — FAILURE MODEL & RECOVERY

| Failure | Detection | Recovery |
|---|---|---|
| Rust Core crash | IPC connection dropped | UI shows "Reconnecting..." modal; Rust Core restarts; UI re-registers signal listener |
| StateChanged but snapshot unavailable | `tera_buf_acquire` returns 0 | UI retries 3x with backoff; shows stale snapshot with "Syncing..." indicator |
| Local Proxy port conflict | Port bind fails | Retry with next random port; up to 5 attempts |
| Local Proxy TTL expired | Player requests after 60s | Player receives 401; UI re-requests new proxy session |
| Control Plane message > 1KB | Size check before send | Rejected; UI dev error logged; never sent to Core |
| NAS unreachable during streaming | Chunk fetch fails | Proxy sends HTTP 503; player shows buffering; UI shows "File temporarily unavailable" |
| Platform IPC broken (Android Binder death) | Binder callback failure | Rust Core reinitializes IPC; UI re-subscribes |

## §11 — ARCHITECTURAL INVARIANTS & AUDIT RESOLUTIONS (IPC & UI)

### 11.1 TeraChat Message as Core-Integrated UI Plugin

**Constraint:** Subjecting fundamental UI messaging routines to the WASM payload boundary generates unnecessary double-encryption and context-switching overhead.
**Resolution:** The `TeraChat Message` component is exempt from the `.tapp` architecture. It resides natively within the `terachat-ui/` bounds (Apache 2.0). A designated CI lint ensures no WASM-compiled code interferes with direct core message rendering pathways.

### 11.2 Cross-Tapp IPC During Mesh Mode

**Constraint:** Offline IPC requests interacting with remote CAS (Content Addressed Storage) fail silently without clear protocol state handling.
**Resolution:** `IpcPayload` supports defined `ResolutionMode` contexts (`RequireOnline`, `LocalCacheOnly`, `Deferred`). Unresolvable Mesh requests utilizing `Deferred` are cached under `PENDING_ONLINE` in `hot_dag.db` and securely delivered once internet connectivity is re-established via `CoreSignal::IpcDeferredDelivery`.

### 11.3 Workspace Widget Loading States

**Constraint:** Blanket UI states misrepresent granular DataGrant lifecycles (Rotation vs. Revocation), creating a jarring User Experience.
**Resolution:** Workspace widget rendering is governed by a `WidgetDataState` matrix computed strictly by Rust Core:

- `NeverLoaded`: Fresh initialization (Render Shimmer Skeleton).
- `StaleServing`: Policy rotation in progress (Render older data with subtle amber synchronization dot).
- `Restoring`: Post-revocation access reinstatement (Render Skeleton with "Restoring access").
- `Fresh`: Fully updated state (Standard render).

### 11.4 Strict Engineering Guardrails (Signals)

- **Rule 6 (Security Priority Channel):** Security Events (`ModelIntegrityViolation`, `DataGrantRevoked`) must never reside in standard asynchronous queues. They are dispatched through a dedicated **Synchronous Priority Channel** polled at the beginning of every UI frame, preventing data backlog (DAG merge pressure) from suppressing urgent security interventions.
