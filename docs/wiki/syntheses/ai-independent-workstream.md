---
type: synthesis
created: 2026-05-11
tags: [terachat, ai, gemma, open-framework, workstream, decoupling]
sources: [open-ai-framework, secure-enclave-ai, narrowed-phase-1-mvp, progressive-complexity-model]
status: resolved
resolves: "Điểm yếu #8 — AI gắn với messaging core"
---

# AI Independent Workstream

**Quyết định:** AI được tách thành workstream độc lập, giao tiếp với messaging core qua một interface boundary rõ ràng. AI có thể ship riêng, fail riêng, và update riêng — không ảnh hưởng đến messaging core.

## Architecture Boundary

```
┌─────────────────────────────────────────┐
│         MESSAGING CORE (Phase 1)          │
│                                           │
│  MLS E2EE · License · OIDC/SAML · Sync   │
│                                           │
│  ┌─────────────────────────────────────┐ │
│  │       AI HOST ABI (Interface)       │ │
│  │                                     │ │
│  │  host_ai_invoke(model, prompt)      │ │
│  │  host_ai_register(model_config)     │ │
│  │  host_ai_status() → ModelStatus     │ │
│  │                                     │ │
│  │  Đây là RANH GIỚI. Dưới đây là AI. │ │
│  └─────────────────────────────────────┘ │
│                    │                      │
└────────────────────┼──────────────────────┘
                     │
┌────────────────────▼──────────────────────┐
│         AI WORKSTREAM (Phase 2D+)          │
│                                            │
│  ┌──────────┐ ┌──────────┐ ┌───────────┐  │
│  │ Gemma 4  │ │  Claude  │ │ Enterprise│  │
│  │ (default)│ │ (API key)│ │  Custom   │  │
│  └────┬─────┘ └────┬─────┘ └─────┬─────┘  │
│       │            │             │        │
│  ┌────▼────────────▼─────────────▼─────┐  │
│  │      SANITIZATION PIPELINE          │  │
│  │  PII Redaction → Egress Guard       │  │
│  └────────────────┬────────────────────┘  │
│                   │                       │
│  ┌────────────────▼────────────────────┐  │
│  │      MODEL RUNTIME                   │  │
│  │  ONNX Runtime · CoreML · GPU/NPU     │  │
│  └─────────────────────────────────────┘  │
│                                            │
│  Có thể ship, fail, update ĐỘC LẬP        │
└────────────────────────────────────────────┘
```

## Interface Contract

```rust
// Trong tc-tapp (messaging core) — AI Host ABI
// File: source/core/tc-tapp/src/ai_abi.rs

/// AI Model status
#[derive(Debug)]
pub enum ModelStatus {
    NotLoaded,
    Loading { progress: f32 },
    Ready { model_id: String, ram_mb: u32 },
    Error { model_id: String, reason: String },
}

/// AI Invocation request
pub struct AiRequest {
    pub model_id: String,           // "gemma4", "enterprise-custom"
    pub sanitized_prompt: Vec<u8>,  // ĐÃ QUA PII redaction
    pub max_tokens: u32,
    pub temperature: f32,
    pub task_type: AiTaskType,      // Summarize, Draft, Extract, Classify
}

/// AI Response
pub struct AiResponse {
    pub text: String,
    pub tokens_used: u32,
    pub model_id: String,
    pub latency_ms: u64,
}

/// AI Error
pub enum AiError {
    ModelNotLoaded(String),
    ModelIntegrityFailed { expected: String, actual: String },
    SanitizationFailed(String),
    EgressBlocked(String),  // PII detected in output
    Timeout,
    OutOfMemory,
}

/// Host ABI functions (exposed to WASM .tapps)
#[host_abi]
pub trait AiHost {
    /// Invoke an AI model with a sanitized prompt
    fn host_ai_invoke(&self, request: AiRequest) -> Result<AiResponse, AiError>;
    
    /// Check model status
    fn host_ai_status(&self, model_id: &str) -> ModelStatus;
    
    /// Register a custom model (enterprise)
    fn host_ai_register(&self, config: ModelConfig) -> Result<(), AiError>;
}
```

## Independent Delivery Path

```
MESSAGING CORE (Phase 1)          AI WORKSTREAM (Phase 2D)
─────────────────────────         ─────────────────────────
Ship: Month 4                     Ship: Month 14
Fail: Không ảnh hưởng AI          Fail: Không ảnh hưởng chat
Update: 2 tuần/cycle              Update: 1 tuần/cycle (model updates)
Test: 100% unit test coverage     Test: Model eval + PII accuracy
Team: Rust + Flutter engineers    Team: ML engineer + Rust engineer
```

## Lợi ích của decoupling

| Lợi ích | Giải thích |
|---------|-----------|
| **Ship độc lập** | Messaging core có thể ship Phase 1 mà không cần AI sẵn sàng |
| **Fail riêng** | Gemma 4 load fail → chat vẫn hoạt động bình thường |
| **Update riêng** | Cập nhật model (Gemma 4.1 → 4.2) không cần update cả app |
| **Team riêng** | ML engineer không cần hiểu MLS E2EE. Rust engineer không cần hiểu ONNX |
| **Test riêng** | AI quality (BLEU score, hallucination rate) test khác với messaging reliability test |
| **Pricing riêng** | Có thể bán AI module như add-on, không bắt buộc trong base license |

## AI Feature Toggle

```yaml
# Enterprise license có thể enable/disable AI
license:
  tier: enterprise
  features:
    ai_module: true          # ← Có thể tắt riêng
    gemma4_default: true
    custom_model_upload: false  # Chỉ Phase 3
    ai_api_bring_your_own: false
```

Nếu `ai_module: false` → Host ABI vẫn có trong binary nhưng trả về `AiError::ModelNotLoaded` cho mọi request. Messaging core không bị ảnh hưởng.

## 🧠 Design Decision

**Tại sao AI là add-on chứ không phải core?** → Không phải mọi doanh nghiệp cần AI trong messaging. Một công ty manufacturing dùng TeraChat cho công nhân trên sàn factory không cần AI summarization. Buộc họ trả tiền cho AI features họ không dùng → giảm conversion rate. AI add-on cho phép: base license rẻ hơn, upsell AI cho khách cần, AI team có P&L riêng.
