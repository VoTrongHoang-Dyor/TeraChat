---
type: synthesis
created: 2026-05-11
tags: [terachat, improvement, restructuring, mvp, gap-resolution, platform-phasing]
sources: [vision-redefinition-2026-05-11, wiki-health-check-2026-05-10]
based_on: "Đánh giá TeraChat thang 100 — 2026-05-11"
---

# Improvement Plan — TeraChat Restructuring

Điểm hiện tại: **65/100**. Mục tiêu: **85/100** sau khi hoàn thành tất cả các cải tiến dưới đây.

## Navigation Hub

Mỗi điểm yếu trong đánh giá đều có một tài liệu giải quyết riêng. Bắt đầu từ đây và theo dõi các liên kết:

```
IMPROVEMENT HUB (file này)
│
├─ KHẨN CẤP (làm trước khi viết code)
│  ├─ [[Narrowed Phase 1 MVP]] — Thu hẹp scope Phase 1 xuống MVP tuyệt đối
│  ├─ [[GAP Resolution Tracker]] — Giải quyết 10 GAPs ở mức spec
│  └─ [[Prototype First Model]] — Dựng prototype trước khi hoàn thiện specs
│
├─ QUAN TRỌNG (làm trong quá trình Phase 1)
│  ├─ [[Platform Rollout Phasing]] — macOS+iPhone trước, 9 nền tảng sau
│  ├─ [[Deployment Automation Spec]] — Spec triển khai 30-phút cho IT admin
│  ├─ [[Quantitative Phase Metrics]] — Metric định lượng cho mỗi phase
│  └─ [[AI Independent Workstream]] — Tách AI khỏi messaging core
│
├─ LIÊN TỤC (cập nhật xuyên suốt)
│  ├─ [[Design Partner Engagement Plan]] — Tìm khách hàng thiết kế trước
│  ├─ [[Security Review Requirements]] — Yêu cầu Applied Cryptographer
│  └─ [[Progressive Complexity Model]] — Mỗi phase thêm MỘT subsystem chính
│
└─ TÍCH HỢP (tổng hợp vào hệ thống hiện có)
   ├─ [[phase/README.md]] — Đã tích hợp tất cả cải tiến
   ├─ [[Phase Framework]] — Đã cập nhật metric định lượng
   └─ [[Terachat Architecture Overview]] — Đã cập nhật scope mới
```

## 14 Điểm Yếu → 14 Giải Pháp

| # | Điểm yếu | Mức độ | Giải pháp | Tài liệu |
|---|----------|--------|-----------|----------|
| 1 | Phase 1 scope quá rộng | 🔴 CRITICAL | MVP tuyệt đối: MLS + License + OIDC + 1 t-app | [[Narrowed Phase 1 MVP]] |
| 2 | 10 GAPs chưa giải quyết | 🔴 CRITICAL | Giải quyết từng GAP ở mức spec trước implementation | [[GAP Resolution Tracker]] |
| 3 | 9 nền tảng từ day 1 | 🔴 CRITICAL | macOS+iPhone trước, mở rộng dần | [[Platform Rollout Phasing]] |
| 4 | Chưa có design partner | 🔴 CRITICAL | Kế hoạch tìm kiếm và làm việc với design partner | [[Design Partner Engagement Plan]] |
| 5 | Timeline 35 ngày phi thực tế | 🔴 CRITICAL | Timeline 18-24 tháng với progressive complexity | [[Progressive Complexity Model]] |
| 6 | Chưa có prototype | 🟠 HIGH | Dựng MLS E2EE chat đơn giản trong 4-6 tuần | [[Prototype First Model]] |
| 7 | Thiếu Applied Cryptographer | 🟠 HIGH | Định nghĩa yêu cầu security review | [[Security Review Requirements]] |
| 8 | AI gắn với messaging core | 🟠 HIGH | Tách AI thành workstream độc lập | [[AI Independent Workstream]] |
| 9 | Không có deployment spec | 🟠 HIGH | Viết spec triển khai — đây là spec ĐẦU TIÊN | [[Deployment Automation Spec]] |
| 10 | Thiếu metric định lượng | 🟡 MEDIUM | Metric cụ thể cho từng phase | [[Quantitative Phase Metrics]] |
| 11 | Dual UI framework | 🟡 MEDIUM | Đánh giá Flutter Desktop unification | [[Platform Rollout Phasing]] |
| 12 | Tài liệu phân mảnh | 🟡 MEDIUM | Quy tắc đồng bộ hóa tài liệu | [[Documentation Sync Protocol]] |
| 13 | Over-engineered Phase 1 | 🟠 HIGH | Progressive complexity: mỗi phase = 1 subsystem | [[Progressive Complexity Model]] |
| 14 | Chưa xác thực thị trường | 🟠 HIGH | Market validation gate trước mỗi phase | [[Quantitative Phase Metrics]] |

