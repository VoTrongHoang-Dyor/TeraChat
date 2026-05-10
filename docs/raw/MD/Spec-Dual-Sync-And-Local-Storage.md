# Spec-Dual-Sync-And-Local-Storage.md — TeraChat Enterprise Platform

```yaml
# DOCUMENT IDENTITY
id: "TERA-SYNC"
title: "TeraChat — Dual-Sync Architecture & Local Storage Specification"
version: "1.0.0"
status: "ACTIVE"
audience: "System Architect, Backend Engineer, Rust Core Dev"
purpose: "Đặc tả kiến trúc lưu trữ và đồng bộ hóa hai tầng: Message Sync Plane (CRDT DAG) và App State Sync Plane (Relational Encrypted). Giải quyết điểm nghẽn chí mạng nhất của TeraChat khi scale lên Enterprise Platform."
depends_on: ["TERA-CORE"]
constraints_global:
  - "CRDT DAG chỉ dùng cho Chat, Collaborative Text, Presence — KHÔNG dùng cho Finance/HR"
  - "App State Sync Plane dùng Vector-Clock Relational Sync cho structured data"
  - "ZeroizeOnDrop bắt buộc cho mọi key material trong storage"
  - "Mọi thay đổi schema DB phải backward-compatible với WAL replay"
  - "Tombstone Vacuum bắt buộc — không để CRDT DB phình to vĩnh viễn"

```

> **Status:** `ACTIVE — Implementation Reference`
> **Audience:** Backend Engineer · Rust Core Dev · System Architect
> **Last Updated:** 2026-03-29
> **Depends On:** → TERA-CORE
> **Consumed By:** → TERA-RUNTIME · → TERA-ENCLAVE · → TERA-CLIENT

---

## §1 — EXECUTIVE SUMMARY & TRUST BOUNDARIES

### 1.1 Mục tiêu & Trách nhiệm

File này **chịu trách nhiệm** cho:

- Kiến trúc dual-sync: Message Plane (CRDT) vs App State Plane (Relational)
- SQLite WAL hai tầng (`hot_dag.db` và `cold_state.db`)
- Blob/File storage (BLAKE3 CAS + AES-256-GCM chunks)
- Blind RAG local vector indexing
- Tombstone Vacuum & State Pruning
- Smart Inbox threading (sub-DAG model)
- Cross-device app state sync (delta-sync cho structured data)

File này **KHÔNG chịu trách nhiệm** cho:

- Crypto primitives → `TERA-CORE`
- WASM sandbox execution → `TERA-RUNTIME`
- AI inference & Enclave → `TERA-ENCLAVE`
- UI rendering → `TERA-CLIENT`

### 1.2 Vấn đề cốt lõi: Sự chuyển dịch lớn nhất

> **CRDT (Eventual Consistency) tuyệt vời cho chat, nhưng thảm họa cho dữ liệu tài chính/kho bãi (cần Strong Consistency).**

TeraChat hiện tại ép mọi thứ vào `hot_dag.db` (Append-only CRDT). File này định nghĩa kiến trúc tách biệt hoàn toàn hai planes:

| Plane | Engine | Use case | Consistency |
|---|---|---|---|
| **Message Sync Plane** | CRDT DAG (`hot_dag.db`) | Chat, Presence, Threads | Eventual |
| **App State Sync Plane** | Vector-Clock Relational | CRM, Tasks, structured data | Strong |
| **File/Blob Sync Plane** | BLAKE3 CAS + chunks | Media, documents | Content-addressed |

### 1.3 Trust Boundaries

| Boundary | Bên trong tin tưởng | Bên ngoài không tin tưởng |
|---|---|---|
| `hot_dag.db` (SQLCipher) | CRDT events, message state | Raw disk, OS |
| `cold_state.db` (SQLCipher) | Structured app data | WASM sandbox directly |
| App State Sync Plane | Delta-sync transactions | Other tenants |
| Blob CAS Storage | Content-hashed chunks | File metadata |

---

## §2 — SYSTEM ARCHITECTURE

### 2.1 Dual-Sync Architecture Overview

