---
type: concept
created: 2026-05-12
updated: 2026-05-30
tags: [adr, event-log, vector-clocks, crdt, sync, dual-plane, phase-0]
sources: [tera-sync-spec, tera-core-spec]
---

# ADR-002: Dual-Plane Sync (Event Log + CRDT + Relational)

> **Cập nhật 2026-05-30:** Chat messaging engine đã được sửa từ CRDT DAG → Append-Only Event Log + Vector Clocks. CRDT giữ lại chỉ cho Collaborative Text. Database `hot_dag.db` → `event_log.db`.

## Status

**ACCEPTED** — 2026-05-12

## Context

Enterprise messaging requires three fundamentally different sync models:

1. **Chat messages, presence** — linear event stream, không có concurrent edit, append-only
2. **Collaborative text, shared objects** — concurrent multi-user edit, merge-friendly, eventually consistent
3. **Finance, HR, structured data** — transactional, numeric precision, relational integrity

Dùng CRDT cho chat messages tạo ra State Bloat nghiêm trọng và overhead lưu trữ không cần thiết. Chat message là sự kiện tuyến tính — Event Log là cơ chế phù hợp nhất. CRDT đúng cho collaborative text nhưng sai cho chat.

## Decision

**Ba sync engine, hai databases, ba merge strategies:**

| Plane | Data | Sync Mechanism | Database | Merge Strategy |
|-------|------|----------------|----------|----------------|
| **Message Plane** | Chat messages, Presence, Reactions | **Append-Only Event Log + Vector Clocks** | `event_log.db` (SQLite WAL, append-only) | Tuyến tính theo Vector Clock — không xung đột tự động |
| **CRDT Scope** | Collaborative Notes, Thread Titles | CRDT (giới hạn) | Namespace trong `event_log.db` | Tự động merge (LWW + causal ordering) |
| **App State Sync** | Finance, HR, Structured | Vector-Clock Relational | `cold_state.db` (SQLite + SQLCipher AES-256) | Phát hiện xung đột, yêu cầu giải quyết thủ công |

### Key Design Choices

- `event_log.db` is **APPEND-ONLY** — no UPDATE or DELETE trên dòng gốc. Edit/Delete là override events mới. UI squash events thành view cuối.
- **CRDT giới hạn** chỉ ở Collaborative Notes và Thread Titles — không bao giờ dùng cho chat messages.
- `cold_state.db` uses **shadow DB atomic rename** for schema migrations — never migrates in-place.
- Content addressing uses **tenant-salted CAS**: `BLAKE3(workspace_id || salt || chunk)` to prevent cross-tenant dedup side-channel (TD-013).
- UUID v7 time-ordered primary keys for causal ordering without wall-clock dependency.

## Consequences

### Positive
- ✅ Chat merges automatically — no user intervention needed for message ordering
- ✅ Finance data never auto-merges — human review required for conflicts
- ✅ Offline-first for both planes — no central coordinator
- ✅ Append-only DAG provides immutable audit trail

### Negative
- ❌ Two sync engines to maintain, test, and debug
- ❌ Cross-plane queries require joining across databases
- ❌ Tombstone vacuum is a mandatory operational task

## Related

- [[Dual-Sync Pattern]] — chi tiết cơ chế đồng bộ (thay thế CRDT Dual-Sync)
- [[ADR-007 Shadow Graph AI Resolution]] — AI chỉ ghi vào Shadow Branch, không được write vào event_log.db trực tiếp
- [[Terachat Architecture Overview]] — where sync fits in the stack
- TERA-SYNC spec — full specification
