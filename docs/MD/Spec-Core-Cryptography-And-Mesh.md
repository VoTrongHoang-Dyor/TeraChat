# Spec-Core-Cryptography-And-Mesh.md — TeraChat Enterprise Platform

```yaml
# DOCUMENT IDENTITY
id: "TERA-CORE"
title: "TeraChat — Core Cryptography & Mesh Network Specification"
version: "1.0.0"
status: "ACTIVE"
audience: "System Architect, Backend Engineer, Security / Cryptography Engineer, Rust Core Dev"
purpose: "Đặc tả toàn bộ primitives mật mã học, MLS E2EE, Hybrid PQ-KEM, Hardware Root of Trust, và Survival Mesh Networking. Đây là chân lý mật mã (Cryptographic Truth) của nền tảng."
depends_on: []
constraints_global:
  - "ZeroizeOnDrop bắt buộc cho mọi struct giữ key material"
  - "Không mlock() trên iOS — dùng kCFAllocatorMallocZone + ZeroizeOnDrop"
  - "Mọi FFI endpoint KHÔNG trả raw ptr — dùng Token Protocol"
  - "Ed25519 signed, append-only Audit Log — không thể delete/modify"
  - "Mọi crypto đi qua ring crate hoặc RustCrypto — không self-implement"
  - "Mọi Private Key phải nằm trong Secure Enclave (iOS/macOS) / StrongBox (Android) / TPM 2.0 (Desktop)"

```

> **Status:** `ACTIVE — Implementation Reference`
> **Audience:** Backend Engineer · Security / Cryptography Engineer · Rust Core Dev
> **Last Updated:** 2026-03-29
> **Consumed By:** → TERA-SYNC · → TERA-RUNTIME · → TERA-GOV · → TERA-CLIENT · → TERA-ENCLAVE

---

## §1 — EXECUTIVE SUMMARY & TRUST BOUNDARIES

### 1.1 Mục tiêu & Trách nhiệm

File này **chịu trách nhiệm** cho:

- Toàn bộ cryptographic primitives (MLS, PQ-KEM, HKMS)
- Hardware Root of Trust (Secure Enclave, StrongBox, TPM 2.0)
- Survival Mesh Networking (BLE 5.0, Wi-Fi Direct, EMDP)
- Binary transparency & update integrity

File này **KHÔNG chịu trách nhiệm** cho:

- WASM runtime & .tapp isolation → `TERA-RUNTIME`
- Storage & Sync architecture → `TERA-SYNC`
- UI/FFI bridge → `TERA-CLIENT`
- AI Enclave → `TERA-ENCLAVE`
- Identity governance & OPA → `TERA-GOV`

### 1.2 3 Đảm bảo Cốt lõi

1. **Server không thể đọc nội dung** — ciphertext-only relay.
2. **Mesh P2P duy trì liên lạc khi mất Internet hoàn toàn.**
3. **Mọi key material tự hủy ngay khi scope kết thúc** — ZeroizeOnDrop everywhere.

### 1.3 Trust Boundaries

| Boundary | Bên trong tin tưởng | Bên ngoài không tin tưởng |
|---|---|---|
| Secure Enclave / StrongBox / TPM | Private key ops | RAM, OS, Admin |
| Rust Core (Crypto domain) | Crypto logic, Signing | UI layer, WASM sandbox |
| Mesh Peer | Relaying signed packets | Unsigned/unverified data |
| Blind Relay VPS | Routing ciphertext | Plaintext content |

---

## §2 — SYSTEM ARCHITECTURE

### 2.1 Lõi Rust Độc Tài — Shared Core Philosophy

- **Lõi Rust (TeraChat Core)** nắm giữ 100% sinh mệnh: MLS E2EE, SQLCipher I/O, P2P Mesh, CRDT Sync. Biên dịch ra native binary cho mọi platform.
- **Tầng UI (Flutter / Tauri):** Pure Renderer — cấm tuyệt đối port Crypto/Business Logic lên Dart/JS Thread.
- **IPC — Tách Control/Data Plane:**
  - _Control Plane:_ Protobuf qua FFI/JSI — lệnh nhỏ <1KB.
  - _Data Plane:_ Dart FFI TypedData (Mobile) / `SharedArrayBuffer` (Desktop) — Zero-Copy, throughput ~400–500MB/s.
- **Unidirectional State Sync:** Rust bắn signal `StateChanged(table, version)` qua IPC. UI kéo snapshot tại thời điểm rảnh — không polling, không push JSON cục.

### 2.2 Blind Routing & Zero-Knowledge

