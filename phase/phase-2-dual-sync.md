# Phase 2 — Dual-Sync Correctness & Recovery

```yaml
id: "TERA-PHASE-2"
title: "Dual-Sync Correctness & Recovery Plane"
duration: "5 days (Day 11–15)"
economic_phase: "Phase 1 — 'Enough to Sign the Pilot'"
priority: 🔴 CRITICAL — Data correctness and crash recovery
teams: [State Integrity Team, Trust Kernel Team]
debt_targets: [TD-013, TD-014, GAP-A, GAP-B, GAP-C]
exit_criteria:
  - SC-37, SC-39, SC-40 have first harness
  - Dual-plane write has idempotent recovery path
  - CAS v2 tenant-salted dual-hash implemented
  - Boot recovery < 300ms
```

## System Design: Phase 2 Connections

```
┌──────────────────────────────────────────────────┐
│  PHASE 2 BUILDS                                   │
│                                                   │
│  Phase 1 (Daemon) ────────────────┐               │
│                                    ↓               │
│  tc-crdt-sync                     tc-store        │
│  ├─ SagaRecoveryGuard              ├─ hot_dag.db   │
│  ├─ NseRingBufferDrain             ├─ cold_state.db│
│  ├─ WalIntegrityCheck              ├─ CAS v2       │
│  └─ Dual-plane schema              └─ Vacuum       │
│                                    ↓               │
│                              Phase 3 (Bridge)      │
└──────────────────────────────────────────────────┘
```

---

## Task Box 2.1 — Dual-Plane Schema + Saga Journal

```yaml
task_id: "PH2-T01"
name: "Define Dual-Plane Schema & SagaEntry Contract"
status: pending
priority: 🔴 CRITICAL
debt: GAP-A
index: IDX-05-SYNC-SAGA
platforms: [macOS, Windows, Linux, iPhone, Android, Oppo, Huawei]
estimated_hours: 10
```

### Code Tasks

| # | Task | Source Path | Platform |
|---|------|-------------|----------|
| 1 | Lock `hot_dag.db` schema: CRDT DAG tables, UUIDv7 PK, BLAKE3 hashes | `source/core/tc-store/src/schema_hot.rs` | All |
| 2 | Lock `cold_state.db` schema: Relational tables (Finance, HR, Config) | `source/core/tc-store/src/schema_cold.rs` | All |
| 3 | Define `SagaEntry` struct: `saga_id`, `status (Pending/Committed/Compensating/Tombstone)`, `compensating_event`, `timestamp` | `source/core/tc-crdt-sync/src/saga.rs` | All |
| 4 | `SagaJournal` append-only log for all saga transitions | `source/core/tc-crdt-sync/src/journal.rs` | All |
| 5 | Dual-plane write protocol: atomic saga commit across both planes or rollback | `source/core/tc-crdt-sync/src/dual_write.rs` | All |
| 6 | Compensating event: if compensating action fails → nested saga with WAL savepoint | `source/core/tc-crdt-sync/src/compensate.rs` | All |

### Testing

| # | Test | Tool | Platform | Chaos Ref |
|---|------|------|----------|-----------|
| 1 | Write to hot_dag.db + cold_state.db in saga → kill process mid-write → verify consistency | `cargo test --test saga_recovery` | All | — |
| 2 | Compensating event fails → nested saga triggers → verify idempotent recovery | `cargo test --test saga_nested` | All | — |
| 3 | 1000 rapid saga commits → verify journal integrity | `cargo test --test saga_journal` | All | — |
| 4 | SQLite WAL corruption → verify saga rollback without data loss | Chaos simulation | All | SC-37 |

### Deployment

Bundled in Rust Core — schema migration on daemon startup.

### Request / Review

| Department | Reviewer | Gate |
|------------|----------|------|
| State & CRDT Systems | Distributed Systems Lead | Saga correctness proof |
| State & CRDT Systems | DBA | Schema normalization |
| Architecture & Leadership | Tech Lead | Dual-plane invariant: no Finance in CRDT DAG |

### Design Requirements

- `hot_dag.db`: CRDT DAG ONLY — chat messages, files, reactions
- `cold_state.db`: Relational ONLY — Finance, HRM, workspace config, audit metadata
- Saga MUST be idempotent: re-running same saga produces same outcome
- WAL savepoint for every compensating event

### Reference Documentation

- `TERA-SYNC §8.4` — SagaRecoveryGuard specification
- `TERA-SYNC §1.2` — Dual-plane sync invariant
- `docs/raw/MD/Tech_Debt.md` — GAP-A, GAP-B, GAP-C

### System Design Connection

- **Input from:** Phase 1 (daemon startup), Task Box 1.1 (secure storage)
- **Output to:** Phase 3 (client data access), Phase 4 (WASM state access)
- **Connects:** tc-crdt-sync → tc-store (both databases) → daemon (boot sequence)

