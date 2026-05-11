# TeraChat — Phase Execution Map (V3)

```yaml
id: "TERA-PHASE-MAP"
version: "3.0.0"
date: "2026-05-11"
principle: "Prototype → MVP → Progressive Complexity (1 subsystem/phase)"
timeline: "18-24 months (not 35 days)"
```

## Tổng quan Timeline

```
PROTOTYPE (4-6 tuần)
  │  MLS Chat cơ bản. Demo được.
  │
  ├─ PHASE 1 MVP (Tháng 1-4): MLS + License + OIDC + 1 t-app
  │    │  Subsystem: Identity Integration
  │    │  Gate: 3 signed pilots, 0 data loss
  │    │
  │    ├─ PHASE 2A (Tháng 5-6): PQ-KEM Hybrid
  │    │    │  Subsystem: Post-Quantum Crypto
  │    │    │
  │    │    ├─ PHASE 2B (Tháng 7-9): Survival Mesh
  │    │    │    │  Subsystem: Offline P2P
  │    │    │    │
  │    │    │    ├─ PHASE 2C (Tháng 10-11): .tapp Marketplace
  │    │    │    │    │  Subsystem: WASM + App Store
  │    │    │    │    │
  │    │    │    │    ├─ PHASE 2D (Tháng 12-14): Gemma 4 AI
  │    │    │    │    │    │  Subsystem: Local AI
  │    │    │    │    │    │
  │    │    │    │    │    ├─ PHASE 3A (Tháng 15-18): Open AI + Gov
  │    │    │    │    │    │    │  Subsystem: Multi-Model AI + Compliance
  │    │    │    │    │    │    │
  │    │    │    │    │    │    └─ PHASE 3B (Tháng 19-24): Scale
  │    │    │    │    │    │         Subsystem: Enterprise Scale
  │    │    │    │    │    │
  Mỗi mũi tên = 1 subsystem chính. Không thêm 2 subsystem cùng lúc.
```

## Platform Rollout (Progressive)

| Giai đoạn | Nền tảng | Khi nào | Lý do |
|-----------|----------|---------|-------|
| **Prototype** | macOS + iPhone | Tuần 1-6 | Cùng hệ sinh thái Apple, cùng Secure Enclave, 1 team |
| **Phase 1** | macOS + iPhone | Tháng 1-4 | Pilot trên 2 nền tảng chính |
| **Phase 2A-C** | + Android, Oppo | Tháng 5-11 | Thị trường Việt Nam/Á, sau khi có revenue |
| **Phase 2D** | + Windows | Tháng 12-14 | Doanh nghiệp Windows-heavy |
| **Phase 3** | + Linux, Huawei, Server | Tháng 15-24 | Gov/Defense, toàn cầu |

**Nguyên tắc:** Chỉ thêm nền tảng khi có 3+ khách hàng trả tiền yêu cầu. Không thêm "vì có thể."

---

## Phase 0 — Architecture & Design Foundation

**Duration:** 1 tuần
**Goal:** Khóa ranh giới kiến trúc, CI/CD, design system

| File | Nội dung chính |
|------|---------------|
| [Phase 0](phase-0-architecture-foundation.md) | 5 task boxes: Domain boundaries, gRPC/Protobuf, CI baseline, Design system, ADRs |

---

## Prototype Phase — "TeraChat Zero"

**Duration:** 4-6 tuần
**Goal:** MLS E2EE chat chạy được trên macOS + iPhone. Demo được cho khách hàng.
**Subsystem:** Chỉ MLS cơ bản (Curve25519, không PQ)

| File | Nội dung chính |
|------|---------------|
| [Prototype Phase](prototype-phase.md) | MLS E2EE chat, License JWT, macOS+iPhone, TeraRelay single binary |

**Exit Gate:** Demo được cho 5+ doanh nghiệp. Thu thập feedback.

---

## Phase 1 MVP — "Sign the Pilot"

**Duration:** 3-4 tháng (sau prototype)
**Subsystem mới:** Identity Integration (OIDC/SAML)
**Economic goal:** 3 signed pilots → ít nhất 1 chuyển thành paid

### Scope: VÀO / RA

| VÀO (4 components) | RA (Deferred) |
|---------------------|---------------|
| MLS E2EE Internal Messaging | ~~Hybrid PQ-KEM~~ → Phase 2A |
| License JWT + DeviceIdentityKey | ~~Survival Mesh~~ → Phase 2B |
| OIDC/SAML (Google Workspace + Azure AD) | ~~.tapp Marketplace~~ → Phase 2C |
| 1 Reference .tapp (Expense Approval) | ~~Gemma 4 AI~~ → Phase 2D |
| Deployment Automation (30-min target) | ~~Mac mini HA Cluster~~ → Phase 3 |
| | ~~Customer Messaging~~ → NEVER |

