# Spec-Ecosystem-And-Trust-Chain.md — TeraChat Enterprise Platform

```yaml
# DOCUMENT IDENTITY
id: "TERA-ECO"
title: "TeraChat — Ecosystem Governance & Trust Chain Specification"
version: "1.0.0"
status: "ACTIVE"
audience: "Platform Engineer, Plugin Developer, IT Admin, Security Auditor"
purpose: "Đặc tả vòng đời phát hành ứng dụng .tapp: App Signing Trust Hierarchy, Enterprise Private Distribution (MDM/EMM), Publisher Trust Tiers, Security Review pipeline, Emergency Kill-switch, và Registry Transparency. Định nghĩa niềm tin mã hóa (Cryptographic Trust) của toàn bộ ecosystem."
depends_on: ["TERA-CORE", "TERA-RUNTIME", "TERA-GOV"]
constraints_global:
  - "Không có chữ ký TeraChat CA → Rust Core từ chối load — không có ngoại lệ"
  - "Mọi .tapp capability khai báo trong Manifest — không xin permission runtime"
  - "Revocation phải effective ≤ 60s trên mọi device online"
  - "Emergency Kill-switch không cần app update hay store review"
  - "Private Enterprise Distribution: .tapp không cần qua TeraChat public Registry"
  - "WasmParity CI gate bắt buộc: wasm3 vs wasmtime semantically identical"

```

> **Status:** `ACTIVE — Implementation Reference`
> **Audience:** Platform Engineer · Plugin Developer · IT Admin · Security Auditor
> **Last Updated:** 2026-03-29
> **Depends On:** → TERA-CORE · → TERA-RUNTIME · → TERA-GOV
> **Consumed By:** _(enterprise governance boundary)_

---

## §1 — EXECUTIVE SUMMARY & TRUST BOUNDARIES

### 1.1 Mục tiêu & Trách nhiệm

File này **chịu trách nhiệm** cho:
- App Signing Trust Hierarchy (Root CA → Enterprise CA → Developer Key → Bundle Sig)
- Enterprise Plugin Registry (controlled, not public App Store)
- Private Enterprise Distribution (MDM/EMM sideload không qua Registry)
- Publisher Trust Tiers
- Security Review Pipeline (automated + manual)
- Emergency Kill-switch
- Registry Transparency Log
- Developer guidelines & Host Function ABI versioning

File này **KHÔNG chịu trách nhiệm** cho:
- WASM execution runtime → `TERA-RUNTIME`
- Crypto primitives → `TERA-CORE`
- RBAC & OPA policy enforcement → `TERA-GOV`

### 1.2 Đây KHÔNG phải App Store công khai

> Enterprise Plugin Registry là hệ thống **khép kín, do IT Admin kiểm soát**.

End user **không thể:**
- Tự duyệt cài plugin
- Truy cập Registry trực tiếp
- Bypass IT Admin policy

### 1.3 Trust Boundaries

| Boundary | Bên trong tin tưởng | Bên ngoài không tin tưởng |
|---|---|---|
| TeraChat Root CA | First-party .tapp, verified publisher bundles | Self-signed publisher keys |
| Enterprise CA | Internally distributed .tapp | External publishers |
| IT Admin approval | Workspace-level deployment | End user install requests |
| Registry Transparency Log | Append-only Merkle-proofed events | Unsigned events |

### 1.4 The "Three Third-Spaces" Integration Boundaries

When TeraChat integrates with external corporate tools (e.g., Gmail, Jira), it strictly governs the data flow to maintain complete sovereignty. Instead of allowing direct API calls from external platforms, the integration occurs strictly within three demarcated ecosystem tiers:

1. **Native UI (External App context):** TeraChat injects native SDK hooks (e.g., Gmail Extension) presenting a familiar UI directly within the third-party application, capturing data at its origin.
2. **Interop Hub (The Gateway):** Triggers and payloads captured by the Native UI are sent to the Zero-Knowledge Interop Hub. The Hub reformats the request and securely routes the task intent to the Enterprise's secure perimeter, ensuring no third-party server can read the data.
3. **AI Worker (The Execution Core):** Residing securely inside the internal enclave, the `.tapp` and AI Workers execute the business logic, pulling any required historical data from the encrypted NAS. The finalized, encrypted response is then piped back through the Interop Hub.

