# Spec-Identity-And-Governance.md — TeraChat Enterprise Platform

```yaml
# DOCUMENT IDENTITY
id: "TERA-GOV"
title: "TeraChat — Identity, RBAC & Governance Specification"
version: "1.0.0"
status: "ACTIVE"
audience: "System Architect, Security Engineer, CISO, Compliance Officer, IT Admin"
purpose: "Đặc tả quản trị doanh nghiệp, phân quyền role-based, OPA policy enforcement, identity federation (OIDC/SAML/SCIM), audit trail không thể xóa, và tenant isolation. CISO của doanh nghiệp khách hàng sẽ thẩm định file này đầu tiên."
depends_on: ["TERA-CORE", "TERA-SYNC"]
constraints_global:
  - "Ed25519 signed, append-only Audit Log — không thể delete/modify"
  - "OPA Policy phải pass Z3 formal verification trước khi deploy"
  - "SCIM offboarding: nhân viên nghỉ phải bị revoke trong < 30s"
  - ".tapp capability permissions: khai báo trong Manifest, không xin runtime"
  - "Legal Hold flag: bắt buộc trước khi vacuum tombstones liên quan"
  - "Approval action: bắt buộc ký Ed25519 — không chỉ là database flag"

```

> **Status:** `ACTIVE — Implementation Reference`
> **Audience:** Security Engineer · CISO · IT Admin · Compliance Officer
> **Last Updated:** 2026-03-29
> **Depends On:** → TERA-CORE · → TERA-SYNC
> **Consumed By:** → TERA-RUNTIME · → TERA-ENCLAVE · → TERA-ECO · → TERA-CLIENT

---

## §1 — EXECUTIVE SUMMARY & TRUST BOUNDARIES

### 1.1 Mục tiêu & Trách nhiệm

File này **chịu trách nhiệm** cho:
- OPA (Open Policy Agent) ABAC rules
- OIDC / SAML / SCIM 2.0 Identity Broker
- `.tapp` Capability Permissions (quyền hạn ngặt nghèo)
- Audit Trail (Ed25519 signed, append-only, không thể xóa)
- Tenant Isolation Model
- Role-based access control (RBAC)
- Legal Hold
- Approval Cryptographic Action Model (non-repudiation)

File này **KHÔNG chịu trách nhiệm** cho:
- Crypto primitives → `TERA-CORE`
- Storage backend → `TERA-SYNC`
- AI inference → `TERA-ENCLAVE`
- App signing & distribution → `TERA-ECO`

### 1.2 Audience Note cho CISO Security Audit

> Đây là file thẩm định đầu tiên cho bất kỳ Enterprise Security Audit (ISO 27001, SOC 2, GDPR).
> Kết hợp với `TERA-CORE` để có full cryptographic security model.

### 1.3 Trust Boundaries

| Boundary | Bên trong tin tưởng | Bên ngoài không tin tưởng |
|---|---|---|
| OPA Policy Engine | Verified policy bundles | Unverified policy sources |
| Identity Broker (Keycloak/Dex) | OIDC/SAML assertions | Raw credentials |
| Audit Log (append-only CRDT) | Ed25519 signed entries | Unsigned entries (rejected) |
| Tenant isolation boundary | Own org data | Cross-tenant data |
| .tapp capability scope | Declared permissions only | Runtime permission requests |

---

## §2 — SYSTEM ARCHITECTURE

### 2.1 OPA Policy Enforcement Points

```text
API GATEWAY
     │
     ├──[OPA Policy Engine] ← GeoHash IndexPing
     │       │
     │   Policy Decision
     │       │
     │       ├── ALLOW: route request
     │       └── DENY: return 403 + append Audit_Log_Entry
     │
CLIENT DEVICE RUST CORE
     │
     ├──[OPA Host ABI Boundary] ← WASM capability check
             │
         Policy Decision
             │
             ├── ALLOW: execute Host Function
             └── DENY: WasmSandboxTerminated signal
```

### 2.2 Identity Federation Flow