```text
┌─────────────────────────────────────────────────────────────┐
│                    CLIENT DEVICE                             │
│                                                              │
│  ┌─────────────────────────┐  ┌────────────────────────────┐│
│  │  MESSAGE SYNC PLANE     │  │  APP STATE SYNC PLANE      ││
│  │  (CRDT DAG)             │  │  (Relational Encrypted)    ││
│  │                         │  │                            ││
│  │  hot_dag.db (SQLCipher) │  │  cold_state.db (SQLCipher) ││
│  │  ├─ CRDT_Events         │  │  ├─ .tapp row-level data   ││
│  │  ├─ HLC_Timestamps      │  │  ├─ CRM records            ││
│  │  ├─ Tombstone_Stubs     │  │  ├─ Task state             ││
│  │  └─ Thread sub-DAGs     │  │  └─ Structured metadata    ││
│  └─────────────────────────┘  └────────────────────────────┘│
│                                                              │
│  ┌──────────────────────────────────────────────────────────┐│
│  │  FILE/BLOB SYNC PLANE                                    ││
│  │  BLAKE3 CAS + AES-256-GCM 2MB chunks                    ││
│  │  NAS / MinIO (content-addressed, blind storage)          ││
│  └──────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
        │ CRDT Gossip          │ Delta-State Push        │ Chunk sync
        ▼                      ▼                         ▼
[Mesh Peers]           [Relay VPS (blind)]         [NAS/MinIO]
```

### 2.2 SQLite WAL Architecture

**Hai tầng DB chính:**

```text
hot_dag.db  — Append-only CRDT DAG
  WAL mode ON
  SQLCipher AES-256
  wal_autocheckpoint = 1000
  Purpose: Chat events, MLS epoch ratchet, tombstones, mesh sync

cold_state.db — Relational Encrypted App State
  WAL mode ON
  SQLCipher AES-256
  Row-level encryption (DEK per tenant)
  Purpose: CRM, Tasks, structured .tapp data, pin/tag metadata
```

### 2.3 Blob & File Storage Pipeline

```text
[File Upload]
     │
     ├── Chunker (2MB chunks, BLAKE3 CAS hash per chunk)
     ├── AES-256-GCM encrypt each chunk (ChunkKey per file)
     ├── Upload to MinIO/NAS via blind CAS path
     └── Store Zero-Byte Stub in hot_dag.db
              │
              │ (file_name, cas_hash, encrypted_thumbnail, storage_ref)

[File Download]
     │
     ├── Fetch encrypted chunks from MinIO/NAS
     ├── Decrypt chunks sequentially (ChunkKey)
     └── Stream via Local Decryption Proxy (127.0.0.1)
```

---

## §3 — DATA MODEL & ENCRYPTION STATE

### 3.1 Domain: DAG State Objects

| Object | Type | Storage | Lifecycle | Security Constraint |
|---|---|---|---|---|
| `CRDT_Event` | Typed append-only log entry | `hot_dag.db` (WAL) | Permanent (Append-Only) | Ed25519 signed per event |
| `HLC_Timestamp` | `{wall_clock, logical_counter}` | Embedded in every Event | Attached to event, immutable | No SystemTime::now() — use HLC |
| `Tombstone_Stub` | `{entity_id, hlc, type=DELETED}` | `cold_state.db` | Permanent (never physically deleted) | Luật 4: Immutable CRDT Rules |
| `Proof_Bundle` | `{Ed25519_Sig, HLC, Evidence}` | Encrypted broadcast | Until dispute resolved | Hardware-bound non-repudiation |
| `AppendBlock` | `{id, content, timestamp, device_sig}` | RAM / `hot_dag.db` | Pending until Desktop reconcile | Ed25519 signed; Optimistic mode |
| `Hash_Frontier` | `{Vector_Clock, Root_Hash}` | `hot_dag.db` | Updated on every Gossip round | BLAKE3 integrity |
| `Hydration_Checkpoint` | `{Snapshot_CAS_UUID, last_chunk_index}` | `hot_dag.db` | Overwritten on each successful chunk | Atomic pre-write before batch |
| `Tapp_Extensibility_Payload` | `Option<Vec<u8>>` | Inside `CRDT_Event` | Cùng event life | Message Event có thể đính kèm dữ liệu từ Tapp (ví dụ: Task Card đính kèm vào tin nhắn) |

### 3.2 Domain: App State Objects (Relational Encrypted — MỚI)

| Object | Type | Storage | Lifecycle | Security Constraint |
|---|---|---|---|---|
| `AppStateRow` | Row-level encrypted record | `cold_state.db` | Mutable with vector-clock | DEK per tenant, AES-256-GCM |
| `VectorClock` | `HashMap<DeviceId, u64>` | Embedded in AppStateRow | Monotonically increasing | Merge on conflict via last-write-wins with clock |
| `DeltaPatch` | `{row_id, field_changes, vector_clock, author}` | In-flight (encrypted) | Ephemeral; applied on receipt | Ed25519 signed |
| `ConflictMarker` | `{row_id, conflict_clock, candidates}` | `cold_state.db` | Until user resolves | Requires explicit resolution for financial data |

### 3.3 Domain: Blob/File Objects