---

## §2 — SYSTEM ARCHITECTURE

### 2.1 App Signing Trust Hierarchy

```text
[TeraChat Root CA Key]  ← Offline cold storage
         │
         ├──> [TeraChat Marketplace CA Key]
         │           │
         │    Signs: Publisher Key Bundle
         │           │
         │    [Publisher Ed25519 Key Pair]
         │           │
         │    Signs: SHA3-256(manifest.json || logic.wasm) → bundle.sig
         │           │
         │    Client Verifies: BOTH Publisher sig AND TeraChat CA sig
         │
         └──> [Enterprise CA Key] ← For private enterprise distribution
                     │
              Signs: Internal .tapp bundles for MDM/EMM push
                     │
              Client Verifies: Enterprise CA chain (no TeraChat public store)
```

### 2.2 Distribution Flows

**Public Registry Flow:**
```text
Publisher submits .tapp
     │
TeraChat Security Review (automated + manual)
     │
.tapp signed by TeraChat Marketplace CA
     │
Appears in Registry (scoped: private or public)
     │
IT Admin of org approves
     │
OPA Policy push → devices deploy .tapp
     │
End user sees plugin (no install control)
```

**Enterprise Private Distribution Flow (MDM/EMM):**
```text
Internal dev builds internal .tapp
     │
Signs with Enterprise CA Key (no TeraChat public CA needed)
     │
Upload to MDM/EMM server (Jamf / Intune / Kandji)
     │
MDM pushes .tapp bundle to enrolled devices
     │
Rust Core: verify Enterprise CA chain → load .tapp
     │
End user sees internal app
```

### 2.3 Emergency Kill-Switch Architecture

```text
TeraChat Security Team detects compromise
     │
Issues KILL_DIRECTIVE:
  {tapp_id, reason, evidence_hash, ed25519_sig (TeraChat Root CA)}
     │
Push via OPA update channel
     │
Devices receive in < 60s (online)
OR at next online sync (offline)
     │
Rust Core: KILL_DIRECTIVE matches tapp_id
     ├── Terminate running instance
     ├── Purge transient state (sled KV)
     ├── Revoke all DelegationTokens of .tapp
     └── Append to Audit_Log_Entry (PLUGIN_KILLED)
     │
IT Admin receives notification + technical report
```

### 2.4 Sovereign Work OS Orchestration Pillars

TeraChat is not merely a chat application; it is built as a Sovereign Work OS. To manage diverse `.tapps` interacting with a massive unified storage layer, it employs two foundational abstraction pillars under the Trust Chain:

1. **DataGrant (Zero-Knowledge Capability Framework):** External integrations and `.tapps` do not receive static API Keys or persistent DB credentials. Instead, they operate on Cryptographic DataGrants. A DataGrant is tightly-scoped, short-lived, and bound strictly to a User ID or specific Chat Thread. When an HR `.tapp` requests approval logs, the Rust Core requires a signed DataGrant before fulfilling the SQLite Virtual Table query.
   - **Tapp Ownership Invariant (Rule 2):** "App Suite Tapps Own Their Data, Nobody Else Does". DataGrant là READ-ONLY. Tapp này KHÔNG BAO GIỜ ghi đè trực tiếp vào bảng namespace của tapp khác. Nếu cần cập nhật dữ liệu, phải thông qua Local Event Bus.
   - **Row Filter Expression Language:** Khác với RBAC Policy dùng Rego, bộ đệm filter cho Row Level DataGrant được giới hạn ở **Predicate DSL**. Tránh execution overhead lãng phí.
     * DSL JSON Format: `{"field": "department", "op": "eq", "value": "HR"}`
     * Các phép toán hỗ trợ: `eq`, `neq`, `in`, `gte`, `lte`.
     * Evaluation: Rust Core đánh giá predicate tại `cold_state.db` *TRƯỚC* khi dữ liệu vào WASM boundary. Nesting logic `and/or` độ sâu <= 3.
