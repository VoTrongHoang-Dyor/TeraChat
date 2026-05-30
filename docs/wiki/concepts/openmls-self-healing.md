---
type: concept
created: 2026-05-15
updated: 2026-05-30
tags: [crypto, mls, openmls, ai, debug, shadow-graph, human-in-loop]
---

# openmls Self-Healing — Shadow Graph AI Conflict Resolution

> **Cập nhật 2026-05-30 (M-6 Fix):** Loại bỏ hoàn toàn cơ chế "AI auto-apply với confidence > 0.9". AI không bao giờ được ghi trực tiếp vào Root DAG / event_log.db. Thay bằng **Shadow Graph + Human-in-Loop**.

Khi MLS error xảy ra, local AI agent phân tích và tạo Shadow Branch đề xuất — con người kiểm duyệt và approve trước khi commit. Không bao giờ tự động sửa code hay đẩy thay đổi vào group state.

## Problem

MLS protocol errors are hard to debug:
- Group state inconsistency across members
- Epoch mismatch after concurrent operations
- Pending proposal conflicts
- openmls library bugs or version-specific issues

Traditional debugging requires deep MLS expertise. With solo dev, there's no cryptographer on call.

## Solution: Shadow Graph + Human-in-Loop

> **Tại sao không dùng Auto-apply?** MLS protocol errors phức tạp và cần tuyệt đối nhất quán toán học. Một fix sai tự động có thể phá vỡ group state vĩnh viễn, làm mất khả năng giải mã message của toàn bộ workspace. AI hallucination có thể tạo ra code pass `cargo check` nhưng sai về toán học mật mã. Shadow Graph đảm bảo con người là final authority.

```
┌─────────────────────────────────────────────────┐
│              MlsError detected                    │
│                     ↓                             │
│  ErrorContext collected (sanitized):              │
│  - Stack trace (no key material)                 │
│  - openmls version                               │
│  - Group state (member count, epoch, etc.)       │
│  - Recent operations log (types only, no content)│
│                     ↓                             │
│  DebugAgent (local Qwen2.5-7B trên Mac mini)     │
│  Phân tích và tạo Shadow Branch (đề xuất)        │
│                     ↓                             │
│  [⚠️ Human Review Required]                       │
│  UI hiển thị: "AI gợi ý fix này. Bạn có muốn     │
│  áp dụng không? [Chấp nhận] [Xem chi tiết] [Từ chối]"│
│                     ↓                             │
│  Nếu "Chấp nhận": Rust Core ký số (Ed25519)     │
│  và commit Shadow Branch thành Root State         │
│                                                   │
│  Nếu AI crash / hallucinate:                      │
│  Fallback → hiển thị manual merge conflict UI     │
└─────────────────────────────────────────────────┘
```

## ErrorContext — Never Leaks Key Material

```rust
/// Safe error context — stripped of ALL sensitive data
/// Only structural info, no payload content
#[derive(Debug, Serialize)]
pub struct MlsErrorContext {
    error_type: MlsErrorType,
    openmls_version: &'static str,

    // Structural info — NO content
    group_state: GroupStateInfo,

    // Operation types — NO message content
    recent_ops: Vec<MlsOperationType>,

    // Sanitized stack trace
    stack_trace: SanitizedStackTrace,

    // Monotonic timestamp
    occurred_at: u64,
}

#[derive(Debug, Serialize)]
pub struct GroupStateInfo {
    member_count: usize,
    epoch: u64,
    pending_proposals: usize,
    // NEVER: member identities, keys, message content
}

impl MlsErrorContext {
    /// Safe serialization — verifies no sensitive data
    pub fn to_debug_prompt(&self) -> String {
        let json = serde_json::to_string_pretty(self).unwrap();

        // Sanity check: no long hex strings (potential keys)
        assert!(!json.contains_hex_sequence(32),
            "ErrorContext contains potential key material");

        format!(
            "Diagnose this openmls error in TeraChat.\n\
             openmls version: {}\n\
             Error: {:?}\n\
             Context: {}\n\n\
             Diagnose and suggest a fix.\n\
             Format: {{\"diagnosis\": \"...\", \"fix\": \"...\", \
             \"confidence\": 0.0-1.0, \"safe_to_auto_apply\": true/false}}",
            self.openmls_version, self.error_type, json
        )
    }
}
```

## DebugAgent — Shadow Graph Model

