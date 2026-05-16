# TeraChat Agent Context

```yaml
id: "TERA-AGENT-CONTEXT"
version: "1.0.0"
date: "2026-05-15"
purpose: "First file every AI agent must read before touching TeraChat code"
```

## What are you building?

TeraChat is an **enterprise Work OS** with Zero-Knowledge E2EE, self-hosted on Mac mini + NAS, with local AI inference and a .tapp WASM ecosystem.

| Pillar | Technology | Why |
|--------|-----------|-----|
| E2EE Messaging | MLS RFC 9420 (openmls) | Audit-proof crypto for enterprise mid-market |
| Self-Hosted | Mac mini + NAS cluster | No data leaves customer premises |
| Local AI | Qwen2.5 / Gemma 2 (MLX) | AI features without sending data to cloud |
| .tapp Ecosystem | WASM sandbox (wasmtime + wasm3) | Differentiation from Mattermost/Element |
| Mesh Fallback | BLE/Wi-Fi Direct (emergency) | Eliminates single point of failure |

## Reading Order

1. **`CLAUDE.md`** — invariants never to violate, forbidden patterns, dependency policy
2. **`docs/wiki/ubiquitous-language.md`** — shared vocabulary (English + Vietnamese)
3. **`docs/wiki/invariants.md`** — detailed invariant explanations with code examples
4. **`phase/README.md`** — current slice, priorities, timeline
5. **Spec file** relevant to your task (from `docs/raw/MD/` or `docs/wiki/sources/`)

## Before Writing Code

- [ ] **Is there a test?** (TDD — write the test first, then implement)
- [ ] **Does the interface have > 5 public items?** If yes → redesign into sub-modules (Deep Module principle)
- [ ] **Does it violate any CLAUDE.md invariant?** Check before submitting
- [ ] **Are you using the right ubiquitous language terms?**
- [ ] **Is key material handled correctly?** ZeroizeOnDrop on ALL key structs, no raw pointers in FFI, no println! with keys

## File Scope Per Agent

| Agent | Scope | Never Touch |
|-------|-------|-------------|
| tc-crypto | `source/core/tc-crypto/**` only | tc-mesh, tc-sync |
| tc-mesh | `source/core/tc-mesh/**` only | tc-crypto internals |
| tc-sync | `source/core/tc-crdt-sync/**` + `source/core/tc-store/**` | — |
| tc-runtime | `source/core/tc-tapp/**` | — |
| tc-proto | `source/core/tc-proto/**` + `source/core/proto/**` | — |
| tc-client | `source/clients/**` | Rust Core |

## Crypto Stack (Decided — Do Not Change)

- E2EE messaging: `openmls` (MLS RFC 9420)
- Symmetric: `ring::aead::AES_256_GCM`
- Signing: `ring::signature::Ed25519`
- Hash: `blake3`
- PQ-KEM: `ml-kem` crate (NIST FIPS 203) — Phase 2 only
- Signal Protocol: NOT used — MLS is the decision

## Current Priority

<!-- Update this section daily by the human architect -->

- **Current Slice:** Slice 0 — Foundation (Week 1-2)
- **Today's focus:** Repository compiles, CI green, proto scaffolding valid
- **Next milestone:** Slice 1 "Hello E2EE" — two processes on same Mac, MLS roundtrip

## Deep Module Principle

Every module must follow Matt Pocock's deep module design:
- **Simple interface** (≤ 5 public functions/types)
- **Complex interior** (hidden implementation details)
- **CI enforced:** Public items > 7 triggers refactor warning

## Invariant Quick Reference

1. **ZeroizeOnDrop** on ALL key material types
2. **No raw pointer** in `pub extern "C"` — use Token Protocol (opaque `u64`)
3. **hot_dag.db is APPEND-ONLY** — no UPDATE/DELETE on CRDT events
4. **UI is passive renderer** — no business logic in Dart/SwiftUI
5. **All crypto through `ring` or `openmls`** — no self-implemented crypto
6. **iOS election_weight = 0** — iPhone never becomes mesh coordinator
7. **No persist plaintext key to disk** — not even in logs
