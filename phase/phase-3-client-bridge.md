# Phase 3 — Client Bridge & Secure Streaming

```yaml
id: "TERA-PHASE-3"
title: "Client Bridge, Secure Streaming & Security-Visible UX"
duration: "5 days (Day 16–20)"
economic_phase: "Phase 1 — 'Enough to Sign the Pilot'"
priority: 🔴 CRITICAL — Secure IPC and UI trustworthiness
teams: [Client Bridge Team, Trust Kernel Team]
debt_targets: [TD-011, TD-012]
exit_criteria:
  - TD-011 replaced by secure stream path (UDS + OTST)
  - UI renders signals/snapshots only, no domain state computation
  - All widget states (PENDING_SECURE_CHANNEL, MEMORY_PURGE, FCP, E2EE) rendered correctly
  - Reconnect/restart daemon flows tested on all desktop + mobile
```

## System Design: Phase 3 Connections

```
┌──────────────────────────────────────────────────┐
│  PHASE 3 BUILDS                                   │
│                                                   │
│  Phase 1 (Daemon) ────────────────┐               │
│  Phase 2 (Sync) ─────────────────┤               │
│                                    ↓               │
│  Bindings Layer                                       │
│  ├─ UDS/Named Pipes (Desktop)                        │
│  ├─ OTST (Mobile — One-Time Streaming Token)         │
│  ├─ gRPC Bridge (unified IPC)                        │
│  └─ Secure Stream Path                               │
│                                    ↓               │
│  UI Layer                                               │
│  ├─ PENDING_SECURE_CHANNEL widget state              │
│  ├─ Mesh HUD (signal-driven)                         │
│  ├─ Memory Purge Overlay (GPU Tier C)                │
│  ├─ E2EE Indicators                                  │
│  └─ FCP (First Contentful Paint)                     │
│                                    ↓               │
│                              Phase 4 (WASM)          │
└──────────────────────────────────────────────────┘
```

---

## Task Box 3.1 — gRPC Bridge: Non-Breaking Migration

```yaml
task_id: "PH3-T01"
name: "Activate gRPC for New Screens/Signals"
status: pending
priority: 🔴 CRITICAL
index: IDX-04-CORE-DAEMON (continued) + IDX-08-CLIENT-UX
platforms: [macOS, Windows, Linux, iPhone, Android, Oppo, Huawei]
estimated_hours: 8
```

### Code Tasks

| # | Task | Source Path | Platform |
|---|------|-------------|----------|
| 1 | Enable gRPC service on daemon: `127.0.0.1:50051` (desktop), UDS (mobile) | `source/core/daemon/src/grpc.rs` | All |
| 2 | Flutter gRPC client: connect to daemon, handle reconnect | `source/clients/android/` | Android, Oppo |
| 3 | SwiftUI gRPC client via SwiftNIO + grpc-swift | `source/clients/apple/` | iPhone, macOS |
| 4 | Tauri gRPC client via tonic (Rust → Rust, no serialization overhead) | `source/clients/desktop/` | macOS, Windows, Linux |
| 5 | Huawei gRPC client via napi-harmony bridge | `source/clients/harmonyos/` | Huawei |
| 6 | Backward compatibility: old IPC path still works, new screens use gRPC only | All client paths | All |

### Testing

| # | Test | Tool | Platform | Chaos Ref |
|---|------|------|----------|-----------|
| 1 | gRPC health check → daemon responds < 10ms | `grpc_health_probe` | Desktop | — |
| 2 | Flutter kills daemon → gRPC reconnects on restart | Integration test | Android | — |
| 3 | Old IPC path still functional (non-breaking verification) | E2E test | All | — |
| 4 | 100 concurrent gRPC streams → no memory leak > 50MB | `cargo bench --bench grpc_stress` | Desktop | — |

### Deployment

