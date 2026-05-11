---
type: synthesis
created: 2026-05-11
tags: [terachat, metrics, kpi, phase, quantitative, market-validation]
sources: [phase-framework, improvement-plan-2026-05-11, narrowed-phase-1-mvp]
status: resolved
resolves: "Điểm yếu #10 — Thiếu metric định lượng + #14 — Chưa xác thực thị trường"
---

# Quantitative Phase Metrics

Mỗi phase có metric định lượng CỨNG. Không metric = không qua gate.

---

## Phase 1 MVP — "Signed Pilot"

**Timeline:** 4 tháng (sau 4-6 tuần prototype)

### Gate Metrics (phải đạt TẤT CẢ)

| # | Metric | Target | Cách đo |
|---|--------|--------|---------|
| M1.1 | **Signed Pilots** | ≥ 3 tổ chức ký hợp đồng pilot | Signed agreement |
| M1.2 | **IT Admin Deploy Time** | ≤ 30 phút (không cần hỗ trợ) | Timed deployment test |
| M1.3 | **Daily Active Users (per pilot)** | ≥ 20 users dùng liên tục 14 ngày | Analytics dashboard |
| M1.4 | **Message Delivery Reliability** | 100% messages delivered, 0 loss | Automated test + pilot log |
| M1.5 | **Uptime** | ≥ 99.5% trong 30 ngày pilot | Health check endpoint |
| M1.6 | **Pilot → Paid Conversion** | ≥ 1 pilot ký Letter of Intent | Signed LOI |
| M1.7 | **0 Data Loss Incidents** | 0 incidents trong toàn bộ pilot period | Incident log |

### Gate Decision

```
TẤT CẢ metrics đạt → GO Phase 2
≥ 5/7 metrics đạt → Conditional GO (fix remaining trong 30 ngày)
< 5/7 metrics → NO GO → pivot hoặc tiếp tục Phase 1
```

---

## Phase 2 — "Renew and Upsell"

**Timeline:** 8 tháng (Month 5-12)

### Gate Metrics

| # | Metric | Target | Cách đo |
|---|--------|--------|---------|
| M2.1 | **Paying Customers** | ≥ 10 tổ chức trả tiền | Stripe/Invoice records |
| M2.2 | **90-Day Retention** | ≥ 80% users vẫn active sau 90 ngày | Analytics |
| M2.3 | **Net Promoter Score** | ≥ 40 (IT Admin survey) | NPS survey |
| M2.4 | **Upsell Rate** | ≥ 2 khách nâng cấp từ Team → Enterprise | Contract records |
| M2.5 | **Monthly Recurring Revenue** | ≥ $15K MRR | Stripe dashboard |
| M2.6 | **Customer Acquisition Cost** | < $5,000 per customer | Sales spend / new customers |
| M2.7 | **Time-to-Deploy (New Customer)** | ≤ 30 minutes | Timed deployment |
| M2.8 | **Survival Mesh Uptime** | ≥ 95% message delivery during 4h internet outage | Chaos test SC-01 |
| M2.9 | **.tapp Marketplace** | ≥ 5 vetted .tapps available, ≥ 2 with active usage | Marketplace analytics |
| M2.10 | **Gemma 4 AI Usage** | ≥ 30% users dùng AI feature ít nhất 1 lần/tuần | Feature analytics |

### Gate Decision

```
TẤT CẢ metrics ≥ target → GO Phase 3
≥ 7/10 metrics đạt → Conditional GO
< 7/10 metrics → pivot hoặc delay Phase 3
```

---

## Phase 3 — "Moat and Ecosystem"

**Timeline:** 12 tháng (Month 13-24)

### Gate Metrics

| # | Metric | Target | Cách đo |
|---|--------|--------|---------|
| M3.1 | **Enterprise Customers** | ≥ 50 tổ chức trả tiền | Stripe/Invoice records |
| M3.2 | **Annual Recurring Revenue** | ≥ $1M ARR | Financial records |
| M3.3 | **Third-Party .tapp Publishers** | ≥ 10 publishers, ≥ 30 .tapps live | Marketplace records |
| M3.4 | **Gov/Military Contract** | ≥ 1 signed contract | Signed contract |
| M3.5 | **ISO 27001 Certification** | Certified | Audit report |
| M3.6 | **Open AI Framework Models** | ≥ 5 registered enterprise models | Model registry |
| M3.7 | **Customer Churn Rate** | < 5% annual | Contract records |
| M3.8 | **Net Revenue Retention** | ≥ 110% (expansion > churn) | Financial analysis |
| M3.9 | **Mean Time To Resolve (MTTR)** | < 4 hours for critical incidents | Incident management |
| M3.10 | **Platform Uptime SLA** | ≥ 99.99% (measured monthly) | Monitoring |

---

## Market Validation Gates (MỖI PHASE)

Trước khi bắt đầu code cho phase tiếp theo, phải validate:

| Gate | Khi nào | Câu hỏi |
|------|---------|---------|
| **V-GATE-1** | Cuối Prototype | "Doanh nghiệp có thấy giá trị trong MLS E2EE chat không?" |
| **V-GATE-2** | Cuối Phase 1 | "IT admin có thực sự deploy được trong 30 phút không?" |
| **V-GATE-3** | Trước Phase 2 | "Khách hàng sẵn sàng trả bao nhiêu? Có đủ để renew không?" |
| **V-GATE-4** | Trước Phase 2A (Mesh) | "Bao nhiêu khách hàng thực sự cần offline mesh?" |
| **V-GATE-5** | Trước Phase 2D (AI) | "Khách hàng có muốn AI on-device không? Hay họ thích cloud AI?" |
| **V-GATE-6** | Trước Phase 3 | "Có đủ 50+ khách để ecosystem 2-sided marketplace hoạt động không?" |

**Nguyên tắc:** KHÔNG build nếu chưa validate nhu cầu với ít nhất 3 khách hàng hiện tại.

---

## 🧠 Design Decision

**Tại sao metric cứng thay vì "cảm thấy đúng"?** → "Sign pilot" không có nghĩa gì nếu không định nghĩa "signed". "Renew" không có nghĩa gì nếu không có retention number. Metric cứng buộc team đối mặt với sự thật thay vì hy vọng. Nếu không đạt target, đó là tín hiệu để pivot — không phải thất bại.
