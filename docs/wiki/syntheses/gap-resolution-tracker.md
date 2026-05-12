---
type: synthesis
created: 2026-05-11
tags: [terachat, gap, resolution, tracker, architecture]
sources: [tech-debt-registry, tera-core-spec, tera-sync-spec, tera-gov-spec]
status: resolved
resolves: "Điểm yếu #2 — 10 GAPs chưa giải quyết ở mức spec"
---

# GAP Resolution Tracker

10 GAPs (GAP-A đến GAP-J) là lỗ hổng kiến trúc — chúng KHÔNG phải là task lập trình. Mỗi GAP phải được giải quyết ở mức SPEC trước khi có dòng code đầu tiên.

## Nguyên tắc giải quyết

1. Mỗi GAP có MỘT quyết định kiến trúc rõ ràng
2. Quyết định được ghi vào spec gốc (không phải file riêng)
3. Không GAP nào được defer nếu nó block Phase 1 MVP
4. CISO có quyền phủ quyết mọi resolution

---

## GAP-A — SagaRecoveryGuard Undefined

| Thuộc tính | Giá trị |
|------------|--------|
| **Mức độ** | 🔴 CRITICAL — block Phase 2 |
| **Spec** | TERA-SYNC §8.4 |
| **Vấn đề** | `SagaRecoveryGuard` chưa được define. Không có `integrity_check` routine lúc boot scan orphaned `CrdtCommitted` Sagas. Nested failure (Compensating Tombstone cũng fail) chưa có recovery path. |
| **Quyết định cần có** | Protocol chính xác: scan cái gì, rollback ra sao, nested failure recovery path |
| **Trạng thái** | ⏳ Pending resolution |

### Đề xuất resolution

```
SagaRecoveryGuard Protocol:
1. Boot scan: SELECT * FROM saga_journal WHERE status = 'CrdtCommitted'
2. For each orphaned saga:
   a. If compensating_event exists → re-execute compensating action
   b. If compensating fails → create nested saga with WAL savepoint
   c. Max retry: 3 lần, sau đó flag để manual intervention
3. Nested saga failure → escalate lên Admin Console
4. Guard completes < 100ms (scan + retry)
```

**Quyết định (2026-05-12):** Chấp nhận nested saga retry limit = 3. Manual intervention path: Admin Console nhận alert với `saga_id`, `failed_at`, `last_error`. Admin có thể: (a) Retry force, (b) Mark resolved (chấp nhận data divergence có ghi nhận), (c) Escalate lên CISO. Nested saga dùng WAL savepoint riêng — nếu compensating event fail, rollback về savepoint, không ảnh hưởng transaction chính.

---

## GAP-B — WAL Handshake Signals Not in CoreSignal

| Thuộc tính | Giá trị |
|------------|--------|
| **Mức độ** | 🔴 CRITICAL — block Phase 2 |
| **Spec** | TERA-SYNC §8.5, TERA-CLIENT §4.3 |
| **Vấn đề** | `WAL_LOCK_REQUEST`, `WAL_FLUSH_ACK`, `WAL_LOCK_GRANTED` không có trong CoreSignal enum. Resolution tồn tại trên paper nhưng không có implementation contract. |
| **Quyết định cần có** | Contract cụ thể: timeout, backoff strategy, fallback behavior |
| **Trạng thái** | ⏳ Pending resolution |

### Đề xuất resolution

```protobuf
// Thêm vào signals.proto
enum CoreSignal {
    // ... existing signals ...
    WAL_LOCK_REQUEST = 50;   // IPC channel → Store: request WAL lock
    WAL_FLUSH_ACK = 51;      // Store → IPC: WAL flushed, lock granted
    WAL_LOCK_GRANTED = 52;   // Store → IPC: lock acquired
    WAL_LOCK_TIMEOUT = 53;   // Store → IPC: lock failed after timeout
}

// Contract:
// - Timeout: 5s initial, exponential backoff (1s, 2s, 4s, 8s)
// - Desktop fallback: read-only mode if lock not granted
// - Mobile: queue writes to outbox, flush when lock acquired
```

**Quyết định (2026-05-12):** Read-only mode là SILENT — không user-facing notification. Thay vào đó, UI hiển thị indicator "Sync Paused" trên thanh trạng thái. Khi lock acquired → tự động flush queue + indicator biến mất. Nếu lock timeout sau 4 lần retry (tổng 15s) → hiển thị "Connection Issue" banner với nút "Retry Now".

---

## GAP-C — NSE Keychain Semaphore TOCTOU Race

| Thuộc tính | Giá trị |
|------------|--------|
| **Mức độ** | 🔴 CRITICAL — block iOS launch |
| **Spec** | TERA-CORE §11.3 |
| **Vấn đề** | NSE Shared Keychain Semaphore có TOCTOU race condition. `nse_staging.db` là SQLite bị multi-process concurrent write — tái tạo vấn đề mà Saga pattern đang giải quyết. |
| **Quyết định cần có** | POSIX flock() đã test trên iOS App Group container chưa? Memory-mapped ring buffer protocol? |
| **Trạng thái** | ⏳ Pending resolution |