2. **Distributed Namespaces:** To seamlessly bridge the Client-local state, VPS Transit nodes, and the Mac mini clusters, TeraChat leverages segmented Namespaces. `.tapps` address data via Universal Resource Identifiers (e.g., `terachat://namespace/hr-approvals`), entirely abstracting whether the physical bytes reside in the mobile RAM, the NAS CAS blob, or are currently locked by a MapReduce consolidation job on a specific Mac mini worker.

### 2.5 DataGrant Cache Invalidation & Pre-fetch Orchestrator

Đảm bảo Authorization Consistency & Storage Efficiency. Tách riêng 3 Phase xử lý DataGrant Workflow để tránh partial data corruption hoặc rendering jitter.

#### 1. DataGrant Active Revocation (Security Event Track)
- **Problem:** Staleness của Sled Cache gây leak dữ liệu nếu Revoked mà chưa hết TTL.
- **Contract:** Mọi DataGrant cấp phát đều mang `generation` number do Rust Core cấp số tăng dần.
- DataGrant Revocation là **SECURITY SIGNAL PUSH**, không phải passive TTL check!
- Khi OPA Policy push lệnh revoke, Rust Core `host_db_cursor_close` các connection đang chạy, bắn ra `CoreSignal::DataGrantRevoked` *đồng bộ qua Secure Channel (không queue)* và quét Prefix Sled Cache (`dg:{grant_id}:*`) để xoá ngay lập tức.
- **Mesh Mode Offline Guard:** Cache có field `max_offline_serve_duration` (Default Enterprise: 3600s, Gov-tier: 0s). Nếu quá thời gian này mà không liên lạc được Host OPA Policy, Widget hiển thị Unavailable, chặn data stale.

#### 2. Background Pre-fetch Orchestration
Không tự nạp thẳng vào bộ nhớ nếu User bị cấp cho lúc 10 loại quyền, chặn RAM Spike OOM.
- `CoreSignal::DataGrantActivated` trigger **Intent Registration** (Foreground / Background / Lazy priority).
- **Orchestrator** chạy Async Tokio task độc lập với Main Core Thread. Enqueue fetches.
- **Memory Arbiter** xin phép RAM Limit (Max 4MB/8MB/32MB tùy platform). `host_db_query` bắt đầu cursor fetch theo Page.
- Dữ liệu fetch về được Insert ngay vào `sled_cache`. Nếu Revocation Signal đến ở giữa pipeline -> Gửi `AbortHandle` xả toàn bộ pipeline ngay, Drop connection, KHÔNG commit partial data.

#### 3. Context-Aware Widget Loading States (Client Sync)
Các Widget DataGrant có 3 State riêng biệt tuỳ chỉnh để tránh Flicker xấu UX (Trạng thái được tính bởi Rust Core `WidgetDataState`, đẩy xuống UI layer để Render theo dạng Passive Client):
- `Scenario A (NeverLoaded)`: Lần đầu được cấp. Hiển thị Shimmer Skeleton tĩnh. Fetch xong Fade In Content 200ms.
- `Scenario B (Restoring)`: Sau khi bị Admin Revoke. Nếu cấp lại, hiển thị Restoring Label. Xong thì Slide Up Content từ dưới.
- `Scenario C (StaleServing - Scope Rotation):` Kế thừa Data cũ đang chạy, Fetch mới ngầm chạy. RENDER DATA CŨ KÈM XUẤT HIỆN 1 *Amber Dot* opacity 0.6 góc thẻ. Khi hoàn tất, silent replace những Row khác.

#### 4. Mesh Revocation Gossip Protocol (CRIT-03)

Revocation phải effective trước offline guard TTL. Nếu Admin revoke DataGrant trong lúc một nhóm field agents đang Mesh-only, OPA push channel không đủ để đảm bảo cấp độ nhất quán:

