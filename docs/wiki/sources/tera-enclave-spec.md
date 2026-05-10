---
type: source
created: 2026-05-10
tags: [terachat, secure-enclave, ai, pii, zk-memory, enclave]
sources: [raw/MD/Spec-Enterprise-Secure-Enclave.md]
depends_on: [tera-core-spec, tera-gov-spec]
---

# Enterprise Secure Enclave & AI Security (TERA-ENCLAVE)

Source: `raw/MD/Spec-Enterprise-Secure-Enclave.md` — v1.0.0, 2026-03-29.

## What It Covers

Enclave-level security mechanisms, PII data protection, and AI permission boundaries within the system. Defines the Local Appliance Model, ZK Memory Agent, and hardware trust boundaries for on-premise AI.

## Key Architectural Decisions

- **AI PII Redaction:** `SanitizedPrompt` newtype enforces non-reversible PII redaction before AI payload submission. NER entity dictionary configured via tenant-specific `DomainPiiPolicy`.
- **Local Appliance Model:** Shifts from cloud AI to on-prem hardware:
  - Control Plane (Mac Node): TeraRelay routing, DAG sync, ZK Memory Agent
  - Compute Plane (RTX Node): Dedicated inference servers for massive RAG
  - Data Plane (NAS Node): TrueNAS for encrypted Blobs and Vector Indices
- **ZK Memory Agent:** Consolidates user memories locally via Unix Domain Socket. No cloud dependency.

## Related Concepts

- [[Secure Enclave & AI Security]]
- [[Zero-Knowledge Architecture]]
- [[Enterprise Identity & Governance]]
