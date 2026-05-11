---
type: synthesis
created: 2026-05-11
tags: [terachat, gap, resolution, tracker, architecture]
sources: [tech-debt-registry, tera-core-spec, tera-sync-spec, tera-gov-spec]
status: in_progress
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

**Cần quyết định:** Chấp nhận nested saga retry limit = 3? Manual intervention path là gì?

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

**Cần quyết định:** Read-only mode cho desktop — user-facing notification hay silent?

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

**Cần quyết định:** Đã verify flock() hoạt động trong iOS App Group container chưa? Nếu không, chấp nhận ring buffer approach?

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

**Cần quyết định:** election_weight algorithm? Simple majority (1 node = 1 vote) hay weighted (HQ = 3, Branch = 1)?

---

## GAP-F — Huawei Enterprise Tier Disclosure Missing

| Thuộc tính | Giá trị |
|------------|--------|
| **Mức độ** | 🟡 MEDIUM — compliance |
| **Spec** | TERA-ECO §7 |
| **Vấn đề** | Huawei Enterprise tier "CRL ≤ 4h polling" vi phạm SCIM < 30s SLA — không được disclose trong Pricing_Packages.html |
| **Quyết định cần có** | Update pricing docs — đơn giản |
| **Trạng thái** | ⏳ Pending resolution |

### Resolution

Thêm vào `Pricing_Packages.html`:

> **Lưu ý cho thiết bị Huawei:** Do hạn chế của HMS Push (không hỗ trợ data-only message), thời gian phản hồi SCIM có thể lên đến 4 giờ. Huawei không nằm trong Enterprise SLA tier (30s). Vui lòng sử dụng Android (Google Mobile Services) cho Enterprise deployment.

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

**Cần quyết định:** Force unfreeze threshold = 2h? Cần mấy Admin signatures?

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

| GAP | Trạng thái | Block gì |
|-----|-----------|----------|
| GAP-A | ⏳ Pending | Phase 2 (Sync) |
| GAP-B | ⏳ Pending | Phase 2 (Sync) |
| GAP-C | ⏳ Pending | iOS launch |
| GAP-D | ✅ Resolved | — |
| GAP-E | ⏳ Pending | Gov/Military tier |
| GAP-F | ⏳ Pending | Huawei compliance |
| GAP-G | ⏳ Pending | Mesh security |
| GAP-H | ✅ Resolved | — |
| GAP-I | ✅ Resolved | — |
| GAP-J | ✅ Resolved | — |

**Blockers cho Phase 1 MVP:** GAP-C (nếu target iOS). Còn lại đều block Phase 2+, không block Phase 1.