- Server là **Blind Relay** — chỉ thấy: `destination_device_id`, `blob_size`, `timestamp`.
- **Sealed Sender:** Header người gửi mã hóa bằng public key người nhận — Server không biết who-to-whom.
- **Oblivious CAS Routing:** Batch 4–10 `Fake_CAS_Hashes` khi gửi hash query (Chaffing). Tra qua Mixnet Proxy Endpoint, không đính `User_ID`.
- **MinIO Blind Storage:** Lưu file theo `cas_hash` path — không biết tên file thực.

### 2.3 Deployment Topologies

```text
[Global Edge / GeoDNS Routing]
         │
         ├──> [Tier A: Mac mini On-Premise Cluster — Recommended]
         │      ├─ 1–N Mac mini nodes (Apple Silicon M2 Pro / M4 Pro)
         │      ├─ NAS Synology / QNAP (SMB3/NFS4.1) — Blob + Memory Store
         │      ├─ ZK Memory Agent (local LLM consolidation)
         │      ├─ TeraRelay Rust Daemon (blind router)
         │      └─ Mesh-consensus HA (1 Mac = 99.9%, 2 Mac = 99.999%)
         │
         ├──> [Tier B: VPS Cloud Relay — Legacy / Remote Branch]
         │      ├─ Single-Binary Rust Daemon (blind relay only)
         │      ├─ No AI inference, no Memory Agent
         │      └─ HA TURN Array (WebRTC Relay, Floating IP)
         │
         ├──> [Tier C: Federation Bridge]
         │      ├─ mTLS Mutual Auth (PKI nội bộ)
         │      └─ Mac mini Cluster A ↔ Cluster B giao tiếp an toàn
         │
         └──> [Tier D: Air-Gapped Bare-Metal]
                ├─ Mac mini + NAS hoàn toàn offline
                └─ HSM FIPS 140-3 cho key ceremony
```

| Quy mô | Topology | Hardware | Setup time |
|---|---|---|---|
| Solo ≤ 50 | 1 Mac mini + 1 NAS 4TB | Mac mini M2 8GB + Synology DS223 | 15 phút |
| SME ≤ 500 | 1 Mac mini M2 Pro 32GB + 1 NAS 16TB | Mac mini M2 Pro + Synology DS923+ | 30 phút |
| Enterprise ≤ 5,000 | 2 Mac mini M4 Pro 48GB (cluster) + 1 NAS 32TB | 2× Mac mini M4 Pro + Synology HD6500 | 1 giờ |
| Gov Air-gapped | 2 Mac mini + NAS + HSM PKCS#11 | Existing hardware | 4 giờ |
| Remote Branch | VPS Rust relay only (no AI) | $6–48/tháng VPS | 5 phút |

---

## §3 — DATA MODEL & ENCRYPTION STATE

### 3.1 Domain: Cryptographic Identity

| Object | Type | Storage | Lifecycle | Security Constraint |
|---|---|---|---|---|
| `DeviceIdentityKey` | Ed25519 Key Pair | Secure Enclave / StrongBox | Permanent (hardware-bound) | Không export. Ký/derive only. |
| `Company_Key` | AES-256-GCM Root Key | HKMS (wrapped by DeviceKey) | Per-workspace, rotated on member exit | Không rời thiết bị thành viên |
| `Epoch_Key` | MLS Leaf Key | RAM (Userspace) | Per MLS Epoch, zeroized on rotation | ZeroizeOnDrop mandatory |
| `ChunkKey` | AES-256-GCM Ephemeral | Rust `ZeroizeOnDrop` struct | 1 chunk (~2MB) lifetime | Zeroized immediately after use |
| `Session_Key` | ECDH Curve25519 Derived | RAM (Userspace) | Per session, zeroized after disconnect | ZeroizeOnDrop mandatory |
| `Push_Key` | AES-256-GCM Symmetric | Shared Keychain (iOS) / StrongBox (Android) | Per push_epoch, OOB from MLS | Versioned: push_key_version (u32) |
| `Master_Unlock_Key` | KDF output | RAM only (<100ms) | Duration of license validation op | ZeroizeOnDrop; never persisted |

### 3.2 Domain: MLS Session Objects