| Platform | gRPC Transport | Bind Address |
|----------|---------------|--------------|
| macOS | UDS (`/tmp/terachat.sock`) | Local socket |
| Windows | Named Pipes (`\\.\pipe\terachat`) | Local pipe |
| Linux | UDS (`/run/user/$UID/terachat.sock`) | Local socket |
| iPhone | UDS (App Group container) | Local socket |
| Android | UDS (app data dir) | `127.0.0.1:50051` fallback |
| Oppo | UDS | Same as Android |
| Huawei | UDS via napi bridge | Local socket |

### Request / Review

| Department | Reviewer | Gate |
|------------|----------|------|
| Client & UX Engineering | Systems Runtime Lead | gRPC migration non-breaking confirmed |
| Architecture & Leadership | Tech Lead | No domain logic leak to UI layer |
| Infra, Ops & Quality | SRE | Connection lifecycle robustness |

### Design Requirements

- gRPC local-only: bind to `127.0.0.1` or UDS, NEVER `0.0.0.0`
- Connection timeout: 5s initial, 30s reconnect
- All gRPC payloads carry `TraceId` + `WorkspaceId`
- Old IPC path deprecated but NOT removed until Phase 6 (RC)

### Reference Documentation

- `TERA-CLIENT §12.3` — gRPC bridge migration spec
- `TERA-CORE §12.6` — Daemon architecture
- `docs/raw/MD/Tech_Debt.md §5.1` — Rust Core + Headless Daemon (CRIT-01)

### System Design Connection

- **Input from:** Phase 1 (daemon gRPC skeleton), Phase 2 (sync data access)
- **Output to:** Phase 4 (WASM tapp gRPC), Phase 5 (governance RPCs)
- **Connects:** Daemon → gRPC Bridge → Flutter/SwiftUI/Tauri/ArkUI

---

## Task Box 3.2 — Secure Streaming: UDS + OTST

```yaml
task_id: "PH3-T02"
name: "Replace Localhost Proxy with Secure Streaming Path"
status: pending
priority: 🔴 CRITICAL
debt: TD-011
index: IDX-07-CLIENT-STREAM
platforms: [macOS, Windows, Linux, iPhone, Android, Oppo, Huawei]
estimated_hours: 10
```

### Code Tasks

| # | Task | Source Path | Platform |
|---|------|-------------|----------|
| 1 | Desktop UDS + SO_PEERCRED process authentication | `source/core/daemon/src/stream_desktop.rs` | macOS, Linux |
| 2 | Windows Named Pipes + ACL process authentication | `source/core/daemon/src/stream_windows.rs` | Windows |
| 3 | Mobile OTST (One-Time Streaming Token): 256-bit, TTL=30s | `source/core/daemon/src/stream_mobile.rs` | iPhone, Android |
| 4 | Streaming proxy replacement: Rust Core decrypts → secure channel → UI render | `source/core/daemon/src/streaming.rs` | All |
| 5 | SharedArrayBuffer Data Plane for zero-copy on Tauri (COOP+COEP headers) | `source/clients/desktop/` | Desktop |
| 6 | Remove `127.0.0.1` plaintext HTTP fallback — secure path only | All daemon paths | All |

### Testing

| # | Test | Tool | Platform | Chaos Ref |
|---|------|------|----------|-----------|
| 1 | Unauthorized process attempts UDS connection → SO_PEERCRED rejects | `cargo test --test uds_auth` | Desktop | — |
| 2 | OTST token expired (31s) → connection rejected | `cargo test --test otst_expiry` | Mobile | — |
| 3 | OTST reuse → second use rejected (nonce check) | `cargo test --test otst_replay` | Mobile | — |
| 4 | Plaintext port scan on `127.0.0.1` → no open port | Security scan (nmap) | Desktop | — |
| 5 | Video streaming 1080p via secure path → no frame drops at 30fps | `cargo bench --bench stream_perf` | Desktop | — |

### Deployment

Secure streaming path bundled in daemon + client bindings. Plaintext path removed.

### Request / Review