| Object | Type | Storage | Lifecycle | Security Constraint |
|---|---|---|---|---|
| `FileChunk` | AES-256-GCM ciphertext, 2MB | NAS / MinIO (CAS path) | Permanent until file deleted | BLAKE3 content hash |
| `ZeroByteStub` | `{file_name, cas_hash, encrypted_thumbnail, storage_ref}` | `hot_dag.db` | Lifetime of file | Lưu trong DAG, không expose file content |
| `ChunkManifest` | `{chunk_hashes[], total_size, encryption_header}` | In-flight + `cold_state.db` | Per file | Ed25519 signed by uploader |

### 3.4 Domain: Search Index Objects

| Object | Type | Storage | Scope | Notes |
|---|---|---|---|---|
| `FTS5Index` | SQLite FTS5 | `hot_dag.db` local | Last 30 days of chat | Plain text (under SQLCipher) |
| `ZKMemoryIndex` | Zero-Knowledge Memory Agent | NAS / Mac mini local LLM cluster | Documents & long-term context | **Replaces** Blind RAG / BlindVectorIndex. ZK invariant: no vector embeddings leave device boundary. |
| `SearchableEncryptedField` | AES-SIV deterministic encrypted | `cold_state.db` | CRM/App structured fields | Allows exact-match lookup without decryption |

---

## §4 — PROTOCOL & EXECUTION CONTRACT

### 4.1 CRDT Message Sync Protocol

**Gossip Round:**

```text
[Device A] ──sends Hash_Frontier──> [Device B]
                                          │
                              Compare with local frontier
                                          │
                         ┌────────────────┴────────────────┐
                         │ Frontier diverged                │ In sync
                         ▼                                  ▼
               Send missing CRDT_Events            No-op (ack)
                         │
               Merge + apply to hot_dag.db
                         │
               Update Hash_Frontier
```

**Conflict Resolution & Integrity Rules (CRDT):**

- **LWW (Last Write Wins):** Text/presence data phân giải theo HLC timestamp.
- **Sub-DAG Lazy-loading:** Thread merges thông qua việc đính kèm `reply_to_id`. Chỉ load khi mở Thread.
- **Luật 4 (Immutable CRDT Rules):** Tombstone_Stub là bất biến và lưu mãi mãi (cho tới khi Vacuum). Mọi thao tác chỉnh sửa CRDT Event đã kí Hash là bị cấm. Nếu sửa, Hash thay đổi -> Event Mới. Suy ra App State Sync cũng phải chạy theo Immutable Event Sourcing nếu liên kết dữ liệu với `Tapp_Extensibility_Payload`.
- **EMDP Tainted Events (Enterprise Mesh Data Protection):** 
  - Do Tapp có thể đính kèm payload tuỳ ý vào lưới chat qua `Tapp_Extensibility_Payload`.
  - Nếu Mesh Sync node phát hiện Event chứa Tapp Payload vượt quá DataGrant, vi phạm Schema, hoặc size quá khổ -> Event đó sẽ bị flag là **`Tainted`**.
  - Node nội hạt (LAN/Mesh) nhận được event `Tainted` sẽ filter bỏ đi payload độc hại, bảo vệ các node xung quanh khỏi lỗi dây chuyền, nhưng vẫn giữ nguyên vẹn core Message Text để không đứt chuỗi DAG.
- **EMDP Forced Flag (Border Node Failure):**
  - Khi Border Node failure dẫn đến EMDP_FORCED kích hoạt:
    - `hot_dag.db` trên iOS ghi nhận `emdp_forced_reason: "border_node_lost"`
    - Mọi CRDT_Event trong window này mang flag `emdp_forced: true`
    - Khi Desktop reconnect: events này được ưu tiên merge nhưng bị **quarantine** trong review queue trước khi apply vào `cold_state.db` Finance/HR
    - Lý do: không muốn 60 phút quyết định phê duyệt chi tiêu trong EMDP forced mode được merge silently vào ledger chính thức.

### 4.2 App State Sync Protocol (Relational — MỚI)

**Delta-Sync Flow:**

```text
[Device A modifies AppStateRow]
     │
     ├── Increment VectorClock[device_A]
     ├── Create DeltaPatch (field_changes + new vector_clock)
     ├── Ed25519 sign DeltaPatch
     └── Push to relay (encrypted)

[Device B receives DeltaPatch]
     │
     ├── Verify Ed25519 signature
     ├── Compare VectorClocks
     │     ├── No conflict → apply patch
     │     └── Conflict detected → mark ConflictMarker
     │                           ── notify user if financial data
     └── Update local cold_state.db
```

**Conflict Resolution (Relational):**

