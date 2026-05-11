---
type: concept
created: 2026-05-10
modified: 2026-05-11
tags: [terachat, ai, secure-enclave, pii, on-premise, zk-memory, gemma, open-framework]
sources: [tera-enclave-spec, tera-core-spec, tera-gov-spec]
---

# Secure Enclave & AI Security

TeraChat's approach to running AI workloads without leaking enterprise data to cloud providers. Uses on-premise hardware enclaves, PII redaction, local-only memory consolidation, and an **open AI framework** that lets enterprises integrate their own models. The default bundled model is **Google Gemma 4** — running entirely on-device.

## The Local Appliance Model

Instead of sending data to OpenAI/Anthropic clouds:

```
┌─────────────────────────────────────────┐
│         CUSTOMER ON-PREMISE              │
│                                          │
│  Control Plane (Mac mini M4 Pro)         │
│  • TeraRelay routing                     │
│  • DAG synchronization                   │
│  • ZK Memory Agent                       │
│                                          │
│  Compute Plane (RTX Node - Optional)     │
│  • Dedicated inference servers           │
│  • Heavy AI tasks (bulk summarization)   │
│  • LAN-connected only                    │
│                                          │
│  Data Plane (TrueNAS)                    │
│  • Encrypted Blobs                       │
│  • Local AI Model Registry               │
└─────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│         EMPLOYEE DEVICE (Local AI)       │
│                                          │
│  Gemma 4 (ONNX, bundled)                │
│  • Runs 100% on-device                   │
│  • No cloud call — data stays local      │
│  • Task automation for employees         │
│                                          │
│  Open AI Framework                       │
│  • Enterprise can register custom models │
│  • All prompts sanitized before inference│
│  • BLAKE3 integrity check on every load  │
└─────────────────────────────────────────┘
```

## Gemma 4 — Bundled Local AI

Google Gemma 4 is the default AI model bundled with TeraChat:

| Property | Specification |
|----------|---------------|
| Model | Gemma 4 (Google open model family) |
| Format | ONNX (cross-platform) / CoreML (Apple) |
| RAM Budget | 4–8 GB depending on variant |
| Execution | 100% on-device — no network call |
| Tasks | Summarization, drafting, extraction, classification, translation |
| Customization | Enterprise fine-tuning via LoRA adapters |

**Why Gemma 4:** Open weights mean no vendor lock-in and no API costs. Runs locally which preserves Zero-Knowledge. Small enough for employee machines. Google's open model commitment provides longevity.

## Open AI Framework

TeraChat does not lock enterprises into a single AI provider. The open AI framework allows:

- **Register any ONNX model** via Admin Console
- **Bring your own API key** for cloud models (Claude, etc.) — prompts proxied through enterprise relay with sanitization
- **Department/region scoping** — deploy specific models to specific teams
- **Model integrity verification** — BLAKE3 hash checked on every load

For details, see [[Open AI Framework]].

## PII Protection

**SanitizedPrompt:** A newtype wrapper that guarantees non-reversible PII redaction before any data reaches an AI model — regardless of whether the model is local (Gemma 4) or remote (bring-your-own API key). The embedded NER model's entity dictionary is configured via tenant-specific `DomainPiiPolicy`.

This applies to ALL models in the open framework — the sanitization layer is universal, not per-model.

## ZK Memory Agent

Consolidates user memories (preferences, frequent contacts, patterns) locally via Unix Domain Socket. Never leaves the device. No cloud dependency. Consolidation trigger is explicit, not automatic.

## 🧠 Design Decisions (Q&A)

- **Why on-device AI instead of server-side?** → Server-side AI would see plaintext employee data — violates Zero-Knowledge. Local execution means the AI runs on the same machine that owns the data. For heavy workloads, the on-prem compute plane processes sanitized prompts only. Trade-off: higher RAM requirements per device (8GB+ recommended).

- **Why SanitizedPrompt as a type-level guarantee?** → Runtime checks can be bypassed or forgotten. A newtype in Rust ensures compiler-enforced redaction — code that passes a raw string to the AI endpoint won't compile. This applies to all models in the open framework, not just Gemma 4. Trade-off: redaction must happen at the exact boundary, adding latency.

- **Why an open framework instead of one bundled model?** → Enterprises have diverse AI needs. A bank's compliance model differs from a manufacturer's defect-detection model. Forcing one model blocks adoption. The open framework with a strong default (Gemma 4) gives both simplicity and flexibility. Trade-off: model compatibility matrix, framework maintenance overhead.

- **What case does PII redaction miss?** → Contextual PII — information identifiable only in combination (e.g., "the CFO" + company name). Micro-NER models catch explicit PII but miss inferential PII. Trade-off: documented limitation, enterprise must configure DomainPiiPolicy thoroughly.