```text
[Enterprise IdP: Azure AD / Okta / Google Workspace]
          │ SAML Assertion / OIDC ID Token
          ▼
[Identity Broker: Keycloak/Dex]
          │ Map to TeraChat Role + Ed25519 Capability Token
          ▼
[TeraChat API Gateway]
          │ Verify Capability Token
          ▼
[Rust Core: enforce role-based IPC permissions]
          │
[SCIM Listener: sync user provisioning/deprovisioning]
          │ employee exits
          ▼
[Automatic revocation < 30s: MLS Epoch rotation + key revoke]
```

### 2.3 Audit Trail Architecture

```text
[Any governance action (Approve, Revoke, Admin change, .tapp install)]
          │
          ├── Ed25519 sign with DeviceIdentityKey
          ├── Append to Audit_Log (append-only CRDT chain)
          │       ├── Cannot delete
          │       └── Cannot modify (CRDT append-only)
          └── Replicate to Admin Console viewer
```

---

## §3 — DATA MODEL & ENCRYPTION STATE

### 3.1 Identity Objects

| Object | Type | Storage | Lifecycle | Security Constraint |
|---|---|---|---|---|
| `OidcCapabilityToken` | Ed25519-signed JWT | RAM only | Per session (TTL configured) | Maps SAML assertion → TeraChat role |
| `ScimProvisionEvent` | `{user_id, action, timestamp, saml_attributes}` | Relational (Enclave) | Permanent | Ed25519 signed; append-only |
| `RoleAssignment` | `{user_id, role, scope, granted_by, hlc}` | `cold_state.db` | Until revoked | Signed by granting Admin |
| `DelegationToken` | `{source_id, capability, target_id, ttl, sig}` | In-flight + RAM | Per delegation session | Ed25519 signed; expiry enforced |

### 3.2 OPA Policy Objects

| Object | Type | Storage | Lifecycle | Security Constraint |
|---|---|---|---|---|
| `PolicyBundle` | Rego policy package (OPA format) | VPS + client (cached) | Until next push | Ed25519 signed by TeraChat Root CA |
| `PolicyDecision` | `{allow: bool, reasons: Vec<String>}` | RAM only | Per request | ZeroizeOnDrop after enforcement |
| `GeoHashIndex` | GeoHash prefix strings | OPA memory | Per policy reload | String comparison only (O(1) lookup) |
| `VoprfToken` | Blind Rate-Limit Token | Client + Gateway | Per rate window | Unlinkable to user identity |

### 3.3 .tapp Capability Permission Objects

```yaml
# Full Capability Matrix — defined in .tapp Manifest
capabilities:
  # Chat access
  read_message_context: false       # Cannot read chat history without user @bot command
  send_message_to_channel: false    # Cannot spam channels

  # Storage
  app_local_storage_mb: 50         # SQLite Virtual Tables quota
  read_shared_vault: false         # Cannot read other users' vault

  # Finance/HR (routes to Stateful Enclave, not CRDT)
  write_finance_ledger: false
  read_finance_ledger: false
  write_hrm_records: false
  read_hrm_records: false

  # Network (Egress_Outbox only)
  write_egress_outbox: true        # Can push data out (OPA DLP filtered)
  egress_domain_whitelist: ["api.partner.com"]

  # Inter-app
  event_bus_publish: ["task.created"]
  event_bus_subscribe: ["approval.granted"]

  # Background
  background_tick_interval_s: 300

  # AI
  request_ai_inference: true       # Can trigger AI (Consent-Driven Context only)
```

### 3.4 Audit Log Objects

| Object | Type | Storage | Lifecycle | Security Constraint |
|---|---|---|---|---|
| `Audit_Log_Entry` | `{device_id, timestamp, action, payload_hash, ed25519_sig}` | Append-only CRDT chain | **Permanent** | Ed25519 signed; cannot delete or modify |
| `LegalHoldFlag` | `{entity_id, hold_type, legal_basis, court_order_ref, placed_by}` | `cold_state.db` | Until legal order released | Blocks vacuum of related tombstones |
| `ApprovalSignature` | `{approval_id, approver_device_id, payload_hash, ed25519_sig, hlc}` | Append-only Audit Log | Permanent | Non-repudiation via hardware-bound key |