| Object | Type | Storage | Lifecycle | Security Constraint |
|---|---|---|---|---|
| `KeyPackage` | MLS RFC 9420 Struct | Server (public) / Local DB | Refreshed periodically | Public info only |
| `Welcome_Packet` | ECIES-encrypted payload | Encrypted in-flight | Single use, consumed on join | ECIES Curve25519, receiver-keyed |
| `TreeKEM_Update_Path` | MLS tree delta | In-memory, broadcast | Per epoch rotation | Ed25519 signed |
| `Epoch_Ratchet` | Sequence counter | `hot_dag.db` | Monotonically increasing | Tamper-evident via append-only log |
| `Enterprise_Escrow_Key` | Shamir-split AES-256 | M-of-N hardware tokens | Per KMS bootstrap | 3-of-5 Shamir; no single holder |

### 3.3 Domain: Mesh Network Objects

| Object | Type | Storage | Lifecycle | Security Constraint |
|---|---|---|---|---|
| `BLE_Stealth_Beacon` | 31-byte BLE Adv PDU | Broadcast-only (air) | Ephemeral per scan cycle | HMAC-wrapped; no static identifiers |
| `Identity_Commitment` | `HMAC(R, PK_identity)[0:8]` | Embedded in Beacon | Per session nonce | R rotated every 5min |
| `Shun_Record` | `{Node_ID, Ed25519_Sig, HLC}` | `hot_dag.db` broadcast | Until node is rehabilitated | Enterprise CA signed |
| `MergeCandidate` | `{Node_ID, BLAKE3_Hash, HLC}` | RAM only | Duration of Split-brain resolution | Ephemeral; no persist |
| `Hash_Frontier` | `{Vector_Clock, Root_Hash}` | `hot_dag.db` | Updated on every Gossip round | BLAKE3 integrity |
| `EmdpKeyEscrow` | AES-256 session key | BLE Control Plane (in-flight) | EMDP session (max 60min) | ECIES-encrypted to relay device pubkey |

### 3.4 Domain: Recovery & Audit Objects

| Object | Type | Storage | Lifecycle | Security Constraint |
|---|---|---|---|---|
| `Snapshot_CAS` | Content-Addressable Hash | `TeraVault VFS` | Permanent | SHA-256 integrity |
| `Monotonic_Counter` | TPM 2.0 hardware counter | TPM chip register | Hardware-bound, tamper-evident | Rollback-proof; hardware only |
| `Audit_Log_Entry` | `{device_id, timestamp, payload_hash, ed25519_sig}` | Append-only CRDT chain | Permanent | Ed25519 signed; cannot delete |

---

## §4 — PROTOCOL & EXECUTION CONTRACT

### 4.1 Hardware Root of Trust & Anti-Extraction (F-01)

**Key Mechanisms:**

- **Biometric-Bound Cryptographic Handshake:** `Device_Key` sinh với `kSecAccessControlBiometryCurrentSet`. Biometric required cho mọi signing op.
- **Enterprise PIN Độc lập & Dual-Wrapped KEK:** Argon2id (0.5s CPU) sinh `Fallback_KEK`. Device_Key bọc 2 bản độc lập.
- **Ruthless Cryptographic Self-Destruct:** Counter `Failed_PIN_Attempts` (max 5) → Crypto-Shredding toàn bộ Local DB → Factory Reset.

**Platform Signing APIs:**

| Platform | API | Mechanism |
|---|---|---|
| 📱 iOS | `LAContext` | `SecKeyCreateSignature` + `.biometryCurrentSet` |
| 📱 Android | `BiometricPrompt` | `setUserAuthenticationRequired(true)` + Hardware Keystore |
| 💻 macOS | `CryptoTokenKit` | `kSecAttrTokenIDSecureEnclave` |
| 💻🖥️ Windows | `CNG` | `Microsoft Platform Crypto Provider (TPM 2.0)` + `NCryptSignHash` |
| 🗄️ Gov-Grade | PKCS#11 | SafeNet/Viettel/VNPT CA — Rust `pkcs11` crate |

**Remote Attestation:**

| Platform | API | Requirement |
|---|---|---|
| 📱 iOS | `DCAppAttestService` | App gốc, không Jailbreak |
| 📱 Android | `Play Integrity API` | `MEETS_STRONG_INTEGRITY` |
| 📱 Huawei | HMS SafetyDetect `DeviceIntegrity()` | TrustZone (ARM) |
| 💻🖥️ Windows | `TPM 2.0 Health Attestation` | PCR check + BitLocker ON |
| 💻 macOS | Notarization + Hardened Runtime | Secure Enclave |

### 4.2 Key Management System — HKMS (F-04)

**Key Hierarchy:**

```text
[Master Key]  — HSM / TPM / Secure Enclave (không rời chip)
      └──> [KEK]  — giải mã trong RAM, ZeroizeOnDrop protected
                └──> [DEK]  — mã hóa nội dung thực tế
                          └──> [DB / File / Channel / API Key]
```

