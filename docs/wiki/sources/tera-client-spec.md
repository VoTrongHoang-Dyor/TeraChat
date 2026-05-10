---
type: source
created: 2026-05-10
tags: [terachat, ipc, ffi, ui, flutter, tauri, frontend]
sources: [raw/MD/Spec-Client-IPC-And-UI-Bridge.md]
depends_on: [tera-core-spec]
---

# Client IPC & UI Bridge (TERA-CLIENT)

Source: `raw/MD/Spec-Client-IPC-And-UI-Bridge.md` — v1.0.0, 2026-03-29.

## What It Covers

The communication contract between Rust Core and Native Frontends (Flutter/Tauri/Swift). Frontend devs only need this file — no need to understand MLS or Crypto. Specifies FFI Token Protocol, SharedArrayBuffer Data Plane, UICommands, CoreSignals, and Streaming Decryption Local Proxy.

## Key Constraints

- UI Layer absolutely must not hold state — only render data via FFI Token
- UI absolutely must not port Crypto/Business Logic to Dart/JS Thread
- All FFI endpoints return Token Protocol — no raw pointers
- Unidirectional: Core pushes StateChanged signal → UI pulls snapshot
- Streaming Decryption Proxy: stream to 127.0.0.1 loopback — no plaintext on disk

## Related Concepts

- [[Glassmorphism Design System]]
- [[Terachat Architecture Overview]]
- [[Zero-Knowledge Architecture]]