---

## §4 — PROTOCOL & EXECUTION CONTRACT

### 4.1 OPA ABAC Policy Evaluation

**At API Gateway:**

```rego
# Example OPA policy (Rego)
package terachat.api

default allow = false

allow {
    input.method == "POST"
    input.path == "/api/v1/messages"
    has_valid_token(input.token)
    not is_rate_limited(input.client_id)
    geo_policy_allows(input.geo_hash, data.org_policy.allowed_geos)
}

geo_policy_allows(geo, allowed) {
    some prefix in allowed
    startswith(geo, prefix)
}
```

**Formal Verification:**
- Every policy bundle → SMT Model → **Z3 Solver** → block deploy if logical gap found.
- Deploy requires: policy_bundle.ed25519_sig valid AND Z3 verification passed.

### 4.2 SCIM 2.0 Offboarding Protocol

```text
[HR fires SCIM PATCH: user status = inactive]
     │
[SCIM Listener receives event]
     │
[Rust Core actions (< 30s total):]
     ├── Mark user_id as revoked in OPA policy cache
     ├── Trigger MLS Epoch Rotation for all affected groups
     ├── Revoke DelegationTokens with source_id = user_id
     ├── Push Shun_Record signed by Enterprise CA to mesh
     └── Append Audit_Log_Entry (action: "user_revoked")
```

### 4.3 Approval Cryptographic Action Model

**"Nút Duyệt" phải là một hành động pháp lý, không chỉ là database flag:**

```text
[User clicks "Approve"]
     │
[Rust Core builds ApprovalPayload:]
     ├── approval_id: UUID
     ├── target_request_hash: BLAKE3(request_content)
     ├── approver: device_id
     ├── timestamp: HLC
     └── context: {org_id, role, legal_basis_if_required}
     │
[Sign: Ed25519(DeviceIdentityKey, ApprovalPayload)]
     │
[Append to Audit_Log_Entry]
     │
[Return ApprovalSignature to requester]
```

**Non-repudiation guarantee:** `ApprovalSignature` uses hardware-bound `DeviceIdentityKey` — cannot be forged or denied.

### 4.4 Tenant Isolation Protocol

- All data in `cold_state.db` tagged with `tenant_id`.
- OPA enforces cross-tenant data access = DENY.
- AI Enclave: each inference session isolated per `session_id` (no cross-tenant prompt bleed).
- Audit Log: tenant_id embedded in every entry; cross-tenant log access = DENY.

### 4.5 .tapp Permission Check Protocol

```text
[.tapp calls host_app_state_write(patch)]
     │
[Rust Core: OPA check]
     ├── Does tapp_id have "app_local_storage_mb" > 0?
     ├── Does tapp_id have right to write this table?
     └── Is quota exceeded?
     │
     ├── DENY → return error code + append Audit_Log_Entry
     └── ALLOW → execute write
```

---

## §5 — STATE MACHINE

### 5.1 User Lifecycle State

```text
[PROVISIONED] → SCIM PATCH status=inactive → [REVOKED]
     │                                             │
     │ Active user                                 │ all access denied
     ▼                                             ▼
[ACTIVE]           [SUSPENDED]              [TOMBSTONED]
     │ Admin suspends      │ Reactivate          (after X days)
     └──> [SUSPENDED] ─────┘
```

### 5.2 .tapp Permission State

```text
[PENDING_REVIEW] → IT Admin approves → [PERMITTED]
     │                                      │ capability revoked
     │ Admin rejects                         ▼
     ▼                               [CAPABILITY_REVOKED]
[REJECTED]
```

### 5.3 Legal Hold State

