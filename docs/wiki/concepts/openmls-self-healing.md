---
type: concept
created: 2026-05-15
updated: 2026-05-15
tags: [crypto, mls, openmls, ai, debug, self-healing]
---

# openmls Self-Healing with AI Debug Loop

When MLS errors occur, a local AI agent analyzes and proposes fixes — without leaking key material. Private inference only; no data leaves the device.

## Problem

MLS protocol errors are hard to debug:
- Group state inconsistency across members
- Epoch mismatch after concurrent operations
- Pending proposal conflicts
- openmls library bugs or version-specific issues

Traditional debugging requires deep MLS expertise. With solo dev, there's no cryptographer on call.

## Solution: Local AI Debug Loop

```
┌─────────────────────────────────────────────────┐
│              MlsError detected                    │
│                     ↓                             │
│  ErrorContext collected (sanitized):              │
│  - Stack trace (no key material)                 │
│  - openmls version                               │
│  - Group state (member count, epoch, etc.)       │
│  - Recent operations log                         │
│                     ↓                             │
│  DebugAgent (local LLM on Mac mini)              │
│                     ↓                             │
│  Diagnosis + suggested fix                       │
│                     ↓                             │
│  Auto-apply if confidence > 0.9 AND fix is safe  │
│  Manual review if confidence < 0.9               │
│                     ↓                             │
│  Outcome → local training dataset                │
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

## DebugAgent

```rust
pub struct MlsDebugAgent {
    inference: Arc<dyn InferenceGateway>,
    knowledge_base: MlsKnowledgeBase,
    outcome_store: DebugOutcomeStore,
}

impl MlsDebugAgent {
    pub async fn diagnose(&self, ctx: MlsErrorContext) -> DebugDiagnosis {
        // 1. Check knowledge base first (fast path)
        if let Some(known) = self.knowledge_base.lookup(&ctx) {
            return DebugDiagnosis::KnownFix(known);
        }

        // 2. Query local LLM
        let response = self.inference.complete(InferenceRequest {
            prompt: ctx.to_debug_prompt(),
            max_output_tokens: 512,
            privacy_level: Privacy::LocalOnly,  // NEVER send externally
            latency_sla: Duration::from_secs(30),
        }).await;

        // 3. Parse + safety gate
        let diagnosis: RawDiagnosis = serde_json::from_str(&response.text)
            .unwrap_or(RawDiagnosis::unknown());

        let action = if diagnosis.safe_to_auto_apply && diagnosis.confidence > 0.9 {
            DiagnosisAction::AutoApply(diagnosis.fix.clone())
        } else {
            DiagnosisAction::ManualReview {
                suggestion: diagnosis.fix.clone(),
                reason: "Low confidence or unsafe operation",
            }
        };

        DebugDiagnosis { diagnosis, action }
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
3. **Auto-apply gate** — confidence > 0.9 AND safe_to_auto_apply both required
4. **Training data stays local** — fine-tuning happens on-device
5. **Monotonic timestamps** — no wall-clock correlation attacks

## Related Pages

- [[Terachat Architecture Overview]] — Where MLS fits in the stack
- [[AI Inference Offloading]] — The InferenceGateway used by DebugAgent
- [[Secure Enclave & AI Security]] — PII redaction principles
