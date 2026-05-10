---
type: source
created: 2026-05-10
tags: [terachat, ecosystem, trust-chain, tapp, pki, marketplace, mdm]
sources: [raw/MD/Spec-Ecosystem-And-Trust-Chain.md]
depends_on: [tera-core-spec, tera-runtime-spec, tera-gov-spec]
---

# Ecosystem Governance & Trust Chain (TERA-ECO)

Source: `raw/MD/Spec-Ecosystem-And-Trust-Chain.md` — v1.0.0, 2026-03-29.

## What It Covers

The .tapp application lifecycle: App Signing Trust Hierarchy, Enterprise Private Distribution (MDM/EMM), Publisher Trust Tiers, Security Review pipeline, Emergency Kill-switch, and Registry Transparency. Defines the cryptographic trust of the entire ecosystem.

## Key Constraints

- No TeraChat CA signature → Rust Core refuses to load — no exceptions
- All .tapp capabilities declared in Manifest — no runtime permission requests
- Revocation must be effective ≤ 60s on all online devices
- Emergency Kill-switch requires no app update or store review
- Private Enterprise Distribution: .tapp does not need TeraChat public Registry
- WasmParity CI gate mandatory: wasm3 vs wasmtime semantically identical

## Related Concepts

- [[WASM Tapp Runtime]]
- [[Enterprise Identity & Governance]]
- [[Zero-Knowledge Architecture]]