### Đề xuất resolution

```
Thay Keychain semaphore bằng:
1. POSIX flock() trên file trong App Group container
   - iOS supports POSIX locks in shared containers (iOS 14+)
   - flock() is process-safe, not just thread-safe
2. Fallback: memory-mapped ring buffer trong shared memory region
   - Writer (NSE) → ring buffer → Reader (Main App)
   - Atomic write pointer, atomic read pointer
   - Max buffer: 1MB (đủ cho ~100 messages pending)
```

**Quyết định (2026-05-12):** Dùng POSIX flock() trên App Group container file — iOS hỗ trợ POSIX locks trong shared containers từ iOS 14+. Fallback: memory-mapped ring buffer (1MB, ~100 messages pending). Ring buffer protocol: atomic write pointer + atomic read pointer, writer (NSE) advance write, reader (Main App) advance read. Không dùng SQLite cho NSE staging — thay hoàn toàn bằng ring buffer.

---

## GAP-D — MemoryPressureWarning Missing ui_emergency_mode

| Thuộc tính | Giá trị |
|------------|--------|
| **Mức độ** | 🟠 HIGH — block security UX |
| **Spec** | Design.md §8/§12 |
| **Vấn đề** | `CoreSignal::MemoryPressureWarning` không có `ui_emergency_mode: bool` flag. GPU Tier C không được force trước khi render SECURE MEMORY PURGE overlay. |
| **Quyết định cần có** | Add flag vào signal — đơn giản |
| **Trạng thái** | ✅ Resolved — đã có trong Phase 1 task box 1.5 |

### Resolution

```rust
// Trong CoreSignal
MemoryPressureWarning {
    pressure_level: PressureLevel,  // .fair, .serious, .critical
    ui_emergency_mode: bool,        // true = force GPU Tier C
    wal_size_mb: u32,
    available_ram_mb: u32,
}
```

---

## GAP-E — DataGrant Quorum Protocol Undefined

| Thuộc tính | Giá trị |
|------------|--------|
| **Mức độ** | 🔴 CRITICAL — block Gov/Military tier |
| **Spec** | TERA-ECO §8.3, §2.5 |
| **Vấn đề** | DataGrant `generation` counter: node offline khi grant issued không thể distinguish "grant never seen" (gen 0) vs "revoked grant" (gen > 0). Gov-tier quorum protocol undefined. |
| **Quyết định cần có** | Algorithm quorum cụ thể, không phải mô tả |
| **Trạng thái** | ⏳ Pending resolution |

### Đề xuất resolution

```
DataGrant Quorum Protocol:
1. Each node has election_weight (configurable per tier)
2. Grant activation requires: SUM(confirmed_weights) > SUM(all_weights) / 2
3. generation counter: monotonic, per-grant, starts at 1
4. gen = 0 means "never seen this grant" → PENDING_QUORUM
5. gen > 0 means grant exists — check if revoked (revoked_at != null)
6. Unconfirmed grant: return PENDING_QUORUM, DO NOT serve data
7. Quorum gossip: Hash_Frontier with BLAKE3 hash of grant state
```

**Quyết định (2026-05-12):** Dùng weighted voting: `election_weight` được config per-tier (HQ = 3, Regional = 2, Branch = 1, Mobile = 1). Quorum = SUM(confirmed_weights) > SUM(all_weights) / 2. Node offline > 30 phút → weight tạm thời = 0 (không tính vào quorum). DataGrant activation cần quorum. generation counter bắt đầu từ 1 (gen=0 reserved cho "never seen").

---

## GAP-F — Huawei Enterprise Tier Disclosure Missing

| Thuộc tính | Giá trị |
|------------|--------|
| **Mức độ** | 🟡 MEDIUM — compliance |
| **Spec** | TERA-ECO §7 |
| **Vấn đề** | Huawei Enterprise tier "CRL ≤ 4h polling" vi phạm SCIM < 30s SLA — không được disclose trong Pricing_Packages.html |
| **Quyết định cần có** | Update pricing docs — đơn giản |
| **Trạng thái** | ⏳ Pending resolution |

**Quyết định (2026-05-12):** Đã cập nhật `Pricing_Packages.html` với disclosure rõ ràng. Huawei bị loại khỏi Enterprise SLA tier — chỉ hỗ trợ "Standard" tier với SLA 4h. SCIM real-time (<30s) chỉ available trên GMS Android và iOS. Huawei users được thông báo rõ ràng trong app và trong pricing page.

---

## GAP-G — Burner Agent + EMDP Freeze Race