### Quantitative Gate Metrics

| # | Metric | Target |
|---|--------|--------|
| M1 | Signed Pilots | ≥ 3 tổ chức |
| M2 | IT Admin Deploy Time | ≤ 30 phút (không hỗ trợ) |
| M3 | Daily Active Users (per pilot) | ≥ 20 users, 14 ngày liên tục |
| M4 | Message Delivery | 100%, 0 loss |
| M5 | Uptime | ≥ 99.5% trong 30 ngày |
| M6 | Pilot → Paid LOI | ≥ 1 signed Letter of Intent |
| M7 | Data Loss Incidents | 0 |

**Gate:** ≥ 5/7 metrics đạt → GO Phase 2. < 5/7 → pivot hoặc tiếp tục Phase 1.

| File | Nội dung chính |
|------|---------------|
| [Phase 1](phase-1-trust-kernel.md) | MLS E2EE, License JWT, OIDC/SAML, Deployment Automation, 1 Reference .tapp |

---

## Phase 2A — Post-Quantum Cryptography

**Duration:** 2 tháng (Month 5-6)
**Subsystem mới:** ML-KEM-768 Hybrid PQ-KEM
**Prerequisite:** Phase 1 GO + ít nhất 1 khách yêu cầu PQ

**Gate Metrics:**
- MLS Epoch Rotation với 100 members < 1s
- PQ handshake không làm chậm UX quá 200ms
- Zeroize verification pass dưới cargo miri

---

## Phase 2B — Survival Mesh

**Duration:** 3 tháng (Month 7-9)
**Subsystem mới:** BLE 5.0 + Wi-Fi Direct P2P Mesh
**Prerequisite:** 3+ khách hàng hỏi "nếu mất internet thì sao?"

**Gate Metrics:**
- SC-01: Internet partition 30 phút → full recovery < 120s, 0 data loss
- SC-38: BLE 100kbps cap + 250ms RTT → P0 delivered < 2s
- EmdpSessionTerminated không bị drop bởi P2 transfer

---

## Phase 2C — .tapp Marketplace

**Duration:** 2 tháng (Month 10-11)
**Subsystem mới:** WASM Dual-Engine + Web Marketplace + Self-Service Deploy
**Prerequisite:** ≥ 5 khách hàng trả tiền (cần user base cho marketplace)

**Gate Metrics:**
- ≥ 5 vetted .tapps trên marketplace
- ≥ 2 .tapps có active usage
- Self-service deploy < 10 phút (không cần IT admin)
- WasmParity CI: wasm3 ≡ wasmtime (delta ≤ 20ms)

---

## Phase 2D — Gemma 4 Local AI

**Duration:** 3 tháng (Month 12-14)
**Subsystem mới:** Gemma 4 ONNX on-device + SanitizedPrompt Pipeline
**Prerequisite:** ≥ 30% khách hàng survey nói "cần AI trong chat"

**Gate Metrics:**
- ≥ 30% users dùng AI feature ít nhất 1 lần/tuần
- PII redaction: 0 false negatives (PII lọt qua)
- Model load < 5s, inference < 2s với prompt ≤ 2000 tokens
- RAM peak < 4GB khi Gemma 4 loaded

---

## Phase 3A — Open AI Framework + Governance

**Duration:** 4 tháng (Month 15-18)
**Subsystem mới:** Multi-Model AI Framework + SCIM/Legal Hold đầy đủ
**Prerequisite:** Gemma 4 AI có usage ≥ 30%

**Gate Metrics:**
- ≥ 5 enterprise custom models registered
- SCIM offboarding < 30s
- ISO 27001 audit initiated
- 90-day retention ≥ 80%

---

## Phase 3B — Scale & Gov/Military

**Duration:** 6 tháng (Month 19-24)
**Subsystem mới:** Air-Gapped Deployment + Gov Certification
**Prerequisite:** ISO 27001 certified + ≥ 2 commercial references

**Gate Metrics:**
- ≥ 50 enterprise customers
- ≥ $1M ARR
- ≥ 1 Gov/Military contract signed
- ≥ 10 third-party .tapp publishers
- NRR ≥ 110%

---

## Platform Coverage Matrix

