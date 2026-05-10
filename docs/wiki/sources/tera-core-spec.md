---
type: source
created: 2026-05-10
tags: [terachat, crypto, mesh, e2ee, mls, pq-kem, hardware-security]
sources: [raw/MD/Spec-Core-Cryptography-And-Mesh.md]
---

# Core Cryptography & Mesh Network (TERA-CORE)

Source: `raw/MD/Spec-Core-Cryptography-And-Mesh.md` — v1.0.0, 2026-03-29.

## What It Covers

The cryptographic truth of the platform. Defines all crypto primitives, MLS E2EE protocol, Hybrid PQ-KEM, Hardware Root of Trust, and Survival Mesh Networking.

## Key Constraints

- ZeroizeOnDrop mandatory for all key-material structs
- All crypto via `ring` crate or RustCrypto — no self-implementation
- All private keys must reside in Secure Enclave (Apple) / StrongBox (Android) / TPM 2.0 (Desktop)
- No `mlock()` on iOS — use `kCFAllocatorMallocZone` + ZeroizeOnDrop
- All FFI endpoints use Token Protocol — no raw pointers
- Ed25519 signed, append-only Audit Log

## Consumed By

TERA-SYNC, TERA-RUNTIME, TERA-GOV, TERA-CLIENT, TERA-ENCLAVE

## Related Concepts

- [[Zero-Knowledge Architecture]]
- [[Survival Mesh Networking]]
- [[Enterprise Identity & Governance]]