- Non-financial data: LWW by vector clock.
- Financial data: **NEVER auto-merge.** ConflictMarker persistent until explicit resolution.
- HR data: Server Enclave as Single Source of Truth (see TERA-ENCLAVE).

### 4.3 File Blob Sync Protocol

1. Chunker splits file into 2MB chunks, computes BLAKE3 hash per chunk.
2. Each chunk encrypted with `ChunkKey` (AES-256-GCM, ephemeral per file).
3. Chunks uploaded to MinIO/NAS via CAS path (`/cas/<BLAKE3_hash>`).
4. `ZeroByteStub` stored in `hot_dag.db` as CRDT event.
5. Recipients receive Stub in message sync, download chunks on-demand.

### 4.4 Smart Inbox & Threading (Sub-DAG model)

**Thread Model:**

- Threads modeled as sub-DAG branches off main message DAG.
- `reply_to_id` links child messages to parent.
- Mobile: lazy-load thread contents; only load thread root in Smart Inbox.
- Pin/Tags stored as metadata overlay in `cold_state.db` — NOT in main DAG.

### 4.5 Search Strategy

| Data Type | Engine | Scope | Notes |
|---|---|---|---|
| Chat text (< 30 days) | SQLite FTS5 (local) | On-device only | Fast, private |
| Chat text (> 30 days) | ZK Memory Search (ZKMemoryIndex) | Mac mini local cluster | Zero-Knowledge: data never leaves device boundary |
| App/CRM fields | Searchable Symmetric Encryption (AES-SIV) | Server-side exact match | Server sees hash, not data |
| Documents/PDFs | ZK Memory Agent (local LLM consolidation) | Mac mini / NAS Enclave | **Replaces Blind RAG.** AI context built locally; no embedding egress. |

### 4.6 Tombstone Vacuum Policy

- Vacuum trigger: `hot_dag.db` WAL size > 500MB OR — trigger by growth rate: nếu WAL growth > 10MB/hour, trigger incremental vacuum trong background.
- Eligible for vacuum: Tombstone_Stubs older than 365 days with no legal hold flag.
- Vacuum log: Append to append-only `Audit_Log_Entry` with Ed25519 signature.
- Mobile: Vacuum during charging + Wi-Fi only window.
- Metric `wal_growth_rate_mb_per_hour` được thêm vào `ClientMetricBatch` để proactive monitoring. Weekly schedule là insufficient cho Enterprise tier (50MB/ngày × 10 = 500MB trong 10 ngày không vacuum).

---

## §5 — STATE MACHINE

### 5.1 CRDT Sync State

```text
[ONLINE] ──sync complete──> [IN_SYNC]
    │                           │
    │ network drops             │ CRDT event received
    ▼                           ▼
[OFFLINE_QUEUE]          [MERGING]
    │                           │
    │ reconnect                 │ merge complete
    ▼                           ▼
[REPLAY_DELTA]           [IN_SYNC]
    │
    │ replay complete
    ▼
[IN_SYNC]
```

### 5.2 File Sync State

```text
[PENDING_UPLOAD]
     │ chunk 1 of N uploaded
     ▼
[UPLOADING (chunk n/N)]
     │ error
     ├──> [RETRY_CHUNK (Hydration_Checkpoint stored)]
     │ all chunks done
     ▼
[STUB_PUBLISHED]
     │ recipient requests download
     ▼
[DOWNLOADING]
     │ streaming via local proxy
     ▼
[AVAILABLE_LOCAL]
```

---

## §6 — API / IPC / EVENT BUS

### 6.1 Storage Signals

| Signal | Trigger | Consumer |
|---|---|---|
| `StateChanged(table, version)` | Any write to hot_dag.db or cold_state.db | TERA-CLIENT (pull snapshot) |
| `DagVacuumTriggered(size_before_mb, size_after_mb)` | Tombstone vacuum completed | TERA-GOV (audit log) |
| `ConflictDetected(row_id, table)` | Vector clock conflict in App State | TERA-CLIENT (notify user) |
| `ChunkUploadProgress(file_id, chunk_n, total_n)` | Chunk uploaded | TERA-CLIENT (progress bar) |
| `SearchIndexUpdated(scope)` | FTS5 index rebuilt | TERA-CLIENT |

### 6.2 API Surface for WASM (.tapp) — via Host ABI only

```rust
// .tapp may NOT directly access hot_dag.db or cold_state.db
// All access goes through Rust Core Host Functions:

extern "C" {
    // Query structured app data (sanitized SQL, no joins across tenants)
    fn host_app_state_query(sql_ptr: *const u8, sql_len: usize, out_ptr: *mut u8) -> i32;
    
    // Write delta patch to App State Sync Plane
    fn host_app_state_write(patch_ptr: *const u8, patch_len: usize) -> i32;
    
    // Request blob download (returns local proxy URL)
    fn host_blob_request(cas_hash_ptr: *const u8) -> i32;
}
```

