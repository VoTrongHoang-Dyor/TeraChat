---
type: concept
created: 2026-05-15
updated: 2026-05-16
tags: [language, agent-context, invariants]
---

# Ubiquitous Language

Shared vocabulary for all TeraChat contributors — humans and AI agents. Use these terms consistently in code, documentation, and AI prompts.

## Core Domain Terms

### Crypto & Security

| Term (EN) | Term (VI) | Definition |
|-----------|-----------|------------|
| Epoch | Epoch | MLS key rotation cycle — each member add/remove creates a new epoch |
| KeyHandle | Tham chiếu khóa | Opaque reference to key material in the GlobalKeyArena (never a raw pointer) |
| KeyMaterial | Vật liệu khóa | Any struct containing cryptographic key bytes — MUST derive ZeroizeOnDrop |
| DataGrant | Quyền truy cập dữ liệu | Cryptographic permission for a .tapp to read specific data |
| Blind Router | Bộ định tuyến mù | TeraRelay routes ciphertext without ever seeing plaintext |
| Token Protocol | Giao thức Token | FFI safety: pass opaque `u64` tokens instead of raw pointers |
| BSL 1.1 | BSL 1.1 | Business Source License 1.1 — source-readable, competitive use prohibited, auto-converts to MIT after 4 years |
| TeraAiAdapter | Bộ điều hợp AI Tera | Trait abstracting AI backend: local MLX, remote API (Azure/AWS), or custom ONNX bundle — all behind unified interface |

### Messaging & Sync

| Term (EN) | Term (VI) | Definition |
|-----------|-----------|------------|
| CoreSignal | Tín hiệu lõi | Event pushed from Rust Core to UI layer (passive rendering) |
| Tombstone | Dấu xóa | CRDT soft-delete marker — messages are never truly deleted |
| HLC | Đồng hồ lai | Hybrid Logical Clock timestamp for causal ordering |
| WalEntry | Bản ghi WAL | Write-Ahead Log entry — the unit of replication between primary and secondary |
| CRDT DAG | DAG CRDT | Append-only directed acyclic graph for chat message sync |
| Epoch | Epoch | Also refers to CRDT epoch in sync context |

### Device & Network

| Term (EN) | Term (VI) | Definition |
|-----------|-----------|------------|
| DeviceIdentity | Định danh thiết bị | Unique cryptographic identity per device (Ed25519 keypair) |
| TeraLinkPeer | Đồng đẳng TeraLink | Another device reachable via TeraLink Fallback Network (any tier) |
| TeraLink Fallback Network | Mạng dự phòng TeraLink | 3-tier fallback: T1 = LAN/Wi-Fi (normal), T2 = mDNS/Multipeer (server down), T3 = BLE emergency-only |
| TeraLink Tier | Cấp TeraLink | Current network degradation level — one of T1, T2, or T3 |
| Floor Subnet | Mạng con theo tầng | BLE subnet scoped to a physical floor — max 50 devices per subnet |
| Floor Gateway | Cổng kết nối tầng | Device bridging a Floor Subnet to the wider network — exactly one elected per floor |
| Relay | Máy chủ trung chuyển | TeraRelay binary — blind router between devices (T1 only) |
| Compute Node | Nút tính toán | Mac mini or HPE MicroServer running TeraChat Core and relay — non-ECC RAM, no persistent storage authority |
| NAS ECC Storage Node | Nút lưu trữ ECC NAS | NAS with ECC RAM — sole authority for persistent database writes (SQLite WAL, blob storage) |
| AI Inference Node | Nút suy luận AI | Separate optional node for local AI inference — not bundled with Compute Node |
| Concurrent Active Sessions | Phiên hoạt động đồng thời | Unit for tier sizing — peak simultaneous active users, not total registered accounts |
| ElectionWeight | Trọng số bầu cử | Device capability score for mesh coordinator election (iOS = 0) |
| BYO-Server | Máy chủ tự trang bị | Bring-Your-Own-Server — Khách hàng tự triển khai TeraRelay trên phần cứng của họ, không ép mua gói phần cứng của TeraChat |

### .tapp & Runtime

| Term (EN) | Term (VI) | Definition |
|-----------|-----------|------------|
| .tapp | .tapp | WASM mini-app running in TeraChat sandbox |
| TappContext | Ngữ cảnh tapp | Read-only view of Core state injected into .tapp |
| Host ABI | Giao diện Host | Contract between .tapp WASM and Rust Core host |
| Fuel | Nhiên liệu | Instruction budget — deterministic resource limit for WASM |
| TappValidator | Trình xác thực .tapp | CLI tool that validates .tapp bundles before submission |

### AI

| Term (EN) | Term (VI) | Definition |
|-----------|-----------|------------|
| InferenceGateway | Cổng suy luận | Unified interface for AI inference across device/mac/cluster |
| ThermalMonitor | Giám sát nhiệt | Background monitor for device thermal state and RAM pressure |
| ModelTier | Cấp mô hình | Device-appropriate model size (Tiny/Small/Medium/Large/XLarge) |
| BYOM | Mang mô hình riêng | Bring-Your-Own-Model — Doanh nghiệp tự tích hợp custom model (ONNX) qua Open AI Framework |

## Anti-Patterns

**NEVER use these terms.** They imply wrong mental models.

| Avoid | Use Instead | Why |
|-------|-------------|-----|
| "delete message" | "create tombstone" | CRDT messages are never truly deleted |
| "get key" | "acquire key handle" | Keys are never exposed — only opaque handles |
| "user ID" | "device identity" | Identity is device-based, not user-based |
| "send to server" | "publish to relay" | Relay is blind — it doesn't "receive" messages, it routes ciphertext |
| "plugin" or "extension" | ".tapp" | .tapp is the canonical term for WASM apps |
| "timeout" (WASM) | "fuel exhaustion" | WASM uses instruction counting, not wall-clock |
| "API" (Host ↔ .tapp) | "Host ABI" | It's a low-level binary interface, not a REST API |
| "webhook" or "callback" | "CoreSignal" | Events flow from Core → UI, not the reverse |
| "State" or "Props" | "TappContext" | .tapp context is read-only and scoped |
| "Permission" or "Role" | "DataGrant" | Data access is cryptographic, not RBAC |
| "Mesh" or "BLE Mesh" | "TeraLink Fallback Network" | TeraLink is 3-tier; BLE is only T3 emergency tier |
| "total accounts" or "total users" | "concurrent active sessions" | Tier sizing uses peak concurrent load, not registered count |
| "MeshPeer" | "TeraLinkPeer" | Updated terminology — peer in TeraLink network, not just mesh |

## Language Convention

- **Code identifiers:** English (e.g., `MeshBuffer`, `KeyHandle`, `InferenceScheduler`)
- **Documentation:** Vietnamese for architecture/concept docs, English for code comments and API docs
- **Agent prompts:** Bilingual — invariants in Vietnamese (for precision), code instructions in English
