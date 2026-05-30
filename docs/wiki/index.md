# Wiki Index

Last updated: 2026-05-30 — **Wiki Sync Sprint (v2.0)** — Đồng bộ với nguồn sự thật (Senior Architect Perspective). Sửa 9 mâu thuẫn kiến trúc.

## TeraChat — Product Definition

TeraChat is an **enterprise Work OS** for internal and branch-company communication. It combines Zero-Knowledge E2EE (MLS RFC 9420), self-hosted Mac mini + NAS, local AI inference (Qwen2.5/MLX), and a .tapp WASM ecosystem — all under a blind router architecture where servers never see plaintext.

**Development model:** Vertical Slice (shippable every 6-8 weeks) with Multi-Agent Harness (LangGraph + Claude Code).
**Philosophy:** Deep Modules (Matt Pocock) — simple interfaces, complex interiors.

**Scope:** Internal messaging + branch company communication. NOT customer-facing messaging.

## Entities

*No pages yet*

## Concepts

### Vision & Roadmap
- [[Phase Framework]] — Three economic phases: Sign Pilot, Renew/Upsell, Moat/Ecosystem — 2026-05-11
- [[Vision Redefinition 2026-05-11]] — Synthesis: scope, authority model, Work OS, local AI — 2026-05-11

### Methodology & Development
- [[Vertical Slice Development]] — Shippable every 6-8 weeks, one use case across all layers — 2026-05-15
- [[Multi-Agent Harness]] — LangGraph orchestrator, agent types and scope boundaries, daily workflow — 2026-05-15
- [[Deep Module Design]] — Matt Pocock principle: simple interfaces (≤ 5 items), complex interiors — 2026-05-15

### Core Architecture
- [[Terachat Architecture Overview]] — System layers, communication model, spec dependency graph, role-based navigation — 2026-05-11
- [[Zero-Knowledge Architecture]] — Blind router model, key material isolation, mathematical security guarantees — 2026-05-10
- [[Enterprise License Model]] — License-gated access, deployment tiers, cryptographic license entanglement — 2026-05-10
- [[ADR-006 AI Gateway Architecture]] — **ACCEPTED** Loại bỏ local proxy: TeraRelay Extension (Phase 1) → Native Rust SDK tc-enclave (Phase 2D), PII Gate mandatory — 2026-05-12
- [[Platform Architecture]] — 3-tier license (CLOSED/BSL 1.1/MIT), BSL boundary, module diagram, ecosystem flywheel — 2026-05-16
- [[Threat Model]] — STRIDE for 3 attack vectors: relay compromise, device compromise, .tapp sandbox escape — 2026-05-16
- [[Codebase Directory Guide]] — Monorepo directory tree, module dictionary, config files, onboarding navigation — 2026-05-17
- [[ADR-007 Shadow Graph AI Resolution]] — **ACCEPTED 2026-05-30** AI chỉ được tạo Shadow Branch, human approval bảt buộc trước commit, fallback manual merge — 2026-05-30
- [[ADR-008 Delegated Proposer TreeKEM]] — **ACCEPTED 2026-05-30** Mobile ủy quyền tính toán TreeKEM cho Fat Client, TEE chỉ Verify + Sequence — 2026-05-30

### Communication Model
- [[Hierarchical Authority Messaging]] — Authority-gated communication, internal + branch only, no customer channels — 2026-05-11

### Data & Sync
- [[Dual-Sync Pattern]] — **[Cập nhật v2.0]** Append-Only Event Log + Vector Clocks cho chat; CRDT chỉ cho Collaborative Notes/Titles; Relational cho Finance/HR — 2026-05-30
- [[Data Sovereignty & Export]] — SPF format, streaming decryption export, migration from Slack/Teams — 2026-05-10

### Runtime & Work OS
- [[WASM Tapp Runtime]] — Dual-engine sandbox, Work OS marketplace, self-service deployment, regional/departmental scoping — 2026-05-11
- [[Survival Mesh Networking]] — BLE/Wi-Fi Direct P2P, EMDP protocol, BLE QoS priority system — 2026-05-10

### AI & Security
- [[Secure Enclave & AI Security]] — **[Cập nhật v2.0]** Qwen2.5 (llama.cpp + Metal) on-device AI; SEP = key storage ONLY (không phải general compute); ZK Memory Agent — 2026-05-30
- [[Open AI Framework]] — **[Cập nhật v2.0]** Bring-your-own-model, Qwen2.5 default (không phải Gemma 4), Host ABI cho AI inference, model registration — 2026-05-30
- [[AI Inference Offloading]] — Distributed inference scheduler, ThermalMonitor, ModelTier per device, PII gateway — 2026-05-15
- [[openmls Self-Healing]] — **[Cập nhật v2.0]** Shadow Graph AI (không auto-apply), human approval bảt buộc, fallback manual merge — 2026-05-30
- [[PII Redaction Rules]] — SanitizedPrompt newtype, pipeline flow, bang redaction rules (CCCD/Phone/Email/CC/Custom), DomainPiiPolicy — 2026-05-22

