---
type: concept
created: 2026-05-10
updated: 2026-05-30
tags: [terachat, sync, event-log, vector-clocks, crdt, offline-first, storage]
sources: [tera-sync-spec, tera-core-spec]
supersedes: crdt-dual-sync.md
---

# Dual-Sync Pattern (Event Log + CRDT)

> **v2.0 — Cập nhật theo nguồn sự thật 2026-05-30.** Tên cũ "CRDT Dual-Sync" đã bị thay thế. CRDT không còn là engine chính cho chat messaging.

Kiến trúc đồng bộ hai lớp của TeraChat phân tách dữ liệu theo bản chất nghiệp vụ để tránh bẫy "một chuẩn cho tất cả" (one-size-fits-all CRDT). Nguyên tắc: **mỗi kiểu dữ liệu dùng đúng cơ chế phù hợp với bản chất của nó.**

## Hai Lớp Đồng bộ

| Lớp | Dữ liệu | Cơ chế Đồng bộ | Database | Chiến lược Merge |
|-----|---------|----------------|----------|-----------------|
| **Message Plane** | Chat messages, Presence, Reactions | **Append-Only Event Log + Vector Clocks** | `event_log.db` (SQLite WAL, append-only) | Tuyến tính theo Vector Clock — không có xung đột tự động |
| **State Plane** | Finance, HR, dữ liệu quan hệ | Vector-Clock Relational Sync | `cold_state.db` (SQLite + SQLCipher AES-256) | Phát hiện xung đột, yêu cầu giải quyết thủ công |
| **CRDT Scope** | Collaborative Notes, Thread Titles | CRDT (Conflict-free Replicated Data Type) | Embedded trong `event_log.db` (riêng namespace) | Tự động merge (LWW + causal ordering) |

## Tại sao Chat KHÔNG dùng CRDT?

Tài liệu gim (nguồn sự thật) xác định rõ: *"Tuyệt đối không dùng CRDT cho chat messages. Chat là chuỗi sự kiện tuyến tính."*

| Tiêu chí | CRDT DAG | Append-Only Event Log |
|----------|----------|-----------------------|
| Bản chất dữ liệu | Đồ thị phi tuần tự | Chuỗi sự kiện tuyến tính ✅ |
| Overhead lưu trữ | Cao (tombstones, DAG metadata) | Thấp — giảm 90% so với CRDT ✅ |
| Edit/Delete | Phức tạp (DAG rewrite) | Override Event đơn giản ✅ |
| State Bloat | Phình to không kiểm soát | Kiểm soát bằng TTL window ✅ |
| Tombstone GC | Bắt buộc và phức tạp | Không cần (event immutable) ✅ |

## Append-Only Event Log

Chat messages được ghi như các sự kiện bất biến (immutable events):

```
event_log.db (SQLite WAL)
├── MessageSent     { id, author, content_encrypted, vector_clock, timestamp }
├── MessageEdited   { original_id, new_content_encrypted, vector_clock }  ← override event
├── MessageDeleted  { original_id, vector_clock }                          ← tombstone event
└── ReactionAdded   { message_id, emoji, author, vector_clock }
```

**Nguyên tắc bất biến:**
- Không bao giờ UPDATE hoặc DELETE dòng gốc
- Edit/Delete là các sự kiện mới ghi đè ở lớp hiển thị
- UI squash events thành trạng thái cuối cùng — server không cần xử lý

## CRDT — Giới hạn chỉ ở Collaborative Text

CRDT vẫn được dùng, nhưng **chỉ ở những vùng dữ liệu thực sự có tính đồng thời cao:**

```
CRDT namespace trong event_log.db:
├── notes.*           ← Collaborative Notes (multi-user concurrent edit)
├── thread.title.*    ← Thread Titles (ai cũng có thể đổi tên)
└── whiteboard.*      ← Shared whiteboard objects (Phase 3+)
```

Tombstone Garbage Collection chỉ cần chạy trên namespace CRDT nhỏ này — không phải toàn bộ message history.

## Quyết định Kiến trúc Chính

- **`event_log.db` là APPEND-ONLY** — không UPDATE, không DELETE dòng gốc. Tính bất biến đảm bảo audit trail và chống tampering từ client bị compromise.
- **`cold_state.db` dùng shadow DB atomic rename** cho schema migration — không bao giờ migrate in-place.
- **CAS với tenant salt:** `BLAKE3(workspace_id || salt || chunk)` — chống side-channel cross-tenant dedup.
- **UUID v7** time-ordered primary keys cho causal ordering không phụ thuộc wall-clock.

## 🧠 Design Decisions (Q&A)

- **Tại sao không dùng CRDT cho chat?** → Chat message là sự kiện tuyến tính theo thời gian. CRDT DAG tạo ra overhead lưu trữ khổng lồ (tombstones, merge metadata) và phình to State không kiểm soát (State Bloat). Event Log + Vector Clocks giảm 90% dung lượng cục bộ.

- **Edit/Delete hoạt động thế nào với Event Log?** → Edit = ghi thêm `MessageEdited` event. Delete = ghi thêm `MessageDeleted` event. UI squash events thành view cuối. Server/Core không cần biết "trạng thái hiện tại" — tất cả được tính từ event stream.

- **Khi nào CRDT được dùng?** → Chỉ khi nhiều người cùng edit một object đồng thời và thứ tự không quan trọng (collaborative notes, shared thread title). Không bao giờ dùng cho dữ liệu tuyến tính (messages) hay dữ liệu quan hệ (Finance/HR).

- **Dual-hash identity (BLAKE3 + SHA-512) cho ai?** → Gov/Military: loại trừ toán học chosen-prefix collision attacks (NSA Suite B extended). Trade-off: ~64 bytes extra per blob. SME: BLAKE3 only.

## Related

- [[ADR-002 Dual-Plane Sync]] — quyết định kiến trúc ban đầu (sẽ được cập nhật)
- [[ADR-007 Shadow Graph AI Resolution]] — AI chỉ ghi vào Shadow Branch, không Root
- [[Invariants]] — Invariant I-04: event_log.db append-only
- [[Terachat Architecture Overview]] — vị trí sync trong toàn bộ stack
