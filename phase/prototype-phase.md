# Prototype Phase — "TeraChat Zero"

```yaml
id: "TERA-PROTOTYPE"
title: "MLS E2EE Chat Prototype — Prove the Architecture"
duration: "4-6 weeks"
priority: 🔴 CRITICAL — First running code, prove architecture before more specs
platforms: [macOS, iPhone]
subsystem: "MLS E2EE (Curve25519 only, no PQ, no mesh, no WASM, no AI)"
goal: "Demo được MLS E2EE chat cho design partner tiềm năng"
```

## Why Prototype First?

```
HIỆN TẠI: 23+ documents, 0 dòng code chạy được → không biết architecture có hoạt động không
ĐỀ XUẤT: Spec tối thiểu → Prototype (4-6 tuần) → Học → Spec hoàn chỉnh → Code
```

## System Design: Prototype Scope

```
┌──────────────────────────────────────────────┐
│  PROTOTYPE: Chỉ những gì CẦN để demo         │
│                                               │
│  ✅ MLS E2EE group chat (Curve25519)          │
│  ✅ License JWT validation                    │
│  ✅ OIDC login (Google Workspace)             │
│  ✅ macOS desktop app (Tauri)                 │
│  ✅ iPhone mobile app (Flutter)               │
│  ✅ TeraRelay single binary                   │
│  ✅ File transfer E2EE (nhỏ < 10MB)           │
│  ✅ Crash recovery (không mất tin)            │
│                                               │
│  ❌ Hybrid PQ-KEM → Phase 2A                 │
│  ❌ Survival Mesh → Phase 2B                  │
│  ❌ WASM .tapp → Phase 2C                     │
│  ❌ Gemma 4 AI → Phase 2D                     │
│  ❌ SCIM offboarding → Phase 3A               │
│  ❌ Android/Windows/Linux → Sau prototype     │
│  ❌ Customer messaging → NEVER                │
└──────────────────────────────────────────────┘
```

---

## Task Box P.1 — Rust Core: MLS E2EE + License

```yaml
task_id: "PH0-P01"
name: "MLS E2EE Group Chat + License JWT Core"
status: pending
priority: 🔴 CRITICAL
platforms: [macOS, iPhone]
estimated_weeks: 2
```