### Infrastructure & Operations
- [[Mac Mini HA Cluster]] — Zero-ops setup, mDNS discovery, Raft consensus, 99.99% SLA with TeraLink fallback — 2026-05-15
- [[Hardware Specification]] — Compute/Storage/AI Node separation, ECC RAM requirement, HPE for Gov/Military, tiers by concurrent sessions — 2026-05-16
- [[TeraLink Fallback Network]] — **[Cập nhật v2.0]** 3-tier fallback (T1/T2/T3), DMS 4-tier device classification, PQ-KEM tự disable khi T3 BLE — 2026-05-30
- [[iOS Mesh Storage Tiers]] — BufferTier (Minimal/Standard/Enhanced/Full), auto-detection, LRU eviction — 2026-05-15
- [[Tapp Community Framework]] — .tapp SDK, TappValidator CLI, TDD workflow for contributors — 2026-05-15

### Governance
- [[Enterprise Identity & Governance]] — DID, OPA Policy Engine, RBAC, SCIM, immutable audit trail, authority hierarchy enforcement — 2026-05-10

### Design
- [[Glassmorphism Design System]] — Security-visible UI, adaptive modes, IPC signal-to-visual mapping — 2026-05-10

### Legacy (Sample)
- [[Large Language Models]] — AI systems trained on text for next-token prediction — 2026-05-06
- [[Transformer Architecture]] — Dominant LLM architecture introduced by Vaswani et al. 2017 — 2026-05-06

### Agent Core Files (2026-05-16 v2.1 Update)
- [[Ubiquitous Language]] — Shared vocabulary EN+VI, anti-patterns, code conventions — 2026-05-16
- [[Invariants]] — 13 non-negotiable architectural rules with enforcement mechanisms, forbidden patterns, crypto stack — 2026-05-16

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
- [[tera-enclave-spec]] — TERA-ENCLAVE: AI security, PII redaction, Qwen2.5 local AI (không phải Gemma 4), open framework, ZK Memory Agent — 2026-05-10
- [[tera-gov-spec]] — TERA-GOV: DID, OPA ABAC, SCIM/OIDC/SAML, Audit Trail, RBAC, Legal Hold, authority hierarchy — 2026-05-10
- [[tera-client-spec]] — TERA-CLIENT: FFI Token Protocol, IPC Data Plane, CoreSignals, Streaming Proxy — 2026-05-10
- [[tera-eco-spec]] — TERA-ECO: .tapp Web Marketplace, self-service deploy, App Signing PKI, MDM, Kill-switch — 2026-05-10

### Additional Specs
- [[tera-migration-spec]] — Third-party migration: Slack/Teams/Google Chat → TeraChat DID + CRDT + OPA — 2026-05-10
- [[tera-export-spec]] — Data export: streaming decryption pipeline, SPF format, cryptographic manifest — 2026-05-10

### Legacy (Sample)
- [[llm-overview]] — Sample LLM intro: training, Transformer, capabilities, limitations — 2026-05-06

## Syntheses

### Master Documents
- [[00_Architecture_Overview]] — Kiến trúc tổng quan, WorkOS, BYO-Server, UI/UX, Phase roadmap — 2026-05-21
- [[01_Mesh_and_Crypto]] — Chi tiết mật mã (Zero-Knowledge, MLS, PQ-KEM), Mạng lưới TeraLink Mesh, Crypto audit fixes — 2026-05-21
- [[02_WorkOS_and_Tapp_Ecosystem]] — Hệ sinh thái .tapp, Marketplace vs Enterprise, DataGrant, Trust Chain — 2026-05-21
- [[03_Local_AI_Integration]] — Tích hợp AI cục bộ: Qwen2.5 default, PII redaction pipeline, ZK Memory Agent, inference offloading, BYOM, multi-agent harness, ADR-006 — 2026-05-21

### Improvement & Restructuring (2026-05-11 / 2026-05-12)
- [[improvement-plan-2026-05-11]] — **IMPROVEMENT HUB** — Central navigation: 14 weaknesses → 14 solutions, step-by-step execution order
- [[narrowed-phase-1-mvp]] — Narrowed Phase 1 scope: MLS + License + OIDC + 1 t-app (deferred: PQ, mesh, AI, marketplace)
- [[gap-resolution-tracker]] — GAP Resolution Tracker: 10 GAPs (A-J), all resolved with final architectural decisions — 2026-05-12
- [[platform-rollout-phasing]] — Platform rollout: macOS+iPhone first → Android → Windows → Linux → Huawei
- [[prototype-first-model]] — Prototype-first + Progressive Complexity: 1 subsystem per phase, demo in 4-6 weeks
- [[deployment-automation-spec]] — Deployment spec: 1-command install, backup/recovery, monitoring, staging, cost model
- [[ci-cd-pipeline-spec]] — CI/CD pipeline: progressive gates, quality checks per phase, secrets management, hermetic builds — 2026-05-12
- [[platform-limitation-registry]] — Cross-platform limitation registry: 10 XPLAT items, disclosure requirements, platform SLA matrix — 2026-05-12
- [[quantitative-phase-metrics]] — Quantitative metrics for every phase + market validation gates
- [[ai-independent-workstream]] — AI decoupled from messaging core via Host ABI boundary
- [[security-review-requirements]] — Security review requirements: who, what, when, budget estimates
- [[vision-redefinition-2026-05-11]] — Vision redefinition: scope, authority model, Work OS, Gemma 4, open AI framework

### Health Checks
- [[wiki-health-check-2026-05-10]] — Health check audit: 5 factual issues, 4 critical Phase 1 gaps, 2 Phase 2 gaps, cross-ref consistency — 2026-05-10

## Other

*No pages yet*
