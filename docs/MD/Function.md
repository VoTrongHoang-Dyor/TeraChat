```yaml
# DOCUMENT IDENTITY
id:       "TERA-FUNC"
title:    "TeraChat — Function & Capability Blueprint"
version:  "0.3.0"
audience: "Product Manager, CEO, Sales Engineer, Customer Success, Developer, Investor"
purpose:  "Strategic product reference: full functional architecture, system capabilities,
           component interactions, and core value propositions of TeraChat.
           Includes TeraChat App Suite (.tapp ecosystem) covering Work OS, HR, Finance, CRM, and internal communications."

ai_routing_hint: |
  "Open this file to understand what TeraChat can do and why — covering role-based
   permissions, business flows, AI integration, cross-org Federation, the .tapp
   plugin ecosystem, and the full TeraChat App Suite (Work, HR, Finance, CRM, Comms).
   This is the Product-level source of truth."
```

---

> *"Trong thế giới mà dữ liệu là quyền lực, ai kiểm soát khóa mã hóa — kẻ đó làm chủ cuộc chơi.
> TeraChat trao lại chìa khóa đó về tay doanh nghiệp."*
>
> — CEO, TeraChat

---

# TeraChat — Function & Capability Blueprint

**TERA-FUNC v0.3.0 · Q2 2026 · Internal Document**

---

## Tóm Tắt Điều Hành

TeraChat không phải là ứng dụng nhắn tin. Đây là **Hệ điều hành Công việc Chủ quyền** (Sovereign Work OS) — nền tảng cộng tác doanh nghiệp được bảo vệ bằng toán học, nơi không có bất kỳ máy chủ, nhà cung cấp hay đối thủ nào có thể đọc được dữ liệu mà TeraChat truyền tải.

| Giá Trị Cốt Lõi | Cơ Chế Thực Thi |
|---|---|
| Bảo mật bằng Toán học | Zero-Knowledge: server chỉ định tuyến ciphertext, không bao giờ plaintext |
| Sinh tồn Offline | BLE 5.0 + Wi-Fi Direct Mesh — hoạt động khi không có Internet |
| Kiểm soát Doanh nghiệp | Admin-owned keys, HSM-backed KMS, OPA Zero-Trust enforcement |
| Nền tảng Mở rộng | .tapp WASM sandbox Marketplace với chữ ký mật mã học |
| Kết nối Liên tổ chức | Federation Bridge mTLS + Sealed Sender Protocol |
| AI Ưu tiên Riêng tư | SLM trên thiết bị + PII redaction trước mọi cuộc gọi LLM bên ngoài |
| Work OS Toàn diện | TeraChat App Suite: 20+ ứng dụng doanh nghiệp tích hợp trong một nền tảng |

**Biểu tượng platform:**

- 📱 iOS / Android / Huawei (Mobile)
- 💻 Laptop / macOS
- 🖥️ Desktop / Windows / Linux
- 🗄️ Bare-metal Server
- ☁️ VPS Cluster / Cloud

**Chế độ hoạt động:**

- ☀️ **Online Mode** — Giao diện Sáng, kết nối Cloud/Server
- 🌑 **Mesh Mode** — Giao diện Tối, hoạt động offline P2P

---

## Module 1 — Lõi Mật Mã & Quản lý Khóa

> **Nguyên tắc bất biến:** Khóa riêng tư không bao giờ rời khỏi phần cứng. Plaintext không bao giờ tồn tại ngoài phạm vi. Không có ngoại lệ.

### 1.1 Hệ thống Quản lý Khóa Phân tầng (HKMS)

```
[Master Key] — HSM FIPS 140-3 L3 / Secure Enclave / TPM 2.0 (không bao giờ xuất ra ngoài)
      │
      └──► [Company_Key] — mã hóa toàn bộ dữ liệu workspace
                 │
                 ├──► [Epoch_Key] — khóa phiên MLS, ZeroizeOnDrop khi rotation
                 ├──► [Push_Key] — chuỗi HKDF một chiều, cô lập khỏi TreeKEM
                 └──► [Escrow_Key] — Shamir SSS M-of-N, tồn tại trong RAM < 100ms
```

- **Shamir Secret Sharing (mặc định 3-of-5):** `Enterprise_Escrow_Key` chia thành 5 mảnh cho 5 C-Level. Cần đúng 3 mảnh mới tái tạo được.
- **ZeroizeOnDrop:** Mọi struct chứa plaintext tự xóa bộ nhớ (ghi đè 0x00) khi hết phạm vi.
- **Dead Man Switch:** Bộ đếm monotonic TPM 2.0 giới hạn offline TTL (Consumer 24h / Enterprise configurable / GovMilitary 30 ngày).

### 1.2 Mã hóa Nhóm MLS RFC 9420

- 📱💻🖥️ Hỗ trợ tới **10.000 thành viên** trong một nhóm E2EE.
- **TreeKEM:** phân phối khóa O(log n) — hiệu quả băng thông ngay cả nhóm lớn.
- **Epoch Rotation:** kích hoạt khi thành viên rời nhóm, Admin yêu cầu, hoặc theo lịch 24h.
- **Batched Update_Path:** nhóm ≤1.000: cửa sổ 60s; ≤5.000: 300s — ngăn bão mạng.
- **Post-Quantum Hybrid (ML-KEM-768 + X25519):** tuân thủ CNSA 2.0 / NIST FIPS 203.

### 1.3 Phần cứng Bảo vệ Khóa