**KMS Bootstrap Ritual:**

1. Workspace khởi tạo → App sinh `terachat_master_<domain>.terakey` (Master Key bọc AES-256 bằng Argon2id từ Admin password).
2. Lõi Rust **Block** tạo Database cho đến khi Admin xác nhận lưu Key Backup.
3. Shamir 3-of-5 phân phát vào YubiKey/Smartcard của C-Level.
4. HSM Decrementing Monotonic Counter: mỗi lần issue cert → counter giảm → chống cloning.

**Dead Man Switch:**

- Mỗi unlock DB → Counter++. Server lưu "Last Valid Counter". `Counter < Server's Value` → từ chối + Self-Destruct.
- **Offline Grace:** Tối đa theo `OfflineTTLProfile` (mặc định 72h consumer, 720h GovMilitary).

### 4.3 Message Layer Security — MLS RFC 9420 (F-05)

**Key Properties:**

- **TreeKEM:** Mã hóa O(log n) cho nhóm 5000+ user.
- **Self-Healing (Epoch Rotation):** Xoay Session Key định kỳ theo thời gian hoặc khi số lượng tin nhắn đạt ngưỡng (ví dụ: 10,000 requests) để duy trì Forward Secrecy, và rotate ngay lập tức khi member rời nhóm.
- **Sealed Sender:** Server không biết người gửi.
- **Multi-Device Queue:** N bản copy (1/device). Device ACK → xóa bản đó. TTL 14 ngày → Crypto-Shred KEK.
- **Enterprise Escrow KEM:** Shamir's Secret Sharing — M-of-N Recovery Key cho Supervisors.

**OOB Symmetric Push Ratchet (NSE isolation):**

- `Push_Key = HKDF(Company_Key, "push-ratchet" || chat_id || push_epoch)` — hash-chain một chiều, **độc lập với MLS TreeKEM**.
- NSE chỉ đọc `Push_Key_current` từ Shared Keychain → giải mã payload O(1) RAM (<5MB) → `ZeroizeOnDrop`.
- NSE **không bao giờ** tái dựng TreeKEM. Main App chịu trách nhiệm ratchet khi Foreground.

### 4.4 Hybrid PQ-KEM Kyber768 (F-06)

**Key Derivation:**

```text
Final_Session_Key = HKDF(X25519_Shared || Kyber768_Shared)
```

Tuân thủ chuẩn CNSA 2.0 và NSA SNDL/HNDL requirements.

**Bandwidth Optimization:**

- **Quantum Checkpoints:** ML-KEM payload (~1.18KB) chỉ đính kèm vào `KeyPackage` MLS Handshake hoặc mỗi 10.000 tin nhắn.
- **Survival Mesh Fragmentation:** BLE 5.0 MTU ~512 bytes → băm nhỏ Kyber blob 1.18KB thành mảnh 400 bytes + Sequence ID + FEC (RaptorQ RFC 6330).

### 4.5 ALPN & Protocol Fallback (F-08)

```text
ALPN Protocol State Machine (Auto-negotiation, < 50ms total)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Step 1 │ QUIC/HTTP3 over UDP:443
       │ ◉ ACK within 50ms → ONLINE_QUIC (0-RTT, ~30ms)
       │ ✗ No ACK / Firewall DROP → fallback Step 2
       ▼
Step 2 │ gRPC over HTTP/2 (TCP:443)
       │ ◉ TLS handshake OK → ONLINE_GRPC (1-RTT, ~80ms)
       │ ✗ → fallback Step 3
       ▼
Step 3 │ WebSocket Secure (wss:// over TCP:443)
       │ ◉ WS Upgrade OK → ONLINE_WSS (1-RTT, ~120ms)
       │ ✗ All transports fail → MESH_MODE (BLE/Wi-Fi Direct)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**Luật Mesh Mode Degradation:**
- Nếu kết nối WebRTC trực tiếp fail, hệ thống sẽ downgrade qua TLS-over-WebSocket đi qua Relay Node.
- *Nguyên tắc tối thượng:* Performance degradation is acceptable (tăng latency), but Security degradation is strictly PROHIBITED (Toàn bộ MLS E2EE, Zero-Knowledge Routing, ZeroizeOnDrop đều phải được bảo toàn 100%).
```

---

## §5 — STATE MACHINE

### 5.1 Mesh Network State

