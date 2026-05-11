# Phase 1 MVP — "Sign the Pilot"

```yaml
id: "TERA-PHASE-1"
title: "MLS E2EE Messaging + License + OIDC/SAML + 1 Reference .tapp"
duration: "3-4 months (after prototype)"
economic_phase: "Phase 1 — 'Enough to Sign the Pilot'"
priority: 🔴 CRITICAL — This is the revenue milestone
platforms: [macOS, iPhone]
subsystem_added: "Identity Integration (OIDC/SAML)"
goal: "3 signed pilots, ≥ 1 converted to paid"
deferred_to_phase_2: [PQ-KEM, Survival Mesh, .tapp Marketplace, Gemma 4 AI, Mac mini HA]
never_in_scope: [Customer-facing messaging]

exit_criteria:
  - 3 signed pilot agreements
  - IT admin deploy ≤ 30 minutes (không hỗ trợ)
  - 100% message delivery, 0 data loss trong suốt pilot
  - 1 signed Letter of Intent (pilot → paid)
  - Deployment automation script verified trên macOS sạch
```

## System Design: Phase 1 Scope

```
┌──────────────────────────────────────────────────┐
│  PHASE 1: CHỈ những gì cần để ký pilot           │
│                                                   │
│  ✅ MLS E2EE Internal Messaging (Curve25519)      │
│  ✅ License JWT + DeviceIdentityKey               │
│  ✅ OIDC/SAML (Google Workspace + Azure AD)       │
│  ✅ 1 Reference .tapp (Expense Approval)          │
│  ✅ Deployment Automation (30-min target)         │
│  ✅ macOS + iPhone                                │
│  ✅ Crash recovery (không mất tin)                │
│  ✅ Admin Console (minimal)                       │
│                                                   │
│  ❌ PQ-KEM → Phase 2A                             │
│  ❌ BLE Mesh → Phase 2B                           │
│  ❌ .tapp Marketplace → Phase 2C                  │
│  ❌ Gemma 4 AI → Phase 2D                         │
│  ❌ Android/Windows/Linux → Sau pilot             │
│  ❌ Customer messaging → NEVER                    │
└──────────────────────────────────────────────────┘
```

---

## Task Box 1.1 — Production MLS E2EE Messaging

```yaml
task_id: "PH1-T01"
name: "Harden MLS E2EE from Prototype to Production"
status: pending
priority: 🔴 CRITICAL
platforms: [macOS, iPhone]
estimated_weeks: 4
builds_on: "Prototype Phase Task Box P.1"
```

### Code Tasks
| # | Task | Source Path | Platform |
|---|------|-------------|----------|
| 1 | Hardening MLS: epoch rotation on member leave, replay protection, message ordering | `source/core/tc-crypto/src/mls.rs` | All |
| 2 | GlobalKeyArena + `ffi_boundary!` macro (TD-006 fix — zeroize on panic) | `source/core/tc-crypto/src/arena.rs` | All |
| 3 | `std::panic::set_hook` — wipe Arena before abort | `source/core/tc-crypto/src/panic_hook.rs` | All |
| 4 | iOS XOR RAM Masking (TD-007 mitigation) | `source/core/tc-crypto/src/xor_mask.rs` | iPhone |
| 5 | Composite key derivation: HKDF(device_key || session_nonce) | `source/core/tc-crypto/src/key_derivation.rs` | All |

### Testing
| # | Test | Tool | Platform |
|---|------|------|----------|
| 1 | 10,000 messages sent → 0 loss, đúng thứ tự | `cargo test --test mls_stress` | All |
| 2 | Member leave → epoch rotate → old member cannot decrypt new messages | `cargo test --test epoch_rotation` | All |
| 3 | Panic during crypto op → Arena zeroized (miri) | `cargo miri test --test ffi_boundary_zeroize` | All |
| 4 | Replay old message → rejected | `cargo test --test replay_protection` | All |

### Deployment
macOS Tauri bundle + iPhone TestFlight.

### Reference Documentation
- `TERA-CORE §4.3` — MLS E2EE
- `TERA-CORE §12.1` — GlobalKeyArena
- `docs/raw/MD/Tech_Debt.md` — TD-006, TD-007

### System Design Connection
- **Input from:** Prototype Phase (MLS cơ bản)
- **Output to:** Phase 2A (upgrade lên PQ-KEM)
- **Connects:** tc-crypto → TeraRelay → Flutter/Tauri UI

---

## Task Box 1.2 — License JWT + DeviceIdentityKey (Production)