---

## §7 — PLATFORM MATRIX & CONSTRAINTS

| Constraint | 📱 iOS | 📱 Android | 💻🖥️ Desktop | ☁️ VPS |
|---|---|---|---|---|
| DB location | App sandbox only | App sandbox | User data dir | /data/terachat/ |
| WAL max size | 200MB (Jetsam risk) | 500MB | 2GB | 10GB |
| Background vacuum | Charging + Wi-Fi window only | Background service | Scheduled task | Cron job |
| FTS5 index max | 30 days rolling | 30 days rolling | Unlimited | N/A |
| Chunk size | 2MB (memory constraint) | 2MB | 2MB | 2MB |
| Blob sync background | ❌ (Jetsam) | ✅ limited | ✅ | ✅ |
| App State WASM access | Via Host ABI only | Via Host ABI only | Via Host ABI only | N/A |

---

## §8 — NON-FUNCTIONAL REQUIREMENTS (NFR)

| Requirement | Target | Notes |
|---|---|---|
| CRDT merge latency | < 100ms for 10k events | P95, mobile |
| App State delta-sync | < 500ms round-trip | Via relay |
| FTS5 search response | < 50ms | Last 30 days, local |
| Blob chunk upload throughput | > 10MB/s | On Wi-Fi |
| hot_dag.db max size (mobile) | < 1GB before vacuum | Trigger at 500MB |
| Tombstone vacuum duration | < 30s | Background window |
| ConflictMarker resolution | User-driven, no auto-merge for financial | Manual only |
| Schema migration | Zero-downtime, backward-compatible | WAL replay safe |

---

## §9 — SECURITY & THREAT MODEL

| Attack | Vector | Mitigation |
|---|---|---|
| DB extraction from stolen device | cold_state.db copied | SQLCipher AES-256; key in Secure Enclave |
| Split-brain financial data | Network partition + CRDT merge | Financial data NEVER in CRDT; Server Enclave only |
| Search leaking data via pattern | FTS5 index readable if DB decrypted | FTS5 under SQLCipher; Blind Index for remote |
| Blob replay attack | Old chunk hash re-submitted | CAS = content-addressed; duplicate chunk = same hash, ignored |
| Tombstone evasion | Attacker re-constructs deleted data | Tombstones permanent; replayed events with deleted tombstone rejected |
| DAG flooding via mesh | Attacker pushes massive CRDT events | Rate limit per device_id; malformed events rejected by Ed25519 verify |

---

## §10 — FAILURE MODEL & RECOVERY

| Failure | Detection | Recovery |
|---|---|---|
| WAL corruption | `PRAGMA integrity_check` at startup | Restore from last `Snapshot_CAS` |
| Chunk upload interrupted | `Hydration_Checkpoint` stores last chunk index | Resume from checkpoint on next attempt |
| App State conflict unresolved | `ConflictMarker` persists | User notified; UI shows conflict resolution UI |
| FTS5 index corruption | FTS5 query returns error | Drop and rebuild FTS5 index (background, non-blocking) |
| cold_state.db schema migration fail | Version mismatch on open | Rollback to previous schema snapshot; alert admin |
| Blob CAS hash mismatch | BLAKE3 verify fail on download | Discard chunk; re-download from source |
| Jetsam kill during vacuum | vacuum_in_progress flag not cleared | On next startup: reset flag; re-trigger vacuum |
| NAS unreachable | Blob sync queued; local stub only | Queue chunks in `Egress_Outbox`; retry on reconnect |


## §8 — ARCHITECTURAL INVARIANTS & AUDIT RESOLUTIONS (SYNC & STORAGE)

### 8.1 EMDP + App Suite Transactional Integrity
**Constraint:** Essential Text-Only operations produced under Enterprise Mesh Data Protection (EMDP) risk state corruption if blindly merged into financial logs following offline epoch rotations.
**Resolution:** Disconnected transactions constructed mid-EMDP embed an `emdp_tainted: true` attribute payload within the `CRDT_Event`. Upon network restabilization, these events bypass automatic `cold_state.db` merges and populate an explicit Human-in-the-Loop review queue, ensuring financial sequence safety.

### 8.2 CRDT_Event content_type Extensibility
**Constraint:** An unversioned `content_type` paradigm triggers synchronization crashes when an older core encounters an unrecognized payload type.
**Resolution:** `content_type` strings adopt strict namespacing (`namespace/type@version`). The Rust Core merge algorithm enforces **unknown-type tolerance**, persisting opaque versions identically into the DAG structure without evaluating payload semantics or halting the mesh sync protocol.