```text
[Internet Available] ─────────────────────────────────────┐
                                                           │ Network Lost
                                                           ▼
[BLE Scan] ──peers found──> [BLE Active Mesh]
     │                              │
     │ no peers                     │ Internet restored          │ Border Node timeout
     ▼                              ▼                            │ + No Desktop present
[Standby - SOS only]         [Hybrid Sync]                       ▼
                              (send queued CRDT events)  [EMDP_FORCED]
                                                          (text-only, Key Escrow transferred)
                                                                  │ New Border Node detected
                                                                  ▼
                                                          [Hybrid Sync resume]
```

**EMDP_FORCED Trigger Conditions (Border Node Failure):**

```
EMDP Trigger Condition (Border Node Failure):
  Border Node (internet-capable) mất kết nối > T_border_timeout (default: 30s)
  VÀ không có Border Node backup nào trong mesh:
    → Rust Core trên mọi thiết bị emit CoreSignal::BorderNodeLost
    → Nếu không có Desktop Super Node trong mesh:
        → EMDP tự động kích hoạt (không cần manual trigger)
        → iOS với pin cao nhất nhận EmdpKeyEscrow từ peer cuối cùng có session key

Border Node Election Recovery:
  Nếu một thiết bị mới có internet xuất hiện trong vòng EMDP_TTL (60 phút):
    → Tự động promote thành Border Node
    → EMDP terminated cleanly, MLS epoch sync resume
    → EmdpTerminationProof emit với reason: BORDER_RESTORED

Failure Case — Ungraceful Border Node Shutdown:
  Nếu Key Escrow chưa kịp transfer trước khi Border Node mất điện đột ngột:
    → Session key bị mất theo máy đó
    → Recovery path: Desktop re-derive session key từ Company_Key khi có mạng
    → Emit: EMDP_STALE_KEY_RECOVERY signal (xem §10)
    → UI: "Encryption key expired — re-establishing secure channel"
```

### 5.2 Key Lifecycle State

```text
[Key Generation] ──hardware bound──> [Active Key]
                                           │
                              epoch_rotation / member_exit
                                           ▼
                                    [Zeroized Key]
                                    (ZeroizeOnDrop)
```

### 5.3 Device Recovery State (F-02)

```text
[Nhân viên mất thiết bị]
        │
        ▼
[Báo HR/Admin] ──BLE Physical Presence──▶ [2/3 Admin xác nhận Biometric]
        │                                         │
        ▼                                         ▼
[QR Code Recovery Ticket]            [Lagrange Interpolation in mlock arena]
        │                                         │
        ▼                                         ▼
[Thiết bị mới quét QR]              [Enterprise_Escrow_Key (RAM <100ms)]
        │                                         │
        └─────────────────────────────────────────┘
                          ▼
                [Append Audit Log → Epoch Rotation]
```

---

## §6 — API / IPC / EVENT BUS

### 6.1 Crypto Host ABI

```rust
// Host function ABI — WASM calls these, Rust implements
extern "C" {
    fn host_blake3_hash(data_ptr: *const u8, data_len: usize, out_ptr: *mut u8) -> i32;
    fn host_ed25519_sign(key_id: u64, msg_ptr: *const u8, msg_len: usize, sig_out: *mut u8) -> i32;
    fn host_aes256gcm_encrypt(
        key_id: u64, nonce_ptr: *const u8, plaintext_ptr: *const u8,
        plaintext_len: usize, ciphertext_out: *mut u8
    ) -> i32;
}
// key_id references a ZeroizeOnDrop key in Rust Core — never crosses boundary as bytes
```

### 6.2 IPC Signals (Crypto Domain)

| Signal | Trigger | Consumer |
|---|---|---|
| `SecurityEvent::HardwareAttestationFailed` | Attestation fail | TERA-CLIENT (show alert) |
| `MlsEpochRotated(epoch_n)` | Member join/leave | TERA-SYNC |
| `MlsKeyDesync(chat_id)` | Push key version mismatch | TERA-SYNC |
| `NetworkProtocolChanged(old, new)` | ALPN fallback | TERA-CLIENT |
| `SecurityEvent::DMA_INTRUSION` | PCIe device join | Self-destruct pipeline |

---

## §7 — PLATFORM MATRIX & CONSTRAINTS

| Feature | 📱 iOS | 📱 Android | 💻 macOS | 🖥️ Desktop | ☁️ VPS |
|---|---|---|---|---|---|
| Secure Key Storage | Secure Enclave | StrongBox Keymaster | Secure Enclave (SEP) | TPM 2.0 | HSM PKCS#11 |
| Biometric Binding | FaceID/TouchID | BiometricPrompt | TouchID | TPM UserPresence | N/A |
| mlock() | ❌ (use kCFAllocatorMallocZone) | ✅ | ✅ | ✅ | ✅ |
| BLE Mesh | ✅ (background limited) | ✅ | ✅ | ✅ | ❌ |
| Remote Attestation | DCAppAttestService | Play Integrity API | Notarization | TPM Health | N/A |
| PQ-KEM Support | wasm3 (interpreter) | wasmtime JIT | wasmtime JIT | wasmtime JIT | N/A |