```text
[NORMAL] → Legal Hold requested → [HOLD_PENDING]
                                         │ court_order_ref provided
                                         ▼
                                   [LEGAL_HOLD_ACTIVE]
                                         │ order released
                                         ▼
                                   [NORMAL] (vacuum eligible again)
```

---

## §6 — API / IPC / EVENT BUS

### 6.1 OPA Host ABI (for WASM Boundary)

```rust
extern "C" {
    // Check OPA policy before executing host function
    fn host_opa_check(
        policy_input_ptr: *const u8,
        policy_input_len: usize,
        decision_out: *mut u8
    ) -> i32;  // Returns: 1 = allow, 0 = deny

    // Append to audit log
    fn host_audit_log(
        action_ptr: *const u8,
        action_len: usize,
        payload_hash: *const u8
    ) -> i32;
}
```

### 6.2 Signals (Governance Domain)

| Signal | Trigger | Consumer |
|---|---|---|
| `UserRevoked(user_id)` | SCIM PATCH status=inactive | TERA-CORE (MLS epoch + Shun_Record) |
| `PolicyBundleUpdated(policy_id, version)` | New OPA bundle pushed | All clients + API Gateway |
| `TappCapabilityDenied(tapp_id, capability)` | OPA deny at Host ABI | TERA-CLIENT (show permission error) |
| `LegalHoldPlaced(entity_id, hold_id)` | Legal Hold flag set | TERA-SYNC (block vacuum) |
| `ApprovalSigned(approval_id)` | Ed25519 approval action | TERA-CLIENT (show confirmation) + Audit Log |
| `AuditLogExported(export_id, requester)` | Admin requests audit export | TERA-CLIENT (notify Admin) |

---

## §7 — PLATFORM MATRIX & CONSTRAINTS

| Feature | 📱 iOS | 📱 Android | 💻🖥️ Desktop | ☁️ Gateway | 🗄️ Mac mini |
|---|---|---|---|---|---|
| OPA enforcement | Via Host ABI in Rust Core | Via Host ABI in Rust Core | Via Host ABI in Rust Core | Full OPA engine | N/A |
| SCIM listener | N/A | N/A | N/A | ✅ | ✅ (optional) |
| Audit Log write | ✅ (Ed25519 signed) | ✅ (Ed25519 signed) | ✅ (Ed25519 signed) | ✅ | ✅ |
| OIDC/SAML bridge | N/A (client only) | N/A | N/A | Keycloak/Dex | N/A |
| Certificate Pinning | ✅ (SHA-256 SPKI) | ✅ | ✅ | N/A | N/A |
| Legal Hold enforcement | TERA-SYNC blocks vacuum | TERA-SYNC blocks vacuum | TERA-SYNC blocks vacuum | ✅ | ✅ |
| ZKP Attribute Routing | ✅ (zk-SNARKs in OPA) | ✅ | ✅ | ✅ | N/A |

---

## §8 — NON-FUNCTIONAL REQUIREMENTS (NFR)

| Requirement | Target | Notes |
|---|---|---|
| SCIM offboarding latency | < 30s end-to-end | From SCIM event to key revoke |
| OPA policy evaluation | < 10ms P99 | Per API request |
| Audit log write | < 10ms | Ed25519 sign + append |
| Policy formal verification (Z3) | < 60s | Before deploy |
| MLS Epoch rotation on revoke | < 5s for group ≤ 1000 | Per revoke event |
| ApprovalSignature generation | < 500ms | Including biometric confirmation |
| Cross-tenant isolation failure rate | 0% | Hard requirement |
| Audit log replay integrity | 100% replayable | No gaps in append-only chain |

---

## §9 — SECURITY & THREAT MODEL