- Revocation signal phải được broadcast như một signed `CRDT_Event` vào `hot_dag.db` (không chỉ qua OPA push channel).
- `content_type: "governance/data_grant_revoked@v1"` — namespace chuẩn per §8.2.
- Mọi node khi serve DataGrant phải check local DAG: `SELECT 1 FROM crdt_events WHERE content_type = 'governance/data_grant_revoked@v1' AND payload->>'grant_id' = ? AND hlc > grant_issued_at`
- Gov-tier `max_offline_serve_duration = 0s` — biến thành policy "require-quorum-confirmation" thay vì binary "0s" để tương thích Offline Survival invariant.
- Cost: một SQLite query per DataGrant access — negligible.

---

## §3 — DATA MODEL & ENCRYPTION STATE

### 3.1 .tapp Bundle Objects

| Object | Type | Storage | Lifecycle | Security Constraint |
|---|---|---|---|---|
| `TappBundle` | `{manifest.json + logic.wasm + bundle.sig}` | Registry CDN / MDM server | Until revoked | Content-addressed: CAS_UUID = BLAKE3(bundle_bytes) |
| `PublisherKeyRegistration` | Ed25519 public key + KYC metadata | Registry DB | Until revoked | Registered with TeraChat CA |
| `BundleSignature` | `Ed25519_sign(Publisher_Key, SHA3-256(manifest || wasm))` | Embedded in TappBundle | Per version | Both publisher + CA sig required |
| `TeraChatCARootSig` | Ed25519 (Root CA, offline) | Registry CDN + Embedded | Per marketplace CA rotation | Cold storage; cannot be online |
| `EnterpriseBundleSig` | Ed25519 (Enterprise CA) | MDM server + device | Per internal version | Does not require TeraChat public CA |

### 3.2 Registry Objects

| Object | Type | Storage | Lifecycle | Security Constraint |
|---|---|---|---|---|
| `RegistryListing` | `{tapp_id, publisher_tier, cas_uuid, security_scan_result, last_updated}` | Registry DB | Until delisted | Read-only for external; writable by TeraChat ops |
| `TransparencyLogEntry` | `{event_type, tapp_id, timestamp, evidence_hash, merkle_proof}` | Append-only, Merkle-proofed | Permanent | Cannot delete; IT Admin can verify independently |
| `KillDirective` | `{tapp_id, reason, evidence_hash, ed25519_sig}` | Broadcast via OPA channel | Permanent record | Signed by TeraChat Root CA |
| `ITAdminApprovalEvent` | `{tapp_id, admin_id, org_id, approved_scope, timestamp, sig}` | Audit Log | Permanent | Ed25519 signed by Admin's DeviceIdentityKey |

### 3.3 Manifest Schema (Full)

```json
{
  "tapp_id": "acme-crm-integration-v2",
  "publisher": "Acme Corp",
  "publisher_public_key": "ed25519:...",
  "host_api_version": "1.3.0",
  "min_host_api_version": "1.0.0",
  "max_host_api_version": "2.0.0",

  "permissions": ["network.egress", "storage.persist"],

  "egress_schemas": [
    {
      "endpoint": "api.acme.com/crm",
      "method": "POST",
      "tls_pin": "sha256/XXXXXXX",
      "max_payload_bytes": 4096,
      "schema": {
        "type": "object",
        "properties": {
          "contact_ref": { "type": "string", "maxLength": 64 },
          "action": { "type": "string", "enum": ["lookup", "update"] }
        },
        "required": ["contact_ref", "action"],
        "additionalProperties": false
      }
    }
  ],

  "version_hash": "blake3:a3f2e1d4...",

  "background_tick_interval_s": 300,
  "app_local_storage_mb": 50,

  "event_bus_publish": ["crm.contact_updated"],
  "event_bus_subscribe": ["user.profile_changed"],

  "enterprise_only": false,
  "private_distribution": false
}
```

### 3.4 Publisher Trust Tiers

