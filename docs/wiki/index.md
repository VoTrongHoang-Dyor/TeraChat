# Wiki Index

Last updated: 2026-05-10

## Entities

*No pages yet*

## Concepts

### Core Architecture
- [[Terachat Architecture Overview]] — System layers, spec dependency graph, role-based navigation — 2026-05-10
- [[Zero-Knowledge Architecture]] — Blind router model, key material isolation, mathematical security guarantees — 2026-05-10
- [[Enterprise License Model]] — License-gated access, deployment tiers, cryptographic license entanglement — 2026-05-10

### Data & Sync
- [[CRDT Dual-Sync Pattern]] — Two-plane sync (CRDT DAG + Relational), why not CRDT for everything — 2026-05-10
- [[Data Sovereignty & Export]] — SPF format, streaming decryption export, migration from Slack/Teams — 2026-05-10

### Runtime & Security
- [[WASM Tapp Runtime]] — Dual-engine sandbox (wasmtime/wasm3), Host ABI, permission model — 2026-05-10
- [[Survival Mesh Networking]] — BLE/Wi-Fi Direct P2P, EMDP protocol, BLE QoS priority system — 2026-05-10
- [[Enterprise Identity & Governance]] — DID, OPA Policy Engine, RBAC, SCIM, immutable audit trail — 2026-05-10
- [[Secure Enclave & AI Security]] — On-premise AI appliance, PII redaction, ZK Memory Agent — 2026-05-10

### Design
- [[Glassmorphism Design System]] — Security-visible UI, adaptive modes, IPC signal-to-visual mapping — 2026-05-10

### Legacy (Sample)
- [[Large Language Models]] — AI systems trained on text for next-token prediction — 2026-05-06
- [[Transformer Architecture]] — Dominant LLM architecture introduced by Vaswani et al. 2017 — 2026-05-06

## Sources

### Gateway & Supporting Docs
- [[tera-intro]] — System gateway: product definition, architecture invariants, deployment models, doc navigation — 2026-05-10
- [[tera-design]] — Design contract: Glassmorphism, visual modes, animation timing, IPC signal mapping — 2026-05-10
- [[tera-arrange]] — Documentation changelog: when files were added, removed, or restructured — 2026-05-10
- [[tera-note]] — Engineering notes: DevSecOps, dependencies, build tools, DBs, credentials, Prompt Injection — 2026-05-10

### Quality Engineering
- [[tera-tech-debt]] — Technical debt registry: 16 debt items, 10 platform limitations, 10 critical gaps — 2026-05-10
- [[tera-test-matrix]] — Chaos engineering: 40 scenarios across 5 layers, CI gates, Gov/Military checklist — 2026-05-10

### Core Domain Specs (7)
- [[tera-core-spec]] — TERA-CORE: MLS E2EE, Hybrid PQ-KEM, Hardware Root of Trust, Survival Mesh — 2026-05-10
- [[tera-sync-spec]] — TERA-SYNC: CRDT DAG + Relational dual-sync, SQLite WAL, Blob CAS, FTS5 — 2026-05-10
- [[tera-runtime-spec]] — TERA-RUNTIME: WASM dual-engine, Host ABI, Event Bus, Background Execution — 2026-05-10
- [[tera-enclave-spec]] — TERA-ENCLAVE: AI security, PII redaction, Local Appliance Model, ZK Memory Agent — 2026-05-10
- [[tera-gov-spec]] — TERA-GOV: DID, OPA ABAC, SCIM/OIDC/SAML, Audit Trail, RBAC, Legal Hold — 2026-05-10
- [[tera-client-spec]] — TERA-CLIENT: FFI Token Protocol, IPC Data Plane, CoreSignals, Streaming Proxy — 2026-05-10
- [[tera-eco-spec]] — TERA-ECO: App Signing PKI, Registry, MDM Distribution, Kill-switch, Transparency Log — 2026-05-10

### Additional Specs
- [[tera-migration-spec]] — Third-party migration: Slack/Teams/Google Chat → TeraChat DID + CRDT + OPA — 2026-05-10
- [[tera-export-spec]] — Data export: streaming decryption pipeline, SPF format, cryptographic manifest — 2026-05-10

### Legacy (Sample)
- [[llm-overview]] — Sample LLM intro: training, Transformer, capabilities, limitations — 2026-05-06

## Syntheses

*No pages yet*

## Other

*No pages yet*
