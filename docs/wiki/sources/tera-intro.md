---
type: source
created: 2026-05-10
tags: [terachat, architecture, overview, enterprise]
sources: [raw/MD/Introduction.md]
---

# TeraChat System Gateway (TERA-INTRO)

Source: `raw/MD/Introduction.md` — v0.5.0, 2026-03-29.

## What It Covers

The system entry point for understanding TeraChat: a Zero-Knowledge, E2EE enterprise messaging platform. Defines the product, architecture invariants, deployment models, and document navigation map.

## Key Claims

- **Enterprise-only:** No public accounts. Every access is license-gated via JWT signed by HSM FIPS 140-3.
- **5 Immutable Architecture Principles:**
  1. Zero-Knowledge Server — relay is a blind router, never sees plaintext
  2. Key Material never leaves Secure Enclave/StrongBox/TPM
  3. Offline-First Survival — BLE 5.0 + Wi-Fi Direct P2P mesh when internet is lost
  4. Zero-Trust by Design — OPA Policy Engine enforces at device, not just server
  5. License Entanglement — wrong license = wrong key = database is garbage
- **Tech Stack:** Rust Core (shared binary), Flutter (mobile), Tauri (desktop), MLS RFC 9420 + QUIC/gRPC
- **Deployment Tiers:** Self-Hosted Cloud → On-Premise → Air-Gapped → Hybrid
- **Document map:** 7 Domain Specs + supporting files guide navigation by role

## Related Concepts

- [[Zero-Knowledge Architecture]]
- [[Enterprise License Model]]
- [[Terachat Architecture Overview]]