### 8.3 Strict Engineering Guardrails (DAG Immutability)
- **Rule 5 (Immutable Append-Only Events):** Operations against the `hot_dag.db` are strictly append-only. No `UPDATE` or `DELETE` mutation is permitted under any circumstances. Removals or alterations must utilize Compensating Events with explicit causal `parent_id` bindings. CI verification actively blocks mutable DML against the ledger.

### 8.4 Cross-DB Saga Pattern — Atomic Dual-Plane Write (CRIT-01)
**Constraint:** `hot_dag.db` và `cold_state.db` là hai SQLCipher instances độc lập. Không có cross-DB transaction. Nếu write thành công vào `hot_dag.db` nhưng fail ở `cold_state.db` (ví dụ Jetsam-kill trên iOS), CRDT DAG hiển thị "approved" nhưng Finance ledger không có record tương ứng — data inconsistency không phát hiện được.
**Resolution:** Implement **Saga Journal** trong `hot_dag.db` chính nó:

```rust
pub struct SagaEntry {
    saga_id: Uuid,
    crdt_event_id: Uuid,
    cold_state_patch_hash: Blake3Hash,
    status: SagaStatus, // Pending | CrdtCommitted | FullyCommitted | Compensated
    hlc: HLCTimestamp,
}
```

Flow: write Saga `Pending` → write CRDT event → update Saga `CrdtCommitted` → write cold_state → update Saga `FullyCommitted`. Khi restart, scan Sagas có status `CrdtCommitted` và replay cold_state write idempotently. Nếu cold_state write fail, emit compensating Tombstone event vào DAG kèm `error: cold_state_write_failed`. UI nhận signal này và hiển thị "Chờ đồng bộ" thay vì "Đã duyệt".

### 8.5 EMDP-to-NORMAL Atomic Transition (CRIT-05)
**Constraint:** TERA-SYNC §5.1 mô tả state machine `OFFLINE_QUEUE → REPLAY_DELTA → IN_SYNC` nhưng không nói gì về trường hợp iOS Tactical Relay đang mid-write vào `hot_dag.db` WAL khi Desktop reconnect và bắt đầu gửi 1000 CRDT events. SQLite WAL mode không hỗ trợ concurrent writers từ hai process — behavior là undefined.
**Resolution:** Khi Desktop reconnect, Rust Core phải acquire WAL write lock trước khi bắt đầu REPLAY_DELTA. iOS Tactical Relay phải flush và release WAL lock trước khi Desktop bắt đầu replay. Sequence: `iOS: WAL_FLUSH_ACK` → `Desktop: WAL_LOCK_ACQUIRED` → `Desktop: BEGIN REPLAY_DELTA`.

### 8.6 ZK Memory Agent — Blind RAG Removal (DEBT-01)
**Constraint:** `BlindVectorIndex` đang được mô tả như live component trong §3.4 và §4.5 nhưng ZK Memory Agent đã **fully replaced** Vector DB + Embeddings. Documentation drift này gây ra tech debt: engineer mới sẽ implement Blind RAG thay vì ZK Memory Agent.
**Resolution:** `BlindVectorIndex` đã được thay thế hoàn toàn bởi `ZKMemoryIndex` (Zero-Knowledge Memory Agent). Mọi reference đến "Blind RAG", "BlindVectorIndex", "blind vector indexing" trong toàn bộ spec suite phải được cập nhật thành ZK Memory Agent. Search §4.5 đã được cập nhật ở bín dưới.

---

## §9 — DEEP AUDIT RESOLUTIONS (WAVE 2 & 3 — STORAGE SECURITY & SCALABILITY)

### 9.1 CAS Side-Channel Attack — Tenant-Salted Deduplication (SECURITY-CAS-01)
**Constraint:** File blob chunks được lưu theo `cas_hash = BLAKE3(chunk_ciphertext)` global. Nếu cùng một plaintextchunk được 2 user khác nhau upload, `cas_hash` trùng nhau và NAS tiết kiệm 1 bản (global deduplication). Điều này tạo **Proof-of-Existence Side-Channel**: User B có thể biết User A đã upload tài liệu X bằng cách upload cùng tài liệu đó — nếu upload time là `0ms` (dedup bỏ qua write), User B biết tài liệu đã tồn tại trong hệ thống. Dù không đọc được nội dung, việc biết "tài liệu X đã tồn tại" là thông tin tình báo có giá trị trong Gov/Military context.

**Resolution — Workspace-scoped Tenant Salting:**

