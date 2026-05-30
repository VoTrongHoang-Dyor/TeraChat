---
type: concept
created: 2026-05-11
updated: 2026-05-30
tags: [terachat, ai, local-ai, qwen, llama-cpp, metal, open-framework, automation]
sources: [tera-enclave-spec, tera-runtime-spec]
---

# Open AI Framework & Local AI Integration

> **Cập nhật 2026-05-30 (M-8 Fix):** Model mặc định thay từ "Gemma 4" → **Qwen2.5 (llama.cpp + Metal API)**. Gemma 4 chưa tồn tại. Qwen2.5 là model open-weight production-ready chạy tốt nhất trên Apple Silicon qua Metal.

TeraChat brings AI to the employee's machine — not to a cloud dashboard. Mục tiêu là task automation qua local models, với open framework cho enterprise plug in model riêng. Model mặc định bundled là **Qwen2.5 (Alibaba open weights)** chạy qua llama.cpp + Metal API.

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
│  │  │ Qwen2.5  │ │  Claude  │ │  Enterprise  │  │    │
│  │  │(default) │ │ (bring)  │ │  Custom Model│  │    │
│  │  └──────────┘ └──────────┘ └──────────────┘  │    │
│  └──────────────────────────────────────────────┘    │
│                         │                            │
│  ┌──────────────────────▼────────────────────────┐   │
│  │           LOCAL EXECUTION                     │   │
│  │  llama.cpp (Metal) · ONNX Runtime · CoreML    │   │
│  │  Apple Neural Engine · Qualcomm NPU            │   │
│  └──────────────────────────────────────────────┘    │
│                                                      │
│  Data never leaves the device unless enterprise      │
│  explicitly configures an on-premise inference node. │
└─────────────────────────────────────────────────────┘
```

## Qwen2.5 — Default Local Model

**Qwen2.5** (Alibaba open weights) là model bundled mặc định của TeraChat:

| Property | Specification |
|----------|---------------|
| Model Family | **Qwen2.5** (Alibaba DAMO Academy) |
| Runtime | **llama.cpp** + **Metal API** (Apple Silicon) / ONNX Runtime (Windows/Linux) |
| Variants | 0.5B (mobile), 7B (Mac Mini), 32B (RTX node optional) |
| RAM Budget | 1-2GB (0.5B), 8-16GB (7B), 64GB+ (32B) |
| Tasks | Thread summarization, response drafting, extraction, classification, translation |
| Execution | 100% local — no cloud API call |
| Hardware | Apple Neural Engine + Unified Memory bandwidth (M-series) |
| Customization | Fine-tunable qua LoRA adapters trên Mac Mini |

**Tại sao Qwen2.5:**
- Open weights + commercial license phù hợp enterprise deployment
- Production-ready MLX/llama.cpp format — chạy tốt trên Apple Silicon ngay hôm nay
- llama.cpp gọi trực tiếp Metal API: tận dụng Apple Neural Engine, không cần Docker
- Nhiều size variants: 0.5B cho mobile NPU, 7B cho Mac Mini, 32B cho RTX node
- Google Gemma 4 chưa tồn tại / chưa có stable production runtime tại thời điểm này

## Open AI Framework — Host ABI Extension

The WASM Host ABI is extended with an AI inference interface:

```rust
// Host ABI: AI Inference (open framework)
fn host_ai_invoke(
    model_id: &str,           // "qwen2.5-7b", "claude-opus", "enterprise-custom"
    prompt: &[u8],
    max_tokens: u32,
    temperature: f32,
) -> Result<AiResponse, AiError>;
```

### Model Registration

Any enterprise can register an AI model into the framework:

1. **Package model** in GGUF format (llama.cpp) or ONNX format (Windows/Linux)
2. **Declare capabilities:** `manifest.json` — model name, version, RAM budget, supported tasks
3. **Sign with enterprise key:** Ed25519 signature from enterprise CA
4. **Deploy via Admin Console:** Push to specific departments or regions
5. **Model integrity check:** BLAKE3 hash verified on every load

### Supported Providers (Planned)

| Provider | Model | Deployment |
|----------|-------|------------|
| Alibaba | **Qwen2.5** (default, bundled) | llama.cpp + Metal local |
| Anthropic | Claude (bring-your-own-key) | API via enterprise relay + PII redaction |
| Enterprise | Custom fine-tuned model | GGUF/ONNX via Admin push |
| Open Source | Llama, Mistral, etc. | GGUF self-register |

## Data Sovereignty Guardrails

Even with an open framework, Zero-Knowledge invariants hold:

1. **Local-first:** Qwen2.5 và ONNX models chạy 100% on-device. No data leaves the machine.
2. **API-based models (Claude, etc.):** Nếu enterprise mang API key riêng, prompts được proxied qua enterprise relay với PII redaction bắt buộc. TeraChat Inc. không bao giờ nhìn thấy prompt.
3. **Audit log:** Mọi AI invocation được Ed25519 sign và append vào immutable audit trail — model nào, user nào, khi nào.

## Employee Task Automation (Target Use Cases)

| Task | Model | Data Source | Output |
|------|-------|-------------|--------|
| Summarize long thread | Qwen2.5-7B (Mac Mini) | Channel messages (last 200) | Bullet-point summary |
| Draft response | Qwen2.5-7B | Thread context + user style | Drafted message |
| Extract action items | Qwen2.5-7B | Meeting notes / chat | Task list |
| Classify document | Qwen2.5-7B | File attachment | Category + tags |
| Translate message | Qwen2.5-0.5B (on-device) | Foreign language text | Translated text |
| Data extraction | Custom enterprise model | Form / invoice image | Structured JSON |

## 🧠 Design Decisions (Q&A)

- **Tại sao Qwen2.5 thay vì Gemma 4?** → Qwen2.5 có production-ready GGUF format và llama.cpp Metal backend cho Apple Silicon ngay hôm nay. Gemma 4 chưa tồn tại. Khi Gemma (hoặc model khác) có production Metal runtime tốt hơn, enterprise có thể swap qua Open AI Framework mà không cần update app. Trade-off: Qwen2.5 có naming ít quen thuộc hơn với Western market.

- **Tại sao Open Framework thay vì single bundled model?** → Enterprise có AI needs đa dạng. Bank cần compliance-trained model. Manufacturer cần defect-detection model. Lock vào một provider giết adoption. Trade-off: framework complexity, model compatibility matrix.

- **Tại sao chạy trên employee machine thay vì central server?** → Central server AI thấy toàn bộ employee data plaintext — vi phạm Zero-Knowledge. Local execution = data ở trên machine sở hữu nó. Với tasks cần compute lớn hơn, enterprise có thể deploy on-premise inference node (Mac mini + RTX) chỉ xử lý sanitized prompts. Trade-off: hardware requirements cao hơn mỗi máy (8GB+ RAM recommended).

- **Tại sao llama.cpp thay vì ONNX Runtime trên Mac?** → llama.cpp gọi trực tiếp Metal API, tận dụng Apple Neural Engine và Unified Memory bandwidth của M-series. ONNX Runtime trên macOS không exploit được NPU. Docker/K8s Linux tạo overhead ảo hóa 30%. Native daemon dưới launchd = kiểm soát tuyệt đối thermal và clock.