---

## Task Box 2.2 — CoreBootSequence Protocol

```yaml
task_id: "PH2-T02"
name: "Implement Crash-Recovery Boot Sequence"
status: pending
priority: 🔴 CRITICAL
debt: GAP-A, GAP-B, GAP-C
index: IDX-05-SYNC-SAGA (continued)
platforms: [macOS, Windows, Linux, iPhone, Android, Oppo, Huawei]
estimated_hours: 10
```

### Code Tasks

| # | Task | Source Path | Platform |
|---|------|-------------|----------|
| 1 | `SagaRecoveryGuard`: scan orphaned `CrdtCommitted` sagas on boot | `source/core/tc-crdt-sync/src/recovery.rs` | All |
| 2 | `NseRingBufferDrain`: drain NSE ring buffer via flock-protected drain | `source/core/tc-crdt-sync/src/nse_drain.rs` | iPhone |
| 3 | `WalIntegrityCheck`: `PRAGMA integrity_check` on both databases | `source/core/tc-store/src/integrity.rs` | All |
| 4 | Boot sequence ordering: RecoveryGuard → NseDrain → WalCheck → open IPC | `source/core/daemon/src/boot.rs` | All |
| 5 | `WAL_LOCK_REQUEST`, `WAL_FLUSH_ACK`, `WAL_LOCK_GRANTED` signals added to CoreSignal enum | `source/core/proto/signals.proto` | All |
| 6 | 5s timeout + exponential backoff for WAL lock handshake | `source/core/tc-store/src/wal_lock.rs` | All |
| 7 | Desktop fallback: read-only mode if WAL lock not granted | `source/core/tc-store/src/wal_lock.rs` | Desktop |

### Testing

| # | Test | Tool | Platform | Chaos Ref |
|---|------|------|----------|-----------|
| 1 | Kill process during saga → reboot → SagaRecoveryGuard recovers | `cargo test --test boot_recovery` | All | SC-37 |
| 2 | NSE ring buffer drain with concurrent writes → verify no data loss | iOS XCTest | iPhone | SC-39 |
| 3 | WAL corruption on `cold_state.db` → WalIntegrityCheck catches → rebuild from hot_dag | Chaos simulation | All | SC-40 |
| 4 | WAL lock timeout → desktop enters read-only mode → user notified | Integration test | Desktop | — |
| 5 | Full boot sequence < 300ms wall time | Benchmark (`cargo bench`) | All | — |

### Deployment

Bundled in daemon — runs automatically on every daemon start, before IPC channels open.

### Request / Review

| Department | Reviewer | Gate |
|------------|----------|------|
| State & CRDT Systems | Distributed Systems Lead | Recovery correctness |
| Client & UX Engineering | iOS Runtime Lead | NSE drain path |
| Architecture & Leadership | CISO | No data leak during recovery |

### Design Requirements

- Each guard < 100ms, total boot overhead < 300ms
- NSE drain: use POSIX `flock()` on App Group container, NOT Keychain semaphore
- WAL lock: 5s timeout, exponential backoff (1s, 2s, 4s, 8s)
- Read-only mode: desktop + user visible indicator

### Reference Documentation

- `TERA-SYNC §8.4` — SagaRecoveryGuard
- `TERA-SYNC §8.5` — WAL lock handshake
- `TERA-CLIENT §4.3` — WAL signals in CoreSignal
- `TERA-CORE §11.3` — NSE Shared Keychain semaphore issue
- `docs/raw/MD/Tech_Debt.md` — GAP-A, GAP-B, GAP-C

### System Design Connection

- **Input from:** Task Box 2.1 (dual-plane schema), Phase 1 (daemon boot)
- **Output to:** Phase 3 (IPC channel open), Phase 6 (SC-37, SC-39, SC-40 chaos)
- **Connects:** tc-store → tc-crdt-sync → daemon boot → IPC channels → clients

---

## Task Box 2.3 — CAS v2: Tenant-Salted Dual-Hash

```yaml
task_id: "PH2-T03"
name: "Implement Content-Addressable Storage v2"
status: pending
priority: 🟠 HIGH
debt: TD-013, TD-014
index: IDX-06-SYNC-CAS
platforms: [macOS, Windows, Linux, iPhone, Android, Oppo, Huawei]
estimated_hours: 8
```

### Code Tasks

| # | Task | Source Path | Platform |
|---|------|-------------|----------|
| 1 | Tenant-salted CAS: `BLAKE3(workspace_id || salt || chunk)` — dedup only within workspace | `source/core/tc-store/src/cas.rs` | All |
| 2 | Dual-hash identity: `BLAKE3(data) || SHA-512[data](0:32)` for Gov/Military compliance | `source/core/tc-store/src/cas.rs` | All |
| 3 | CAS migration path: detect old global-hash blobs → re-salt per tenant | `source/core/tc-store/src/cas_migration.rs` | All |
| 4 | Blob CAS VFS (Virtual File System) with chunked storage | `source/core/tc-store/src/vfs.rs` | All |