| Department | Reviewer | Gate |
|------------|----------|------|
| Client & UX Engineering | Security Architect | SO_PEERCRED / OTST correctness |
| Infra, Ops & Quality | SecOps | Port scan verification — no plaintext |
| Architecture & Leadership | CISO | Veto if any plaintext path remains |

### Design Requirements

- Desktop: UDS with `SO_PEERCRED` (Linux/macOS), Named Pipes ACL (Windows)
- Mobile: OTST 256-bit CSPRNG token, TTL 30s, single-use
- Zero-copy: SharedArrayBuffer with COOP+COEP headers on Tauri
- NO `127.0.0.1` HTTP — removed entirely

### Reference Documentation

- `TERA-CLIENT §12.1` — UDS + OTST secure streaming
- `docs/raw/MD/Tech_Debt.md` — TD-011, XPLAT-05

### System Design Connection

- **Input from:** Phase 1 (daemon), Task Box 3.1 (gRPC bridge)
- **Output to:** Phase 4 (WASM video/file streaming)
- **Connects:** Daemon secure stream → gRPC → Client UI

---

## Task Box 3.3 — Widget States & Security Channel UX

```yaml
task_id: "PH3-T03"
name: "Implement Security-Visible Widget State Machine"
status: pending
priority: 🟠 HIGH
index: IDX-08-CLIENT-UX
platforms: [macOS, Windows, Linux, iPhone, Android, Oppo, Huawei]
estimated_hours: 10
```

### Code Tasks

| # | Task | Source Path | Platform |
|---|------|-------------|----------|
| 1 | `PENDING_SECURE_CHANNEL` widget state: render when key escrow incomplete | `source/clients/*/widgets/pending_secure.rs` | All |
| 2 | Outbox guard: queue messages when channel not secure, flush on secure | `source/clients/*/widgets/outbox.rs` | All |
| 3 | Mesh HUD: signal-driven BLE/AWDL/Wi-Fi Direct status indicator | `source/clients/*/widgets/mesh_hud.rs` | Mobile |
| 4 | Memory Purge overlay: GPU Tier C force before rendering overlay | `source/clients/*/widgets/memory_purge.rs` | All |
| 5 | E2EE indicators: per-message/channel encryption status (MLS, PQ, or fallback) | `source/clients/*/widgets/e2ee_indicators.rs` | All |
| 6 | FCP (First Contentful Paint): render within 500ms of daemon signal | `source/clients/*/widgets/fcp.rs` | All |
| 7 | Security priority channel: synchronous rendering for security signals | `source/clients/*/channel.rs` | All |

### Testing

| # | Test | Tool | Platform | Chaos Ref |
|---|------|------|----------|-----------|
| 1 | Outbox queue TTL 24h → verify UI shows "Messages could not be sent securely" | `cargo test --test outbox_ttl` | All | SC-35 |
| 2 | Key escrow incomplete → PENDING_SECURE_CHANNEL renders → user blocked from sending | E2E test | All | SC-35 |
| 3 | GPU Tier C → Memory Purge overlay renders before any other frame | Perf test / XCTest | All | — |
| 4 | E2EE indicator: downgrade to Curve25519 → indicator changes to "Standard" vs "PQ" | E2E test | All | — |
| 5 | Security priority channel: security signal renders within 16ms (1 frame at 60fps) | Perf test | All | — |

### Deployment

UI widgets bundled per-platform: Flutter (Android/Oppo), SwiftUI (iPhone/macOS), Tauri HTML/CSS (Desktop), ArkUI (Huawei).

### Request / Review

| Department | Reviewer | Gate |
|------------|----------|------|
| Client & UX Engineering | Product UI Lead | Widget state machine completeness |
| Client & UX Engineering | Design Lead | Glassmorphism fidelity in all states |
| Architecture & Leadership | Security Architect | Security indicators non-spoofable |

### Design Requirements