Thay đổi công thức `cas_hash`:
```
// TRƯỚC (CŨ — không an toàn):
cas_hash = BLAKE3(chunk_ciphertext)

// SAU (MỚI — safe):
cas_hash = BLAKE3(workspace_id || context_salt || chunk_ciphertext)
```

Trong đó:
- `workspace_id`: ID của Workspace/tenant (already available in context).
- `context_salt`: Random 32-byte salt được generate lần đầu khi Workspace khởi tạo, stored in `cold_state.db`, không bao giờ thay đổi.
- Deduplication chỉ hoạt động **trong scope của cùng một Workspace** — hai Workspaces khác nhau với cùng file sẽ không tìm thấy trùng nhau.

**Trade-off:** Mất deduplication cross-tenant (NAS storage tăng tỷ lệ với số Workspace × file). Acceptable trade-off để đổi lấy Zero-Knowledge invariant hoàn toàn.

**Migration:** Existing blobs dùng old `cas_hash` format phải được migrated. Add `hash_version: u8` field vào `BlobManifest`:
- `hash_version = 1`: legacy BLAKE3(content) — read-only support.
- `hash_version = 2`: Tenant-salted BLAKE3 — required for new uploads.

### 9.2 Dual-Hash BLAKE3+SHA-512 — Collision Resistance (SECURITY-HASH-01)
**Constraint:** Toàn bộ blob identity và integrity verification dựa vào duy nhất `BLAKE3`. Dù BLAKE3 có collision resistance tốt (256-bit output), trong một hệ thống Gov/Military lưu trữ decades với hàng tỷ chunks, xác suất **Birthday Attack** không phải zero về mặt lý thuyết. Quan trọng hơn: nếu tồn tại **Chosen-Prefix Collision** (adversarially crafted) trong một thuật toán, dùng hai thuật toán độc lập khác hệ (hash families khác nhau) loại bỏ attack hoàn toàn.

**Resolution — Dual-Hash Identity:**
```
cas_hash = BLAKE3(workspace_id || context_salt || chunk_ciphertext)
         || SHA-512(chunk_ciphertext)[0:32]  // 32 bytes = 256 bits từ SHA-512
```

- `cas_hash` = 32 bytes BLAKE3 + 32 bytes SHA-512/256 = **64 bytes total**.
- Storage key = `BLAKE3_part || SHA512_part` (hex-encoded, lexicographically sortable).
- Tạo ra collision yêu cầu break cả BLAKE3 **VÀ** SHA-512 simultaneously — bất khả thi về mặt toán học trong vũ trụ hiện tại.
- NSA Suite B compliance: SHA-512 là chuẩn được NSA phê duyệt cho classified systems.

**Performance:** SHA-512 trên BLAKE3's throughput target (~GB/s) — overhead < 8% trên modern hardware. Acceptable for Gov/Military.

### 9.3 Tombstone Vacuum — Adaptive Rate-Based Trigger (SCALE-VACUUM-01)
**Constraint:** Spec định nghĩa Tombstone Vacuum trigger tại "WAL size > 500MB OR weekly schedule". Với Enterprise tier (500+ users, 50MB WAL growth/day), weekly schedule không đủ — WAL đạt 500MB trong 10 ngày. Metric `wal_growth_rate_mb_per_hour` đã được thêm vào `ClientMetricBatch` nhưng **không có server-side aggregation logic** để trigger proactive vacuum dựa trên growth rate. Pattern với 5,000 Gov users sẽ gây WAL bloat trong vài ngày sau go-live.

**Resolution — Adaptive Vacuum Trigger:**

Thêm logic vào `tc-store` crate, chạy background vacuum check mỗi 15 phút:

```rust
#[derive(Debug)]
pub struct VacuumPolicy {
    /// Absolute size trigger
    wal_size_trigger_mb: u64,           // Default: 200MB (thay 500MB)
    /// Growth rate trigger  
    growth_rate_trigger_mb_per_hour: f64, // Trigger nếu rate > X MB/h
    /// Time-since-last-vacuum trigger
    max_vacuum_interval_hours: u64,     // Default: 24h (thay weekly)
    /// Gov/Military tier: aggressive
    tombstone_age_days: u32,           // Default: 90 days (thay 365)
}

pub fn should_vacuum(policy: &VacuumPolicy, stats: &WalStats) -> VacuumDecision {
    if stats.size_mb > policy.wal_size_trigger_mb {
        return VacuumDecision::Immediate("size_threshold");
    }
    if stats.growth_rate_mb_per_hour > policy.growth_rate_trigger_mb_per_hour {
        return VacuumDecision::Scheduled("growth_rate_threshold", Duration::from_hours(1));
    }
    if stats.hours_since_last_vacuum > policy.max_vacuum_interval_hours {
        return VacuumDecision::Scheduled("interval_threshold", Duration::from_mins(30));
    }
    VacuumDecision::Skip
}
```

