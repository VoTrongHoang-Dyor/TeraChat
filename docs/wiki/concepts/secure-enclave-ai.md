---
type: concept
created: 2026-05-10
updated: 2026-05-30
tags: [terachat, ai, secure-enclave, tpm, on-premise, local-ai, qwen, metal]
sources: [tera-enclave-spec, tera-core-spec, tera-gov-spec]
---

# Secure Enclave & AI Security

> **Cập nhật 2026-05-30:** Sửa 2 lỗi kỹ thuật nghiêm trọng từ phiên bản cũ:
> 1. **Model mặc định:** Gemma 4 → **Qwen2.5 (Metal/llama.cpp)** — Gemma 4 chưa tồn tại.
> 2. **SEP misuse:** Apple Secure Enclave **KHÔNG** chạy general computation (TreeKEM, CRDT). SEP = key material storage ONLY.

## Apple Secure Enclave — Đúng Vai trò

**Apple Secure Enclave Processor (SEP)** là một coprocessor nhỏ chuyên biệt với phạm vi rất hẹp:

| SEP **CÓ THỂ** làm | SEP **KHÔNG THỂ** làm |
|---|---|
| Sinh và lưu private keys | Chạy general computation (MLS, TreeKEM) |
| Thực hiện biometric verification | Xử lý CRDT merge |
| Ký / Verify bằng hardware-backed key | Chạy AI inference |
| Encrypt/Decrypt với hardware key | Được lập trình trực tiếp qua API công khai |

**Thực tế triển khai TeraChat:**
- `DeviceIdentityKey` được sealed trong SEP (iOS/macOS) / StrongBox (Android) / TPM 2.0 (Desktop)
- Mọi computation MLS, sync, AI đều chạy trên **main processor** trong **Rust Core process**
- Rust Core dùng memory isolation (process separation, OS-level sandboxing) để bảo vệ key material trong RAM
- ZeroizeOnDrop bắt buộc trên mọi struct chứa key material (Invariant I-02)

**Clarification quan trọng:** "SysAdmin không thể extract Session Keys" là đúng vì keys được sealed trong SEP/TPM — **không phải** vì computation chạy trong SEP. Một SysAdmin với root access vẫn có thể dump process memory của Rust Core và thấy giá trị trung gian computation, nhưng không thể extract raw key material vì chúng không bao giờ ra khỏi SEP.

## TEE (Trusted Execution Environment) — Cho Server

Trên Mac Mini server, TEE (Trusted Execution Environment) là cơ chế bảo vệ computation:
- **Apple Silicon Secure Enclave** trên Mac Mini: dùng cho key storage + Sequencer/Verifier trong Delegated Proposer
- **Intel TDX / AMD SEV-SNP**: Option B Cloud deployment cho >100K users
- TEE trên server verify và sequence TreeKEM proposals từ Fat Client — không compute TreeKEM

## Model AI Mặc định: Qwen2.5 (Metal/llama.cpp)

TeraChat bundled model mặc định là **Qwen2.5** của Alibaba, chạy via llama.cpp (Metal API trên Apple Silicon):

| Property | Specification |
|----------|---------------|
| Model | **Qwen2.5** (Alibaba open weights) |
| Runtime | **llama.cpp** + **Metal API** (Apple Silicon) / ONNX (Windows/Linux) |
| RAM Budget | 8–32 GB tùy variant (1.5B → 32B) |
| Execution | 100% on-device — no network call |
| Hardware | Tận dụng Apple Neural Engine + Unified Memory bandwidth |
| Tasks | Thread summarization, response drafting, classification, translation |

**Tại sao Qwen2.5 (không phải Gemma 4):**
- Open weights + commercial license phù hợp với enterprise deployment
- Hiệu năng tốt trên Apple Silicon qua Metal API
- llama.cpp native macOS daemon — không cần Docker, không overhead ảo hóa
- Model size phù hợp với Mac Mini M4 Pro 24-48GB Unified Memory

**Tại sao không dùng Gemma 4:** Gemma 4 chưa tồn tại tại thời điểm thiết kế hệ thống. Sử dụng Qwen2.5 là quyết định thực tế dựa trên model hiện có.

## The Local Appliance Model

```
┌─────────────────────────────────────────┐
│         CUSTOMER ON-PREMISE              │
│                                          │
│  Control Plane (Mac mini M4 Pro)         │
│  • TeraRelay routing                     │
│  • Event Log sync (tc-crdt-sync)         │
│  • ZK Memory Agent (daemon, UDS)         │
│  • Qwen2.5 via llama.cpp + Metal         │
│                                          │
│  Compute Plane (RTX Node - Optional)     │
│  • Heavy AI (Qwen2.5-32B bulk tasks)     │
│  • LAN-connected gRPC only               │
│                                          │
│  Data Plane (TrueNAS ECC)               │
│  • Encrypted Blobs (CAS)                 │
│  • Vector Indices (ZK Memory)            │
│  • Local AI Model Registry               │
└─────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│         EMPLOYEE DEVICE (Local AI)       │
│                                          │
│  Qwen2.5-0.5B / Whisper Tiny            │
│  • On-device NPU inference               │
│  • Fallback khi Mac Mini không available │
│                                          │
│  Open AI Framework (Host ABI)            │
│  • Enterprise register custom ONNX model │
│  • BLAKE3 integrity check on every load  │
└─────────────────────────────────────────┘
```

## Open AI Framework

TeraChat không lock enterprise vào single AI provider. Open AI Framework cho phép:

- **Register bất kỳ ONNX model** qua Admin Console
- **Bring your own API key** cho cloud models (Claude, GPT-4) — prompts proxied qua enterprise relay với PII redaction bắt buộc
- **Department/region scoping** — deploy specific models cho specific teams
- **Model integrity verification** — BLAKE3 hash checked on every load
- **Audit log** — mọi AI invocation được Ed25519 sign → immutable audit trail

Xem chi tiết: [[Open AI Framework]]

## ZK Memory Agent

Daemon riêng (`mlx-server` hoặc `llama-server`) chạy trên Mac Mini, giao tiếp qua Unix Domain Socket (không TCP):

- Vector embeddings sinh và lưu cục bộ trên NAS
- Không embedding egress — không gửi vectors lên cloud
- Consolidation chạy theo lịch (02:00 AM local time) để tránh thermal saturation

## Design Decisions (Q&A)

- **Tại sao on-device AI thay vì server-side?** Server-side AI sẽ nhìn thấy plaintext employee data — vi phạm Zero-Knowledge. Local execution = AI chạy trên machine sở hữu data. Trade-off: RAM cao hơn (8GB+ recommended).

- **Tại sao llama.cpp thay vì ONNX Runtime trên Mac?** llama.cpp gọi trực tiếp Metal API, tận dụng Apple Neural Engine và Unified Memory bandwidth của M-series. ONNX Runtime trên macOS không tận dụng được NPU. Docker/K8s Linux tạo overhead ảo hóa 30%. Chạy native daemon dưới launchd = kiểm soát tuyệt đối thermal và clock speed.

- **Tại sao Open Framework thay vì one bundled model?** Enterprise có AI needs đa dạng. Bank cần compliance model khác với manufacturing. Default mạnh (Qwen2.5) + open framework = cả simplicity lẫn flexibility. Trade-off: model compatibility matrix overhead.

- **SEP và computation có liên quan không?** Không. SEP = key storage hardware. Computation (MLS TreeKEM, CRDT merge, AI) chạy trên main processor. Bảo vệ computation dùng process isolation + ZeroizeOnDrop + OS memory sandbox, không phải SEP.