| Attack | Vector | Mitigation |
|---|---|---|
| Privilege escalation | .tapp claims additional capability at runtime | Capabilities locked in Manifest; OPA verifies vs manifest on every Host ABI call |
| Audit log tampering | Delete or modify Audit_Log_Entry | Append-only CRDT; Ed25519 signed; structural replay audit |
| Cross-tenant data leak | OPA policy misconfiguration | Formal Z3 verification before deploy; tenant_id enforced at DB row level |
| Slow offboarding | SCIM events delayed | SCIM listener priority queue; < 30s SLA enforced |
| Approval forgery | Impersonate approver | ApprovalSignature uses hardware-bound DeviceIdentityKey; biometric required |
| Legal Hold bypass | Delete data under hold | LegalHoldFlag blocks vacuum at TERA-SYNC level; cross-checked before any delete |
| Policy injection | Attacker pushes malicious OPA bundle | Bundle must be Ed25519 signed by TeraChat Root CA; Z3 verified |
| Rate limit evasion | Flood API with anonymous requests | VOPRF blind tokens: rate limit enforced without linking requests to identity |

---

## §10 — FAILURE MODEL & RECOVERY

| Failure | Detection | Recovery |
|---|---|---|
| OPA engine crash | API gateway health check | Failover to cached policy (last known good); alert Admin |
| SCIM listener timeout | Event not processed in 30s | Retry queue; Admin notified; manual revoke via Admin Console |
| Audit Log chain broken | BLAKE3 integrity check on replay | Alert Admin; Isolate broken segment; Legal team investigation |
| Z3 verification timeout | > 60s on policy build | Block deploy; manual review required |
| ApprovalSignature hardware fail | Biometric API unavailable | Fallback to PIN + audit log `AttestationFallback` |
| Legal Hold placed but vacuum runs | Race condition | TERA-SYNC checks LegalHoldFlag atomically before vacuum; hold wins |
| OPA policy divergence (client vs gateway) | Policy version mismatch | Client re-fetches latest bundle; grace period 60s |

---

## §11 — COMPLIANCE DISCLOSURES & AUDIT RESOLUTIONS (GOVERNANCE)

### 11.1 Huawei HMS — SLA Disclosure in Enterprise Contracts (GAP-F)
**Constraint:** Huawei HMS Push không hỗ trợ `data-only` message type. Enterprise SCIM < 30s SLA không thể đảm bảo — polling interval tối thiểu là 4h. Không có disclosure nào trong pricing/feature documentation hiện tại.
**Resolution:**
- **Enterprise contracts** phải explicitly disclose: *"Huawei HarmonyOS devices không nằm trong Enterprise SLA tier (SCIM < 30s). Huawei devices dùng Polling Mode (CRL ≤ 4h) và TIDAK eligible cho Gov/Military deployment tier."*
- **Pricing_Packages.html** phải cò footnote rõ ràng cho mọi package listing tính năng mà Huawei không hỗ trợ.
- Spec cross-reference: TERA-ECO §7 — Huawei HMS Trust Tier Limitation.
- **Tech_Debt**: XPLAT-03 và GAP-F.

### 11.2 Tauri Linux — Cross-Origin Isolation Enforcement (XPLAT-05)
**Constraint:** Tauri WebView (GTK WebKitGTK) trên Linux sử dụng `SharedArrayBuffer` cho zero-copy Data Plane. Trình duyệt webkit modern yêu cầu `COOP: same-origin` + `COEP: require-corp` headers được set bởi Rust Core local HTTP server. Nếu không set, `SharedArrayBuffer` trở về `undefined` và Data Plane **silently degrade** về JSON serialization mà không có bất kỳ error hay log nào. Không được mention trong bất kỳ spec nào.
**Resolution:**
- Rust Core local HTTP server **phải** set headers:
  ```
  Cross-Origin-Opener-Policy: same-origin
  Cross-Origin-Embedder-Policy: require-corp
  ```
- CI gate trên Linux: after startup, assert `typeof SharedArrayBuffer !== 'undefined'`.
- Nếu Linux distro không hỗ trợ Cross-Origin Isolation (cụ khối GTK < 2.36): **Explicit degradation warning** trong Admin Console — không silent.
- Cross-reference: TERA-CLIENT sẽ thêm `XPLAT_SHARED_ARRAY_BUFFER_UNAVAILABLE` signal vào CoreSignal table.
- **Tech_Debt**: XPLAT-05.
