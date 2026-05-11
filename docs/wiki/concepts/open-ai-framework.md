---
type: concept
created: 2026-05-11
tags: [terachat, ai, local-ai, gemma, open-framework, automation]
sources: [tera-enclave-spec, tera-runtime-spec]
---

# Open AI Framework & Local AI Integration

TeraChat brings AI to the employee's machine — not to a cloud dashboard. The goal is task automation through local models, with an open framework that allows enterprises to plug in their own AI providers. The initial target model is **Google Gemma 4**.

## Core Principle

```
┌─────────────────────────────────────────────────────┐
│                 TERACHAT AI STACK                     │
│                                                      │
│  ┌──────────────────────────────────────────────┐    │
│  │           OPEN AI FRAMEWORK (Host ABI)        │    │
│  │                                              │    │
│  │  Bring your own model. Register it. Run it.  │    │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────────┐  │    │
│  │  │ Gemma 4  │ │  Claude  │ │  Enterprise  │  │    │
│  │  │ (default)│ │ (bring)  │ │  Custom Model│  │    │
│  │  └──────────┘ └──────────┘ └──────────────┘  │    │
│  └──────────────────────────────────────────────┘    │
│                         │                            │
│  ┌──────────────────────▼────────────────────────┐   │
│  │           SANITIZATION LAYER                  │   │
│  │  PII Redaction → DomainPiiPolicy → Egress Guard│   │
│  └──────────────────────────────────────────────┘    │
│                         │                            │
│  ┌──────────────────────▼────────────────────────┐   │
│  │           LOCAL EXECUTION                     │   │
│  │  ONNX Runtime · CoreML · HiAI · GPU/NPU       │   │
│  └──────────────────────────────────────────────┘    │
│                                                      │
│  Data never leaves the device unless enterprise      │
│  explicitly configures an on-premise inference node. │
└─────────────────────────────────────────────────────┘
```

## Gemma 4 — Default Local Model

Google Gemma 4 is the initial target for local AI integration:

| Property | Target |
|----------|--------|
| Model Family | Gemma 4 (Google open model) |
| Deployment | ONNX format, bundled with TeraChat |
| RAM Budget | 4–8 GB (depending on variant) |
| Tasks | Document summarization, email drafting, data extraction, meeting notes |
| Execution | 100% local — no cloud API call |
| Customization | Fine-tunable per enterprise via LoRA adapters |

**Why Gemma 4:**
- Open weights — no vendor lock-in
- Small enough for local execution on employee machines
- ONNX export path for cross-platform deployment (CoreML for Apple, ONNX Runtime for Windows/Linux)
- Google's open model commitment means long-term availability

## Open AI Framework — Host ABI Extension

The WASM Host ABI is extended with an AI inference interface:

```rust
// Host ABI: AI Inference (open framework)
fn host_ai_invoke(
    model_id: &str,           // "gemma4", "claude-opus", "enterprise-custom"
    sanitized_prompt: &[u8],  // Already passed through PII redaction
    max_tokens: u32,
    temperature: f32,
) -> Result<AiResponse, AiError>;
```

### Model Registration

Any enterprise can register an AI model into the framework:

1. **Package model** in ONNX format (or CoreML `.mlmodelc` for Apple)
2. **Declare capabilities:** `manifest.json` — model name, version, RAM budget, supported tasks
3. **Sign with enterprise key:** Ed25519 signature from enterprise CA
4. **Deploy via Admin Console:** Push to specific departments or regions
5. **Model integrity check:** BLAKE3 hash verified on every load

### Supported Providers (Planned)

| Provider | Model | Deployment |
|----------|-------|------------|
| Google | Gemma 4 (default, bundled) | ONNX local |
| Anthropic | Claude (bring-your-own-key) | API with sanitized prompt proxy |
| Enterprise | Custom fine-tuned model | ONNX via Admin push |
| Open Source | Llama, Mistral, etc. | ONNX self-register |

## Data Sovereignty Guardrails

Even with an open framework, Zero-Knowledge invariants hold:

1. **All prompts pass through SanitizedPrompt** — PII redaction BEFORE model inference, regardless of model provider
2. **Local-first:** Gemma 4 and ONNX models run entirely on-device. No data leaves the machine.
3. **API-based models (Claude, etc.):** If enterprise brings their own API key, prompts are sanitized, then proxied through the enterprise's own relay. TeraChat Inc. never sees the prompt.
4. **Egress guard:** Model output is scanned for PII leakage before being returned to the user.
5. **Audit log:** Every AI invocation is signed and appended to the immutable audit trail — which model, which sanitized prompt hash, which user, when.

## Employee Task Automation (Target Use Cases)

| Task | Model | Data Source | Output |
|------|-------|-------------|--------|
| Summarize long thread | Gemma 4 | Channel messages (last 200) | Bullet-point summary |
| Draft response | Gemma 4 | Thread context + user style | Drafted message |
| Extract action items | Gemma 4 | Meeting notes / chat | Task list |
| Classify document | Gemma 4 | File attachment | Category + tags |
| Translate message | Gemma 4 | Foreign language text | Translated text |
| Data extraction | Custom model | Form / invoice image | Structured JSON |

## 🧠 Design Decisions (Q&A)

- **Why Gemma 4 as default instead of a proprietary model?** → Proprietary models require cloud API calls, breaking Zero-Knowledge. Gemma 4 is open-weight, runs locally, and Google has committed to the Gemma open model family. Trade-off: local model quality is lower than cloud giants, but privacy guarantee is absolute.

- **Why an open framework instead of a single bundled model?** → Enterprises have diverse AI needs and existing vendor relationships. A bank might have a compliance-trained model. A manufacturer might have a defect-detection model. Locking them into one provider kills adoption. Trade-off: framework complexity, model compatibility matrix.

- **Why run on employee machines instead of a central server?** → Central server AI sees all employee data in plaintext — violates Zero-Knowledge. Local execution means data stays on the machine that owns it. For tasks requiring more compute, the enterprise can deploy an on-premise inference node (Mac mini + RTX) that processes sanitized prompts only. Trade-off: higher hardware requirements per employee machine (8GB+ RAM recommended).