- ALL widget states driven by CoreSignal, never by local UI logic
- Security priority channel: synchronous render on main thread, max 16ms
- Outbox TTL = 24h, then user notification
- GPU Tier C overlay: no animation, solid background, security message
- PENDING_SECURE_CHANNEL: blocks all outbound sends

### Reference Documentation

- `TERA-CLIENT §11.3–11.5` — Widget states and priority channel
- `docs/HTML/Design.html §8–17` — Security overlay design
- `docs/wiki/concepts/glassmorphism-design-system.md`
- `docs/raw/MD/Tech_Debt.md` — GAP-D, GAP-J

### System Design Connection

- **Input from:** Phase 0 (design system), Phase 1 (thermal signals), Task Box 3.2 (secure stream)
- **Output to:** Phase 4 (WASM UI integration), Phase 5 (governance UX)
- **Connects:** CoreSignal → Widget State Machine → Platform UI

---

## Task Box 3.4 — Daemon Reconnect & Crash Resilience

```yaml
task_id: "PH3-T04"
name: "Implement Daemon Reconnect/Restart Across All Platforms"
status: pending
priority: 🟠 HIGH
platforms: [macOS, Windows, Linux, iPhone, Android, Oppo, Huawei]
estimated_hours: 8
```

### Code Tasks

| # | Task | Source Path | Platform |
|---|------|-------------|----------|
| 1 | Desktop: daemon crash → systemd/launchd/SCM auto-restart → client reconnects | `source/core/daemon/src/lifecycle.rs` | Desktop |
| 2 | Android: Binder death notification → restart Foreground Service → client reconnect | `source/clients/android/` | Android, Oppo |
| 3 | iOS: XPC service restart → NSE re-initialize → main app reconnect | `source/clients/apple/` | iPhone |
| 4 | Windows lock serialization: suspend daemon on lock, resume on unlock | `source/core/daemon/src/lock_windows.rs` | Windows |
| 5 | Exponential backoff reconnect: 1s → 2s → 4s → 8s → 30s max | `source/clients/*/reconnect.rs` | All |

### Testing

| # | Test | Tool | Platform | Chaos Ref |
|---|------|------|----------|-----------|
| 1 | Kill daemon with SIGKILL → auto-restart < 5s → client reconnects < 10s | `cargo test --test daemon_crash` | Desktop | — |
| 2 | Android: kill Flutter app → Foreground Service continues → app restart reconnects | Firebase Test Lab | Android, Oppo | — |
| 3 | iOS: suspend to background → NSE continues → foreground resume without data loss | XCTest | iPhone | — |
| 4 | Windows lock screen → daemon suspends crypto ops → unlock → resume | Integration test | Windows | — |

### Deployment

Daemon lifecycle management per-platform as defined in Task Box 1.4.

### Request / Review

| Department | Reviewer | Gate |
|------------|----------|------|
| Client & UX Engineering | Systems Runtime Lead | Reconnect robustness across all platforms |
| Infra, Ops & Quality | SRE | Mean Time To Recover (MTTR) < 15s |
| Architecture & Leadership | Tech Lead | UI crash never kills core |

### Design Requirements

- Daemon restart: < 5s (desktop), < 10s (mobile)
- Client reconnect: < 10s (exponential backoff 1s → 30s max)
- No data loss during reconnect window (outbox queues all ops)
- Lock screen: crypto ops suspended, not terminated

### Reference Documentation

- `TERA-CORE §12.6` — Background execution resilience
- `TERA-CLIENT §12.3` — Client reconnect protocol
- `docs/raw/MD/Tech_Debt.md §5.1` — Daemon-UI separation

### System Design Connection

- **Input from:** Phase 1 (daemon), Task Box 3.1 (gRPC bridge), Task Box 3.3 (outbox)
- **Output to:** Phase 6 (SC-01, SC-03 chaos scenarios)
- **Connects:** Daemon lifecycle → gRPC reconnect → Outbox flush → UI restore
