---
type: concept
created: 2026-05-10
tags: [terachat, ai, secure-enclave, pii, on-premise, zk-memory]
sources: [tera-enclave-spec, tera-core-spec, tera-gov-spec]
---

# Secure Enclave & AI Security

TeraChat's approach to running AI workloads without leaking enterprise data to cloud providers. Uses on-premise hardware enclaves, PII redaction, and local-only memory consolidation.

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
│  • Massive RAG tasks                     │
│  • LAN-connected only                    │
│                                          │
│  Data Plane (TrueNAS)                    │
│  • Encrypted Blobs                       │
│  • Vector Indices                        │
└─────────────────────────────────────────┘
```

## PII Protection

**SanitizedPrompt:** A newtype wrapper that guarantees non-reversible PII redaction before any data reaches an AI model. The embedded NER model's entity dictionary is configured via tenant-specific `DomainPiiPolicy` — regional banking formats, healthcare IDs, etc.

## ZK Memory Agent

Consolidates user memories (preferences, frequent contacts, patterns) locally via Unix Domain Socket. Never leaves the device. No cloud dependency. Consolidation trigger is explicit, not automatic.

## 🧠 Design Decisions (Q&A)

- **Why on-premise AI instead of cloud API?** → Zero-Knowledge principle: cloud AI provider would see plaintext prompts. Even with "we don't train on your data" claims, the data leaves the trust boundary. On-premise keeps everything inside. Trade-off: customer must provision and maintain hardware.
- **Why SanitizedPrompt as a type-level guarantee?** → Runtime checks can be bypassed or forgotten. A newtype in Rust ensures compiler-enforced redaction — code that passes a raw string to the AI endpoint won't compile. Trade-off: redaction must happen at the exact boundary, adding latency.
- **What case does PII redaction miss?** → Contextual PII — information that is only identifiable in combination (e.g., "the CFO" + company name). Micro-NER models catch explicit PII but miss inferential PII. Trade-off: documented limitation, enterprise must configure DomainPiiPolicy thoroughly.