```yaml
task_id: "PH1-T02"
name: "Production License Validation & Cryptographic Device Binding"
status: pending
priority: 🔴 CRITICAL
platforms: [macOS, iPhone]
estimated_weeks: 2
builds_on: "Prototype Phase Task Box P.1"
```

### Code Tasks
| # | Task | Source Path | Platform |
|---|------|-------------|----------|
| 1 | License JWT: HSM-signed, parse, validate chain, check expiry, check seat limit | `source/core/tc-crypto/src/license.rs` | All |
| 2 | DeviceIdentityKey: generate in Secure Enclave (iOS/macOS), never exported | `source/core/tc-crypto/src/device_key.rs` | All |
| 3 | Key derivation: KDF(license_jwt, device_identity_key) → encryption key | `source/core/tc-crypto/src/key_derivation.rs` | All |
| 4 | License states: Valid (green) → T-30 (amber) → T-0 (amber lock) → T+90 (red lock) | `source/core/tc-crypto/src/license.rs` | All |

### Testing
| # | Test | Tool | Platform |
|---|------|------|----------|
| 1 | License valid → full access. Expired T+0 → admin warning. T+90 → full lockout | `cargo test --test license_states` | All |
| 2 | Copy license JWT to unauthorized device → wrong key → DB unreadable | `cargo test --test license_binding` | All |
| 3 | Seat limit exceeded → new user blocked, admin notified | `cargo test --test seat_limit` | All |

### Deployment
License server: TeraChat Inc. HSM issues JWTs. Relay caches và validates locally.

### Reference Documentation
- `docs/wiki/concepts/enterprise-license-model.md`
- `TERA-CORE §11.7` — Key derivation

### System Design Connection
- **Input from:** Prototype Phase (license cơ bản)
- **Output to:** Revenue model — đây là cách TeraChat kiếm tiền
- **Connects:** License JWT → Device Key → Encryption Key → All data access

---

## Task Box 1.3 — OIDC/SAML Integration

```yaml
task_id: "PH1-T03"
name: "Enterprise Identity Integration — Google Workspace + Azure AD"
status: pending
priority: 🔴 CRITICAL
platforms: [macOS, iPhone]
estimated_weeks: 3
```

### Code Tasks
| # | Task | Source Path | Platform |
|---|------|-------------|----------|
| 1 | Keycloak/Dex bridge: OIDC Google Workspace + SAML Azure AD | `source/core/daemon/src/identity/` | All |
| 2 | SAML attribute → TeraChat role mapping table | `source/core/daemon/src/identity/saml_map.rs` | All |
| 3 | Authority hierarchy: map `department`, `manager` attributes → messaging scope | `source/core/daemon/src/identity/hierarchy.rs` | All |
| 4 | Just-in-time provisioning: first login → auto-create user với role từ SAML | `source/core/daemon/src/identity/provision.rs` | All |
| 5 | No new passwords: user chỉ dùng Google/Azure account hiện có | All |

### Testing
| # | Test | Tool | Platform |
|---|------|------|----------|
| 1 | Login với Google Workspace → role map đúng → vào đúng workspace | E2E test | All |
| 2 | SAML attribute "finance_dept" → TeraChat role "FinanceUser" → chỉ thấy Finance channels | Integration test | All |
| 3 | User bị xóa khỏi Azure AD → next sync → TeraChat account disabled | `cargo test --test saml_deprovision` | All |

### Deployment
Keycloak/Dex chạy cùng TeraRelay (single binary). Cấu hình OIDC/SAML qua Admin Console.

### Reference Documentation
- `TERA-GOV §2.2` — OIDC/SAML flow
- `docs/wiki/concepts/hierarchical-authority-messaging.md`

### System Design Connection
- **Input from:** Prototype Phase (OIDC cơ bản)
- **Output to:** Phase 3A (SCIM offboarding, legal hold)
- **Connects:** Identity Provider → Keycloak → TeraChat Roles → Authority Scope

---

## Task Box 1.4 — Deployment Automation (THE Priority Spec)

```yaml
task_id: "PH1-T04"
name: "One-Command Deploy — IT Admin Không Cần DevOps"
status: pending
priority: 🔴 CRITICAL — Đây là tiêu chí thành công quan trọng nhất
platforms: [macOS]
estimated_weeks: 2
```

