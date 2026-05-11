---
type: synthesis
created: 2026-05-11
tags: [terachat, prototype, mvp, progressive-complexity, architecture-validation]
sources: [improvement-plan-2026-05-11, narrowed-phase-1-mvp, terachat-architecture-overview]
status: resolved
resolves: "Điểm yếu #6 — Chưa có prototype + #13 — Over-engineered Phase 1"
---

# Prototype-First Model & Progressive Complexity

**Quyết định:** Dựng prototype MLS E2EE chat đơn giản trước khi viết thêm bất kỳ spec nào. Mỗi phase sau chỉ thêm MỘT subsystem chính.

## Tại sao Prototype-First?

```
HIỆN TẠI: Specs → Specs → Specs → (không có code)
                ↓
          23+ documents, 0 dòng code chạy được
                ↓
          Không biết architecture có hoạt động không

ĐỀ XUẤT:   Spec tối thiểu → PROTOTYPE → Học → Spec hoàn chỉnh → Code
                ↓
          Làm ra cái chạy được trong 4-6 tuần
                ↓
          Biết chính xác cái gì hoạt động, cái gì không
```

## Prototype: "TeraChat Zero"

### Scope

Một MLS E2EE chat app tối giản — **không mesh, không WASM, không AI, không PQ, không marketplace.**

| Component | Có trong prototype? | Stack |
|-----------|---------------------|-------|
| MLS E2EE group chat (Curve25519) | ✅ Có | Rust Core + ring crate |
| License JWT validation | ✅ Có | Rust Core |
| OIDC login (Google only) | ✅ Có | Keycloak bridge |
| macOS desktop app | ✅ Có | Tauri (Rust-native) |
| iPhone mobile app | ✅ Có | Flutter + uniffi-apple |
| Message gửi/nhận qua relay | ✅ Có | TeraRelay single binary |
| File transfer (E2EE) | ✅ Có | Blob CAS đơn giản |
| Hybrid PQ-KEM | ❌ Không | Để Phase 2 |
| Survival Mesh (BLE) | ❌ Không | Để Phase 2 |
| WASM .tapp runtime | ❌ Không | Để Phase 2 |
| AI (Gemma 4) | ❌ Không | Để Phase 2 |
| SCIM offboarding | ❌ Không | Để Phase 2 |
| Android/Windows/Linux | ❌ Không | Để sau macOS+iPhone |

### Timeline: 4-6 tuần

```
TUẦN 1-2: Rust Core cơ bản
├─ MLS group chat (mls-rs crate hoặc custom với ring)
├─ License JWT validation
├─ TeraRelay: single binary, SQLite WAL, auto TLS
└─ Test: 2 clients gửi/nhận 1000 messages, 0 loss

TUẦN 3-4: UI + Relay
├─ Flutter app (iPhone): login → chat list → send message
├─ Tauri app (macOS): login → chat list → send message
├─ IPC: gRPC local giữa UI và Rust Core
└─ Test: gửi tin giữa macOS và iPhone

TUẦN 5-6: Hardening
├─ OIDC via Keycloak (Google Workspace)
├─ File transfer E2EE
├─ Crash recovery: kill app mid-message → mở lại → không mất tin
└─ DEMO READY — có thể show cho design partner
```

### Tiêu chí thành công

- [ ] 2 người dùng (macOS + iPhone) gửi 1000 messages, 0 loss
- [ ] Kill app mid-message → mở lại → tin đến nơi
- [ ] License JWT hết hạn → app khóa đúng
- [ ] Deploy relay trong < 5 phút (1 lệnh)
- [ ] File 10MB gửi thành công, E2EE, không plaintext trên relay

---

## Progressive Complexity Model

**Nguyên tắc:** Mỗi phase chỉ thêm MỘT subsystem chính vào core đã hoạt động.

```
PROTOTYPE: MLS Chat cơ bản (4-6 tuần)
    │  Chạy được. Demo được. Có thể cho khách xem.
    │
    ├─ PHASE 1 MVP (+3 tháng): Thêm OIDC/SAML + 1 Reference .tapp + Deployment
    │    │  Subsystem mới: Identity integration
    │    │  Kết quả: KÝ PILOT
    │    │
    │    ├─ PHASE 2A (+2 tháng): Thêm PQ-KEM Hybrid
    │    │    │  Subsystem mới: Post-quantum cryptography
    │    │    │
    │    │    ├─ PHASE 2B (+3 tháng): Thêm Survival Mesh
    │    │    │    │  Subsystem mới: Offline P2P communication
    │    │    │    │
    │    │    │    ├─ PHASE 2C (+2 tháng): Thêm .tapp Marketplace
    │    │    │    │    │  Subsystem mới: WASM sandbox + app store
    │    │    │    │    │
    │    │    │    │    ├─ PHASE 2D (+2 tháng): Thêm Gemma 4 AI
    │    │    │    │    │    │  Subsystem mới: Local AI on-device
    │    │    │    │    │    │
    │    │    │    │    │    ├─ PHASE 3A (+3 tháng): Thêm Open AI Framework
    │    │    │    │    │    │    │  Subsystem mới: Multi-model AI
    │    │    │    │    │    │    │
    │    │    │    │    │    │    └─ PHASE 3B (+6 tháng): Gov/Military Certification
    │    │    │    │    │    │         Subsystem mới: Air-gapped compliance
```

**Mỗi mũi tên = 1 subsystem. Không thêm 2 subsystem cùng lúc.**

---

## So sánh: Trước vs Sau

| | Trước (35-day plan) | Sau (Progressive) |
|---|---|---|
| **Cách tiếp cận** | 7 phase, 5 ngày/phase, tất cả subsystem song song | 1 phase = 1 subsystem chính, tuần tự |
| **Thời gian** | 35 ngày (phi thực tế) | 18-24 tháng (thực tế) |
| **Rủi ro** | Tất cả cùng fail một lúc | Mỗi subsystem fail riêng, không lan |
| **Demo được** | Sau 35 ngày (nếu may mắn) | Sau 4-6 tuần (prototype) |
| **Học được** | Sau khi build xong hết | Học sau mỗi subsystem |
| **Pivot được** | Khó — đã code quá nhiều thứ | Dễ — mới code 1 subsystem |

## 🧠 Design Decision

**Tại sao không build tất cả song song?** → 35-day plan giả định mọi thứ hoạt động ngay lần đầu. Với 16 tech debt CRITICAL + 10 GAPs chưa giải quyết + chưa có prototype, xác suất mọi thứ hoạt động cùng lúc gần như bằng 0. Progressive complexity cho phép: phát hiện lỗi sớm, sửa trong phạm vi nhỏ, demo liên tục, pivot nếu cần.
