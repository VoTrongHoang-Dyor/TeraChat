---
type: concept
created: 2026-05-12
updated: 2026-05-12
tags: [adr, ai-gateway, byom, security, phase-1, phase-2d]
sources: [tera-enclave-spec, tera-client-spec, tera-note]
---

# ADR-006: AI Gateway Architecture — Loại bỏ Local Proxy

## Status

**ACCEPTED** — 2026-05-12

## Context

Môi trường dev hiện tại sử dụng pattern:

```bash
ANTHROPIC_BASE_URL="http://localhost:8082" claude
```

Tức là mọi AI request đi qua một **HTTP proxy không mã hóa trên loopback** (`127.0.0.1:8082`). Pattern này tạo ra 5 rủi ro không chấp nhận được trong production:

1. **Plaintext trên loopback** — Bất kỳ process nào cùng máy đều có thể intercept. Vi phạm invariant *"No plaintext outside org boundary"*.
2. **PII Redaction Gate bypass** — HTTP proxy không enforce ONNX Micro-NER trước khi request thoát ra ngoài.
3. **Không có audit trail** — Request không được ghi vào immutable Ed25519 audit log.
4. **Không scale** — Single proxy process không phù hợp với Mac mini cluster.
5. **Single point of failure** — Proxy chết = toàn bộ AI feature chết.

Bốn hướng tiếp cận được đánh giá:

| Hướng | Mô tả | Verdict |
|-------|--------|---------|
| **H1: Native Rust SDK** | AI call trực tiếp trong `tc-enclave`, `reqwest` TLS 1.3 | ⭐⭐⭐⭐⭐ |
| **H2: Unix Domain Socket** | UDS thay TCP loopback, OS-level isolation | ⭐⭐⭐⭐ |
| **H3: TeraRelay Extension** | Mở rộng relay binary thêm AI Gateway route | ⭐⭐⭐⭐⭐ |
| **H4: Cloudflare AI Gateway** | External managed gateway | ❌ REJECTED |

## Decision

**Hybrid H3 → H1 theo phase:**

### Phase 1 MVP: TeraRelay Extension (H3)

Mở rộng `TeraRelay` binary — thêm `/ai/v1/` route tích hợp vào existing auth/relay infrastructure:

```
Client App (Flutter/Tauri)
    │
    ▼ gRPC (existing encrypted IPC channel)
TeraRelay (single binary)
    ├── [existing] Message routing
    ├── [existing] License JWT validation
    └── [NEW] AI Gateway (/ai/v1/)
            ├── Auth: License JWT → per-tenant config
            ├── Rate limit: per seat
            ├── PII Redaction: ONNX Micro-NER (MANDATORY)
            └── ──TLS 1.3──► BYOM Endpoint (configurable)
```

**Rationale:**
- Giữ nguyên "1 binary, 1 command" deployment → IT Admin ≤ 30 phút không bị ảnh hưởng
- Reuse auth stack: License JWT validation đã có sẵn trong relay middleware
- PII Gate là **bắt buộc** tại choke point duy nhất
- Admin Console config AI endpoint per tenant (BYOM)

### Phase 2D BYOM AI: Native Rust SDK (H1)

Khi AI workstream trở thành independent feature (Phase 2D), migrate sang `tc-enclave` native SDK:

```rust
// source/core/tc-enclave/src/ai_gateway.rs
pub struct AiGateway {
    pii_gate: OnnxPiiRedactor,   // MANDATORY — PII stripped trước request
    endpoint: Url,               // configurable: Ollama / vLLM / internal
    client: reqwest::Client,     // TLS 1.3, no proxy, no intermediate hop
    audit: AuditTrailSigner,     // Ed25519 sign mọi AI request
}

impl AiGateway {
    pub async fn complete(&self, raw_prompt: &str) -> Result<AiResponse> {
        // WHY: PII Gate mandatory — cannot bypass from caller side
        let redacted = self.pii_gate.redact(raw_prompt)?;
        let response = self.client.post(&self.endpoint).json(&redacted).send().await?;
        self.audit.sign_entry(AuditEvent::AiRequest { redacted_prompt_hash })?;
        Ok(response)
    }
}
```

## Consequences

### Positive
- ✅ PII Redaction Gate không thể bypass — bắt buộc ở transport layer
- ✅ Không có intermediate plaintext hop trên mạng
- ✅ Mọi AI request được ghi vào Ed25519 audit trail
- ✅ BYOM endpoint configurable per tenant qua Admin Console
- ✅ Offline-first: khi AI endpoint không khả dụng, messaging core không bị ảnh hưởng
- ✅ Phase 1 deployment "1 binary" giữ nguyên

### Negative
- ❌ TeraRelay binary size tăng nhẹ (Phase 1)
- ❌ AI failure phải được isolated: `AiGateway` error không cascade vào message routing
- ❌ ONNX inference trong relay process → cần memory budget cẩn thận

### Rejected Approaches

**H4 (Cloudflare AI Gateway):** Permanently rejected. Vi phạm 3 invariants:
- Plaintext request metadata rời khỏi org boundary → vi phạm Zero-Knowledge
- Không hoạt động offline → vi phạm Offline-First Survival
- Không compatible với Air-gapped Gov tier → vi phạm deployment model

**Local HTTP Proxy (127.0.0.1):** Permanently rejected trong production. Chỉ acceptable trong dev environment với explicit env var opt-in.

## Implementation Gate

Trước khi implement, cần lock 5 câu hỏi:

1. **AI endpoint Phase 1:** Ollama local trên Mac mini, hay cho phép external BYOM ngay từ Phase 1?
2. **ONNX PII Gate location:** Chạy trong relay process (H3) hay trong client trước khi gửi IPC?
3. **Rate limiting unit:** Per user, per seat, hay per tenant?
4. **Offline AI behavior:** Khi AI endpoint không khả dụng → fail fast với error, hay queue + retry?
5. **Audit scope:** AI request metadata có nằm trong immutable Ed25519 audit trail không?

## Related

- [[Secure Enclave & AI Security]] — AI security model, PII redaction spec
- [[Open AI Framework]] — BYOM architecture, model registration ABI
- [[Enterprise License Model]] — License JWT → per-tenant config mapping
- [[Terachat Architecture Overview]] — TeraRelay single binary model
- [[ADR-003]] — gRPC over FFI for IPC (existing IPC channel reused by H3)
