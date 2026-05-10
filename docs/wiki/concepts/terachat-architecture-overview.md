---
type: concept
created: 2026-05-10
tags: [terachat, architecture, overview, rust, flutter, tauri]
sources: [tera-intro, tera-core-spec, tera-client-spec, tera-sync-spec, tera-runtime-spec, tera-enclave-spec, tera-gov-spec, tera-eco-spec]
---

# TeraChat Architecture Overview

A synthesis map of how TeraChat's components fit together — the "big picture" for onboarding.

## System Layers

```
┌──────────────────────────────────────────────┐
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
│  └──────────────────────────────────────────┘ │
├──────────────────────────────────────────────┤
│              HARDWARE ROOT OF TRUST           │
│  Secure Enclave (Apple) · StrongBox (Android)│
│  TPM 2.0 (Desktop)                           │
└──────────────────────────────────────────────┘
```

## Domain Spec Map

| Spec | Domain | What It Owns |
|------|--------|-------------|
| TERA-CORE | Crypto & Mesh | MLS, PQ-KEM, Hardware keys, Survival mesh |
| TERA-SYNC | Sync & Storage | CRDT DAG, relational sync, SQLite, Blob CAS |
| TERA-RUNTIME | WASM Runtime | .tapp sandbox, Host ABI, Event Bus |
| TERA-ENCLAVE | Secure Enclave | AI security, PII redaction, on-prem appliance |
| TERA-GOV | Identity & Governance | DID, OPA, RBAC, SCIM, Audit trail |
| TERA-CLIENT | IPC & UI Bridge | FFI protocol, UI signals, streaming proxy |
| TERA-ECO | Ecosystem | App signing, PKI, Registry, Kill-switch |

## Dependency Flow

```
TERA-CORE (no deps — foundation)
  ├→ TERA-SYNC
  │    ├→ TERA-RUNTIME
  │    │    ├→ TERA-CLIENT
  │    │    └→ TERA-ECO
  │    └→ TERA-GOV
  │         ├→ TERA-RUNTIME
  │         ├→ TERA-ENCLAVE
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
| Security Auditor | TERA-CORE + TERA-GOV |
| System Architect | All 7 specs + Introduction |
| IT Admin | TERA-GOV + TERA-ECO |
