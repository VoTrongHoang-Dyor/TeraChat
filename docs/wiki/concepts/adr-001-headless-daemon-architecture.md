---
type: concept
created: 2026-05-12
updated: 2026-05-12
tags: [adr, architecture, daemon, headless, phase-0]
sources: [tera-core-spec, tera-client-spec, tech-debt]
---

# ADR-001: Headless Daemon Architecture

## Status

**ACCEPTED** — 2026-05-12

## Context

TeraChat's Rust Core handles cryptography, MLS key management, CRDT sync, BLE mesh networking, and WASM sandbox execution. These operations must survive UI lifecycle events (app killed by OS, memory pressure, user swipe-close) and OEM battery management (ColorOS/MIUI/OriginOS aggressive background killing).

Three architectural options were evaluated:

| Option | Description | Verdict |
|--------|-------------|---------|
| **A: Embedded Library** | Rust Core as static/dynamic library loaded by UI process | ❌ REJECTED |
| **B: Headless Daemon** | Rust Core as independent system service, UI connects via IPC | ⭐ ACCEPTED |
| **C: Hybrid** | Critical ops in daemon, lightweight ops in-process | ❌ REJECTED (complexity) |

## Decision

**Rust Core runs as an independent headless daemon** (system service). UI applications (Flutter/SwiftUI/Tauri) are pure rendering clients that connect to the daemon via gRPC/IPC.

### Platform Implementation

| Platform | Daemon Form | Service Type |
|----------|------------|--------------|
| **macOS** | `launchd` agent | `com.terachat.core` LaunchAgent |
| **iOS** | App Extension + Foreground Service | NSE for push, App Group for state |
| **Android** | `ForegroundService` | Persistent notification (OEM-resistant) |
| **Windows** | Windows Service | `sc.exe` registered service |
| **Linux** | `systemd` unit | `terachat-core.service` |

### IPC Channel

- **Desktop (macOS/Windows/Linux):** Unix Domain Socket (UDS) with `SO_PEERCRED` auth
- **Mobile (iOS/Android):** gRPC over localhost with One-Time Streaming Token (OTST)
- **All:** Protobuf control plane (<1KB), SharedArrayBuffer data plane (SAB, bulk)

## Consequences

### Positive
- ✅ UI can crash/update/restart without losing encrypted sessions or mesh connections
- ✅ BLE mesh continues operating even when app is in background
- ✅ MLS epoch rotation happens independent of UI state
- ✅ CRDT sync continues offline without UI interaction
- ✅ Android ForegroundService survives OEM battery management (XPLAT-08 mitigation)

### Negative
- ❌ More complex deployment — two processes instead of one
- ❌ IPC overhead (~50μs per round-trip on UDS, negligible for gRPC)
- ❌ macOS requires user to grant "Login Items" permission for LaunchAgent
- ❌ iOS constraints: no true daemon, must use App Extension workarounds

## Related

- [[CRDT Dual-Sync]] — sync must survive UI lifecycle
- [[Survival Mesh Networking]] — mesh must continue in background
- [[ADR-003]] — gRPC over FFI for IPC
- Tech_Debt §5.1 — Rust-Core + Headless Daemon refactoring