| Tier | Yêu cầu | Egress Privilege | Badge |
|---|---|---|---|
| **Unverified** | Ed25519 key đã đăng ký | HTTP GET only, < 1KB payload | 🔵 Community |
| **Verified** | KYC + Key + Security Review | File < 10MB, standard consent | ✅ Verified |
| **Enterprise** | SOC2/ISO27001 cert | Full file egress, custom consent | 🏢 Enterprise |
| **TeraChat Native** | First-party .tapp | Unrestricted (subject to OPA) | ⭐ Native |

---

## §4 — PROTOCOL & EXECUTION CONTRACT

### 4.1 .tapp Submit & Review Protocol

```text
[Publisher submits .tapp]
     │
Step 1: Automated Static Analysis
     ├── WASM bytecode: Abstract Interpretation
     │     ├── buffer overflow detection
     │     ├── forbidden syscall check (allowlist only)
     │     └── data accumulation pattern detection
     ├── Manifest: egress domain validation, schema completeness
     ├── LLVM IR: obfuscated string detection, unusual CFG
     ├── Dependency audit: third-party WASM imports
     └── WasmParity: run against wasm3 + wasmtime test vectors
     │
Step 2: Result evaluation
     ├── Any critical finding → Manual Security Review Queue
     ├── All clear → Marketplace CA sign bundle
     └── Rejected → Publisher notified with specific findings
     │
Step 3: Registry Listing
     ├── Bundle stored at CAS path (BLAKE3 UUID)
     ├── TransparencyLogEntry (publish event) appended
     └── Available for IT Admin discovery
```

### 4.2 IT Admin Approval & Deployment Protocol

```text
[IT Admin finds .tapp in Registry]
     │
View Security Report:
     ├── Egress domains declared
     ├── Permission scope
     ├── Publisher tier
     └── Automated scan results
     │
IT Admin: Approve for workspace (select target groups)
     │
[Rust Core Action:]
     ├── Compile Manifest → OPA Rego policy
     ├── Sign policy bundle with Enterprise CA (2-of-3: CISO + CTO + Legal)
     ├── Push OPA bundle to target devices
     └── Append ITAdminApprovalEvent to Audit Log
     │
Devices (≤ 60s for online):
     ├── Receive OPA policy update
     ├── Download .tapp bundle (verify CAS + Ed25519 sigs)
     └── .tapp available in user launcher
```

### 4.3 Revocation Protocol

```text
[IT Admin revokes .tapp]
     │
[Rust Core:]
     ├── OPA Policy push: {tapp_id: "REVOKED"}
     ├── Effective ≤ 60s (online devices)
     ├── At next online sync (offline devices)
     │
[On each device receiving policy:]
     ├── Rust Core: disable .tapp execution
     ├── Purge transient state (sled KV)
     ├── Revoke DelegationTokens of .tapp
     └── Audit_Log_Entry: PLUGIN_REVOKED {plugin_id, admin_id, timestamp, reason}
```

### 4.4 Enterprise Private Distribution (MDM/EMM) Protocol

1. Internal developer builds .tapp with `"private_distribution": true` in manifest.
2. Signs with Enterprise CA Key (separate chain from TeraChat Marketplace CA).
3. Uploads to corporate MDM server (Jamf Pro / Microsoft Intune / Kandji).
4. MDM policy pushes .tapp bundle to enrolled device group.
5. Rust Core: verify `Enterprise CA chain` → load allowed.
6. No visibility in TeraChat public Registry.
7. IT Admin controls deployment same as Registry .tapp.

### 4.5 Binary Transparency Protocol

```text
[TeraChat Root CA signs new binary / model / .tapp version]
     │
Append to Global_Update_Log (Append-Only CRDT Log)
     │
Gossip BLAKE3 hash to all peers via Mesh
     │
Client: before loading any module
     ├── Check hash against Global_Update_Log
     ├── Match → LOAD
     └── Mismatch → BLOCK + alert + admin notification
```

---

## §5 — STATE MACHINE