## Thứ tự thực hiện

```
TUẦN 1-2: Khóa quyết định kiến trúc
├─ Giải quyết 10 GAPs ở mức spec
├─ Thu hẹp Phase 1 scope → MVP document
├─ Chốt platform phasing: macOS + iPhone trước
└─ Viết Deployment Automation Spec

TUẦN 3-6: Xây dựng prototype
├─ Rust Core: MLS E2EE group chat (không PQ, không mesh)
├─ Flutter UI: iOS + macOS (passive renderer)
├─ License JWT validation
└─ Demo được cho design partner tiềm năng

TUẦN 7-8: Tìm design partner
├─ Demo prototype cho 5-10 doanh nghiệp mục tiêu
├─ Thu thập yêu cầu thực tế
├─ Điều chỉnh scope Phase 1 dựa trên feedback
└─ Ký design partner agreement

THÁNG 3-4: Phase 1 MVP
├─ Hoàn thiện MLS E2EE messaging
├─ OIDC/SAML integration
├─ 1 reference .tapp
├─ Deployment automation (30-phút target)
└─ PILOT SIGNED ← ĐÂY LÀ CỘT MỐC

THÁNG 5-12: Phase 2 (sau pilot)
├─ PQ-KEM hybrid
├─ Survival Mesh
├─ .tapp Marketplace (5+ tapps)
├─ Gemma 4 local AI (initial)
└─ Mac mini HA cluster

THÁNG 13-24: Phase 3 (scale)
├─ Open AI Framework (full)
├─ ZK Memory Agent
├─ Gov/Military certification
└─ 50+ enterprise customers
```

## Cách sử dụng hệ thống này

1. **Bắt đầu từ đây** — Improvement Hub là cổng vào duy nhất
2. **Theo dõi liên kết** — Mỗi tài liệu liên kết đến tài liệu liên quan
3. **Cập nhật hai chiều** — Khi bạn sửa một tài liệu, kiểm tra tài liệu liên kết đến nó
4. **Đánh dấu tiến độ** — Mỗi tài liệu có trạng thái: `pending → in_progress → resolved`
5. **Log mọi thay đổi** — `docs/wiki/log.md` là nhật ký trung tâm

## Trạng thái hiện tại

| Tài liệu | Trạng thái |
|----------|-----------|
| Narrowed Phase 1 MVP | ✅ Created |
| GAP Resolution Tracker | ✅ Created |
| Prototype First Model | ✅ Created |
| Platform Rollout Phasing | ✅ Created |
| Deployment Automation Spec | ✅ Created |
| Quantitative Phase Metrics | ✅ Created |
| AI Independent Workstream | ✅ Created |
| Progressive Complexity Model | ✅ Created |
| Design Partner Engagement Plan | ⏳ Pending |
| Security Review Requirements | ⏳ Pending |
| Documentation Sync Protocol | ⏳ Pending |
