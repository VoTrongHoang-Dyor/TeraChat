---
type: concept
created: 2026-05-10
tags: [terachat, crdt, sync, dag, offline-first, storage]
sources: [tera-sync-spec, tera-core-spec]
---

# CRDT Dual-Sync Pattern

TeraChat's approach to offline-first data synchronization uses two distinct sync planes for different data types, avoiding the "one-size-fits-all" CRDT trap.

## The Two Planes

| Plane | Data Types | Sync Mechanism | Storage |
|-------|-----------|----------------|---------|
| **Message Sync** | Chat, Collaborative Text, Presence | CRDT DAG (Directed Acyclic Graph) | `hot_dag.db` (SQLite WAL, append-only) |
| **App State Sync** | Finance, HR, structured data | Vector-Clock Relational Sync | `cold_state.db` (SQLite + SQLCipher AES-256) |

## Why Two Planes

- **CRDT is excellent for text collaboration** — merge semantics are well-understood, no central coordinator needed, eventual consistency guaranteed.
- **CRDT is wrong for Finance/HR** — these need transactional guarantees, numeric precision, and relational integrity. Vector-Clock sync provides conflict detection without automatic merge.

## Key Architecture Decisions

- **Tombstone Vacuum mandatory:** CRDT DAG grows infinitely without periodic garbage collection of deleted/merged entries.
- **WAL crash-safe replay:** Both databases use SQLite WAL mode. On crash, replay from last checkpoint — no data loss.
- **Shadow DB atomic rename:** Schema migrations use `cold_state_shadow.db` → atomic rename → delete old. Never migrates in-place.
- **CAS with tenant salt:** Content-addressable storage uses `BLAKE3(workspace_id || salt || chunk)` to prevent cross-tenant dedup side-channel.

## 🧠 Design Decisions (Q&A)

- **Why not use CRDT for everything?** → Finance/HR data needs ACID transactions and relational integrity. CRDT's automatic merge would silently corrupt numeric values. Trade-off: two sync engines to maintain instead of one.
- **Why append-only for hot_dag.db?** → Immutable history. Compromised client cannot rewrite past messages. Vacuum only removes tombstones, never original events. Trade-off: storage grows until vacuum.
- **What case does dual-hash identity solve?** → Gov/Military requires BLAKE3 + SHA-512 dual identity to mathematically exclude chosen-prefix collision attacks (NSA Suite B extended). Trade-off: ~64 bytes extra per blob.