### 5.1 .tapp Listing State (Registry)

```text
[PENDING_REVIEW]
     │ automated + manual review pass
     ▼
[REGISTRY_LISTED]
     │ IT Admin approves for workspace
     ▼
[DEPLOYED (in org workspace)]
     │ policy violation detected        │ IT Admin revokes
     ▼                                  ▼
[SUSPENDED]                       [REVOKED]
     │ fix + re-review                  │ (permanent)
     ▼
[REGISTRY_LISTED]
```

### 5.2 .tapp Device Lifecycle State

```text
[PENDING_DOWNLOAD]
     │ BLAKE3 verify pass + sig verify pass
     ▼
[INSTALLED]
     │ User opens .tapp
     ▼
[RUNNING] ← full lifecycle as per TERA-RUNTIME §5.1
     │
     │ Revoke policy received
     ▼
[TERMINATED]
     │ state purge complete
     ▼
[REMOVED]
```

### 5.3 Emergency Kill-Switch State

```text
[KILL_DIRECTIVE received]
     │ (all devices, targeted tapp_id)
     ▼
[TERMINATING] → ZeroizeOnDrop state
     │
     ▼
[KILLED] → Audit_Log written
     │
[IT_ADMIN_NOTIFIED] → full technical report
```

---

## §6 — API / IPC / EVENT BUS

### 6.1 Host Function ABI (for .tapp Developers)

```rust
// Crypto (offloaded to Rust Core — WASM không tự chạy crypto)
fn host_blake3_hash(data: *const u8, len: usize, out: *mut u8) -> i32;
fn host_ed25519_sign(key_id: u64, msg: *const u8, msg_len: usize, sig_out: *mut u8) -> i32;
fn host_aes256gcm_encrypt(
    key_id: u64, nonce: *const u8,
    pt: *const u8, pt_len: usize,
    ct_out: *mut u8
) -> i32;

// Network (via Egress_Outbox — không direct socket)
fn host_egress_write(endpoint_id: u64, payload: *const u8, len: usize) -> i32;
// Returns: 0=OK, 1=QuotaExceeded, 2=SchemaViolation, 3=OPADeny, 4=MeshRestricted

// Storage (scoped per .tapp namespace — see TERA-RUNTIME)
fn host_storage_get(key: *const u8, key_len: usize, out: *mut u8, out_max: usize) -> i32;
fn host_storage_set(key: *const u8, key_len: usize, val: *const u8, val_len: usize) -> i32;

// App State (structured data — see TERA-RUNTIME)
fn host_app_state_query(sql_ptr: *const u8, sql_len: usize, out_ptr: *mut u8) -> i32;
fn host_app_state_write(patch_ptr: *const u8, patch_len: usize) -> i32;
```

### 6.2 ABI Versioning & Deprecation

- Breaking changes: **major version bump only**
- Minor version: additive-only (backward compatible)
- TeraChat supports **2 major versions simultaneously**
- Deprecation window: **12 months** from announce date
- `.tapp` with `host_api_version` outside supported range → **rejected at install**

### 6.3 Signals (Ecosystem Domain)

| Signal | Trigger | Consumer |
|---|---|---|
| `TappInstalled(tapp_id, org_id)` | Successful install on device | TERA-GOV (audit log) |
| `TappRevoked(tapp_id, reason)` | Revocation policy received | TERA-RUNTIME (terminate) + TERA-GOV |
| `KillDirectiveReceived(tapp_id)` | Emergency kill | TERA-RUNTIME (force terminate) |
| `RegistryListingUpdated(tapp_id, version)` | New version available | IT Admin Console notification |
| `SecurityScanFailed(tapp_id, finding)` | Automated scan finding | Developer notified; TERA-GOV audit |

---

## §7 — PLATFORM MATRIX & CONSTRAINTS