### Testing

| # | Test | Tool | Platform | Chaos Ref |
|---|------|------|----------|-----------|
| 1 | Upload same file to two workspaces → different CAS hash → no cross-tenant leak | `cargo test --test cas_tenant_isolation` | All | — |
| 2 | Gov/Military: verify dual-hash identity BLAKE3 + SHA-512 | `cargo test --test cas_dual_hash` | All | — |
| 3 | CAS migration: old single-hash blobs → re-salted dual-hash | `cargo test --test cas_migration` | All | — |
| 4 | Large file (1GB) chunked → all chunks correctly reassembled | `cargo test --test cas_vfs` | Desktop | — |

### Deployment

CAS VFS bundled in Rust Core. Migration runs once on daemon start if old blobs detected.

### Request / Review

| Department | Reviewer | Gate |
|------------|----------|------|
| State & CRDT Systems | DBA | CAS v2 correctness |
| Governance & Compliance | CISO | Gov/Military dual-hash compliance |
| Architecture & Leadership | Security Architect | No cross-tenant dedup |

### Design Requirements

- Tenant salt: `workspace_id` from DAO record, immutable
- Dual-hash: ANY Gov/Military deployment MUST use BLAKE3 + SHA-512
- CAS migration: one-time, background, non-blocking for existing data

### Reference Documentation

- `TERA-SYNC §9.1` — Tenant-salted CAS specification
- `TERA-SYNC §9.2` — Dual-hash blob identity
- `docs/raw/MD/Tech_Debt.md` — TD-013, TD-014

### System Design Connection

- **Input from:** Task Box 2.1 (schema), Task Box 1.1 (key arena for salts)
- **Output to:** Phase 4 (WASM blob access)
- **Connects:** tc-store CAS → tc-crypto (hash) → tc-crdt-sync (blob references)

---

## Task Box 2.4 — Adaptive Vacuum + Retention Policy

```yaml
task_id: "PH2-T04"
name: "Implement Storage Maintenance & Legal Hold"
status: pending
priority: 🟡 MEDIUM
platforms: [macOS, Windows, Linux, iPhone, Android, Oppo, Huawei]
estimated_hours: 6
```

### Code Tasks

| # | Task | Source Path | Platform |
|---|------|-------------|----------|
| 1 | Adaptive vacuum: trigger when WAL > 200MB (mobile) or 1GB (desktop) | `source/core/tc-store/src/vacuum.rs` | All |
| 2 | Legal-hold-aware retention: rows under legal hold excluded from vacuum | `source/core/tc-store/src/legal_hold.rs` | All |
| 3 | ZKMemory cleanup: purge expired ZK indexes | `source/core/tc-store/src/zk_cleanup.rs` | All |
| 4 | `CoreSignal::MemoryPressureWarning` with `ui_emergency_mode` flag | `source/core/proto/signals.proto` | All |

### Testing

| # | Test | Tool | Platform | Chaos Ref |
|---|------|------|----------|-----------|
| 1 | WAL bloat > 200MB → vacuum triggers → WAL < 50MB | `cargo test --test vacuum_trigger` | Mobile | SC-11 |
| 2 | Legal hold on workspace → vacuum skips held rows → other rows cleaned | `cargo test --test legal_hold_vacuum` | All | — |
| 3 | ZKMemory index expired → cleanup removes → no residual data | `cargo test --test zk_cleanup` | All | — |

### Deployment

Bundled in Rust Core — runs as background task in daemon.

### Request / Review

| Department | Reviewer | Gate |
|------------|----------|------|
| State & CRDT Systems | DBA | Vacuum policy safety |
| Governance & Compliance | Governance Lead | Legal hold verification |
| Architecture & Leadership | CISO | No data loss on vacuum |

### Design Requirements

- Mobile WAL threshold: 200MB → vacuum
- Desktop WAL threshold: 1GB → vacuum
- Legal hold: rows under legal hold NEVER vacuumed (until hold released)
- `ui_emergency_mode: true` on `MemoryPressureWarning` when WAL > critical

### Reference Documentation

- `TERA-SYNC §9.3` — Adaptive vacuum
- `docs/HTML/Design.html §8–12` — MemoryPressureWarning signal
- `docs/raw/MD/Tech_Debt.md` — GAP-D

### System Design Connection

- **Input from:** Task Box 2.2 (boot integrity), Task Box 2.3 (CAS)
- **Output to:** Phase 5 (legal hold integration)
- **Connects:** tc-store → daemon (background task) → UI (pressure signals)