### Code Tasks
| # | Task | Source Path | Platform |
|---|------|-------------|----------|
| 1 | MLS group chat: create group, add members, send message, receive message | `source/core/tc-crypto/src/mls.rs` | All |
| 2 | Curve25519 key generation + E2EE encrypt/decrypt | `source/core/tc-crypto/src/e2ee.rs` | All |
| 3 | License JWT: parse, validate signature, check expiry, bind to device key | `source/core/tc-crypto/src/license.rs` | All |
| 4 | TeraRelay: single binary, SQLite WAL, auto TLS (Let's Encrypt) | `source/core/daemon/src/relay.rs` | macOS |
| 5 | Message routing: client A → relay → client B, relay sees only ciphertext | `source/core/daemon/src/route.rs` | All |

### Testing
| # | Test | Tool | Target |
|---|------|------|--------|
| 1 | 2 clients gửi 1000 messages → 0 loss | `cargo test --test mls_reliability` | All |
| 2 | License JWT valid → cho phép connect. Invalid → từ chối | `cargo test --test license_validation` | All |
| 3 | Relay: ciphertext only — verify relay không có plaintext | `cargo test --test relay_blind` | All |
| 4 | Kill app mid-message → reopen → message delivered | `cargo test --test crash_recovery` | All |

### Deployment
- macOS: `cargo build --release` → binary (~15MB)
- iPhone: `cargo build --release --target aarch64-apple-ios` → uniffi binding

### Reference Documentation
- `TERA-CORE §4.3` — MLS E2EE specification
- `docs/wiki/concepts/zero-knowledge-architecture.md`
- `docs/raw/MD/Tech_Debt.md` — TD-006 (zeroize on panic)

### System Design Connection
- **Output to:** Phase 1 (add OIDC/SAML), Phase 2A (upgrade to PQ-KEM)
- **Connects:** tc-crypto → TeraRelay → Flutter/Tauri clients

---

## Task Box P.2 — macOS + iPhone UI

```yaml
task_id: "PH0-P02"
name: "Passive Renderer UI — macOS Tauri + iPhone Flutter"
status: pending
priority: 🔴 CRITICAL
platforms: [macOS, iPhone]
estimated_weeks: 2
```

### Code Tasks
| # | Task | Source Path | Platform |
|---|------|-------------|----------|
| 1 | Tauri app: login screen → chat list → chat view → send message | `source/clients/desktop/` | macOS |
| 2 | Flutter app: login screen → chat list → chat view → send message | `source/clients/apple/` | iPhone |
| 3 | gRPC local connection: UI → Rust Core (127.0.0.1:50051 hoặc UDS) | Both clients | All |
| 4 | E2EE indicator: "Encrypted" badge per message (data từ CoreSignal) | Both clients | All |
| 5 | License status indicator: valid (green) / expiring (amber) / expired (red) | Both clients | All |

### Testing
| # | Test | Tool | Target |
|---|------|------|--------|
| 1 | macOS gửi tin → iPhone nhận → E2EE indicator hiển thị đúng | E2E test | Cross-platform |
| 2 | UI crash → mở lại → reconnect → không mất state | Integration test | All |
| 3 | UI chỉ render signals từ Core — không tự tính toán domain state | Code review | All |

### Deployment
- macOS: Tauri bundle (.dmg)
- iPhone: Xcode archive → TestFlight (internal)

### Design Requirements
- Glassmorphism: light/dark mode, blur effect
- E2EE indicators: green lock = MLS encrypted
- License status: visible nhưng không intrusive

### Reference Documentation
- `TERA-CLIENT §11.3-11.5` — Widget states
- `docs/wiki/concepts/glassmorphism-design-system.md`

### System Design Connection
- **Input from:** Task Box P.1 (Rust Core)
- **Output to:** Phase 1 (add OIDC login flow, add reference .tapp UI)
- **Connects:** gRPC → CoreSignal → Passive UI render

---

## Task Box P.3 — OIDC Login + TeraRelay Deploy

```yaml
task_id: "PH0-P03"
name: "Google Workspace Login + Relay One-Command Deploy"
status: pending
priority: 🟠 HIGH
platforms: [macOS, iPhone]
estimated_weeks: 1
```

### Code Tasks
| # | Task | Source Path | Platform |
|---|------|-------------|----------|
| 1 | Keycloak/Dex bridge: OIDC flow với Google Workspace | `source/core/daemon/src/oidc.rs` | All |
| 2 | Token validation: verify JWT, extract email, map to TeraChat user | `source/core/daemon/src/oidc.rs` | All |
| 3 | Install script: `curl -fsSL https://install.terachat.io \| sudo bash` | `source/infra/install.sh` | macOS |
| 4 | Auto TLS via Let's Encrypt (hoặc self-signed fallback) | `source/core/daemon/src/tls.rs` | All |

### Testing
| # | Test | Tool | Target |
|---|------|------|--------|
| 1 | Fresh macOS → chạy install script → TeraChat ready < 5 phút | Manual test | macOS |
| 2 | Login với Google Workspace account → vào được chat | E2E test | All |
| 3 | Token hết hạn → refresh tự động → không logout | Integration test | All |

### Deployment
- Install script: single curl pipe bash, detect OS, download binary, setup service
- Health check: `GET /health` → JSON status

### Reference Documentation
- `docs/wiki/syntheses/deployment-automation-spec.md`
- `TERA-GOV §2.2` — OIDC/SAML flow

### System Design Connection
- **Input from:** Task Box P.1 (relay), Task Box P.2 (UI login screen)
- **Output to:** Phase 1 (add Azure AD SAML, full deployment automation)

---

## Task Box P.4 — Hardening & Demo Prep

```yaml
task_id: "PH0-P04"
name: "Crash Recovery + File Transfer + Demo Package"
status: pending
priority: 🟠 HIGH
platforms: [macOS, iPhone]
estimated_weeks: 1
```

### Code Tasks
| # | Task | Source Path | Platform |
|---|------|-------------|----------|
| 1 | Crash recovery: kill process mid-message → restart → tin đến nơi | `source/core/tc-crdt-sync/src/recovery.rs` | All |
| 2 | File transfer: encrypt file với session key → upload blob → decrypt on receive | `source/core/tc-crypto/src/file_transfer.rs` | All |
| 3 | Demo script: "IT Admin deploys in 5 min, sends first message in 10 min" | `docs/demo-script.md` | All |
| 4 | Demo video: screen record macOS + iPhone cross-device messaging | — | All |

### Testing
| # | Test | Tool | Target |
|---|------|------|--------|
| 1 | Kill process 100 lần tại random points → 0 message loss | `cargo test --test crash_loop` | All |
| 2 | File 10MB: encrypt → upload → download → decrypt → verify identical | `cargo test --test file_transfer` | All |
| 3 | Demo script: IT admin (không technical) làm theo → thành công | Manual test | macOS |

### Deployment
- Demo package: README + install script + demo video + test accounts

### Reference Documentation
- `TERA-SYNC §8.4` — Crash recovery
- `TERA-CORE §4.3` — File transfer E2EE

### System Design Connection
- **Input from:** Task Boxes P.1-P.3
- **Output to:** Design partner demo → feedback → Phase 1 scope refinement

---

## Prototype Exit Gate

Trước khi bắt đầu Phase 1:

- [ ] Demo được MLS E2EE chat cho 5+ doanh nghiệp tiềm năng
- [ ] Thu thập feedback: cái gì thiếu, cái gì thừa, cái gì sai
- [ ] Điều chỉnh Phase 1 scope dựa trên feedback thực tế
- [ ] Quyết định: GO Phase 1 (có ít nhất 1 doanh nghiệp muốn pilot) hoặc PIVOT

**Nguyên tắc:** Không build Phase 1 nếu chưa có doanh nghiệp nào muốn pilot.