| Feature | 📱 iOS | 📱 Android | 📱 HarmonyOS | 💻🖥️ Desktop |
|---|---|---|---|---|
| WASM format | `.wasm` (wasm3) | `.wasm` (wasmtime) | `.waot` bundle | `.wasm` (wasmtime) |
| Registry source | Enterprise Registry / MDM | Enterprise Registry / MDM | Enterprise Registry / AppGallery | Enterprise Registry / MDM |
| Enterprise MDM | Jamf / Kandji | Intune / MDM | Huawei MDM | Jamf / Intune |
| Kill-switch response | < 60s (online) | < 60s (online) | < 60s (online) | < 60s (online) |
| Private distribution | ✅ Enterprise CA | ✅ Enterprise CA | ✅ HarmonyOS signed | ✅ Enterprise CA |
| Static analysis gate | Must pass wasm3 CI | Must pass wasmtime CI | Must pass .waot CI | Must pass wasmtime CI |

**HarmonyOS Special Case:**
- AppGallery requires `.waot` bundle (WebAssembly Optimized Translation).
- Runtime: JIT-with-AOT-fallback.
- WasmParity CI must validate both JIT and AOT execution paths.

> ⚠️ **HMS Gov/Military Tier — Fundamental Incompatibility (XPLAT-03):**  
> HMS Push Service (HPush) không hỗ trợ `data-only` message type như FCM. Mọi push notification trên HarmonyOS phải có visible notification body, nghĩa là HMS sẽ thấy notification metadata — vi phạm Sealed Sender principle.  
> **Gov/Military tier trên Huawei device là impossible** theo thiết kế hiện tại nếu cần Zero-Knowledge notification. Feature matrix cho Gov/Military không nên list Huawei cho đến khi HMS gải quyết được data-only push. Phiên bản hiện tại: Huawei chỉ hỗ trợ tới Enterprise tier với polling mode (CRL ≤ 4h).

---

## §8 — NON-FUNCTIONAL REQUIREMENTS (NFR)

| Requirement | Target | Notes |
|---|---|---|
| OPA policy push (revoke/kill) | < 60s for online devices | Hard SLA |
| Registry search response | < 500ms | CDN-backed |
| .tapp bundle integrity verify | < 2s on install | BLAKE3 + Ed25519 verify |
| Static analysis scan | < 5 min per submission | Automated pipeline |
| Transparency Log query | < 200ms | IT Admin audit interface |
| Emergency Kill-switch propagation | < 60s p90 | Online fleet |
| WasmParity CI gate | 100% identical output | Both engines |
| MDM push delivery | MDM SLA (vendor-dependent) | Jamf ~2min, Intune ~5min |

---

## §9 — SECURITY & THREAT MODEL

| Attack | Vector | Mitigation |
|---|---|---|
| Malicious .tapp in Registry | Submit compromised plugin | Static analysis + manual review + Emergency Kill-switch |
| Publisher key compromise | Attacker steals Ed25519 private key | Key revocation; all bundles with that key become invalid; Transparency Log records revocation |
| Registry CDN tampering | Serve modified .tapp bundle | BLAKE3 CAS + Ed25519 sig verify on device before load |
| Bypass IT Admin policy | End user installs directly | No end-user install pathway; OPA enforced in Rust Core |
| Silent update (same CAS UUID) | Replace bundle at same hash | CAS = content-addressed; same hash = same bytes; different content = different hash |
| Dependency supply chain attack | Third-party WASM import malicious | Dependency audit in static analysis; all imports must be in allowlist |
| ObfuscatedString egress | .tapp builds domain at runtime to bypass manifest | LLVM IR analysis for string construction patterns; failed → manual review |
| Kill-switch bypass (offline) | Device disconnected; continues running blocked .tapp | Kill directive cached; enforced on reconnect; OPA local cache also checked |

---

## §10 — FAILURE MODEL & RECOVERY

