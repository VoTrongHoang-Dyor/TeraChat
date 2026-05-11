# Wiki Index

Last updated: 2026-05-11

## TeraChat — Product Definition

TeraChat is an **enterprise Work OS** for internal and branch-company communication, governed by hierarchical authority. It combines secure messaging, self-service business t-apps, and local AI (Gemma 4 + open framework) — all under a Zero-Knowledge architecture where servers never see plaintext.

**Scope:** Internal messaging + branch company communication. NOT customer-facing messaging.

## Entities

*No pages yet*

## Concepts

### Vision & Roadmap
- [[Phase Framework]] — Three economic phases: Sign Pilot, Renew/Upsell, Moat/Ecosystem — 2026-05-11
- [[Vision Redefinition 2026-05-11]] — Synthesis: scope, authority model, Work OS, local AI — 2026-05-11

### Core Architecture
- [[Terachat Architecture Overview]] — System layers, communication model, spec dependency graph, role-based navigation — 2026-05-11
- [[Zero-Knowledge Architecture]] — Blind router model, key material isolation, mathematical security guarantees — 2026-05-10
- [[Enterprise License Model]] — License-gated access, deployment tiers, cryptographic license entanglement — 2026-05-10

### Communication Model
- [[Hierarchical Authority Messaging]] — Authority-gated communication, internal + branch only, no customer channels — 2026-05-11

### Data & Sync
- [[CRDT Dual-Sync Pattern]] — Two-plane sync (CRDT DAG + Relational), why not CRDT for everything — 2026-05-10
- [[Data Sovereignty & Export]] — SPF format, streaming decryption export, migration from Slack/Teams — 2026-05-10

### Runtime & Work OS
- [[WASM Tapp Runtime]] — Dual-engine sandbox, Work OS marketplace, self-service deployment, regional/departmental scoping — 2026-05-11
- [[Survival Mesh Networking]] — BLE/Wi-Fi Direct P2P, EMDP protocol, BLE QoS priority system — 2026-05-10

### AI & Security
- [[Secure Enclave & AI Security]] — Gemma 4 on-device AI, open AI framework, PII redaction, ZK Memory Agent — 2026-05-11
- [[Open AI Framework]] — Bring-your-own-model, Gemma 4 default, Host ABI for AI inference, model registration — 2026-05-11

### Governance
- [[Enterprise Identity & Governance]] — DID, OPA Policy Engine, RBAC, SCIM, immutable audit trail, authority hierarchy enforcement — 2026-05-10

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
- [[tera-runtime-spec]] — TERA-RUNTIME: WASM dual-engine, Host ABI, Event Bus, AI inference ABI, Background Execution — 2026-05-10
- [[tera-enclave-spec]] — TERA-ENCLAVE: AI security, PII redaction, Gemma 4 local AI, open framework, ZK Memory Agent — 2026-05-10
- [[tera-gov-spec]] — TERA-GOV: DID, OPA ABAC, SCIM/OIDC/SAML, Audit Trail, RBAC, Legal Hold, authority hierarchy — 2026-05-10
- [[tera-client-spec]] — TERA-CLIENT: FFI Token Protocol, IPC Data Plane, CoreSignals, Streaming Proxy — 2026-05-10
- [[tera-eco-spec]] — TERA-ECO: .tapp Web Marketplace, self-service deploy, App Signing PKI, MDM, Kill-switch — 2026-05-10

### Additional Specs
- [[tera-migration-spec]] — Third-party migration: Slack/Teams/Google Chat → TeraChat DID + CRDT + OPA — 2026-05-10
- [[tera-export-spec]] — Data export: streaming decryption pipeline, SPF format, cryptographic manifest — 2026-05-10

### Legacy (Sample)
- [[llm-overview]] — Sample LLM intro: training, Transformer, capabilities, limitations — 2026-05-06

## Syntheses

### Improvement & Restructuring (2026-05-11)
- [[improvement-plan-2026-05-11]] — **IMPROVEMENT HUB** — Central navigation: 14 weaknesses → 14 solutions, step-by-step execution order
- [[narrowed-phase-1-mvp]] — Narrowed Phase 1 scope: MLS + License + OIDC + 1 t-app (deferred: PQ, mesh, AI, marketplace)
- [[gap-resolution-tracker]] — GAP Resolution Tracker: 10 GAPs (A-J), concrete decisions needed, current status
- [[platform-rollout-phasing]] — Platform rollout: macOS+iPhone first → Android → Windows → Linux → Huawei
- [[prototype-first-model]] — Prototype-first + Progressive Complexity: 1 subsystem per phase, demo in 4-6 weeks
- [[deployment-automation-spec]] — Deployment spec: 1-command install, 30-minute IT admin target
- [[quantitative-phase-metrics]] — Quantitative metrics for every phase + market validation gates
- [[ai-independent-workstream]] — AI decoupled from messaging core via Host ABI boundary
- [[security-review-requirements]] — Security review requirements: who, what, when, budget estimates
- [[vision-redefinition-2026-05-11]] — Vision redefinition: scope, authority model, Work OS, Gemma 4, open AI framework

### Health Checks
- [[wiki-health-check-2026-05-10]] — Health check audit: 5 factual issues, 4 critical Phase 1 gaps, 2 Phase 2 gaps, cross-ref consistency — 2026-05-10

## Other

*No pages yet*
