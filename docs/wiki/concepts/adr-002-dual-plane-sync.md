---
type: concept
created: 2026-05-12
updated: 2026-05-12
tags: [adr, crdt, sync, dual-plane, phase-0]
sources: [tera-sync-spec, tera-core-spec]
---

# ADR-002: Dual-Plane Sync (CRDT + Relational)

## Status

**ACCEPTED** — 2026-05-12

## Context

Enterprise messaging requires two fundamentally different sync models:

1. **Chat, collaborative text, presence** — eventually consistent, merge-friendly, high-throughput
2. **Finance, HR, structured data** — transactional, numeric precision, relational integrity

Using CRDT for everything would silently corrupt financial data (automatic merge of conflicting salary records). Using relational sync for everything would require a central coordinator (violating offline-first).

## Decision

**Two distinct sync planes, two databases, two merge strategies:**

| Plane | Data | Sync Mechanism | Database | Merge Strategy |
|-------|------|----------------|----------|----------------|
| **Message Sync** | Chat, Text, Presence | CRDT DAG | `hot_dag.db` (SQLite WAL, append-only) | Automatic merge (LWW + causal ordering) |
| **App State Sync** | Finance, HR, Structured | Vector-Clock Relational | `cold_state.db` (SQLite + SQLCipher AES-256) | Conflict detection, manual resolution |

### Key Design Choices

- `hot_dag.db` is **APPEND-ONLY** — no UPDATE or DELETE. Tombstones mark deletions; vacuum removes only tombstones.
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

- [[CRDT Dual-Sync]] — detailed sync mechanism
- [[Terachat Architecture Overview]] — where sync fits in the stack
- TERA-SYNC spec — full specification
