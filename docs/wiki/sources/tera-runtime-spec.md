---
type: source
created: 2026-05-10
tags: [terachat, wasm, runtime, tapp, sandbox, sdk]
sources: [raw/MD/Spec-Wasm-Tapp-Runtime.md]
depends_on: [tera-core-spec, tera-sync-spec]
---

# WASM .tapp Runtime & SDK (TERA-RUNTIME)

Source: `raw/MD/Spec-Wasm-Tapp-Runtime.md` — v1.0.0, 2026-03-29.

## What It Covers

The dual-engine WASM execution environment for enterprise mini-apps (.tapp): Host ABI, Local Event Bus, Background Execution, Encrypted App Storage. Defines the memory boundary and RAM limits for .tapp developers.

## Key Constraints

- iOS: `wasm3` interpreter only — JIT absolutely forbidden (W^X)
- All crypto ops in WASM must delegate to Rust Core via Host ABI
- WASM sandbox strips `wasi-sockets` — no direct TCP/UDP
- Egress_Outbox hard limit 2MB — exceed = sealed + terminate
- Inbound Webhook isolated from Egress_Outbox (separate quota & pathway)
- Background task WASM: max 10MB RAM when OS suspended
- .tapp permissions must be fully declared in Manifest — no runtime requests

## Consumed By

TERA-CLIENT, TERA-ECO

## Related Concepts

- [[WASM Tapp Runtime]]
- [[Enterprise Identity & Governance]]
- [[Zero-Knowledge Architecture]]