| Platform | Nơi lưu trữ Khóa | Cổng Xác thực |
|---|---|---|
| 📱 iOS / macOS | Secure Enclave Processor (SEP) | Face ID / Touch ID — bắt buộc |
| 📱 Android | StrongBox Keymaster HAL | BiometricPrompt — bắt buộc |
| 📱 Huawei | TrustZone TEE via HMS | HMS Biometric — bắt buộc |
| 💻🖥️ Windows | TPM 2.0 — CNG Platform Provider | Windows Hello |
| 🖥️ Linux | TPM 2.0 — tpm2-pkcs11 | PIN — bắt buộc |
| 🗄️ Bare-metal | HSM FIPS 140-3 L3 (PKCS#11) | Physical presence + Shamir quorum |

### 1.4 Cryptographic Self-Destruct

- 📱💻 **PIN thất bại 5 lần:** Crypto-Shredding toàn bộ DB nội địa + factory reset. Bộ đếm HMAC-xác thực chống giả mạo.
- 📱💻 **Remote Wipe:** Xóa `DeviceIdentityKey` khỏi phần cứng, drop mọi bảng DB, xóa WASM sandbox storage. Không thể bị người dùng ngắt.
- 📱💻 **Cryptographic Erasure (GDPR/PDPA):** Hết TTL → Crypto-Shredding khóa giải mã; ciphertext vĩnh viễn không đọc được.

---

## Module 2 — Nhắn tin, Cộng tác & UX

### 2.1 Engine Nhắn tin Cốt lõi

- 📱💻🖥️ **E2EE đa phương thức:** Văn bản, tệp tin, thoại, video — mã hóa và giải mã tại thiết bị.
- 📱💻 **CRDT DAG:** Nhật ký sự kiện append-only đảm bảo nhất quán trên mọi thiết bị không cần điều phối trung tâm.
- 📱💻 **Hybrid Logical Clock (HLC):** Sắp xếp toàn phần sự kiện phân tán — không cần timestamp authority phía server.
- 📱💻 **Full-text Search Zero-Knowledge:** SQLite FTS5 tại thiết bị — server không bao giờ thấy nội dung tìm kiếm.
- 📱💻 **Message TTL:** Hết hạn → Crypto-Shredding tự động.
- 📱💻 **TeraLink Multipath:** Gửi song song qua 4G + Wi-Fi LAN + BLE Mesh — 0ms latency khi roaming.

### 2.2 Quản lý Tin nhắn Nâng cao

**Nhắn tin theo Luồng (Threads):**
Mỗi thread là một nhánh (branch) trong CRDT Causal Graph. Quan hệ cha-con giữa tin nhắn được biểu diễn qua `parent_id` trong `CRDT_Event`, đảm bảo tính nhân quả không phá vỡ kể cả khi offline. Desktop merge theo O(N log N); Mobile nhận Materialized Snapshot.

**Gắn thẻ & Đánh nhãn (Tags/Labels):**
Tagging được lưu dưới dạng metadata CRDT_Event trong SQLite FTS5 nội bộ thiết bị. Query siêu tốc O(1) — không cần round-trip server. Tag được index tự động và hỗ trợ filter nâng cao cho tìm kiếm.

**Ghim tin nhắn (Pin):**
Bảng `pinned_messages` trong `cold_state.db` với cơ chế pre-fetch Tier 1 — nội dung ghim được tải sẵn về local. Admin có thể ghim toàn channel; User ghim trong scope cá nhân. Giới hạn 50 pin/channel để kiểm soát storage.

**Reactions:**
Triển khai dưới dạng lightweight CRDT_Event với `content_type: Reaction`. Server không bao giờ biết nội dung reaction — chỉ thấy encrypted blob. Aggregation xảy ra tại client khi render.

**Chỉnh sửa & Thu hồi tin nhắn:**
Chỉnh sửa → PROPOSE_CHANGE via Shadow Branch; tin nhắn gốc giữ nguyên cho đến khi author approve. Thu hồi → Tombstone_Stub (không xóa vật lý), lưu trong `cold_state.db` để giữ tính nhân quả DAG. Zero-Byte transformation ngăn Replay Attack.

**Hẹn giờ gửi (Scheduled Send):**
Tin nhắn được mã hóa và đẩy vào `Egress_Outbox` (SQLite WAL độc lập) với trường `scheduled_at: HLC_Timestamp`. iOS BGProcessingTask / Android WorkManager xử lý dispatch khi đến giờ — không cần app foreground.

### 2.3 Thoại & Video (WebRTC)

- 📱💻 **HD Voice & Video** qua WebRTC DTLS-SRTP, E2EE đầu cuối.
- 💻 **ICE Pool Pre-warming:** Kết nối thiết lập trước khi người dùng nhấn "Gọi" — không có màn hình chờ.
- 💻 **Failover tự động < 3s:** P2P Direct → Internal TURN relay. Badge "P2P (Direct)" hoặc "Relayed (E2EE)".
- 📱 **CallKit Integration (iOS):** Dead Man Switch lockout hoãn cho đến khi cuộc gọi kết thúc.
- 📱💻 **Chia sẻ màn hình:** Hỗ trợ trong vùng an toàn (Zone 1); DLP policy tự động làm mờ nếu phát hiện nội dung nhạy cảm.
- 📱💻 **Gọi nhóm:** Tối đa 100 người đồng thời với SFU (Selective Forwarding Unit) tích hợp trong HA TURN cluster.
- 📱💻 **Adaptive Codec:** Online → Opus 128kbps Stereo; Mesh → AMR-NB 4.75kbps; BLE Emergency → Whisper local transcription → text only.

### 2.4 Tương tác Cộng đồng & Thu thập Ý kiến

**Bình chọn (Polls):**
Poll là `.tapp` siêu nhẹ (≤ 50KB WASM), phân phối an toàn qua E2EE channel. Kết quả tổng hợp tại client — server không biết ai vote gì. Hỗ trợ anonymous poll qua ZKP-based attribute routing.

**Kênh thông báo một chiều (Broadcast Channel):**
Thực thi qua OPA Policy với `allow_reply: false` và `allowed_roles: [ADMIN, MODERATOR]`. Members chỉ có thể react, không reply. Phân phối qua CRDT fanout như tin nhắn thường — fully E2EE.

### 2.5 Tài liệu & Smart Approval

- 📱💻 **Smart Document với RBAC:** Viewer → Commenter → Editor (Shadow Branch).
- 💻🖥️ **Smart Approval Workflow:** Ký số với sinh trắc học (Ed25519), phân giai đoạn phê duyệt. Non-repudiation: mọi quyết định Accept/Reject được ký bằng `DeviceIdentityKey` — không thể chối bỏ.
- 📱💻 **Conflict Resolution:** Merge/Discard khi sửa song song — không bao giờ silent LWW trên CONTRACT/POLICY/APPROVAL.
- 📱💻 **TeraVault (Virtual File System):** Kéo thả file từ chat vào VFS, không nhân bản. Preview qua Zero-Byte Stub (~5KB). Hỗ trợ "Make Available Offline" tải trước dữ liệu.
- 📱💻 **Collaborative Editing:** Shadow DAG Protocol — thay đổi lưu thành nhánh PROPOSE_CHANGE chờ duyệt; không ghi đè dữ liệu gốc cho đến khi author confirm.

**FUNC-12: FCP Trust Boundary Declaration**

| Chế độ | Server | AI Agent | Cam kết |
|---|---|---|---|
| **Default (Zero-Knowledge)** | Blind Relay | Nhận masked context | Server không đọc nội dung |
| **FCP Mode (Admin-Opt-In)** | Vẫn Blind Relay | Nhận plaintext context | TLS bảo vệ transit; AI endpoint thấy plaintext |

FCP Mode yêu cầu: YubiKey + typed consent + audit log signed. Phù hợp khi AI endpoint là on-premise model do doanh nghiệp tự host.

**FUNC-13: Burner Agent Cross-Org Collaboration Flow**

```text
[Admin Org A]
    │ Tạo FederationInviteToken (Signed JWT, scope: reconcile_invoices)
    ▼
[Admin Org B] nhận Token → xác nhận scope → counter-sign
    │
    ▼ [BurnerAgent.spawn() — cả hai Admin ký bằng YubiKey]
    │
[Burner Agent — Ephemeral sandbox, TTL=60min]
    │ Đọc dữ liệu trong org_a_scope ∩ org_b_scope
    │ Xử lý task
    │ Emit kết quả vào MLS Group E2EE
    ▼
[Task hoàn thành / TTL hết]
    │ ZeroizeOnDrop soul.db
    │ Generate BurnerTerminationProof (cả hai CA ký)
    ▼
[Admin Org A + B nhận TerminationProof] → "Tác tử đã tự hủy. Xem bằng chứng mật mã."
```

**FUNC-14: Swarm Workspace User Flow**

```text
[User] tạo Group Chat → bấm [+ Add Agent]
    │ Consent Modal: "Agent này sẽ đọc: [scope list]"
    │ User approve bằng Biometric
    ▼
[Agent join MLS Group như member — Agent_DID hiển thị trong member list]
    │ User @mention Agent Orchestrator: "Phân tích Q1 report và tóm tắt top 5 risks"
    ▼
[Orchestrator phân công Worker Agents]
    │ Worker 1: Data extraction
    │ Worker 2: Risk classification
    │ Worker 3: Vietnamese summary
    ▼
[Results stream về Group Chat qua Timeline View]
    │ Items cần approval → Supervisor nhận Biometric prompt
    ▼
[User bấm [Remove Agent] → MLS remove_member → Epoch rotation]
    → Agent không còn access kênh
```

**FUNC-15: Causal Smart Approval — Agent-Backed Decision Flow**

```text
[Agent Worker phân tích request]
    ↓ PROPOSE action (ghi DecisionNode vào DAG)
    ↓
[Lõi Rust check: action có trong "human-required" list?]
    ├── Không → Agent EXECUTE trực tiếp
    └── Có → Notification đến Supervisor
              "Agent X đề xuất: [Tóm tắt action]. Approve?"
              [Supervisor xác thực Biometric → ký HumanApprovalRecord]
              ↓ EXECUTE (ghi DecisionNode(type=Execute))
              ↓ Non-Repudiation Audit → Append-Only Log
```

### 2.6 Hệ thống UX Glassmorphism

| Trạng thái | Chế độ UI | Màu Viền | Indicator |
|---|---|---|---|
| ☀️ Online | Light Frosted Glass | Xanh lam #24A1DE | "E2EE Active · Key Epoch N" |
| 🌑 Mesh Fallback | Dark Navy #0F172A | Radar Pulse | "📡 Survival Mesh Active" |
| ⚠️ Warning | Amber Glow | #F59E0B | Warning banner |
| 🔴 Containment | Red Border 4px | #EF4444 | FCP pulse overlay |
| ☠️ Kill-Switch | Blood-Red Frosted | Đỏ máu | Shatter animation + Shield |

---

## Module 3 — Survival Mesh Network

> **Cam kết:** Khi Internet sụp đổ, TeraChat không suy giảm. Nó kích hoạt mạng P2P tự tổ chức, bảo mật mật mã học, không cần server trung tâm.

### 3.1 Mô hình Kết nối 4 Tầng

| Tầng | Transport | Throughput | Kích hoạt khi |
|---|---|---|---|
| ☀️ Tier 1 — Online | QUIC / gRPC / WSS | > 100 Mbps | Bình thường |
| 🌑 Tier 2 — Wi-Fi Mesh | AWDL / Wi-Fi Direct | 250–500 MB/s | LAN, không Internet |
| 🌑 Tier 3 — BLE Control | BLE 5.0 (< 15 mW) | ~50ms latency | Control plane |
| 🌑 Tier 4 — BLE Emergency | BLE Long Range | Text-only | EMDP active |

### 3.2 Vai trò Mesh Node

| Vai trò | Platform | Lưu trữ | Trách nhiệm |
|---|---|---|---|
| **Super Node** | 💻🖥️ Desktop (AC) | 500MB–1GB / 48–72h | Backbone, DAG merge dictator |
| **Relay Node** | 📱 Android (RAM ≥ 3GB) | 100MB / 24h | Intermediate relay |
| **Tactical Relay (EMDP)** | 📱 iOS (emergency) | 1MB / 60 phút | Text-only CRDT buffer |
| **Leaf Node** | 📱 iOS (luôn luôn) | 50MB receive-only | Nhận tin, không định tuyến |
| **Border Node** | Bất kỳ (Internet + BLE) | N/A | Bridge TCP/IP ↔ BLE Mesh |

> **Quy tắc kiến trúc bất biến:** `iOS election_weight = 0` — iOS không bao giờ là Merge Dictator.

### 3.3 Emergency Mobile Dictator Protocol (EMDP)

Kích hoạt khi: không có Desktop + không có Internet + ≥ 2 iOS pin > 20%.

- 📱 Text-only Store-and-Forward. Không merge DAG, không MLS Epoch rotation.
- 📱 Key Escrow: Desktop chuyển session key sang iOS qua ECIES/Curve25519 trước khi offline.
- 📱 TTL 60 phút. Extension ở phút 50: chuyển giao sang peer pin cao hơn.

---

## Module 4 — AI Privacy Shield

> **Cam kết:** AI worker không bao giờ trực tiếp truy cập database tin nhắn. PII luôn được redact trước bất kỳ cuộc gọi ra ngoài.

### 4.1 Pipeline AI Cô lập

```
User Prompt
    ↓ (bắt buộc)
Micro-NER PII Detection (ONNX < 1MB trên thiết bị)
    → Phát hiện: Tên, SĐT, Email, CMND, Tài khoản ngân hàng, Địa chỉ
    ↓
SanitizedPrompt (newtype — không thể tạo nếu không qua redaction)
    ↓
SessionVault { [MASK_01] → real@email.com } — ZeroizeOnDrop < 100ms
    ↓
AI Worker Process (cô lập OS — crash không ảnh hưởng Rust Core)
    → On-device SLM (CoreML / ONNX) hoặc External LLM (nếu Admin cho phép)
    ↓
Response Vec<ASTNode> (HTML/Markdown raw bị reject bởi AST Sanitizer)
    ↓
SessionVault.restore_and_drop() — alias map zeroized
```

### 4.2 Tích hợp AI với TeraChat App Suite

- 📱💻 **AI trong Base Workflow:** Tự động đề xuất bước tiếp theo trong quy trình; phân loại request; gợi ý assignee dựa trên lịch sử.
- 📱💻 **AI trong Base HRM:** Phân tích CV tự động (NER local); ranking ứng viên; không upload raw CV ra ngoài.
- 📱💻 **AI trong Base Finance:** Phát hiện anomaly chi tiêu; gợi ý category cho transaction; tất cả chạy local-first.
- 📱💻 **AI trong Base CRM:** Tóm tắt lịch sử khách hàng; gợi ý follow-up; sentiment analysis trên E2EE channel.

### 4.3 AI Runtime

| Platform | Runtime | Model tối đa | Ghi chú |
|---|---|---|---|
| 📱 iOS | CoreML (.mlmodelc) | 74MB / 39MB | W^X: không dynamic WASM AI |
| 📱 Android | ONNX Runtime | 39MB | HiAI fallback Huawei |
| 📱 Huawei | HiAI / ONNX | 39MB | AOT bundle only |
| 💻 macOS | CoreML / ONNX | 74MB | Isolated XPC Worker |
| 🖥️ Windows/Linux | ONNX Runtime | 74MB | CPU; GPU optional |

---

## Module 5 — Hệ sinh thái Plugin (.tapp)

> **Lập trường:** Không có .tapp nào được phép yêu cầu exception khỏi Sandbox. Mọi capability khai báo tường minh trong manifest, OPA Policy kiểm soát. Không có ngoại lệ.

### 5.1 Vòng đời .tapp

```
[Submit] → [WASM Static Scan] → [Manifest Audit] → [LLVM IR Heuristics]
    ↓
[Ed25519 Bundle Signing bởi Marketplace CA]
    ↓
[Client: BLAKE3 verify → Sandbox Launch]
    ↓
[Execute: OPA + Egress Circuit Breaker] ↔ [Suspend: AES-256-GCM Snapshot]
    ↓
[Terminate: RAM freed · Capability Tokens revoked · KV-Cache cleared]
```

### 5.2 Tapp Data Namespace (MARKETPLACE-12)

Mỗi .tapp được cấp một SQLite namespace riêng, encrypt bằng AES-256-GCM:

```json
{
  "tapp_id": "base-hrm",
  "data_schema": {
    "tables": [
      {
        "name": "employees",
        "columns": ["id UUID", "name TEXT", "department TEXT"],
        "max_rows": 50000,
        "encrypted": true
      }
    ]
  },
  "max_storage_mb": 256
}
```

Host function `host_db_query(sql, params)` — tapp không bao giờ thấy raw key. SQL injection không nguy hiểm vì isolated SQLite instance, không phải `hot_dag.db`.

### 5.3 Structured DataGrant (MARKETPLACE-13)

Column-level ACL mở rộng DelegationToken:

```rust
pub struct DataGrant {
    source_tapp:  TappId,
    table:        String,
    columns:      Vec<String>,   // column-level ACL
    row_filter:   Option<String>, // OPA policy expression
    operations:   Vec<DataOp>,   // Read | Aggregate | Subscribe
}
```

Ví dụ: Base Goal đọc `attendance.work_days` từ Base Checkin — nhưng KHÔNG đọc `salary` hay `personal_notes`. Admin approve một lần qua consent modal.

### 5.4 Inbound Webhook (MARKETPLACE-14)

```json
{
  "ingress_endpoints": [
    {
      "path": "/webhook/crm-update",
      "auth": "hmac-sha256",
      "secret_ref": "vault:crm_webhook_secret",
      "max_payload_bytes": 65536
    }
  ]
}
```

Rust Core expose per-tapp webhook endpoint, verify HMAC, deliver vào tapp như host event. Tapp không bao giờ expose raw port ra ngoài.

### 5.5 Host Function ABI Versioning

```json
{ "host_api_version": "1.3.0", "min_host_api_version": "1.0.0", "max_host_api_version": "2.0.0" }
```

Breaking changes chỉ trong major version. Support 2 major versions đồng thời (deprecation window 12 tháng).

---

## Module 6 — TeraChat App Suite

> TeraChat App Suite là tập hợp 20+ ứng dụng doanh nghiệp chạy trong .tapp WASM sandbox, tích hợp sâu với lõi E2EE và CRDT của TeraChat. Mỗi app khai báo schema, permissions, và data grants trong manifest — không có app nào truy cập dữ liệu của app khác nếu không có DelegationToken được Admin duyệt.

---

### 6.1 Nhóm Quản lý Công việc & Vận hành

#### TeraChat Request — Quản lý Đề xuất & Phê duyệt

**Mục đích:** Số hóa toàn bộ luồng đề xuất từ nhân viên → phê duyệt đa cấp → thực thi.

**Kiến trúc:**

- Mỗi Request là một `CRDT_Event` với `content_type: RequestProposal`, lưu trong DAG append-only.
- Workflow state machine chạy trong Rust Core (không trong WASM) — đảm bảo tính nhân quả kể cả khi offline.
- Smart Approval với Ed25519 biometric signing — mọi quyết định approve/reject không thể chối bỏ.
- Offline-first: Request được tạo local, sync khi có mạng; approver nhận notification qua Push E2EE.

**Tính năng:**

- 📱💻 Tạo request với form tùy chỉnh (JSON Schema declared in manifest)
- 📱💻 Multi-level approval chain (serial hoặc parallel)
- 📱💻 Gắn file/tài liệu via Zero-Byte Stub (TeraVault)
- 📱💻 Real-time status tracking qua CoreSignal::StateChanged
- 📱💻 Reminder tự động khi request chờ duyệt quá SLA
- 💻🖥️ Dashboard tổng quan request theo phòng ban / trạng thái
- 💻🖥️ Export audit trail có chữ ký số (Ed25519) cho compliance

**Cross-tapp:** Nhận trigger từ TeraChat Workflow; đẩy kết quả vào TeraChat Finance khi request liên quan chi tiêu.

---

#### TeraChat Work — Quản lý Công việc & Dự án

**Mục đích:** Theo dõi task, project, sprint — tích hợp trực tiếp với kênh chat của team.

**Kiến trúc:**

- Task là CRDT_Event với `content_type: TaskItem` — merge conflict-free khi nhiều người cùng update.
- Gantt/Kanban state lưu trong Tapp Data Namespace (SQLite per-tapp, encrypted).
- @mention task trong chat tự động tạo cross-reference trong DAG.
- Agile board sync qua Structured DataGrant — PM đọc được progress không cần xem trực tiếp chat.

**Tính năng:**

- 📱💻 Tạo task, subtask, milestone với due date
- 📱💻 Kanban board / Gantt chart (render local, không cần server)
- 📱💻 Assign, tag, priority, label — tất cả FTS5-indexed
- 📱💻 Nhắc việc (Reminder) qua BGTask — không cần app foreground
- 📱💻 Progress tracking theo % + burndown tự động
- 📱💻 Link task vào tin nhắn chat hai chiều
- 💻🖥️ Timeline view với dependency visualization
- 💻🖥️ Workload view theo người / phòng ban

**Cross-tapp:** Đọc member list từ TeraChat HRM; đẩy completion event vào TeraChat Goal.

---

#### TeraChat Workflow — Quy trình Liên phòng ban

**Mục đích:** Số hóa và tự động hóa các quy trình nghiệp vụ phức tạp vượt ranh giới phòng ban.

**Kiến trúc:**

- Workflow engine là state machine trong Rust Core, không trong WASM — đảm bảo atomic transition.
- Mỗi step là một CRDT_Event; rollback = append Tombstone_Stub với `type: ROLLBACK`.
- Inbound Webhook (MARKETPLACE-14) nhận trigger từ hệ thống ngoài (ERP, CRM).
- Event bus nội bộ qua `host_workflow_emit_event` — không tốn egress quota.

**Tính năng:**

- 💻🖥️ Visual workflow builder (drag-drop, khai báo trong manifest JSON Schema)
- 📱💻 Auto-trigger theo condition (thời gian, event từ tapp khác, webhook ngoài)
- 📱💻 Human-in-the-loop step với Smart Approval biometric
- 📱💻 Conditional branching (if/else logic trong OPA Rego policy)
- 💻🖥️ Monitoring: workflow instance tracking, bottleneck detection
- 💻🖥️ Versioning: thay đổi workflow không ảnh hưởng instance đang chạy
- 📱💻 Notification tự động theo step qua Push E2EE

**Cross-tapp:** Gọi TeraChat Request, TeraChat Finance, TeraChat HRM như action trong workflow.

---

#### TeraChat Service — Quản lý Dịch vụ Nội bộ

**Mục đích:** Catalog các dịch vụ IT/HR/Hành chính nội bộ; nhân viên tự phục vụ qua chat.

**Kiến trúc:**

- Service catalog lưu trong Tapp Data Namespace; query qua FTS5.
- Request dịch vụ chạy qua TeraChat Request (tích hợp native).
- SLA tracking bằng HLC timestamp — đảm bảo đúng thứ tự kể cả khi offline.

**Tính năng:**

- 📱💻 Catalog dịch vụ tìm kiếm được (IT support, văn phòng phẩm, đặt xe...)
- 📱💻 Self-service request với form động
- 📱💻 SLA countdown và escalation tự động
- 💻🖥️ Admin: quản lý catalog, queue, agent assignment
- 💻🖥️ Báo cáo SLA, thống kê ticket

---

### 6.2 Nhóm Quản trị Nhân sự (HR)

#### TeraChat E-Hiring — Tuyển dụng

**Mục đích:** Quản trị toàn bộ vòng đời tuyển dụng từ JD → CV screening → interview → offer.

**Kiến trúc:**

- CV upload qua `FileTransfer` capability riêng (tách khỏi API egress limit) — tối đa 50MB/file.
- AI local NER scan CV để extract thông tin (tên, kinh nghiệm, kỹ năng) — không upload raw CV ra ngoài.
- Interview scheduling sync với TeraChat Meeting qua DataGrant (read calendar availability).
- Offer letter ký số qua Smart Approval Ed25519.

**Tính năng:**

- 📱💻 Đăng JD, quản lý pipeline ứng viên (Kanban: Applied → Screening → Interview → Offer → Hired/Rejected)
- 📱💻 AI screening CV: extract và rank tự động (local ONNX, không cloud)
- 📱💻 Lịch phỏng vấn: booking tích hợp TeraChat Meeting
- 📱💻 Scorecard phỏng vấn với cấu trúc tùy chỉnh
- 📱💻 Offer letter E2EE: ký số sinh trắc học, không thể chối bỏ
- 💻🖥️ Dashboard pipeline theo JD, phòng ban, recruiter
- 💻🖥️ Export báo cáo tuyển dụng có audit trail

**Cross-tapp:** Khi Hired → tự động trigger onboarding flow trong TeraChat Workflow; tạo profile trong TeraChat HRM.

---

#### TeraChat HRM — Hồ sơ Nhân sự

**Mục đích:** Lưu trữ và quản lý thông tin nhân viên, cơ cấu tổ chức, hợp đồng lao động.

**Kiến trúc:**

- Employee record lưu trong Tapp Data Namespace với schema khai báo (max_rows: 50,000).
- SCIM 2.0 sync với Azure AD / Google Workspace — offboarding tự động < 30s.
- Document (hợp đồng, bằng cấp) lưu qua Zero-Byte Stub trong TeraVault — server không biết nội dung.
- Access control column-level: HR xem lương; Manager chỉ xem tên/phòng ban/role.

**Tính năng:**

- 📱💻 Profile nhân viên: thông tin cá nhân, phòng ban, vai trò, cấu trúc báo cáo
- 📱💻 Org chart tự động từ SCIM sync
- 💻🖥️ Quản lý hợp đồng lao động (upload, versioning, ký số)
- 💻🖥️ Onboarding/Offboarding checklist tích hợp TeraChat Workflow
- 💻🖥️ Lịch sử nghỉ phép, phúc lợi
- 💻🖥️ Bulk import/export qua FileTransfer capability

**Cross-tapp:** DataGrant xuất `employee_id`, `department`, `role` cho TeraChat Checkin, TeraChat Goal, TeraChat Finance.

---

#### TeraChat Checkin & TeraChat Schedule — Chấm công & Xếp ca

**Mục đích:** Chấm công điện tử, xếp ca làm việc, tính toán ngày công — tất cả encrypted.

**Kiến trúc:**

- Checkin record là CRDT_Event với timestamp HLC — đảm bảo thứ tự đúng kể cả offline.
- BLE Checkin: scan BLE beacon từ máy chấm công (capability `ble.peripheral_scan` với device UUID filter declared in manifest).
- Geofence checkin: OPA GeoHash policy — không lưu tọa độ chính xác, chỉ hash prefix.
- Ca làm việc lưu trong Tapp Data Namespace; conflict schedule resolved qua CRDT LWW với HLC tie-breaker.

**Tính năng:**

- 📱💻 Checkin/checkout: mobile tap, BLE beacon, QR code
- 📱💻 Geofence checkin (OPA policy, không GPS raw)
- 📱💻 Xếp ca: drag-drop, template ca, lặp lịch
- 📱💻 Đăng ký nghỉ phép / đổi ca tích hợp TeraChat Request
- 💻🖥️ Bảng tổng hợp ngày công, giờ tăng ca, ngày nghỉ
- 💻🖥️ Export bảng công cho tính lương (kết nối TeraChat Finance)
- 💻🖥️ Cảnh báo thiếu người theo ca tự động

**Cross-tapp:** Xuất `work_days`, `overtime_hours` cho TeraChat Finance (lương); xuất `attendance_rate` cho TeraChat Goal (KPI).

---

#### TeraChat Goal — OKR / KPI

**Mục đích:** Thiết lập, theo dõi và đánh giá mục tiêu cá nhân và phòng ban.

**Kiến trúc:**

- Goal là CRDT_Event có thể nested (Company OKR → Dept OKR → Individual KR).
- Progress auto-update qua DataGrant từ TeraChat Checkin (attendance), TeraChat Work (task completion), TeraChat Finance (revenue).
- Review cycle (quarterly) trigger qua TeraChat Workflow.

**Tính năng:**

- 📱💻 Tạo OKR/KPI theo cấp: công ty → phòng ban → cá nhân
- 📱💻 Link Key Result với task trong TeraChat Work
- 📱💻 Auto-update progress từ data nguồn (Checkin, Work, Finance)
- 📱💻 Check-in định kỳ: comment, confidence level, blockers
- 💻🖥️ Dashboard alignment: cascade OKR visualization
- 💻🖥️ Review & rating cycle với Smart Approval
- 💻🖥️ Báo cáo performance theo phòng ban, cá nhân

---

### 6.3 Nhóm Truyền thông Nội bộ & Giao tiếp

#### TeraChat Meeting — Quản lý Cuộc họp

**Mục đích:** Lên lịch, đặt phòng, tổ chức và lưu trữ biên bản cuộc họp.

**Kiến trúc:**

- Calendar event là CRDT_Event — sync ngay cả khi offline, merge conflict tự động.
- Phòng họp được quản lý qua Resource Booking với OPA policy phân quyền đặt phòng.
- Video call tích hợp native — nhấn vào meeting → join WebRTC session không cần link ngoài.
- Biên bản được lưu E2EE trong TeraVault, tag theo meeting_id.

**Tính năng:**

- 📱💻 Lên lịch meeting: single/recurring, invite by DID
- 📱💻 Đặt phòng họp: xem lịch phòng real-time, booking conflict check
- 📱💻 Video call tích hợp: 1-click join, không app riêng
- 📱💻 Agenda: tạo trước, chia sẻ trong group chat E2EE
- 📱💻 Biên bản: smart template, ký số khi duyệt
- 📱💻 Action items từ meeting → tự động tạo task trong TeraChat Work
- 💻🖥️ Đặt tài nguyên: máy chiếu, xe công ty, thiết bị

**Cross-tapp:** Invite sync với TeraChat HRM (availability); action items push vào TeraChat Work.

---

#### TeraChat Square — Đặt Tài nguyên Công ty

**Mục đích:** Quản lý và đặt lịch tài nguyên dùng chung: phòng họp, xe, thiết bị, chỗ làm việc.

**Tính năng:**

- 📱💻 Catalog tài nguyên với ảnh, mô tả, điều kiện đặt
- 📱💻 Booking flow với OPA approval (xe công ty cần GM duyệt)
- 📱💻 QR check-in khi nhận tài nguyên
- 💻🖥️ Utilization report, conflict detection
- 💻🖥️ Maintenance schedule tích hợp TeraChat Service

---

#### TeraChat Office — Công văn & Thông báo

**Mục đích:** Ban hành, phân phối và lưu trữ công văn, thông báo nội bộ chính thức.

**Kiến trúc:**

- Công văn là CRDT_Event với `content_type: OfficialDocument`, ký số Ed25519 bởi người ban hành.
- Phân phối qua MLS group channel — fully E2EE, server không đọc nội dung.
- Đọc receipt tracking qua CRDT ACK không tiết lộ identity cho server.
- Versioning: mọi sửa đổi append vào DAG, không overwrite.

**Tính năng:**

- 📱💻 Soạn thảo công văn với rich text, đính kèm file (TeraVault)
- 📱💻 Ký số điện tử Ed25519 bởi người có thẩm quyền
- 📱💻 Phân phối đến channel/nhóm/cá nhân theo OPA policy
- 📱💻 Tracking đọc (đã đọc/chưa đọc) không lộ identity cho server
- 💻🖥️ Quản lý sổ công văn đến/đi
- 💻🖥️ Tìm kiếm full-text local (FTS5, không cloud)
- 💻🖥️ Archive và retention policy theo quy định

---

#### TeraChat Inside — Mạng Truyền thông Nội bộ

**Mục đích:** Newsfeed nội bộ doanh nghiệp — thay thế Facebook Workplace, Microsoft Viva.

**Kiến trúc:**

- Post là CRDT_Event fanout tới subscriber list theo OPA visibility policy.
- Feed ranking xảy ra hoàn toàn tại client — server không biết ai đọc gì.
- Comment/like là lightweight CRDT_Event append vào post DAG.
- Media: ảnh/video lưu qua Chunked AEAD → Zero-Byte Stub, tải on-demand.

**Tính năng:**

- 📱💻 Newsfeed: bài đăng văn bản, ảnh, video từ công ty/phòng ban
- 📱💻 Reactions, comments E2EE
- 📱💻 @mention cá nhân hoặc phòng ban
- 📱💻 Broadcast channel: một chiều từ HR/Ban lãnh đạo
- 📱💻 Poll tích hợp (Base Poll .tapp)
- 💻🖥️ Admin: quản lý nội dung, tạo trang chính thức (Company Page)
- 💻🖥️ Analytics nội bộ: reach, engagement (aggregate only, không user-correlated)

---

#### TeraChat Message — Nhắn tin Cốt lõi

**Mục đích:** Công cụ nhắn tin và gọi video nội bộ — đây là tính năng cốt lõi của TeraChat, không phải standalone .tapp.

> **Kiến trúc quan trọng:** TeraChat Message **không** chạy trong WASM sandbox. Nó là UI plugin tích hợp trực tiếp vào TeraChat Core, tránh double-encryption overhead và latency không cần thiết.

**Tính năng đầy đủ:**

- 📱💻🖥️ Chat 1-1 và nhóm E2EE (MLS RFC 9420)
- 📱💻🖥️ Threads, reactions, pin, tag, bookmark
- 📱💻🖥️ File sharing qua Zero-Byte Stub (TeraVault)
- 📱💻🖥️ Voice message với adaptive codec (Opus → AMR-NB → Text)
- 📱💻🖥️ Video call HD E2EE (WebRTC, DTLS-SRTP)
- 📱💻🖥️ Scheduled send qua Egress_Outbox
- 📱💻🖥️ Offline messaging qua BLE Mesh
- 📱💻🖥️ @mention tạo task trong TeraChat Work (deep integration)

---

### 6.4 Nhóm Quản trị Tài chính & Tài sản

#### TeraChat Finance — Quản trị Tài chính

**Mục đích:** Quản trị tổng thể tài chính doanh nghiệp — ngân sách, kế hoạch, báo cáo.

**Kiến trúc:**

- Financial record lưu trong Tapp Data Namespace (max_storage_mb: 512), encrypted.
- Aggregation xảy ra tại client — server không bao giờ biết số liệu tài chính.
- DataGrant nhận payroll data từ TeraChat Checkin; revenue từ TeraChat CRM.
- Export báo cáo có Ed25519 signature cho audit compliance.

**Tính năng:**

- 💻🖥️ Ngân sách: lập kế hoạch theo phòng ban, theo dự án
- 📱💻 Dashboard tài chính: P&L, cashflow (local render, không server)
- 💻🖥️ Phân bổ chi phí theo cost center
- 💻🖥️ Báo cáo tài chính: kỳ, năm — export PDF có chữ ký số
- 💻🖥️ Cảnh báo vượt ngân sách tự động
- 📱💻 Approval chi tiêu qua Smart Approval biometric

---

#### TeraChat Income & Expense — Doanh thu & Chi phí

**Mục đích:** Ghi nhận, theo dõi và phân loại toàn bộ doanh thu và chi phí phát sinh.

**Tính năng:**

- 📱💻 Tạo phiếu thu/chi: manual hoặc import từ BankFeeds
- 📱💻 OCR receipt: chụp hóa đơn → AI extract amount, category (local ONNX)
- 📱💻 Category tự động bằng AI (local model, không cloud)
- 📱💻 Approval chi phí theo mức (tự duyệt < X; manager > X; CFO > Y)
- 💻🖥️ Báo cáo theo category, phòng ban, dự án, kỳ
- 💻🖥️ Reconcile với BankFeeds tự động
- 📱💻 Mobile expense report: chụp - submit - track

---

#### TeraChat BankFeeds — Đối soát Ngân hàng

**Mục đích:** Kết nối và đối soát giao dịch ngân hàng tự động, bảo mật tuyệt đối.

**Kiến trúc:**

- Kết nối ngân hàng qua egress endpoint khai báo cụ thể trong manifest với TLS pinning.
- Response từ bank API phải pass JSON Schema validation trước khi vào DAG — không cho phép raw blob.
- Transaction được mã hóa ngay khi nhận, lưu trong Tapp Data Namespace.
- Mọi reconciliation action được ký Ed25519 và ghi vào Audit Log.
- **Không bao giờ** lưu banking credentials plaintext — luôn đi qua API Secret Management (TeraVault).

**Tính năng:**

- 💻🖥️ Kết nối đa ngân hàng (Open Banking API, bank statement import)
- 💻🖥️ Auto-matching transaction với phiếu thu/chi (fuzzy match + AI suggestion)
- 💻🖥️ Reconciliation dashboard: matched/unmatched/disputed
- 💻🖥️ Cảnh báo giao dịch bất thường (anomaly detection, local AI)
- 💻🖥️ Lịch sử đối soát với audit trail Ed25519-signed
- 📱 Nhận notification giao dịch mới (Push E2EE, không lộ amount cho APNs/FCM)

**Security:** TLS pinning bắt buộc cho mọi bank endpoint. HMAC-SHA256 verify payload. Ed25519 audit trail mọi action. Zero-Knowledge: server TeraChat không bao giờ biết số dư hay giao dịch.

---

#### TeraChat Asset — Quản lý Tài sản

**Mục đích:** Theo dõi vòng đời tài sản doanh nghiệp từ mua → sử dụng → bảo trì → thanh lý.

**Kiến trúc:**

- Asset record lưu trong Tapp Data Namespace.
- Document đính kèm (hóa đơn mua, phiếu bảo hành) dùng Zero-Byte Stub trong TeraVault — server không biết file name hay nội dung.
- QR/barcode scan tích hợp native camera.
- Depreciation calculation chạy hoàn toàn local (không cần server).

**Tính năng:**

- 📱💻 Catalog tài sản: thông tin, location, người phụ trách, trạng thái
- 📱 QR/barcode scan để tra cứu và cập nhật nhanh
- 📱💻 Lịch bảo trì: reminder tự động, tích hợp TeraChat Service
- 📱💻 Bàn giao tài sản: ký số điện tử Ed25519, không thể chối bỏ
- 💻🖥️ Khấu hao tự động (straight-line, declining balance)
- 💻🖥️ Báo cáo tài sản: tổng giá trị, theo loại, theo location
- 💻🖥️ Thanh lý tài sản workflow qua TeraChat Workflow

**Cross-tapp:** Giá trị tài sản xuất vào TeraChat Finance; bàn giao tích hợp TeraChat HRM (employee assignment).

---

### 6.5 Nhóm Khách hàng & Không gian số

#### TeraChat CRM — Quản trị Quan hệ Khách hàng

**Mục đích:** Quản lý pipeline bán hàng, lịch sử khách hàng, và quy trình chăm sóc — hoàn toàn E2EE.

**Kiến trúc:**

- Contact/Deal lưu trong Tapp Data Namespace — không bao giờ lên cloud plaintext.
- External webhook nhận trigger từ web form, email, Zalo OA qua Inbound Webhook (MARKETPLACE-14) với HMAC-SHA256 auth.
- AI local summarize conversation history, suggest follow-up (không cloud AI).
- Giao tiếp với khách hàng qua TeraChat Message tích hợp native — mọi tin nhắn E2EE.
- Federation Bridge: cross-org với đối tác/vendor qua mTLS Sealed Sender.

**Tính năng:**

- 📱💻 Contact management: thông tin, tags, segments, lịch sử tương tác
- 📱💻 Deal pipeline: Kanban theo stage, value, probability
- 📱💻 Activity log: call, email, meeting — tự động sync từ TeraChat Meeting
- 📱💻 AI suggest: follow-up next action, conversation summary (local)
- 📱💻 Quote / Proposal: tạo, ký số, gửi E2EE cho khách hàng
- 💻🖥️ Inbound webhook: nhận lead từ web form, Zalo, email gateway
- 💻🖥️ Sales dashboard: pipeline value, win rate, cycle time
- 💻🖥️ Báo cáo doanh số xuất vào TeraChat Finance

**Cross-tapp:** Revenue data xuất vào TeraChat Finance (Income); meeting log sync từ TeraChat Meeting; proposal approval qua TeraChat Request.

---

#### TeraChat Workspace — Không gian Làm việc Cá nhân

**Mục đích:** Dashboard cá nhân hóa cho từng nhân viên — tổng hợp thông tin từ toàn bộ App Suite.

**Kiến trúc:**

- Workspace là aggregated view — không lưu data riêng, chỉ aggregate qua DataGrant read-only từ các tapp khác.
- Personalization config lưu trong sled KV per-DID (encrypted).
- Widget system: mỗi widget là một mini-view của tapp khác, render local.
- Không có data nào về workspace usage lên server — hoàn toàn local.

**Tính năng:**

- 📱💻 My Dashboard: task hôm nay (TeraChat Work), lịch họp (TeraChat Meeting), request pending (TeraChat Request)
- 📱💻 My Goals: OKR/KPI cá nhân (TeraChat Goal) với progress bar
- 📱💻 My Checkin: trạng thái chấm công hôm nay (TeraChat Checkin)
- 📱💻 Inbox tổng hợp: tin nhắn ưu tiên cao, mention, approval cần xử lý
- 📱💻 Quick action: tạo task, tạo request, checkin — không cần mở app riêng
- 💻🖥️ Widget customization: drag-drop, resize, pin/unpin
- 💻🖥️ Focus mode: ẩn notification, hiển thị chỉ priority items
- 💻🖥️ Activity digest: tóm tắt ngày của team (aggregate, không detail)

---

## Module 7 — Chia sẻ Đa phương tiện & Tệp tin

### 7.1 Media & File Transfer

- 📱💻🖥️ **Chunked AEAD Streaming (max 10GB):** File chunk 2MB → per-chunk HKDF key → AES-256-GCM → MinIO. RAM tối đa tại mọi thời điểm: ~10MB (Double Buffer). Không bao giờ tràn RAM.
- 📱💻 **Compression:** WebP/HEIC auto-convert cho ảnh; video giữ nguyên hoặc transcode theo tag "Gửi HD".
- 📱 **iOS Background Transfer:** NSURLSession Background Transfer Service với pre-signed URL (TTL 15 phút, HMAC bound).
- 📱 **Android Background:** WorkManager với constraint `NetworkType.CONNECTED + requiresBatteryNotLow`.
- 📱💻 **Codec Tự động Hạ cấp:** Voice message Online → Opus 128kbps; Mesh → AMR-NB 4.75kbps; BLE Emergency → Whisper transcription → text.
- 📱💻 **Deduplication (Server-Blind):** Salted MLE `BLAKE3(ciphertext + Channel_Key)` — server không học được gì từ dedup.
- 📱💻 **Zero-Byte Stub Preview:** Thumbnail ẩn trong stub, decrypt local khi render — server không biết ảnh preview.

### 7.2 TeraVault VFS

- 📱💻 **Virtual File System:** Cây thư mục ảo, pointer `cas_ref` phi vật lý — không tốn storage khi pin.
- 📱💻 **Tier tự động:** Tier 1 Auto (channel files), Tier 2 Manual (organized by user).
- 📱💻 **Make Available Offline:** Pre-fetch chunks về local cache, tự decrypt khi cần.
- 📱💻 **Version history:** Mọi thay đổi file append vào DAG — rollback bất kỳ version.
- 💻🖥️ **Bulk operations:** Multi-select, move, copy — tất cả local operation, không re-upload.

---

## Module 8 — Identity, RBAC & Quản trị

### 8.1 Role-Based Access Control

| Vai trò | Năng lực | Ràng buộc |
|---|---|---|
| **CISO / Admin** | Toàn quyền: KMS, policy, revocation, federation, audit | HSM quorum cho policy changes |
| **HR Recovery Officer** | Device re-provisioning, identity recovery | Không thể đọc nội dung tin nhắn |
| **Editor** | Tạo/sửa tài liệu, `ffi_propose_change` | Không có quyền key ceremony |
| **Commenter** | Ghi chú shadow branch | Read + comment only |
| **Viewer** | Chỉ đọc, FFI pointer frozen | `ZeroizeOnDrop` khi thoát |

### 8.2 OPA Policy Engine

- 📱💻☁️ Mọi hành động phải qua OPA Rego Policy. Không có bypass.
- 🗄️☁️ **M-of-N HSM Quorum (2-of-3: CISO + CTO + Legal):** Không một cá nhân đơn lẻ nào có thể thay đổi production policy.
- ☁️ **Policy Rollback Protection:** Monotonic version counter — client từ chối bundle version thấp hơn.
- 📱💻 **Local enforcement:** OPA WASM build tại thiết bị. Không round-trip server.

### 8.3 Đăng nhập & Device Recovery

**Đăng nhập:**
Không hỗ trợ OTP SMS hay đăng nhập social. TeraChat dùng Identity Broker (Keycloak/Dex) với Azure AD, Google Workspace, Okta (SAML/OIDC). Deep-link tự động liên kết hardware key. Magic 1-tap link: `pre_authenticated`, `Device_Key` sinh trước OIDC hoàn tất.

**TH1 — Mất CẢ thiết bị:**

1. CEO khởi tạo Break-glass → BLE beacon đến C-Level trong phạm vi < 2m.
2. ≥ 3-of-5 C-Level xác thực sinh trắc học trong **10 phút** (Quorum Timer).
3. Lagrange Interpolation tái tạo `Escrow_Key` trong mlock-protected arena **< 100ms** → zeroize.
4. Thiết bị cũ nhận Remote Wipe khi chạm mạng.

**TH2 — Mất Điện thoại, Laptop còn:**

1. Laptop → Gossip Crypto-Shredding Device_Key cũ.
2. Điện thoại mới sinh khóa, QR → Laptop quét → Pending Approval trên VPS.
3. HR xác minh qua video call → ký Authorization Ticket.
4. Company_Key truyền qua P2P (BLE/Wi-Fi Direct). **Zero cloud exposure.**

### 8.4 SCIM 2.0 & Identity Sync

- ☁️ Azure AD / Google Workspace: nghỉ việc → tài khoản khóa trong **< 30 giây**.
- ☁️ Federation Revocation: cross-org mTLS vô hiệu trong < 30 giây.
- 📱💻 **1-Tap Magic Deep-Link:** App tự chuyển `pre_authenticated`, `Device_Key` sinh trước OIDC hoàn tất.

---

## Module 9 — Federation & Cross-Org Communication

### 9.1 Kiến trúc

- Mỗi tổ chức vận hành **Private Cluster độc lập** với ranh giới mã hóa riêng.
- ☁️ **Federation Bridge (Zone 2):** mTLS + **Sealed Sender** — server nhận không thể xác định người gửi.
- 🗄️ **Trust Registry:** append-only SQLite — không dùng public CA.
- Schema Version: ±1 minor → read-only federation. ±1 major → `SCHEMA_INCOMPATIBLE`, rejected.
- **OPA Policy channel** luôn hoạt động bất kể schema version gap — bảo đảm security policy (CRL, revocation) luôn đến được Branch cluster.

### 9.2 Quy trình Kết nối

```
Admin HQ → Federation Invite Token (Signed JWT)
    ↓
Admin Branch → nạp Token → gửi kết nối kèm Public Key
    ↓
HQ approve → key exchange vào federation_trust_registry
    ↓
OPA Rate Limiting ngăn cross-org DoS
    ↓
SCIM offboarding → mTLS vô hiệu < 30s tự động
```

---

## Module 10 — Compliance, DLP & Audit

> **Nguyên tắc:** Tuân thủ thực thi bằng toán học, không phải hợp đồng. Không Admin nào có thể thay đổi lịch sử mà không phá vỡ Merkle chain.

### 10.1 Data Loss Prevention

- 📱💻 **OPA Egress Whitelist:** Egress đến domain không khai báo → bị chặn tại OS.
- 📱💻 **Byte-Quota Circuit Breaker:** 4KB/call. Vượt 100% session quota → Egress khóa 24h, CISO alert.
- 📱💻 **Format Whitelist:** [.pdf, .txt, .jpg]. `.vbs`, `.xlsm` bị chặn.
- 💻📱 **Redaction Rules:** Rust Core local ML redact CC numbers, nội bộ IP khỏi AI prompt.
- 🗄️ **Anti-Remanence Policy (ARP):** TTL Crypto-Shredding. CISO-initiated Wipe → zero-fill với Ed25519-signed checkpoint.

### 10.2 Tamper-Proof Audit Log

- 🗄️ Mọi entry mang chữ ký Ed25519. Entry không ký → reject khi ghi, không bao giờ lưu.
- 🗄️ **Merkle Chain:** Xóa bất kỳ entry nào → phá vỡ chain ngay lập tức, audit độc lập được.
- ☁️ Marketplace Transparency Log: append-only, Merkle-proofed.

### 10.3 TeraChat App Suite Compliance

Mọi action trong TeraChat App Suite đều được ghi vào Audit Log:

| App | Sự kiện bắt buộc ghi Audit |
|---|---|
| TeraChat Request | Mọi approve/reject với biometric signature |
| TeraChat Finance | Mọi transaction trên ngưỡng cấu hình |
| TeraChat BankFeeds | Mọi reconciliation action |
| TeraChat HRM | Onboarding/offboarding, contract signing |
| TeraChat E-Hiring | Offer letter signing, hiring decision |
| TeraChat Asset | Bàn giao, thanh lý tài sản |
| TeraChat CRM | Deal won/lost, quote signing |

---

## Module 11 — Infrastructure & Deployment

### 11.1 Server Topology

```
[GeoDNS]
    ├──► Zone 1: VPS Relay (Rust Blind Relay · SQLite WAL · NATS JetStream)
    ├──► Zone 2: Federation Bridge (mTLS · Sealed Sender)
    └──► Zone 0: Bare-metal HSM (HSM FIPS 140-3 · PostgreSQL · MinIO EC+4)
```

### 11.2 Scale Tiers

| Quy mô | Topology | Hardware Tối thiểu |
|---|---|---|
| ≤ 10.000 users | Single-node Rust relay | 512MB VPS |
| ≤ 100.000 users | Geo-federated clusters | PostgreSQL HA + MinIO |
| ≥ 1.000.000 users | Multi-cloud active-active | PostgreSQL Geo-Partitioning |

### 11.3 Performance Targets

| Operation | Target | Platform |
|---|---|---|
| ALPN negotiation (full fallback) | < 50ms | All |
| MLS encrypt (single message) | < 5ms | All |
| Push notification decrypt (NSE) | < 500ms | 📱 iOS |
| End-to-end relay delivery | < 200ms | All |
| TURN failover | < 3s | WebRTC |
| .tapp cold start (wasm3 iOS) | < 20ms | 📱 iOS |
| .tapp cold start (wasmtime Desktop) | < 50ms | 💻🖥️ Desktop |
| Tapp Data Namespace query | < 10ms | All (local SQLite) |
| DataGrant cross-tapp fetch | < 50ms | All |
| VPS concurrent WebSocket | ~500.000 | ☁️ 4GB VPS |
| File chunk throughput | 300–500 MB/s | ☁️🗄️ |

---

## Licensing & Service Tiers

| Feature | Community | Enterprise | GovMilitary |
|---|---|---|---|
| Offline TTL | 24 giờ | 7 ngày (configurable) | **30 ngày** |
| EMDP Tactical Relay | ❌ | ✅ | ✅ |
| Air-Gapped License | ❌ | ✅ | ✅ |
| Compliance Retention | ❌ | 90 ngày | **7 năm** |
| TEE Enclaves (SGX) | ❌ | ❌ | ✅ |
| Chaos Engineering | ❌ | Optional | **Bắt buộc** |
| Federation | ❌ | ✅ | ✅ Air-gapped |
| AI Token Quota | 10K/giờ | Unlimited | Unlimited + local-only |
| TeraChat App Suite | Read-only | ✅ Full | ✅ Full + custom .tapp |
| Tapp Data Namespace | ❌ | ✅ | ✅ |
| DataGrant Cross-tapp | ❌ | ✅ | ✅ |
| Inbound Webhook | ❌ | ✅ | ✅ |

**Open-Core Boundary:**

| Component | License | Auditable by |
|---|---|---|
| `terachat-core` (Crypto, MLS, CRDT, Mesh) | AGPLv3 | Gov, Bank, Public |
| `terachat-license-guard` | BSL | Không public |
| `terachat-ui` (Tauri, Flutter) | Apache 2.0 | Public |

---

## Constraints & Open Items

### Platform Hard Constraints

| Platform | Ràng buộc | Giải pháp |
|---|---|---|
| 📱 iOS | W^X: không WASM JIT | wasm3 + AOT .dylib trong XPC Worker |
| 📱 iOS | NSE RAM: 20MB ceiling | Ghost Push + Main App decrypt |
| 📱 iOS | AWDL tắt khi Hotspot/CarPlay | Auto-downgrade BLE Tier 3 |
| 📱 Android | FCM throttled 10/h Restricted battery | FCM high-priority + Companion Device Manager |
| 📱 Huawei | Không có content-available background push | Foreground polling; CRL ≤ 4h |
| 🖥️ Linux | Flatpak không tương thích seccomp-bpf | .deb / .rpm / AppImage Cosign |
| ☁️ VPS | eBPF/XDP yêu cầu bare-metal | Tokio Token Bucket userspace |

### .tapp App Suite Constraints

| Constraint | Impact | Giải pháp |
|---|---|---|
| WASM sandbox memory ≤ 64MB | Finance/HRM với dataset lớn | Tapp Data Namespace pagination + Tapp Data Namespace streaming |
| Egress hard limit 4KB/call | BankFeeds cần payload lớn hơn | FileTransfer capability riêng (tách egress quota) |
| Cross-tapp chỉ qua Honest Broker | Latency DataGrant fetch | Caching trong Tapp Data Namespace với TTL |
| WasmParity gate bắt buộc | Mọi app mới phải test wasm3/wasmtime | CI automation cho tất cả App Suite tapp |
| Storage quota watchdog | Tapp độc hại exhaust disk | INFRA-03 heartbeat table monitor per-tapp storage |

### Blocker Items (Phải hoàn thành trước production)

| Item | Mức độ |
|---|---|
| CI/CD code signing pipeline (5 platform) | **BLOCKER** |
| WasmParity CI gate (wasm3 vs wasmtime) | **BLOCKER** |
| Dart FFI NativeFinalizer Clippy lint | **BLOCKER** |
| MARKETPLACE-12: Tapp Data Namespace implementation | **BLOCKER cho App Suite** |
| MARKETPLACE-13: Structured DataGrant extension | **BLOCKER cho cross-tapp** |
| MARKETPLACE-14: Inbound Webhook | **BLOCKER cho CRM/BankFeeds** |
| AppArmor / SELinux postinstall (Linux) | HIGH |
| Storage quota watchdog (INFRA-03) | HIGH |
| BLE peripheral scan capability (Checkin) | HIGH |
| FileTransfer capability separation | HIGH |

---

## Document Navigation Map

| Audience | Document | Content |
|---|---|---|
| Developer | `Spec-Wasm-Tapp-Runtime.md` (TERA-RUNTIME) | WASM runtime, IPC, OS hooks, .tapp behavior |
| System Architect | `Spec-Core-Cryptography-And-Mesh.md` (TERA-CORE) | MLS, CRDT, Mesh, Crypto |
| Designer | `Design.md` (TERA-DESIGN) | UI state machine, Glassmorphism, animations |
| Ecosystem Builder | `Spec-Ecosystem-And-Trust-Chain.md` (TERA-ECO) | .tapp lifecycle, Marketplace, Plugin registry |
| Security Auditor | `Spec-Identity-And-Governance.md` (TERA-GOV) | OPA, RBAC, Cloud Enclave, Federation |
| Investor / Executive | `BusinessPlan.md` (TERA-BIZ) | GTM, pricing, licensing |
| New Team Member | `Introduction.md` (TERA-INTRO) | Vision, architecture overview, terminology |

---

*TeraChat — Hệ điều hành Công việc Sinh tồn. Trao lại chủ quyền số cho người tiên phong.*

---

```yaml
# CHANGELOG
- version:  "0.3.0"
  date: "2026-03-28"
  changes:
    - "Add Module 6: TeraChat App Suite — 20+ enterprise apps across 5 verticals"
    - "Add TeraChat Request, Work, Workflow, Service (Operations)"
    - "Add TeraChat E-Hiring, HRM, Checkin, Schedule, Goal (HR)"
    - "Add TeraChat Meeting, Square, Office, Inside, Message (Communications)"
    - "Add TeraChat Finance, Income/Expense, BankFeeds, Asset (Finance)"
    - "Add TeraChat CRM, Workspace (Customer & Digital Space)"
    - "Add MARKETPLACE-12/13/14 capability specs (Tapp Data Namespace, DataGrant, Inbound Webhook)"
    - "Add cross-tapp integration matrix for all App Suite tapp"
    - "Expand Module 2 with Threads, Tags, Pin, Reactions, Scheduled Send specs"
    - "Add FUNC-12 FCP Trust Boundary Declaration"
    - "Add FUNC-13 Burner Agent Cross-Org Collaboration Flow"
    - "Add FUNC-14 Swarm Workspace User Flow"
    - "Add FUNC-15 Causal Smart Approval Agent-Backed Decision Flow"
    - "Update Module 7 with full file transfer and TeraVault VFS specs"
    - "Update Licensing tiers with App Suite feature gates"
    - "Update Constraints with .tapp App Suite blockers"
    - "Clarify TeraChat Message as UI plugin, not standalone WASM sandbox"

- version:  "0.2.6"
  date: "2026-03-19"
  changes:
    - "Complete rewrite from CEO/CTO perspective"
    - "Synthesized from legacy specs, Design.md, BusinessPlan.md, Introduction.md"
    - "Added Module 3 (Survival Mesh), Module 4 (AI Shield), Module 5 (.tapp)"
    - "Added TCO Reference, Performance Targets, Platform Constraints tables"
    - "Standardized platform icons and Online/Mesh mode indicators"
    - "Full Licensing Tiers table (Community / Enterprise / GovMilitary)"
    - "Consolidated Recovery flows into Module 6"
```