| Failure | Detection | Recovery |
|---|---|---|
| TeraChat Marketplace CA key compromised | CA key theft detected | Root CA issues new Marketplace CA; all existing bundles re-signed; old CA CRL published |
| Registry CDN unavailable | BLAKE3 verify cannot fetch bundle | Devices use locally cached version; no new installs until CDN recovers |
| Static analysis pipeline crash | Submission stuck in PENDING_REVIEW | Manual review queue fallback; alert platform team |
| OPA policy push failed (device offline) | Device misses push | Policy applied on next online sync; kill/revoke directives cached |
| Emergency Kill-switch blocked by firewall | Kill OPA bundle cannot reach device | Admin manually wipes .tapp via MDM remove command |
| MDM server unreachable | Private distribution stalled | Devices retain last installed .tapp; no new MDM push until restored |
| Publisher submits broken ABI version | `host_api_version` out of range | Rejected at install; TransparencyLogEntry records rejection |
| WasmParity divergence in new release | CI gate fails | Block marketplace listing; fix before release |


## §8 — ARCHITECTURAL INVARIANTS & AUDIT RESOLUTIONS (ECOSYSTEM & TRUST)

### 8.1 DataGrant Latency vs. Workspace Aggregation
**Constraint:** Serial DataGrant query resolution (50ms × 8 = 400ms) during cold launches violates UI responsiveness SLAs.
**Resolution:** DataGrant queries must be pre-fetched and cached continuously into a dedicated `sled` namespace via a background orchestrator. The `WorkspaceSubscription` host function registers data intent, while Rust Core handles the lifecycle and pushes delta signals asynchronously.

### 8.2 DataGrant Row Filter Expression Language
**Constraint:** Executing full OPA Rego evaluation iteratively on individual rows causes unacceptable runtime overhead.
**Resolution:** Row filter capabilities are constrained to a specialized **Predicate DSL** (e.g., `{"field": "dept", "op": "eq", "value": "HR"}`). Rust Core evaluates these natively against `cold_state.db` prior to WASM crossing.

### 8.3 DataGrant Revocation & Cache Invalidation
**Constraint:** Authorization staleness is unacceptable post-revocation, requiring zero-tolerance immediate enforcement.
**Resolution:** DataGrant tokens embed a sequentially-increasing `generation` counter controlled by Rust Core. Revocation triggers an immediate `CoreSignal::DataGrantRevoked` over the synchronous Security priority channel, invalidating all prefixed `sled` entries instantly and halting active fetching. Staleness ends definitively once the OPA policy arrives at the device.

### 8.4 DataGrant Activation & Background Pre-fetch
**Constraint:** Naive activation parallelization triggers extreme resource contention and race conditions with immediate revocations.
**Resolution:** `CoreSignal::DataGrantActivated` registers a pre-fetch intent dynamically prioritized (`Foreground`, `Background`, `Lazy`). A Tokio-based `PrefetchOrchestrator` queues requests sequentially per a constrained memory budget (e.g., 4MB iOS, 32MB Desktop). Revocation explicitly checks generation markers and cleanly aborts in-progress fetches, ensuring no partially-tainted data hits the cache.

### 8.5 Strict Engineering Guardrails (Ecosystem Boundaries)
- **Rule 2 (Tapp Ownership Invariant):** `.tapp` containers exclusively own their internal databases. Tapps are forbidden from direct cross-namespace reads. To collaborate, tapps must message each other explicitly via `host_event_publish`, enforcing total isolation and accountability.

### 8.6 Mesh Revocation Gossip — Zero-Trust Enforcement (CRIT-03)
**Constraint:** Nếu Admin revoke DataGrant trong lúc field agents đang Mesh-only, không có cơ chế nào đảm bảo revocation signal đến *tất cả* nodes theo thứ tự nhất quán. Node A có thể nhận revocation trước Node B 30 giây — vi phạm Zero-Trust invariant (ISO 27001 A.5.18).
**Resolution:** Revocation signal phải được broadcast như signed CRDT_Event với `content_type: "governance/data_grant_revoked@v1"` vào `hot_dag.db` (gossip qua Mesh), không chỉ qua OPA push channel. Mech check: mọi DataGrant serve phải query local DAG trước khi serve data. Gov-tier offline policy: "require-quorum-confirmation" thay vì binary "0s".
