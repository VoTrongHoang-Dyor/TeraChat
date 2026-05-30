---
type: concept
created: 2026-05-30
tags: [adr, shadow-graph, ai, conflict-resolution, human-in-loop, crdt, event-log]
sources: [Senior-Architect-Perspective-v2, tera-sync-spec, tera-enclave-spec]
---

# ADR-007: Shadow Graph AI Conflict Resolution

## Status

**ACCEPTED** — 2026-05-30

## Context

Khi đồng bộ ngoại tuyến, xung đột dữ liệu là không tránh khỏi: hai thiết bị cùng chỉnh sửa một Collaborative Note, một Thread Title, hoặc một app state record trong khi mất kết nối. Local AI có thể đề xuất cách giải quyết, nhưng có rủi ro nghiêm trọng nếu AI được trao quyền ghi trực tiếp vào Root DAG / event_log.db:

1. **AI hallucination** có thể tạo ra fix sai về mặt toán học nhưng pass `cargo check`
2. **Một fix sai** trên CRDT / MLS group state có thể phá vỡ vĩnh viễn khả năng giải mã của workspace
3. **Không có audit trail** cho quyết định sai của AI

## Decision

**AI KHÔNG BAO GIỜ được ghi trực tiếp vào Root State (event_log.db hay Root DAG). AI chỉ được phép tạo Shadow Branch chứa đề xuất. Human approval là bắt buộc trước mọi commit.**

### Shadow Graph Model

```
Root DAG / event_log.db (Read-Only cho AI)
        │
        ├── Branch A (Device 1 changes — offline)
        └── Branch B (Device 2 changes — offline)
                │
                ↓ [Rust Core phát hiện conflict]
                │
        AI Agent (Qwen2.5-7B trên Mac mini)
        ├── ĐỌC Branch A và Branch B
        └── TẠO Shadow Branch (đề xuất merge)
                │
                ↓ [UI hiển thị cho user]
                │
        Human Review:
        ├── "Chấp nhận" → Ed25519 signature → Commit Shadow thành Root
        ├── "Chỉnh sửa" → User edit Shadow → Sign → Commit
        └── "Từ chối" → Manual merge conflict UI (như Git merge)
```

### Phạm vi áp dụng

Shadow Graph áp dụng cho:
- **Collaborative Notes** (CRDT namespace)
- **Thread Titles** (CRDT namespace)
- **App State conflicts** (cold_state.db relational conflicts)
- **MLS Error recovery suggestions** (xem [[openmls Self-Healing]])

Shadow Graph **KHÔNG** áp dụng cho:
- **Chat messages** (event_log.db) — chat là append-only, không có conflict
- **Key material operations** — hoàn toàn cấm AI
- **tc-crypto codebase** — CODEOWNERS lock, AI không có quyền

### Interface với tc-crdt-sync

```rust
pub trait ShadowGraphEngine: Send + Sync {
    /// AI đọc Branch A và B, trả về Shadow Branch (chưa commit)
    async fn propose_resolution(
        &self,
        branch_a: &EventLogBranch,
        branch_b: &EventLogBranch,
    ) -> Result<ShadowBranch, AiError>;
}

pub struct ShadowBranch {
    pub id: ShadowBranchId,
    pub proposed_events: Vec<Event>,
    pub ai_confidence: f32,
    pub ai_reasoning: String,
    /// Luôn = true — không có exception
    pub requires_human_approval: bool,
}

/// Chỉ được gọi sau khi user click "Chấp nhận" và cung cấp signature
pub async fn commit_shadow(
    shadow: ShadowBranch,
    user_approval: UserApprovalProof, // Ed25519 signature từ DeviceIdentityKey
) -> Result<CommittedEvent, CommitError>;
```

### Commit Protocol

1. User click "Chấp nhận" trên UI
2. Rust Core prompt user để Sign bằng DeviceIdentityKey (Secure Enclave/StrongBox)
3. `UserApprovalProof` = Ed25519 signature trên `Shadow Branch ID + timestamp + user_id`
4. Rust Core verify signature → commit Shadow thành Root State
5. Append immutable audit event: `{action: "ai_conflict_resolved", shadow_id, user_id, timestamp, sig}`

## Fallback

Nếu AI không khả dụng (crash, OOM, timeout):
- Rust Core tự động fallback về **manual merge conflict UI**
- UI hiển thị diff giữa Branch A và Branch B
- User chọn thủ công từng conflict (như Git merge conflict resolution)
- Commit với Ed25519 signature như bình thường

## Consequences

### Positive
- ✅ AI không thể phá hủy data ngay cả khi hallucinate
- ✅ Mọi AI-assisted merge đều có cryptographic audit trail
- ✅ Fallback về manual luôn khả dụng
- ✅ User có full control — không bị AI quyết định thay

### Negative
- ❌ Thêm một bước UI approval — chậm hơn so với auto-merge
- ❌ Cần UI component cho Shadow Branch review
- ❌ AI inference tốn RAM khi resolve conflicts (Qwen2.5-7B cần 8-16GB)

## Related

- [[Dual-Sync Pattern]] — Architecture cho event_log.db (Event Log + CRDT)
- [[ADR-002 Dual-Plane Sync]] — Lý do phân tách sync planes
- [[ADR-008 Delegated Proposer TreeKEM]] — Delegated computation pattern tương tự
- [[openmls Self-Healing]] — Shadow Graph áp dụng cho MLS error recovery
- [[Invariants]] — AI không được phép ghi vào tc-crypto
