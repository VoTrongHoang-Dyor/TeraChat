---
type: concept
created: 2026-05-10
modified: 2026-05-11
tags: [terachat, architecture, overview, rust, flutter, tauri, work-os, hierarchical-authority, local-ai, gemma]
sources: [tera-intro, tera-core-spec, tera-client-spec, tera-sync-spec, tera-runtime-spec, tera-enclave-spec, tera-gov-spec, tera-eco-spec]
---

# TeraChat Architecture Overview

A synthesis map of how TeraChat's components fit together — the "big picture" for onboarding.

## Product Redefinition (2026-05-11)

TeraChat has been redefined with a clearer scope boundary:

| Was (Before) | Is (Now) |
|-------------|----------|
| Enterprise messaging + potential customer reach | **Internal + branch company messaging only** |
| Open-ended communication model | **Hierarchical authority-based communication** |
| Admin-deployed plugins | **Self-service t-apps from Web Marketplace (Work OS)** |
| AI as future feature | **Local AI with Gemma 4 + Open AI Framework (now)** |
| Single AI provider | **Open framework — bring your own AI model** |

### What TeraChat Is

- An **internal enterprise communication platform** for organizations and their branch companies
- A **Work OS** — business tasks run through vetted .tapp mini-applications
- A **local AI platform** — Gemma 4 on-device for employee task automation, with an open framework for custom models
- Communication governed by **hierarchical authority** — messages flow along org chart lines

### What TeraChat Is NOT

- A customer messaging tool — we cannot force customers to switch platforms
- A public app or consumer product
- A Slack/Teams clone — the Work OS + local AI + hierarchical authority model is a different category

## System Layers

```
┌──────────────────────────────────────────────┐
│         WORK OS LAYER (Business Tasks)         │
│  .tapp Marketplace · Self-Service Deploy       │
│  Regional/Department Scoping                   │
├──────────────────────────────────────────────┤
│              AI LAYER (Local + Open)           │
│  Gemma 4 (ONNX, bundled) · Open AI Framework  │
│  SanitizedPrompt · PII Redaction · Egress Guard│
├──────────────────────────────────────────────┤
│              UI LAYER (Passive Renderer)      │
│  Flutter (Mobile) · Tauri (Desktop)           │
│  Renders state from Rust Core via IPC         │
├──────────────────────────────────────────────┤
│              IPC / FFI BOUNDARY               │
│  Token Protocol · SharedArrayBuffer Data Plane│
│  CoreSignals (push) · UICommands (pull)       │
├──────────────────────────────────────────────┤
│              RUST CORE (Shared Binary)         │
│  ┌──────────┬──────────┬──────────┬────────┐ │
│  │ tc-crypto│ tc-mesh  │tc-crdt-  │tc-store│ │
│  │ MLS E2EE │ BLE/WiFi │ sync     │ SQLite │ │
│  │ PQ-KEM   │ Direct   │ CRDT DAG │ VFS    │ │
│  ├──────────┴──────────┴──────────┴────────┤ │
│  │ tc-tapp: WASM Sandbox (wasmtime/wasm3)   │ │
│  │ OPA Policy Engine (local enforcement)    │ │
│  │ License Validator + Audit Log            │ │
│  │ AI Host ABI (model invocation)           │ │
│  └──────────────────────────────────────────┘ │
├──────────────────────────────────────────────┤
│              HARDWARE ROOT OF TRUST           │
│  Secure Enclave (Apple) · StrongBox (Android)│
│  TPM 2.0 (Desktop)                           │
└──────────────────────────────────────────────┘
```

## Communication Model

```
         ┌──────────────┐
         │ HEADQUARTERS  │
         │ (Root Authority│
         └──────┬───────┘
                │
    ┌───────────┼───────────┐
    │           │           │
┌───▼───┐   ┌───▼───┐   ┌───▼───┐
│Branch A│  │Branch B│  │Branch C│
│(Region)│  │(Region)│  │(Region)│
└───┬───┘   └───┬───┘   └───┬───┘
    │           │           │
┌───▼───┐      ...         ...
│ Dept  │
│Finance│
└───────┘

Communication flows: Vertical (up/down hierarchy),
Horizontal (peer, same tier), Inter-Branch (HQ-authorized).
NO customer-facing channels.
```

## Domain Spec Map

| Spec | Domain | What It Owns |
|------|--------|-------------|
| TERA-CORE | Crypto & Mesh | MLS, PQ-KEM, Hardware keys, Survival mesh |
| TERA-SYNC | Sync & Storage | CRDT DAG, relational sync, SQLite, Blob CAS |
| TERA-RUNTIME | WASM Runtime | .tapp sandbox, Host ABI, Event Bus, AI inference ABI |
| TERA-ENCLAVE | Secure Enclave | AI security, PII redaction, Gemma 4, open AI framework |
| TERA-GOV | Identity & Governance | DID, OPA, RBAC, SCIM, Audit trail, authority hierarchy |
| TERA-CLIENT | IPC & UI Bridge | FFI protocol, UI signals, streaming proxy |
| TERA-ECO | Ecosystem & Marketplace | .tapp PKI, Web Marketplace, self-service deploy, kill-switch |

## Dependency Flow

```
TERA-CORE (no deps — foundation)
  ├→ TERA-SYNC
  │    ├→ TERA-RUNTIME (includes AI Host ABI)
  │    │    ├→ TERA-CLIENT
  │    │    └→ TERA-ECO (Web Marketplace)
  │    └→ TERA-GOV (authority hierarchy enforcement)
  │         ├→ TERA-RUNTIME
  │         ├→ TERA-ENCLAVE (Gemma 4 + open AI framework)
  │         ├→ TERA-ECO
  │         └→ TERA-CLIENT
  └→ TERA-ENCLAVE
```

## Navigation by Role

| Role | Read Order |
|------|-----------|
| Rust Core Dev | TERA-CORE → TERA-SYNC → TERA-RUNTIME |
| Frontend Dev | TERA-CLIENT only |
| .tapp Developer | TERA-RUNTIME → TERA-ECO |
| AI/ML Engineer | TERA-ENCLAVE → Open AI Framework |
| Security Auditor | TERA-CORE + TERA-GOV |
| System Architect | All 7 specs + Introduction |
| IT Admin | TERA-GOV + TERA-ECO (marketplace, deploy) |
| Branch Manager | Hierarchical Authority Messaging |