| Thuộc tính | Giá trị |
|------------|--------|
| **Mức độ** | 🔴 CRITICAL — sinh zombie member |
| **Spec** | TERA-CORE §4.3, §11.4 |
| **Vấn đề** | Burner Agent (TTL=60min) expire trong lúc EMDP Epoch Freeze active → Burner Agent thành "zombie member" với keys vẫn valid past TTL |
| **Quyết định cần có** | SC-36 spec resolution |
| **Trạng thái** | ⏳ Pending resolution |

### Đề xuất resolution

```
Burner Agent + EMDP Freeze Intersection:
1. Khi EMDP Epoch Freeze active:
   - Burner Agent removal được queue (không execute ngay)
   - Queue entry: {burner_id, queued_at, ttl_expired_at}
2. Khi Freeze terminated:
   - Process queue: remove tất cả expired Burner Agents
   - Epoch advances SAU KHI tất cả removals hoàn tất
3. Nếu Freeze kéo dài > 2h:
   - Force unfreeze (Admin override)
   - Log forced unfreeze vào audit trail
```

**Quyết định (2026-05-12):** Force unfreeze threshold = 2h (configurable per deployment). Cần 1 Admin signature cho Standard tier, 2 Admin signatures (hoặc 1 CISO) cho Gov/Military tier. Queue độc lập với MLS Epoch rotation — Burner Agent removal được ghi vào `pending_removal_queue` trong cold_state.db. Epoch advance chờ đến khi Freeze terminated HOẶC force unfreeze được approve. Nếu queue có > 10 pending removals → Admin Console alert.

---

## GAP-H — Float Detection Not Required in CI

| Thuộc tính | Giá trị |
|------------|--------|
| **Mức độ** | 🟠 HIGH — block financial .tapp safety |
| **Spec** | TERA-RUNTIME §11.3, TERA-ECO §4.1 |
| **Vấn đề** | Float detection không phải required check trong Static Analysis pipeline. Finance .tapp dùng `f64` vẫn pass security review. |
| **Quyết định cần có** | Add float detection vào CI — đơn giản |
| **Trạng thái** | ✅ Resolved — đã có trong Phase 4 task box 4.2 |

### Resolution

```yaml
# Thêm vào CI pipeline
float-detection:
  tool: "custom LLVM IR scanner"
  rule: "block merge if f32/f64 found in financial .tapp manifests"
  exception: "explicitly declared and reviewed (rare)"
```

---

## GAP-I — Binary Transparency Gossip Unsigned

| Thuộc tính | Giá trị |
|------------|--------|
| **Mức độ** | 🟠 HIGH — insider attack vector |
| **Spec** | TERA-CORE §4.5 |
| **Vấn đề** | Binary Transparency gossip message không cần được signed. Insider có thể broadcast fake `Global_Update_Log` hash. |
| **Quyết định cần có** | Sign gossip messages — đơn giản |
| **Trạng thái** | ✅ Resolved — đã có trong Phase 4 task box 4.3 |

### Resolution

```
Tất cả gossip messages phải được signed bằng TeraChat Root CA key.
Peer từ chối unsigned gossip messages.
Thêm signature field vào mọi gossip message struct.
```

---

## GAP-J — Outbox Queue TTL Undefined

| Thuộc tính | Giá trị |
|------------|--------|
| **Mức độ** | 🟡 MEDIUM — UX |
| **Spec** | TERA-CLIENT §11.5 |
| **Vấn đề** | `Outbox Queue TTL` chưa được define. Messages silently expire hay user được notify? |
| **Quyết định cần có** | Define TTL + user notification behavior |
| **Trạng thái** | ✅ Resolved — đã có trong Phase 3 task box 3.3 |

### Resolution

```
Outbox Queue TTL = 24h.
Sau TTL:
- UI hiển thị: "Messages could not be sent securely — please reconnect to a secure channel."
- Messages KHÔNG bị xóa — user có thể retry thủ công
- Enterprise contracts phải document explicit delivery semantics
```

---

## Tổng kết

| GAP | Trạng thái | Block gì | Resolution Date |
|-----|-----------|----------|-----------------|
| GAP-A | ✅ Resolved | Phase 2 (Sync) | 2026-05-12 |
| GAP-B | ✅ Resolved | Phase 2 (Sync) | 2026-05-12 |
| GAP-C | ✅ Resolved | iOS launch | 2026-05-12 |
| GAP-D | ✅ Resolved | — | 2026-05-11 |
| GAP-E | ✅ Resolved | Gov/Military tier | 2026-05-12 |
| GAP-F | ✅ Resolved | Huawei compliance | 2026-05-12 |
| GAP-G | ✅ Resolved | Mesh security | 2026-05-12 |
| GAP-H | ✅ Resolved | — | 2026-05-11 |
| GAP-I | ✅ Resolved | — | 2026-05-11 |
| GAP-J | ✅ Resolved | — | 2026-05-11 |

**Tất cả 10 GAPs đã được giải quyết ở mức spec.** Implementation sẽ theo phase tương ứng. Không GAP nào block Phase 1 MVP (Prototype). GAP-C sẽ được implement trước iOS launch (Phase 1).