### Code Tasks
| # | Task | Source Path | Platform |
|---|------|-------------|----------|
| 1 | Install script: `curl -fsSL https://install.terachat.io \| sudo bash` | `source/infra/install.sh` | macOS |
| 2 | Auto-detect OS, download binary, verify checksum, setup launchd service | `source/infra/install.sh` | macOS |
| 3 | Auto TLS: Let's Encrypt (production) hoặc self-signed (internal) | `source/core/daemon/src/tls.rs` | All |
| 4 | Bootstrap: khởi tạo SQLite, OPA policy, admin credentials | `source/core/daemon/src/bootstrap.rs` | All |
| 5 | Health check endpoint: `GET /health` → JSON status (uptime, users, license, DB) | `source/core/daemon/src/health.rs` | All |
| 6 | Admin Console (minimal): dashboard, users, workspaces, license, deploy .tapp | `source/clients/desktop/src/admin/` | macOS |

### Testing
| # | Test | Tool | Target |
|---|------|------|--------|
| 1 | Fresh macOS → chạy install script → TeraChat hoạt động < 5 phút | Manual timed test | macOS |
| 2 | IT admin không DevOps → deploy thành công chỉ với README | User test | macOS |
| 3 | Kill relay → auto restart < 10s → không mất data | `cargo test --test daemon_restart` | macOS |
| 4 | `GET /health` → response chính xác về trạng thái hệ thống | Integration test | All |

### Deployment
Đây là task deployment QUAN TRỌNG NHẤT. Mọi thứ khác có thể tốt, nhưng nếu IT admin không deploy được trong 30 phút → pilot thất bại.

### Reference Documentation
- `docs/wiki/syntheses/deployment-automation-spec.md`
- `docs/wiki/concepts/phase-framework.md` — Phase 1 goal

### System Design Connection
- **Input from:** Prototype Phase (relay), Task Box 1.3 (OIDC)
- **Success defined by:** IT admin deploy time ≤ 30 phút
- **Connects:** Install script → TeraRelay → Admin Console → User onboarding

---

## Task Box 1.5 — Reference .tapp: Expense Approval

```yaml
task_id: "PH1-T05"
name: "One Reference .tapp — Prove the Work OS Concept"
status: pending
priority: 🟠 HIGH
platforms: [macOS, iPhone]
estimated_weeks: 2
```

### Code Tasks
| # | Task | Source Path | Platform |
|---|------|-------------|----------|
| 1 | Expense Approval .tapp: WASM module — form, submit, approve/reject flow | `source/core/tc-tapp/examples/expense_approval/` | All |
| 2 | WASM runtime (wasmtime cho macOS, wasm3 cho iPhone) | `source/core/tc-tapp/src/` | All |
| 3 | Host ABI cơ bản: storage (get/set), crypto (sign), không network | `source/core/tc-tapp/src/abi.rs` | All |
| 4 | IT Admin deploy .tapp đến Finance department trong < 10 phút | `source/core/daemon/src/tapp_deploy.rs` | All |

### Testing
| # | Test | Tool | Platform |
|---|------|------|----------|
| 1 | Nhân viên tạo expense → manager nhận notification → approve → hoàn tất | E2E test | All |
| 2 | .tapp deploy đến Finance dept → chỉ Finance users thấy | Integration test | All |
| 3 | wasm3 (iOS) output ≡ wasmtime (macOS) output | `cargo test --test wasm_parity` | All |

### Deployment
Tích hợp vào Admin Console: IT Admin → chọn .tapp → chọn department → deploy.

### Reference Documentation
- `TERA-RUNTIME §11.4` — Host ABI
- `docs/wiki/concepts/wasm-tapp-runtime.md`

### System Design Connection
- **Input from:** Task Box 1.1 (MLS), Task Box 1.4 (Admin Console)
- **Output to:** Phase 2C (.tapp Marketplace)
- **Connects:** tc-tapp → WASM sandbox → Host ABI → tc-store

---

## Phase 1 Gate Metrics

**TẤT CẢ phải đạt để GO Phase 2:**

- [ ] M1: ≥ 3 tổ chức ký pilot agreement
- [ ] M2: IT Admin deploy ≤ 30 phút, không cần hỗ trợ (đo thực tế)
- [ ] M3: ≥ 20 DAU per pilot, liên tục 14 ngày
- [ ] M4: 100% message delivery, 0 loss incidents
- [ ] M5: Uptime ≥ 99.5% trong 30 ngày pilot
- [ ] M6: ≥ 1 pilot ký Letter of Intent → paid
- [ ] M7: 0 data loss incidents

**Decision:** ≥ 5/7 → GO Phase 2. < 5/7 → pivot hoặc tiếp tục Phase 1.