| Platform | App Path | Core Engine | UI Framework | Secure Enclave | Phase |
|----------|----------|-------------|--------------|----------------|-------|
| **macOS** | `apps/Laptop/macos` | Rust Core (native) | Tauri | Secure Enclave | Prototype |
| **iPhone** | `apps/Phone/Iphone` | Rust Core (FFI) | SwiftUI | Secure Enclave | Prototype |
| **Android** | `apps/Phone/Android` | Rust Core (Foreground Svc) | Jetpack Compose | StrongBox | Phase 2A |
| **Oppo** | `apps/Phone/Oppo` | Rust Core (Foreground Svc) | Jetpack Compose | StrongBox | Phase 2A |
| **Windows** | `apps/Laptop/windows` | Rust Core (native) | Tauri | TPM 2.0 | Phase 2D |
| **Linux** | `apps/Laptop/linux` | Rust Core (native) | Tauri | TPM 2.0 | Phase 3A |
| **Huawei** | `apps/Phone/Huawei` | Rust Core (FFI) | ArkUI | KeyStore | Phase 3A |
| **Mac Server** | `server/Mac` | TeraRelay binary | — | Secure Enclave | Phase 3A |
| **Physical Srv** | `server/Physical Server` | TeraRelay binary | — | TPM 2.0 / HSM | Phase 3B |

## Responsible Departments

| Department | Phase Focus | Lead Role |
|------------|-------------|-----------|
| **Architecture & Leadership** | Phase 0 + tất cả phase exits | System Architect |
| **Core Mesh & Cryptography** | Prototype, Phase 1, Phase 2A, Phase 2B | Applied Cryptographer |
| **State & CRDT Systems** | Phase 1, Phase 2C | Distributed Systems Engineer |
| **Client & UX Engineering** | Prototype, Phase 1, Phase 2C | Product UI Lead |
| **AI & Enclave Engineering** | Phase 2D, Phase 3A | ML/Enclave Lead |
| **Governance & Compliance** | Phase 3A, Phase 3B | CISO |
| **Infra, Ops & Quality** | Phase 0, Phase 1 (deploy), Phase 3 (chaos) | SRE + SecOps |

## System Design: What Connects to What

```
tc-crypto (MLS E2EE, PQ-KEM)  →  tc-mesh (BLE/WiFi Direct)
                               →  tc-store (encryption keys)
                               →  Hardware (Secure Enclave / TPM)

tc-mesh (BLE, EMDP)           →  tc-crypto (session keys)
                               →  tc-crdt-sync (offline queue)
                               →  UI HUD (signal renderer)

tc-crdt-sync (CRDT DAG)       →  tc-store (hot_dag.db)
                               →  tc-tapp (WASM state)
                               →  Relay (WAL replication)

tc-store (SQLite, CAS VFS)    →  tc-crypto (encryption)
                               →  tc-crdt-sync (read/write)
                               →  Bindings (FFI data path)

tc-tapp (WASM, Host ABI)      →  tc-store (transient state)
                               →  tc-crypto (ABI key delegation)
                               →  AI Module (host_ai_invoke)

AI Module (Gemma 4, Open FW)  →  tc-tapp (Host ABI boundary)
                               →  SanitizedPrompt (PII guard)
                               →  ONNX Runtime (local execution)

Bindings (FFI)                →  Core (all crates via gRPC)
                               →  Clients (Flutter/SwiftUI/Tauri)

Relay (VPS daemon)            →  All clients (mTLS/WSS)
                               →  Object Storage (MinIO/R2)
                               →  PostgreSQL (metadata/audit)
```

## Invariants — Never Violated

1. **Rust Core is domain owner** — UI is passive renderer only
2. **Headless daemon + gRPC** before UI expansion
3. **Dual-plane sync** — CRDT for chat, Relational for structured data
4. **AI only after SanitizedPrompt** — PII redaction + no embedding egress
5. **CISO holds veto** — DataGrant, SCIM, legal hold, kill-switch
6. **Test never trails** — SC-34 to SC-40 are deployment blockers
7. **1 subsystem per phase** — Progressive complexity, không build song song

## Reference Documents

| Document | Location |
|----------|----------|
| Improvement Hub | `docs/wiki/syntheses/improvement-plan-2026-05-11.md` |
| Narrowed Phase 1 MVP | `docs/wiki/syntheses/narrowed-phase-1-mvp.md` |
| GAP Resolution Tracker | `docs/wiki/syntheses/gap-resolution-tracker.md` |
| Platform Rollout Phasing | `docs/wiki/syntheses/platform-rollout-phasing.md` |
| Prototype First Model | `docs/wiki/syntheses/prototype-first-model.md` |
| Deployment Automation Spec | `docs/wiki/syntheses/deployment-automation-spec.md` |
| Quantitative Phase Metrics | `docs/wiki/syntheses/quantitative-phase-metrics.md` |
| AI Independent Workstream | `docs/wiki/syntheses/ai-independent-workstream.md` |
| Security Review Requirements | `docs/wiki/syntheses/security-review-requirements.md` |
| Vision Redefinition | `docs/wiki/syntheses/vision-redefinition-2026-05-11.md` |
| Phase Framework (Economic) | `docs/wiki/concepts/phase-framework.md` |
| Original 35-Day Plan | `phase/terachat-ai-agent-phase-plan.md` (archived reference) |
