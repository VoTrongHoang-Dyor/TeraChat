---
type: source
created: 2026-05-10
tags: [terachat, data-export, sovereignty, spf, streaming, compliance]
sources: [raw/MD/Spec-Data-Export-And-Sovereignty.md]
---

# Data Export & Sovereignty (TERA-EXPORT)

Source: `raw/MD/Spec-Data-Export-And-Sovereignty.md` — added 2026-05-05.

## What It Covers

Technical architecture for secure data export from TeraChat. Since all data is E2EE and stored as CRDT DAGs, traditional server-side database dumps are impossible. Specifies a client-side streaming decryption and transformation pipeline that produces standard formats while maintaining Zero-Knowledge constraints and OPA access rules.

## Key Requirements

- **Input:** Export request + DID signature (cryptographic proof of identity)
- **Output:** Structured ZIP archive with JSON/CSV/HTML + Cryptographic Manifest (`manifest.sig`)
- **Streaming Decryption:** Gigabytes of history require streaming pipeline to prevent OOM
- **OPA Enforcement:** Users cannot export data from channels they were removed from before export initiation
- **Format:** Sovereign Portability Format (SPF) — preserves Organization → Mesh → Channels → Threads hierarchy

## Non-Goals

- Server-side plaintext export (violates ZK architecture)
- Exporting data the user's DID keys cannot mathematically decrypt

## Related Concepts

- [[Data Sovereignty & Export]]
- [[Zero-Knowledge Architecture]]
- [[CRDT Dual-Sync Pattern]]
