---
type: concept
created: 2026-05-30
tags: [adr, treekem, mls, delegated-proposer, mesh-freeze, mobile, desktop]
sources: [Senior-Architect-Perspective-v2, tera-core-spec]
---

# ADR-008: Delegated Proposer cho TreeKEM

## Status

**ACCEPTED** — 2026-05-30

## Context

Với các nhóm chat 5,000+ users, việc xoay khóa MLS TreeKEM liên tục tạo ra hai vấn đề nghiêm trọng:

1. **Pin điện thoại di động kiệt sức:** Tính toán TreeKEM nặng (độ phức tạp O(log N) với 5000 users) trên CPU di động tiêu tốn pin không thể chấp nhận được trong môi trường doanh nghiệp.
2. **TEE server quá tải:** Dồn toàn bộ tính toán TreeKEM vào TEE server tạo hàng đợi tính toán và latency cao.

## Decision

**Delegated Proposer:** Mobile device ủy quyền tính toán TreeKEM cho Fat Client (Desktop/Laptop đang online trong cùng group). TEE server chỉ đóng vai trò Sequencer + Verifier.

### Protocol Chi tiết

```
Mobile Device (needs Leaf Update)
        │
        │ 1. Tìm Fat Client khả dụng trong group
        │    (Desktop/Laptop, cắm điện, online)
        │
        ▼
Fat Client (Desktop)
        │ 2. Nhận ủy quyền từ Mobile
        │ 3. Tính toán TreeKEM mới (O(log N) — CPU-intensive)
        │ 4. Tạo Proposal với Ed25519 signature của Fat Client
        │
        ▼
TEE Server (Mac Mini / Intel TDX / AMD SEV-SNP)
        │ 5. Verify signature của Fat Client
        │ 6. Sequence proposal trong Epoch timeline
        │ 7. Broadcast Commit xuống tất cả members
        │
        ▼
Tất cả Group Members nhận Commit mới
```

### Tại sao TEE chỉ Verify + Sequence (không Compute)?

TEE computation là tài nguyên đắt đỏ và bottleneck. Mỗi ms của TEE compute = latency cho tất cả members. Fat Client Desktop (với CPU đa nhân, cắm điện, RAM dồi dào) có thể tính TreeKEM nhanh hơn nhiều. TEE chỉ cần verify tính đúng đắn cryptographic của kết quả — không cần tự tính.

### Fat Client Selection

```rust
pub fn select_fat_client(group: &MlsGroup) -> Option<PeerId> {
    group.members()
        .filter(|m| m.platform == Platform::Desktop || m.platform == Platform::LaptopPluggedIn)
        .filter(|m| m.is_online() && m.uptime_seconds > 300) // stable 5+ phút
        .max_by_key(|m| m.uptime_seconds) // chọn thiết bị ổn định nhất
}
```

### Security Model

- Fat Client **không có quyền** tạo Commit mà không qua TEE verify
- TEE verify chữ ký Ed25519 của cả Mobile (người ủy quyền) và Fat Client (người tính toán)
- Nếu Fat Client tính sai (dù vô tình hay cố ý), TEE reject proposal
- Audit trail: mọi delegation đều được log với identity của cả hai bên

## Failure Modes & Recovery

### Failure 1: Fat Client offline mid-computation

**Scenario:** Fat Client được chọn làm Delegated Proposer đột ngột offline khi đang tính TreeKEM.

**Recovery:**
1. TEE phát hiện proposal timeout (mặc định: 30 giây)
2. TEE broadcast `ProposalTimeout` event xuống group
3. Mobile device chọn Fat Client khác (next candidate từ danh sách)
4. Nếu không còn Fat Client nào → Mobile tự tính (chấp nhận pin drain ngắn hạn)
5. Nếu Mobile cũng không đủ tài nguyên → Mesh-Freeze mode kích hoạt

### Failure 2: Không có Fat Client trong group

**Scenario:** Toàn bộ group chỉ có mobile devices (field agents không có laptop).

**Recovery:**
1. Mobile với battery cao nhất và CPU mạnh nhất tự tính TreeKEM
2. Nếu battery < 20% → Defer computation, dùng Static Session Keys tạm thời
3. Mesh-Freeze mode: không rotate keys cho đến khi có resource

### Failure 3: Partial Proposal (Fat Client gửi incomplete)

**Scenario:** Fat Client gửi Proposal nhưng bị ngắt kết nối trước khi complete.

**Recovery:**
- TEE reject incomplete Proposal (signature validation fail)
- TEE broadcast `ProposalRejected` event
- Mobile chọn Fat Client khác, retry từ đầu
- Strict TTL: partial proposal buffer tự xóa sau 60 giây

## Mesh-Freeze Mode (T3 BLE Emergency)

Khi T3 BLE mode active:
- OpenMLS Epoch Rotation **bị đóng băng hoàn toàn**
- Tất cả communication dùng **Static Session Keys** (keys từ epoch cuối trước khi mất kết nối)
- Không có Delegated Proposer, không có TreeKEM computation
- Security trade-off chấp nhận: trong emergency offline, priority là duy trì liên lạc
- Keys được rotate lại ngay khi quay về T1/T2 (Online)

```rust
pub enum MlsMode {
    /// Normal: TreeKEM rotation active, Delegated Proposer enabled
    Active {
        current_epoch: u64,
        pending_proposals: Vec<Proposal>,
    },
    /// T3 BLE: frozen at last known epoch, static keys only
    MeshFreeze {
        frozen_epoch: u64,
        frozen_at: MonotonicTimestamp,
        freeze_reason: FreezeReason, // T3BleActive | LowBattery | NoFatClient
    },
}
```

## Resource Budget

| Operation | Computation Location | Estimated Cost |
|-----------|---------------------|----------------|
| TreeKEM cho 100 users | Fat Client Desktop | ~50ms CPU, ~10MB RAM |
| TreeKEM cho 1000 users | Fat Client Desktop | ~200ms CPU, ~50MB RAM |
| TreeKEM cho 5000 users | Fat Client Desktop | ~800ms CPU, ~200MB RAM |
| TEE Verify | TEE Server | ~5ms (constant, không phụ thuộc group size) |
| Mobile self-compute (100 users) | Mobile CPU | ~500ms, ~50MB RAM, ~2% battery |

## Consequences

### Positive
- ✅ Mobile battery drain giảm 90% so với self-compute TreeKEM
- ✅ TEE không còn là bottleneck computation
- ✅ Scalable đến 5000+ users per group
- ✅ Fallback rõ ràng cho mọi failure mode

### Negative
- ❌ Protocol phức tạp hơn (3 bên: Mobile, Fat Client, TEE)
- ❌ Latency key rotation tăng nhẹ (delegation round-trip ~100-200ms LAN)
- ❌ Cần trust model cho Fat Client (chỉ verify computation, không trust keys)

## Related

- [[ADR-007 Shadow Graph AI Resolution]] — Pattern tương tự cho AI conflict resolution
- [[Teralink Fallback Network]] — Mesh-Freeze mode khi T3 BLE
- [[Invariants]] — I-01: Rust Core là Domain Owner cho crypto operations
- TERA-CORE spec §4.3 — MLS TreeKEM protocol details
