---
type: source
created: 2026-05-10
tags: [terachat, crdt, sync, storage, sqlite, offline-first]
sources: [raw/MD/Spec-Dual-Sync-And-Local-Storage.md]
depends_on: [tera-core-spec]
---

# Dual-Sync Architecture & Local Storage (TERA-SYNC)

Source: `raw/MD/Spec-Dual-Sync-And-Local-Storage.md` — v1.0.0, 2026-03-29.

## What It Covers

Two-plane storage and sync architecture: Message Sync Plane (CRDT DAG for chat, collaborative text, presence) and App State Sync Plane (Vector-Clock Relational Sync for structured data like Finance/HR). Addresses the critical bottleneck of scaling to Enterprise.

## Key Constraints

- CRDT DAG only for Chat, Collaborative Text, Presence — NOT Finance/HR
- App State Sync Plane uses Vector-Clock Relational Sync for structured data
- ZeroizeOnDrop mandatory for all key material in storage
- All DB schema changes must be backward-compatible with WAL replay
- Tombstone Vacuum mandatory — prevent unbounded CRDT DB growth

## Consumed By

TERA-RUNTIME, TERA-ENCLAVE, TERA-CLIENT

## Related Concepts

- [[CRDT Dual-Sync Pattern]]
- [[Zero-Knowledge Architecture]]
- [[Data Sovereignty & Export]]
