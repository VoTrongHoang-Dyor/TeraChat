---
type: concept
created: 2026-05-10
tags: [terachat, enterprise, license, deployment, access-control]
sources: [tera-intro, tera-gov-spec, tera-tech-debt]
---

# Enterprise License Model

TeraChat's access control at the organizational level: no public accounts, no self-service signup. Every deployment is gated by a cryptographically enforced license.

## Payment on Web Only — Nguyên tắc Bất biến

**TeraChat không xử lý thanh toán trong app.** Toàn bộ quy trình thanh toán (mua license, gia hạn, nâng cấp tier, mua .tapp) diễn ra trên **trang chủ web TeraChat (terachat.io)**. App chỉ nhận License JWT đã được cấp và xác thực — không có payment form, không có checkout flow, không có billing page trong app.

## License-Gated Architecture

```
Organization visits terachat.io → selects tier → pays via web
         ↓
TeraChat issues License JWT (HSM FIPS 140-3 signed)
  {tenant_id, domain, max_seats, tier, valid_until, features}
         ↓
License JWT delivered to IT Admin via email / web dashboard
         ↓
IT Admin deploys TeraRelay (1 binary, 1 command)
         ↓
IT Admin distributes app via MDM or internal App Store
         ↓
App validates License JWT before functioning
         ↓
No valid license → "Contact IT Administrator" screen
```

**Gia hạn & Nâng cấp:** IT Admin thực hiện trên web dashboard tại terachat.io, không qua app. Admin Console hiển thị trạng thái license và link redirect ra web khi cần gia hạn.

## License States

| State | Visual | User Impact |
|-------|--------|-------------|
| Valid | Green badge | Full access |
| T-30 days | Amber (Admin only) | Admin Console warning |
| T-0 (expired) | Amber lock | "Contact IT Admin to renew" |
| T+90 | Red lock | "License expired — cannot connect" |
| Invalid | Full lockout | "Device not licensed" |

## Deployment Tiers

| Tier | Infrastructure | Setup Time | Target |
|------|---------------|------------|--------|
| Self-Hosted Cloud | VPS 512MB–8GB | 5–20 min | SME, startup |
| On-Premise | Internal server | 1–4 hours | Enterprise, healthcare |
| Air-Gapped | Offline hardware | Half day | Gov, defense, banking |
| Hybrid | On-prem + cloud relay | 1 day | Multi-branch corporation |

## 🧠 Design Decisions (Q&A)

- **Why license entanglement with device key?** → Prevents a valid license JWT from being copied to unauthorized devices. The device derives its encryption key from `KDF(license_jwt, device_identity_key)`. Wrong device = wrong key = database is unreadable. Trade-off: device migration requires explicit IT Admin re-enrollment.
- **Why tier-based feature degradation instead of hard blocks?** → Some constraints are hardware (iOS W^X forces wasm3 interpreter). Others are policy (Huawei HMS can't support Gov tier). Degradation keeps the app functional while disclosing limitations. Trade-off: complex platform capability matrix to maintain.
- **Why payment ONLY on website, never in app?** → TeraChat là enterprise platform, không phải consumer SaaS. Quy trình mua license yêu cầu contract, approval, và procurement cycle của doanh nghiệp — không thể tự động hóa qua in-app purchase. Tách payment khỏi app giữ app đơn giản (chỉ validate license) và tập trung toàn bộ billing/compliance lên web dashboard nơi IT Admin có thể quản lý invoice, payment method, và upgrade path. App không bao giờ chạm vào payment processing.
