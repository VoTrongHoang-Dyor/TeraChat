---
type: concept
created: 2026-05-15
updated: 2026-05-15
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
| SanitizedPrompt | Prompt đã lọc | AI prompt after PII redaction — mandatory before any inference |
| Blind Router | Bộ định tuyến mù | TeraRelay routes ciphertext without ever seeing plaintext |
| Token Protocol | Giao thức Token | FFI safety: pass opaque `u64` tokens instead of raw pointers |

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
| MeshPeer | Đồng đẳng mạng lưới | Another device reachable via BLE or Wi-Fi Direct |
| Relay | Máy chủ trung chuyển | TeraRelay binary — blind router between devices |
| ElectionWeight | Trọng số bầu cử | Device capability score for mesh coordinator election (iOS = 0) |

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
| PiiRedactor | Bộ lọc PII | Strips personally identifiable information before AI prompts |
| ModelTier | Cấp mô hình | Device-appropriate model size (Tiny/Small/Medium/Large/XLarge) |

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

## Language Convention

- **Code identifiers:** English (e.g., `MeshBuffer`, `KeyHandle`, `InferenceScheduler`)
- **Documentation:** Vietnamese for architecture/concept docs, English for code comments and API docs
- **Agent prompts:** Bilingual — invariants in Vietnamese (for precision), code instructions in English
