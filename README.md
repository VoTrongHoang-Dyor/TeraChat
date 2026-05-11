# TeraChat

> **Nền tảng Messaging Doanh nghiệp Zero-Knowledge — Chủ quyền Dữ liệu được bảo đảm bằng Toán học**

[![License: AGPLv3](https://img.shields.io/badge/Core-AGPLv3-blue.svg)](LICENSE)
[![License: BSL](https://img.shields.io/badge/License_Guard-BSL-orange.svg)](LICENSE-BSL)
[![Rust](https://img.shields.io/badge/Rust-1.75.0-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/Platform-iOS%20%7C%20Android%20%7C%20macOS%20%7C%20Windows%20%7C%20Linux-green.svg)](#platform-support)
[![Enterprise Only](https://img.shields.io/badge/Access-Enterprise%20Only-red.svg)](#access-model)

---

## Tổng quan

TeraChat là nền tảng messaging doanh nghiệp **Zero-Knowledge, End-to-End Encrypted** được thiết kế cho các tổ chức yêu cầu kiểm soát tuyệt đối dữ liệu giao tiếp nội bộ — không phụ thuộc vào bất kỳ nhà cung cấp dịch vụ đám mây nào.

**TeraChat không phải ứng dụng công khai.** Mọi installation đều yêu cầu License JWT hợp lệ được cấp bởi tổ chức. Không có tài khoản cá nhân. Không có free tier. Không có consumer pricing.

### Tại sao TeraChat tồn tại

Hầu hết doanh nghiệp hiện đang giao toàn bộ dữ liệu giao tiếp nội bộ cho bên thứ ba — Slack, Microsoft Teams — những nền tảng có thể đọc nội dung, bị ép buộc bởi lệnh tòa án, và sẽ ngừng hoạt động khi internet gián đoạn.

TeraChat giải quyết ba vấn đề cốt lõi:

- **Pháp lý:** Nghị định 13/2023/NĐ-CP (Việt Nam), GR 71 (Indonesia), PDPA (Thái Lan) yêu cầu dữ liệu nội bộ phải nằm trên hạ tầng của tổ chức.
- **Khả dụng:** Mọi nền tảng cloud đều chết khi mất internet. TeraChat không.
- **Bảo mật:** Server-side encryption là lời hứa. End-to-end encryption là toán học.

---

## Tính năng cốt lõi

### Messaging nội bộ và liên chi nhánh

TeraChat phục vụ giao tiếp nội bộ doanh nghiệp và giữa các chi nhánh theo cấu trúc phân quyền rõ ràng. Hệ thống **không** hỗ trợ nhắn tin với khách hàng bên ngoài — đây là quyết định kiến trúc có chủ đích, không phải giới hạn kỹ thuật.

- E2EE nhóm lên đến 10.000 thành viên qua MLS RFC 9420
- Phân quyền theo cấp bậc và phòng ban qua OPA Policy Engine
- Federation mTLS giữa các chi nhánh với Sealed Sender Protocol
- Audit trail bất biến ký bằng Ed25519, không thể xóa hoặc sửa đổi

### Work OS qua `.tapp` Marketplace

Doanh nghiệp mở rộng TeraChat bằng mini-app (`.tapp`) chạy trong WASM sandbox — cô lập hoàn toàn, không thể truy cập dữ liệu ngoài phạm vi được cấp phép.

- IT Admin phê duyệt và phân phối `.tapp` theo phòng ban hoặc khu vực
- WASM sandbox với Host ABI — `.tapp` không bao giờ tự chạy crypto
- Web Marketplace với quy trình kiểm duyệt bảo mật (static analysis + manual review)
- Publisher Trust Tiers: Community → Verified → Enterprise → TeraChat Native
- Emergency Kill-switch: revoke trên toàn bộ fleet trong < 60 giây

### AI local — Bring Your Own Model (BYOM)

TeraChat tích hợp AI qua giao diện mở, không phụ thuộc nhà cung cấp cụ thể.

- OpenAI-compatible REST API — kết nối bất kỳ model nào (Ollama, vLLM, LM Studio, LLM nội bộ)
- PII Redaction Gate bắt buộc: ONNX Micro-NER mask thông tin nhạy cảm trước khi vào prompt
- ZK Memory Agent: AI context được build hoàn toàn trên hạ tầng khách hàng (Mac mini + NAS)
- Plaintext không bao giờ rời khỏi ranh giới tổ chức

### Survival Mesh Network

Khi internet gián đoạn, TeraChat không suy giảm — nó kích hoạt mạng P2P tự tổ chức.

- BLE 5.0 + Wi-Fi Direct tự động tạo mesh khi mất kết nối
- EMDP (Emergency Mobile Dictator Protocol) cho môi trường cực đoan
- Text-only Store-and-Forward trong chế độ khẩn cấp
- Hoạt động hoàn toàn trong môi trường air-gapped

---

## Kiến trúc

### Nguyên lý bất biến

```
1. Zero-Knowledge Server     — Server chỉ thấy ciphertext, không bao giờ plaintext
2. Key Material không rời Chip — Private key tồn tại trong Secure Enclave/StrongBox/TPM 2.0
3. Offline-First Survival    — Không phụ thuộc internet để hoạt động
4. Zero-Trust theo Thiết kế  — Mọi quyền truy cập qua OPA Policy, kể cả TeraChat Inc.
5. License Entanglement       — License JWT neo vào DeviceIdentityKey qua KDF
```

### Stack kỹ thuật

```
┌─────────────────────────────────────────────────────────────┐
│                     THIẾT BỊ DOANH NGHIỆP                   │
│                                                             │
│  ┌───────────────────────────────────────────────────────┐  │
│  │              RUST CORE (Shared Binary)                │  │
│  │  MLS E2EE · CRDT DAG · BLE Mesh · Key Management     │  │
│  │  OPA Policy · WASM Sandbox · Offline Storage          │  │
│  └─────────────────────┬─────────────────────────────────┘  │
│           IPC/FFI       │                                    │
│  ┌──────────────────┐  │  ┌─────────────────────────────┐   │
│  │  UI Layer        │  │  │  Secure Hardware             │   │
│  │  Flutter / Tauri │◄─┘  │  Enclave / StrongBox / TPM  │   │
│  │  (Pure Renderer) │     │  (Key Material — Never Out)  │   │
│  └──────────────────┘     └─────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
          │ TLS 1.3 + mTLS                │ BLE / Wi-Fi Direct
          ▼                               ▼
┌─────────────────────┐       ┌───────────────────────────┐
│  TeraRelay          │       │  Peer Devices (Mesh)       │
│  Blind ciphertext   │       │  Store-and-Forward CRDT    │
│  routing only       │       │  P2P Encrypted             │
└─────────────────────┘       └───────────────────────────┘
```

| Layer | Technology | Platform |
|---|---|---|
| Core Logic | Rust 1.75.0 (shared binary) | All platforms |
| Mobile UI | Flutter + Dart FFI | iOS · Android · Huawei |
| Desktop UI | Tauri (Rust + WebView) | macOS · Windows · Linux |
| Protocol | MLS RFC 9420 + QUIC/gRPC/WSS | All |
| Encryption | AES-256-GCM + ML-KEM-768 (Post-Quantum) | All |
| Storage | SQLite WAL + SQLCipher | All |
| Plugin Runtime | wasm3 (iOS) / wasmtime (Android + Desktop) | All |
| AI Inference | MLX (Apple Silicon) + BYOM endpoint | Mac mini |

### Deployment topologies

| Tier | Infrastructure | User limit | Setup time |
|---|---|---|---|
| Solo | 1 Mac mini M2 8GB + NAS 4TB | ≤ 50 | 15 phút |
| SME | 1 Mac mini M2 Pro 32GB + NAS 16TB | ≤ 500 | 30 phút |
| Enterprise | 2 Mac mini M4 Pro 48GB (cluster) + NAS 32TB | ≤ 5,000 | 1 giờ |
| Gov Air-gapped | 2 Mac mini + NAS + HSM PKCS#11 | Unlimited | 4 giờ |
| Remote Branch | VPS Rust relay only ($6–48/tháng) | — | 5 phút |

---

## Cài đặt nhanh

### Yêu cầu

- macOS 13+ (Apple Silicon) hoặc Linux Ubuntu 22.04 LTS / RHEL 9
- License JWT từ TeraChat (liên hệ sales)
- Rust 1.75.0 (pin qua `rust-toolchain.toml`)

### TeraRelay — Single Binary Deploy

```bash
# Download và chạy installer
curl -fsSL https://install.terachat.io | sudo bash

# Wizard hỏi 3 thứ:
#   1. License JWT
#   2. Domain name
#   3. AI endpoint URL (tùy chọn — BYOM)

# Hệ thống tự động:
# - Detect OS (macOS / Ubuntu / RHEL)
# - Generate TLS certificate
# - Init SQLite WAL databases
# - Setup systemd/launchd service
# - Configure firewall rules
# → TeraChat ready trong < 30 phút
```

### Build từ source

```bash
git clone https://github.com/terachat/terachat-core
cd terachat-core

# Build Rust Core
cargo build --release --locked

# Build TeraRelay
./target/release/terachat-relay \
  --db-path ./data/relay.db \
  --listen 0.0.0.0:443 \
  --cert ./certs/relay.crt \
  --key ./certs/relay.key
```

### BYOM AI Endpoint

TeraChat chỉ yêu cầu model endpoint tuân theo OpenAI-compatible REST API schema:

```bash
# Ví dụ với Ollama
ollama serve  # Mặc định: http://localhost:11434

# Cấu hình trong TeraChat wizard:
# AI Endpoint: http://localhost:11434/v1/chat/completions

# Health check bắt buộc:
# GET /health → HTTP 200
```

Hỗ trợ: Ollama, vLLM, LM Studio, và bất kỳ LLM nội bộ nào đã expose OpenAI-compatible endpoint.

---

## Mô hình truy cập doanh nghiệp

```
Tổ chức ký hợp đồng với TeraChat
         ↓
TeraChat cấp License JWT (HSM FIPS 140-3 signed)
  { tenant_id, domain, max_seats, tier, valid_until, features }
         ↓
IT Admin triển khai TeraRelay (1 binary, 1 command)
         ↓
IT Admin phân phát app đến nhân viên qua MDM
         ↓
Nhân viên cài đặt app → app xác thực License JWT
         ↓
Không có license hợp lệ → "Liên hệ IT Admin"
```

| Thành phần | Vai trò |
|---|---|
| TeraChat Inc. | Cấp license, duy trì binary, hỗ trợ kỹ thuật |
| IT Admin | Triển khai relay, quản lý thiết bị, phê duyệt `.tapp` |
| Nhân viên | Sử dụng trong phạm vi chính sách tổ chức |
| TeraRelay | Blind router — chỉ thấy `destination_device_id`, `blob_size`, `timestamp` |

---

## Bảo mật

### Cryptographic stack

| Primitive | Implementation |
|---|---|
| Group Messaging | MLS RFC 9420 (TreeKEM, Sealed Sender) |
| Post-Quantum KEM | ML-KEM-768 + X25519 Hybrid (CNSA 2.0) |
| Symmetric Encryption | AES-256-GCM |
| Key Derivation | HKDF (SHA-256) |
| Hashing | BLAKE3, SHA-512 |
| Signing | Ed25519 |
| Password Hashing | Argon2id |
| DB Encryption | SQLCipher |
| Memory Safety | ZeroizeOnDrop (toàn bộ key material) |

### Hardware Root of Trust

| Platform | Key Storage | Authentication |
|---|---|---|
| iOS / macOS | Secure Enclave (SEP) | Face ID / Touch ID |
| Android | StrongBox Keymaster | BiometricPrompt |
| Windows | TPM 2.0 (CNG) | Windows Hello |
| Linux | TPM 2.0 (tpm2-pkcs11) | PIN |
| Gov-grade | HSM FIPS 140-3 L3 (PKCS#11) | Physical + Shamir quorum |

### Threat model

TeraChat được thiết kế để chống lại:

- **Nation-state adversary:** Post-Quantum KEM bảo vệ khỏi "Store Now, Decrypt Later"
- **Insider threat:** Shamir 3-of-5 — không một C-Level nào đơn lẻ có thể truy cập key
- **Server compromise:** Server là Blind Relay — không có key, không có plaintext
- **Device theft:** Biometric-bound keys + Crypto-shred sau 5 lần sai PIN
- **Legal compulsion:** ZK architecture — server không có gì để giao

### Invariants không thể vi phạm

```
❌ Không tự implement crypto — chỉ dùng ring crate hoặc RustCrypto
❌ Không persist plaintext key lên disk
❌ Không truyền raw pointer qua FFI
❌ Không cấp file handle OS gốc cho WASM sandbox
❌ Không trust CA công cộng cho mTLS nội bộ
❌ Không có UPDATE hoặc DELETE trên hot_dag.db (append-only)
❌ iOS không bao giờ là Mesh Dictator (election_weight = 0)
```

---

## Cấu trúc dự án

```
TeraChat-Project/
├── source/
│   ├── core/                    # Rust Workspace — Secure by Design
│   │   ├── tc-crypto/           # PQ-KEM, HKMS, ZeroizeOnDrop
│   │   ├── tc-crdt-sync/        # CRDT DAG, Offline-first sync
│   │   ├── tc-mesh/             # BLE Survival Mesh + EMDP
│   │   ├── tc-store/            # SQLite WAL, TeraVault VFS
│   │   └── tc-tapp/             # WASM Engine, Host ABI
│   ├── clients/
│   │   ├── apple/               # SwiftUI (iOS + macOS)
│   │   ├── android/             # Kotlin/Jetpack Compose
│   │   ├── desktop/             # Tauri (Windows + Linux + macOS)
│   │   ├── harmonyos/           # ArkUI (Huawei)
│   │   └── web/                 # React + WebAssembly
│   ├── bindings/
│   │   ├── uniffi-apple/        # Rust → Swift FFI
│   │   ├── uniffi-android/      # Rust → Kotlin/JNI
│   │   ├── napi-harmony/        # Rust → Node.js/C++ (HarmonyOS)
│   │   └── wasm-bridge/         # Rust → WebAssembly
│   └── infra/
│       ├── mac-mini/            # Single Mac mini config
│       ├── mac-mini-clusters/   # HA cluster (Active-Passive)
│       └── bare-metal/          # Air-gapped Gov deployment
├── docs/
│   ├── MD/                      # Spec files (Source of Truth)
│   └── HTML/                    # Compiled business documents
└── tests/
    ├── chaos-mesh/              # 40 chaos engineering scenarios
    ├── cross-platform-e2e/      # End-to-end tests
    └── ffi-stress/              # Memory leak + load testing
```

---

## Tài liệu kỹ thuật

Bộ tài liệu kỹ thuật được tổ chức theo Domain-Driven Architecture. Đọc theo vai trò:

| Đối tượng | Đọc trước |
|---|---|
| Rust Core Dev | `Spec-Core-Cryptography-And-Mesh.md` → `Spec-Dual-Sync-And-Local-Storage.md` → `Spec-Wasm-Tapp-Runtime.md` |
| Frontend Dev (Flutter/Tauri) | `Spec-Client-IPC-And-UI-Bridge.md` |
| `.tapp` Plugin Developer | `Spec-Wasm-Tapp-Runtime.md` → `Spec-Ecosystem-And-Trust-Chain.md` |
| Security Auditor / CISO | `Spec-Core-Cryptography-And-Mesh.md` + `Spec-Identity-And-Governance.md` |
| IT Admin | `Spec-Identity-And-Governance.md` + `Spec-Ecosystem-And-Trust-Chain.md` |
| AI / ML Engineer | `Spec-Enterprise-Secure-Enclave.md` |
| System Architect | Tất cả 7 spec files |

| Spec File | Domain | Mô tả |
|---|---|---|
| `Spec-Core-Cryptography-And-Mesh.md` | TERA-CORE | MLS, PQ-KEM, Hardware Root of Trust, Survival Mesh |
| `Spec-Dual-Sync-And-Local-Storage.md` | TERA-SYNC | CRDT DAG, SQLite WAL, Blob CAS, ZK Memory Agent |
| `Spec-Wasm-Tapp-Runtime.md` | TERA-RUNTIME | WASM dual-engine, Host ABI, Event Bus, Background Exec |
| `Spec-Enterprise-Secure-Enclave.md` | TERA-ENCLAVE | AI BYOM, ZK Memory Agent, PII Redaction |
| `Spec-Identity-And-Governance.md` | TERA-GOV | OPA ABAC, SCIM/OIDC/SAML, Audit Trail, RBAC |
| `Spec-Client-IPC-And-UI-Bridge.md` | TERA-CLIENT | FFI Token Protocol, IPC Data Plane, CoreSignals |
| `Spec-Ecosystem-And-Trust-Chain.md` | TERA-ECO | App Signing PKI, Registry, MDM, Kill-switch |

---

## Quy trình CI/CD

### Gates bắt buộc (blocker trước khi merge)

```bash
# Security
cargo clippy -- -D tera_ffi_raw_pointer   # Không raw ptr trong pub extern C
cargo miri test --test zeroize_verification  # ZeroizeOnDrop coverage
cargo audit --deny warnings               # RUSTSEC advisory check
gitleaks detect --source . --exit-code 1  # Secret leak detection
trivy image --exit-code 1 --severity CRITICAL  # Container CVE scan

# Correctness
cargo nextest run --all-features          # Full test suite
cargo test --test wasm_parity             # wasm3 vs wasmtime semantic identity
cargo test --test crdt_dedup_contract     # CRDT inbound deduplication

# Build integrity
ops/verify-reproducible-build.sh
ops/generate-sbom.sh && cosign sign-blob ...
```

### Chaos Engineering

40 kịch bản kiểm thử bao gồm:

- Network partition + Mesh failover
- Jetsam kill mid-WAL write (iOS)
- Border Node sudden power loss (EMDP trigger)
- BLE congestion + concurrent file transfer
- WAL concurrent write race (Windows NTFS)
- AppArmor/SELinux memory lock denial (Linux)
- WASM sandbox OOM + state recovery

Toàn bộ 40 scenarios phải pass trước Gov/Military contract.

---

## Platform support

| Platform | Client | Relay | Gov/Military |
|---|---|---|---|
| iOS 15+ | ✅ | — | ✅ |
| Android 9+ | ✅ | — | ✅ |
| HarmonyOS 3+ | ✅ | — | ❌ (HMS limitation) |
| macOS 13+ | ✅ | ✅ | ✅ |
| Windows 10+ | ✅ | ✅ | ✅ |
| Ubuntu 22.04 LTS | ✅ | ✅ | ✅ |
| RHEL 9 | ✅ | ✅ | ✅ |

**Lưu ý Huawei:** HMS Push không hỗ trợ `data-only` message type. Huawei devices dùng Polling Mode (CRL ≤ 4 giờ) và không đủ điều kiện cho Gov/Military tier. SCIM < 30s SLA không được đảm bảo.

---

## License

| Component | License | Auditable by |
|---|---|---|
| `terachat-core` (Crypto, MLS, CRDT, Mesh) | AGPLv3 | Public — Gov/Bank có thể tự compile và audit |
| `terachat-ui` (Tauri, Flutter frontend) | Apache 2.0 | Public |
| `terachat-license-guard` | BSL (Business Source License) | Không public |
| Admin Console | Proprietary | Không public |

Lõi mật mã AGPLv3 là cam kết với khách hàng Gov/Military: không có backdoor, có thể verify bằng toán học.

---

## Liên hệ

- **Enterprise Sales:** Liên hệ trực tiếp để nhận License JWT và bắt đầu pilot
- **Technical Support:** Dedicated 24/7 cho gói Enterprise+
- **Security Disclosure:** Báo cáo lỗ hổng bảo mật qua kênh bảo mật riêng

---

*TeraChat — Trao lại chủ quyền số cho người tiên phong.*

*Enterprise Messaging Platform · v1.0.0 · Q2 2026 · Confidential*
