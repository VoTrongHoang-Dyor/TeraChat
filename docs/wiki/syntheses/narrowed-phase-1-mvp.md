---
type: synthesis
created: 2026-05-11
tags: [terachat, mvp, phase-1, pilot, scope-reduction]
sources: [phase-framework, terachat-architecture-overview, vision-redefinition-2026-05-11, improvement-plan-2026-05-11]
status: resolved
resolves: "Điểm yếu #1 — Phase 1 scope quá rộng"
---

# Narrowed Phase 1 MVP

**Quyết định:** Phase 1 chỉ bao gồm những gì CẦN THIẾT để ký được pilot đầu tiên. Mọi thứ khác deferred.

## Nguyên tắc thu hẹp

> "Nếu không có nó, IT admin có deploy được trong 30 phút và demo cho Board không? Nếu có → defer."

## Phase 1 MVP: Cái gì VÀO, cái gì RA

### VÀO (Must Have — 4 components)

| # | Component | Lý do | Effort |
|---|-----------|-------|--------|
| 1 | **MLS E2EE Internal Messaging** | Core value — nhắn tin nội bộ mã hóa đầu cuối | 60% effort |
| 2 | **License JWT + DeviceIdentityKey** | Revenue model — không có cái này, không có tiền | 15% effort |
| 3 | **OIDC/SAML Login** (Google Workspace + Azure AD) | IT admin requirement — không ai muốn tạo password mới | 15% effort |
| 4 | **1 Reference .tapp** (Expense Approval) | Chứng minh Work OS concept — deploy trong 10 phút | 10% effort |

### RA (Deferred — 7 components)

| # | Component | Defer đến | Lý do defer |
|---|-----------|-----------|-------------|
| 1 | **Hybrid PQ-KEM** | Phase 2 | MLS E2EE đủ mạnh cho pilot. PQ là phòng thủ tương lai |
| 2 | **Survival Mesh (BLE/Wi-Fi Direct)** | Phase 2 | Phức tạp nhất hệ thống. Không cần cho office pilot |
| 3 | **Headless Daemon (Android Foreground Svc)** | Phase 2 | Desktop + iOS đủ cho pilot đầu |
| 4 | **WASM Marketplace** | Phase 2 | 1 reference .tapp đủ. Marketplace cần 50+ khách |
| 5 | **Gemma 4 Local AI** | Phase 2 | Chưa cần AI để chứng minh messaging hoạt động |
| 6 | **Open AI Framework** | Phase 3 | Cần AI core hoạt động trước khi mở framework |
| 7 | **Mac mini HA Cluster** | Phase 2 | Single node đủ 99.9% SLA cho pilot |

### RA VĨNH VIỄN (Never)

| # | Component | Lý do |
|---|-----------|-------|
| 1 | **Customer-facing messaging** | Không thể ép khách hàng đổi nền tảng |
| 2 | **Public/anonymous access** | Enterprise-only — không consumer |

## Timeline thực tế

```
THÁNG 1: MLS E2EE Core + License
├─ Tuần 1-2: MLS group chat (không PQ, Curve25519 only)
├─ Tuần 3: License JWT validation flow
└─ Tuần 4: Integration test — gửi 1000 messages, không mất

THÁNG 2: OIDC/SAML + UI
├─ Tuần 5-6: Keycloak/Dex bridge (Google Workspace + Azure AD)
├─ Tuần 7: SAML attribute → role mapping table
└─ Tuần 8: Flutter UI — chat, channel list, E2EE indicators

THÁNG 3: Reference .tapp + Deployment
├─ Tuần 9-10: Expense Approval .tapp (WASM, đơn giản)
├─ Tuần 11: Deployment script (1 lệnh deploy, 30 phút)
└─ Tuần 12: Pilot demo package — script + tài liệu + video

CỘT MỐC: KÝ PILOT ← Đây là lúc có khách hàng thật
```

## Định nghĩa "Ký được Pilot"

Một pilot được coi là THÀNH CÔNG khi:

- [ ] 1 IT admin deploy TeraChat trong ≤ 30 phút, không cần hỗ trợ
- [ ] 20+ nhân viên dùng TeraChat liên tục trong 14 ngày
- [ ] Gửi ít nhất 500 messages, không mất tin nhắn nào
- [ ] IT admin demo được cho Board (không crash, không lỗi)
- [ ] 0 data loss incidents
- [ ] Pilot organization ký Letter of Intent để chuyển thành paid contract

## Những gì KHÔNG thay đổi

Dù MVP thu hẹp, những bất biến kiến trúc vẫn giữ:

1. Rust Core là domain owner — UI là passive renderer
2. Zero-Knowledge — server không thấy plaintext
3. Enterprise license model — không public signup
4. Code được thiết kế để mở rộng: khi thêm PQ-KEM, không cần rewrite

## 🧠 Design Decisions

- **Tại sao giữ License JWT trong MVP?** → Không có license = không có revenue = không có công ty. License là nền tảng kinh doanh, không phải feature.
- **Tại sao 1 reference .tapp thay vì marketplace?** → Cần chứng minh "Work OS" không phải là "chat app". Nhưng 1 t-app là đủ cho demo. Marketplace là bài toán con gà-quả trứng — cần users trước khi có publishers.
- **Tại sao macOS + iPhone trước?** → Cùng hệ sinh thái Apple, cùng Secure Enclave, build chain tương đồng. Một team có thể làm cả hai.