---

## §8 — NON-FUNCTIONAL REQUIREMENTS (NFR)

| Requirement | Target | Measurement |
|---|---|---|
| MLS Epoch rotation (10k users) | < 1s | P99 latency |
| E2EE message latency (client→client) | < 50ms | P95 latency |
| PQ-KEM handshake overhead | < 20ms additional | vs classical X25519 |
| Key zeroization time | < 1ms | ZeroizeOnDrop benchmark |
| BLE discovery time | < 5s | Cold start mesh |
| ALPN fallback negotiation | < 50ms total | All 3 steps combined |
| Audit log write | < 10ms | Per entry |
| Memory: WASM crypto overhead | < 5MB per call | Peak RAM in call |

---

## §9 — SECURITY & THREAT MODEL

### 9.1 Attack Vectors & Mitigations

| Attack | Vector | Mitigation |
|---|---|---|
| Key extraction from memory | iOS mlock bypass / cold boot | kCFAllocatorMallocZone + ZeroizeOnDrop on scope exit |
| MITM on relay | TLS downgrade | Certificate pinning (SHA-256 SPKI hardcoded) + Anti-Downgrade |
| Replay attack | Old CRDT events re-injected | HLC Timestamp monotonicity + Tombstone Stubs |
| Sybil attack in Mesh | Attacker floods fake Node_IDs | Enterprise CA signed Shun_Records only |
| Store-Now-Decrypt-Later | Harvest ciphertext now, decrypt with quantum | Hybrid PQ-KEM (X25519 + ML-KEM-768) |
| DMA attack at PCIe | Rogue device joins PCIe bus | IOMMU check at startup → `SecurityEvent::DMA_INTRUSION` → key shred |
| Attestation bypass | Rooted/jailbroken device | `MEETS_BASIC_INTEGRITY` only → reject + Remote Wipe |
| Admin key theft | Single admin controls all | Shamir 3-of-5 — no single holder of `Enterprise_Escrow_Key` |

### 9.2 Invariants (Never Violate)

- ❌ Không tự implement crypto. Chỉ dùng `ring` hoặc `RustCrypto`.
- ❌ Không persist plaintext key lên disk.
- ❌ Không truyền raw pointer qua FFI.
- ❌ Không cấp file handle OS gốc cho WASM.
- ❌ Không trust CA công cộng cho mTLS nội bộ.

---

## §10 — FAILURE MODEL & RECOVERY

| Failure | Detection | Recovery |
|---|---|---|
| Key zeroization crash mid-write | `XpcTransactionJournal` PENDING state | Replay journal on restart; re-zeroize |
| MLS epoch rotation timeout | `epoch_rotation_latency_ms` > 2s | Split group into logical sub-groups; retry epoch |
| BLE beacon collision | HMAC validation fail | Rotate session nonce R; rescan |
| Attestation API unavailable (offline) | No DCAppAttestService response | Fallback to local PIN verification; log `AttestationSkipped` |
| Push key desync | Key version mismatch on NSE wakeup | Cache raw ciphertext → `content-available:1` → Main App decrypt |
| DMA intrusion detected | `SecurityEvent::DMA_INTRUSION` | Immediate `Session_Key` zeroize → lockscreen → alert Admin |
| HSM unreachable at KMS bootstrap | Bootstrap blocks | Admin must restore from Shamir shard quorum (3-of-5) |
| Monotonic counter rollback | `Counter < Server's Last Valid Counter` | Reject + Self-Destruct + notify Admin |
| Border Node (Starlink) mất điện đột ngột, không có Desktop | `CoreSignal::BorderNodeLost` sau T=30s | EMDP_FORCED kích hoạt; text-only mode; iOS pin cao nhất nhận EmdpKeyEscrow; UI hiển thị "Offline secure mode" |
| Key Escrow chưa kịp transfer khi Border Node mất (<5s window) | Escrow transfer timeout; ACK không nhận được | `EMDP_STALE_KEY_RECOVERY` signal emit; session suspended; UI cảnh báo; Desktop re-derive từ Company_Key khi reconnect; ZeroizeOnDrop được gọi ngay |


## §11 — ARCHITECTURAL INVARIANTS & AUDIT RESOLUTIONS (CRYPTOGRAPHY & MESH)