```rust
pub struct MlsDebugAgent {
    inference: Arc<dyn InferenceGateway>,
    knowledge_base: MlsKnowledgeBase,
    outcome_store: DebugOutcomeStore,
}

impl MlsDebugAgent {
    pub async fn diagnose(&self, ctx: MlsErrorContext) -> ShadowBranchProposal {
        // 1. Check knowledge base first (fast path)
        if let Some(known) = self.knowledge_base.lookup(&ctx) {
            return ShadowBranchProposal::KnownFix(known);
        }

        // 2. Query local LLM — chỉ đọc Root state, không ghi
        let response = self.inference.complete(InferenceRequest {
            prompt: ctx.to_debug_prompt(),
            max_output_tokens: 512,
            privacy_level: Privacy::LocalOnly,  // NEVER send externally
            latency_sla: Duration::from_secs(30),
        }).await;

        // 3. Tạo Shadow Branch (proposal chộ) — chưa commit
        let raw: RawDiagnosis = serde_json::from_str(&response.text)
            .unwrap_or(RawDiagnosis::unknown());

        // QUAN TRỌNG: Không bao giờ tự động apply
        // Luôn trả về proposal để human review
        ShadowBranchProposal {
            diagnosis: raw.diagnosis,
            suggested_fix: raw.fix,
            confidence: raw.confidence,
            // UI sẽ hiển thị proposal này cho user
            // User click "Chấp nhận" → Rust Core ký và commit
            requires_human_approval: true, // luôn luôn
        }
    }

    /// Chỉ được gọi khi USER click "Chấp nhận" trên UI
    pub async fn commit_proposal(
        &self,
        proposal: ShadowBranchProposal,
        user_signature: Ed25519Sig, // bằng chứng human approval
    ) -> Result<(), MlsError> {
        // Verify chữ ký trước khi commit bất kỳ thứ gì
        verify_user_signature(&proposal, &user_signature)?;
        // Commit Shadow Branch thành Root State
        self.apply_fix_to_root_state(proposal.suggested_fix).await
    }
}
```


## Fine-Tuning Loop

Collected outcomes improve future diagnoses:

```rust
impl MlsDebugAgent {
    pub async fn record_outcome(
        &self, ctx: &MlsErrorContext, diagnosis: &DebugDiagnosis,
        outcome: FixOutcome,
    ) {
        self.outcome_store.append(TrainingEntry {
            error_context: ctx.clone(),
            suggested_fix: diagnosis.clone(),
            outcome,
            timestamp: monotonic_now(),
        }).await;

        // Trigger fine-tuning after collecting 50+ entries
        if self.outcome_store.pending_count() > 50 {
            self.trigger_local_finetune().await;
        }
    }

    async fn trigger_local_finetune(&self) {
        // Only run when Mac mini is idle (night, low thermal)
        if !self.inference.health().is_idle() { return; }

        // LoRA fine-tuning with mlx-lm
        tokio::process::Command::new("mlx_lm.lora")
            .args(["--model", "qwen2.5-7b", "--train",
                   "--iters", "100",
                   "--adapter-path", "./terachat-mls-debug-adapter"])
            .spawn()
            .ok();
    }
}
```

## Safety Guarantees

1. **ErrorContext never contains key material** — structural data only, verified by hex sequence check
2. **Privacy::LocalOnly** — diagnosis prompt never leaves the device
3. **Human approval LUÔN BẮT BUỘC** — `requires_human_approval: true` hardcoded, không có exception
4. **Ed25519 commit signature** — bằng chứng cryptographic rằng human đã review và approve
5. **Fallback về manual** — nếu AI crash hoặc hallucinate, UI hiển thị manual merge conflict UI
6. **Monotonic timestamps** — no wall-clock correlation attacks

> **⚠️ Ambitious cho solo dev:** MLS Self-Healing là tính năng Phase 3+. Implement nó đòi hỏi deep MLS expertise để định nghĩa đúng "fix nào là safe". Shadow Graph model giảm rủi ro xuống mức chấp nhận được bằng cách đưa human vào loop.

## Related Pages

- [[ADR-007 Shadow Graph AI Resolution]] — ADR đầy đủ về Shadow Graph pattern
- [[Terachat Architecture Overview]] — Where MLS fits in the stack
- [[Secure Enclave & AI Security]] — AI security model (Qwen2.5, không phải Gemma 4)
- [[AI Inference Offloading]] — The InferenceGateway used by DebugAgent