**Tier configuration:**
| Tier | WAL Trigger | Growth Rate Trigger | Max Interval | Tombstone Age |
|---|---|---|---|---|
| Personal | 500MB | 50MB/h | 7 days | 365 days |
| Enterprise | 200MB | 20MB/h | 24h | 180 days |
| Gov/Military | 100MB | 10MB/h | 12h | 90 days |

**Vacuum race protection (SC-40):** Vacuum process phải acquire WAL write lock (TERA-SYNC §8.5 `WAL_FLUSH_ACK` protocol) trước khi begin. Concurrent vacuum instances prevented via `EXCLUSIVE` SQLite lock.

### 9.4 Federation Schema Gap — Cross-Version Warning (SCALE-FED-01)
**Constraint:** TERA-SYNC §6.x định nghĩa Schema Version ±1 minor → read-only federation. Nhưng không có **automated testing** cho cross-version federation compatibility. Enterprise với Branch cluster tại schema v1.2 và HQ tại v1.4 sẽ có read-only federation mà không có bất kỳ warning nào trong UI cho đến khi Admin manually check version table. Trong field deployments (military units thường lag 2-3 versions), đây là common case, không phải edge case.

**Resolution:**

1. **Automated Cross-Version CI Tests:** Thêm `cargo test --test federation_compat -- --all-versions-matrix` vào CI suite. Test matrix: {v_current-2, v_current-1, v_current} × {v_current-2, v_current-1, v_current} = 9 combinations. Block release nếu bất kỳ combo nào fail.

2. **UI Federation Warning:** `CoreSignal::FederationModeChanged` thêm field:
```rust
pub struct FederationModeChanged {
    peer_cluster_id: ClusterId,
    our_schema_version: SchemaVersion,
    peer_schema_version: SchemaVersion,
    federation_mode: FederationMode, // ReadWrite | ReadOnly | Incompatible
    /// Non-empty nếu ReadOnly hoặc Incompatible
    user_visible_reason: Option<String>,
}
```
UI phải hiển thị amber banner: **"Kết nối với chi nhánh [X] đang ở chế độ chỉ đọc do chênh lệch phiên bản. Liên hệ IT Admin để nâng cấp."**

3. **Pre-sync schema negotiation:** Trước khi sync bắt đầu, hai clusters exchange schema version. Nếu > 2 minor versions apart: **block sync, emit `FederationIncompatible`**, không để silent read-only cascade.

### 9.5 Audit Log Scale — Partitioning & Cold Storage Tiering (SCALE-AUDIT-01)
**Constraint:** Gov/Military tier: 7-year retention, 5,000 users × 100 audit events/day = **1.28 tỷ entries** tổng cộng. PostgreSQL append-only table với Merkle verification không thể scale đến đây mà không có horizontal partitioning. Không có spec nào định nghĩa sharding strategy, cold storage tiering, hoặc query performance SLA cho historical audit data.

**Resolution — Time-partitioned Audit Archive:**

```sql
-- PostgreSQL range partitioning by month
CREATE TABLE audit_log (
    id          UUID NOT NULL,
    hlc         BIGINT NOT NULL,
    partition_key VARCHAR(7) NOT NULL, -- 'YYYY-MM'
    event_type  TEXT NOT NULL,
    actor_id    UUID NOT NULL,
    payload_hash BYTEA NOT NULL,
    ed25519_sig  BYTEA NOT NULL,
    prev_hash   BYTEA NOT NULL  -- Merkle chain
) PARTITION BY RANGE (partition_key);

-- Active partitions: last 6 months on NVMe (hot)
-- Months 7-24: on NAS (warm) 
-- Months 25+: compressed archive on S3/MinIO (cold) with integrity proof
```

**Tiering Policy:**
| Age | Storage | Access Time | Verification |
|---|---|---|---|
| 0-6 months | PostgreSQL NVMe (hot) | < 10ms | Real-time Merkle |
| 7-24 months | PostgreSQL HDD/NAS (warm) | < 500ms | On-demand Merkle |
| 25+ months | Compressed archive MinIO (cold) | < 5s | Archive integrity proof |

**Merkle Chain across tiers:** Monthly Merkle root được computed at end-of-month, stored in `cold_state.db` as immutable `AuditMonthlyRoot`. Cold archive integrity verified bằng cách recompute Merkle root và so sánh với stored root — không cần uncompress full archive.

**Query SLA enforcement:** Gov audit queries (`last 30 days`) phải complete trong < 5s. Queries > 30 days đi qua admin-only async job queue.