### 11.1 MLS Epoch Rotation vs. Long-Running WASM Sessions
**Constraint:** Stateful Tapps preserving older epoch parameters will catastrophically fail decryption following a scheduled member rotation or timeout epoch boundary.
**Resolution:** Rust Core broadcasts a explicit `CoreSignal::EpochRotated { session_ids_affected }`. Tapps holding active cross-epoch multi-message state must checkpoint, discard keys (`ZeroizedOnDrop`), and request re-hydration to receive the superseding `Epoch_Key`.

### 11.2 Strict Engineering Guardrails (Mesh Degradation Limits)
- **Rule 4 (No Silent Security Degradation):** P2P WebRTC fallback (e.g., TLS-via-WebSocket) authorizes routing latency compromises, but strictly rejects any downgrades to the MLS cryptographic protocol. If a secure session verification fails in Mesh Mode, Rust Core generates a `CoreSignal::FeatureDegradedInMesh` error instead of quietly dropping parameters. Fallback logic is fully managed by the Host and never delegated to the Tapp space.

### 11.3 NSE Serial Queue — Concurrent Push Flood Protection (CRIT-02 / CRIT-C Audit Fix)
**Constraint:** iOS NSE với ~5MB peak RAM per invocation có thể bị forked thành nhiều instances khi nhận burst notifications từ nhiều MLS group đồng thời (10 groups × 5MB = 50MB > iOS 20MB ceiling → Jetsam kill toàn bộ → zero notification rendered).

**Previous approach (REJECTED — TOCTOU race):** Shared Keychain semaphore có fundamental race condition: iOS có thể launch nhiều NSE process đồng thời TRƯỚC KHI process A có thời gian write lock vào Keychain. Process B launch và check Keychain — không thấy lock — proceed in parallel. Đây là TOCTOU (Time-of-Check-Time-of-Use) vulnerability. Ngoài ra, `nse_staging.db` là SQLite bị concurrent multi-process write — reintroduce vấn đề Saga pattern đang giải quyết.

**Resolution (POSIX flock + Ring Buffer):** Thay Keychain semaphore bằng POSIX advisory file lock (`flock()`) trên file trong App Group container. File lock là process-safe và atomic tại kernel level.

```rust
// POSIX flock trên App Group container file:
// Path: /AppGroup/com.terachat.nse.lock
pub fn handle_notification(payload: &EncryptedPayload) -> NotificationContent {
    let lock_path = app_group_path().join("nse.lock");
    match flock_try_acquire(&lock_path) {
        Acquired(guard) => {
            // Decrypt và return notification
            guard // auto-release khi drop
        }
        Contended => {
            // Write raw ciphertext vào memory-mapped ring buffer
            // (KHÔNG phải SQLite — tránh concurrent writer deadlock)
            // Path: /AppGroup/com.terachat.nse.ringbuf
            ring_buffer_write(payload.raw_ciphertext);
            // Return generic notification
            NotificationContent::generic("Tin nhắn mới")
        }
    }
}
// Main App drains ring buffer khi foreground:
// pub fn on_foreground() { nse_ring_buffer_drain_and_decrypt(); }
```

### 11.4 EMDP Epoch Freeze Protocol (CRIT-04)
**Constraint:** Khi EMDP session active, nếu một member khác rời MLS group và trigger epoch rotation, `EmdpKeyEscrow` đang giữ session key của epoch cũ đã bị invalidate. Desktop reconnect cố merge sẽ dẫn đến decryption failure trên toàn bộ messages được buffer bởi iOS Tactical Relay → permanent message loss.
**Resolution:** Khi EMDP session active, Rust Core set `emdp_epoch_freeze = true` trên server relay. MLS `Update_Path` vẫn được prepared nhưng **Epoch_Ratchet không advance** cho các messages trong Tactical Relay buffer. Epoch chỉ advance sau khi Desktop reconnect và emit `EmdpSessionTerminated` signal, tại đó Tactical Relay buffer được re-encrypted với new epoch key trước khi delivery ("deferred epoch commit").

### 11.5 mlock() Hard Requirement on Linux (XPLAT-04)
**Constraint:** TERA-CORE §7 nêu `mlock()` available trên Linux. Nếu `mlock()` bị denied, key material có thể bị swap ra disk — đây không phải "degradation," đây là **security violation**.
**Resolution:** Rust Core phải refuse to start nếu `mlock()` không khả dụng, không silent degrade:

```rust
fn check_memory_security() -> Result<(), StartupError> {
    #[cfg(target_os = "linux")]
    {
        if unsafe { libc::mlock(test_ptr, 64) } != 0 {
            return Err(StartupError::MemoryLockUnavailable {
                reason: "mlock() denied by OS/AppArmor. Cannot guarantee key material security.",
                action: "Configure AppArmor profile to allow mlock, or run with CAP_IPC_LOCK."
            });
        }
    }
    Ok(())
}
```

### 11.6 PENDING_SECURE_CHANNEL — SC-35 Key Escrow Race (CRIT-05 extension)
**Constraint:** Khi Key Escrow chưa kịp transfer (SC-35 race condition), cần balance giữa bảo mật tuyệt đối và UX không gây hoang mang.
**Resolution:** Tầng Network bị suspend hoàn toàn (không byte nào rời thiết bị). Tầng UI: user tiếp tục gõ bình thường, tin nhắn mã hóa bằng ephemeral key (RAM/HSM) và đưa vào `Outbox Queue`. UI hiển thị "Securing channel..." indicator thay vì error. Khi Escrow hoàn tất: Core re-key, mã hóa lại payload trong queue và flush lên Mesh. Giới hạn bắt buộc:
- `Max-Queue-Size`: chặn nhập liệu nếu đầy để tránh OOM.
- `TTL = 24h`: nếu Key Escrow không complete sau 24h, messages expire và UI hiển thị "Messages could not be sent securely — please reconnect to a secure channel."
- Nếu app crash khi Queue chưa flush: dữ liệu mất — UI phải cảnh báo user không tắt app khi đang "Securing channel...".

### 11.7 Composite Key Derivation — Defense in Depth (Hardware Zero-Day)
**Constraint:** `DeviceIdentityKey` phụ thuộc hoàn toàn vào Secure Enclave/TPM vi phạm Zero-Trust (trust 100% vào một vendor). Nếu hardware bị compromise, toàn bộ key material bị exposed.
**Resolution:** Áp dụng Defense in Depth — Key thực sự được sinh qua KDF kết hợp 3 yếu tố:

```
Session_Key = KDF(Hardware_Secret || License_JWT_Entropy || User_PIN_or_Biometric)
```

- `Hardware_Secret`: từ Secure Enclave / StrongBox / TPM (không export được)
- `License_JWT_Entropy`: derived từ tenant license JWT (server-controlled)
- `User_PIN_or_Biometric`: user-controlled factor

Nếu attacker crack được TPM, họ thu được `Hardware_Secret` nhưng vẫn cần License entropy và user PIN để reconstruct key. Dữ liệu mã hóa offline vẫn an toàn tuyệt đối dù phần cứng bị bẻ gãy.

### 11.8 CoreBootSequence Protocol — Startup Safety Guards (GAP-A, GAP-B, GAP-C)
**Constraint:** Nhiều critical invariants (Saga orphans, NSE ring buffer, WAL integrity) chỉ có thể được detected và recovered tại startup — không thể detect real-time mà không có significant overhead.
**Resolution:** Mỗi Rust Core startup, trước khi mở bất kỳ IPC channel nào, phải chạy tuần tự (< 300ms tổng):

```rust
pub async fn core_boot_sequence() -> Result<(), BootError> {
    // Phase 1: Saga integrity
    SagaRecoveryGuard::scan_and_recover().await?;
    
    // Phase 2: NSE ring buffer drain
    NseRingBufferDrain::flock_drain_to_decrypt_queue().await?;
    
    // Phase 3: WAL integrity
    WalIntegrityCheck::pragma_integrity_check_both_dbs().await?;
    
    // Chỉ sau đây mới mở IPC channels:
    IpcServer::open_channels().await
}
```

### 11.9 Binary Transparency Gossip Authentication (HIGH-7)
**Constraint:** TERA-CORE §4.5 định nghĩa Binary Transparency via BLAKE3 hash gossip qua Mesh. Gossip message KHÔNG yêu cầu sender authentication — chỉ original binary được signed. Malicious insider có thể broadcast fake `Global_Update_Log` entry với BLAKE3 hash giả, gây DoS (peers block legitimate module).
**Resolution:** Gossip message mang Binary Transparency hash phải được signed bằng TeraChat Root CA key:

```rust
pub struct BinaryTransparencyGossip {
    module_id: String,
    blake3_hash: Blake3Hash,
    /// PHẢI được sign bởi TeraChat Root CA — không phải chỉ per-device key
    root_ca_sig: Ed25519Sig,
    hlc: HLCTimestamp,
}
// Peers: reject nếu root_ca_sig invalid. Không trust gossip message không có CA signature.
```
