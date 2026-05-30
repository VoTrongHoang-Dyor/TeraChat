---
type: concept
created: 2026-05-16
updated: 2026-05-18
tags: [security, threat-model, stride, attack-vectors, relay, sandbox, insider]
sources: [CLAUDE.md, invariants.md, platform-architecture.md, Spec-Core-Cryptography-And-Mesh.md]
---

# Threat Model

STRIDE analysis cho 3 attack vectors chính của TeraChat. Mỗi threat có mitigation cụ thể map đến invariant hoặc code control. Đây là tài liệu bắt buộc trước khi viết code (Slice 0).

## Attack Vector 1: TeraRelay Compromise

Kẻ tấn công đã chiếm quyền root trên TeraRelay server. Relay là blind router — câu hỏi là: attacker thấy gì và làm được gì?

| STRIDE | Threat | Severity | Mitigation | Control |
|--------|--------|----------|------------|---------|
| **S**poofing | Attacker impersonates relay to clients | Critical | mTLS both directions; relay certificate pinned in client binary; cert rotation via OIDC | Code: `tc-crypto` cert pinning |
| **T**ampering | Attacker modifies ciphertext in transit | Critical | TLS 1.3 + MLS end-to-end encryption — relay never sees plaintext, cannot modify without detection | Code: MLS AEAD integrity |
| **R**epudiation | Attacker replays valid messages | High | MLS epoch counters prevent replay; each message has monotonic sequence number | Code: `openmls` epoch validation |
| **I**nfo Disclosure | Attacker reads relay disk or memory | Medium | ALL data on relay is MLS ciphertext; relay is blind router — disk dump yields zero plaintext | Architecture: Blind Router model |
| **D**oS | Attacker floods relay with connections | Medium | Rate limiting per `DeviceIdentity`; WAF for HTTP endpoints; connection limit per IP | Code: `tc-relay` rate limiter |
| **E**levation | Attacker pivots from relay to client devices | None | Relay has NO key material, NO client credentials, NO ability to decrypt — pivot yields nothing | Architecture: Zero-Knowledge |

**Residual Risk:** DoS is possible but mitigated. Relay compromise is a "noisy" event — clients detect via health check failure and escalate to T2 (TeraLink).

### Attack Vector 1B: BLE Mesh Identity Spoofing (TeraLink T3)

Khi TeraLink ở T3 (BLE emergency), threat model thay đổi: attacker không ngồi trên đường truyền internet mà là thiết bị giả mạo trong cùng không gian vật lý.

| Threat | Severity | Description | Mitigation | Control |
|--------|----------|------------|------------|---------|
| Rogue BLE peer impersonation | **HIGH** | Attacker spoofs legitimate device identity in BLE beacon | Full 32-byte Ed25519 public key fingerprint in BLE beacon (NOT 8-byte truncated hash). Challenge-response protocol before accepting peer into mesh | Code: `tc-mesh` beacon validation |
| Brute-force identity commitment | **HIGH** (offline) | Truncated 8-byte HMAC identity commitment có thể brute-force trong môi trường offline | Thay truncated 8-byte bằng full 32-byte Ed25519 fingerprint. Chi phí: thêm 2–3 giây latency khi peer discovery — chấp nhận được | Code: `tc-mesh` `PeerIdentity` type |
| Replay attack on mesh | Medium | Attacker replays captured BLE messages | HLC timestamp + CRDT dedup — đã có, đủ mạnh | Code: `tc-crdt-sync` HLC |
| Metadata correlation | Low-Medium | Attacker observes device ID, message size, timing pattern | Sealed Sender + BLE padding + timing obfuscation | Code: `tc-mesh` BLE transport |
| Sybil attack (fake node flood) | Medium | Attacker floods mesh with fake device identities | Enterprise CA-signed Shun_Records; identity quorum before mesh join | Code: `tc-crypto` CA validation |

**Why this matters:** Trong môi trường offline BLE, không có server để validate identity. Mỗi device phải tự xác minh peer identity bằng cryptographic proof. Dùng 8-byte truncated hash (như BitChat) là không đủ — phải dùng full 32-byte fingerprint.

## Attack Vector 2: Device Compromise (Insider Threat)

Nhân viên nội bộ với quyền truy cập vật lý vào thiết bị của đồng nghiệp, hoặc IT admin với quyền root trên Mac mini.

| STRIDE | Threat | Severity | Mitigation | Control |
|--------|--------|----------|------------|---------|
| **S**poofing | Attacker clones device identity | Critical | `DeviceIdentityKey` in Secure Enclave (iOS/Mac) / TPM (Windows/Linux) — cannot be exported; key attestation at enrollment | Code: `tc-crypto` SE/TPM binding |
| **T**ampering | Attacker modifies local database | High | `cold_state.db` encrypted at rest via `KDF(license_jwt, device_identity_key)`; CRDT integrity via BLAKE3 hashes | Code: `tc-store` SQLCipher |
| **R**epudiation | Attacker denies sending message | High | Ed25519 signature on every message; immutable audit trail; signatures verified by all group members | Code: MLS application-level signing |
| **I**nfo Disclosure | Attacker extracts key material from memory | Critical | `ZeroizeOnDrop` on all key types; Secure Enclave sealed keys; no plaintext key on disk | Code: I-2 enforcement |
| **D**oS | Attacker fills device storage | Low | Per-device quota enforced by relay; LRU eviction on iOS; storage alerts at 80% capacity | Code: `tc-store` quota manager |
| **E**levation | Attacker escalates from user to admin | Critical | OPA RBAC engine; admin actions require quorum; all admin actions logged to immutable cold_state audit trail | Code: `tc-gov` OPA policies |

**Residual Risk:** Physical access + kernel exploit on Mac could bypass Secure Enclave. Mitigated by: Mac mini in locked server room, iOS devices encrypted at rest with device passcode. Gov/Military tier adds HSM (YubiHSM2) for additional key isolation.

### Hybrid PQ-KEM Strategy (Post-Quantum Readiness)

Theo Architecture Report v1.0 Section 2.2 — quyết định PQ phụ thuộc vào phân khúc khách hàng:

| Segment | PQ Needed? | Rationale | Timeline |
|---------|-----------|-----------|----------|
| Enterprise Standard (Finance, Healthcare) | Not yet | Data shelf-life < 5 years; quantum risk doesn't justify cost | Phase 3+ per demand |
| Gov/Military | **MANDATORY** | Strategic data shelf-life 10–20 years. Store Now, Decrypt Later is real threat | Phase 2A — implement early |

**Hybrid KEM Architecture:**
```rust
// Final_Session_Key = HKDF(X25519_shared_secret || ML_KEM_768_shared_secret)
// Overhead per handshake: +1.2 KB (Kyber ciphertext), +3–5 ms latency on Apple Silicon
```

**Per-tier deployment:**
- Enterprise Standard: X25519 only — PQ disabled mặc định, có thể bật theo yêu cầu
- Gov/Military: Hybrid KEM bật mặc định, không thể tắt — ghi vào License JWT
- EMDP Key Escrow trong Mesh: Upgrade từ ECIES/Curve25519 thuần sang Hybrid ECIES + ML-KEM-768

## Attack Vector 3: .tapp WASM Sandbox Escape

.tapp độc hại được cài đặt qua marketplace (approved but malicious) hoặc side-loaded qua admin console.

| STRIDE | Threat | Severity | Mitigation | Control |
|--------|--------|----------|------------|---------|
| **S**poofing | Malicious .tapp impersonates system component | Critical | WASM sandbox isolation; app signing PKI with `TappValidator` CLI; manifest hash verification | Code: `tc-tapp` validator |
| **T**ampering | .tapp modifies another .tapp's state | Critical | DataGrant cryptographic isolation; per-.tapp storage namespace; cross-tapp data access requires explicit cryptographic grant | Code: I-12 + DataGrant quorum |
| **R**epudiation | .tapp performs unauthorized actions | High | All Host ABI calls logged to immutable audit trail per .tapp; admin console shows per-.tapp activity | Code: `tc-tapp` audit logger |
| **I**nfo Disclosure | .tapp reads cross-tenant data | Critical | DataGrant quorum protocol (GAP-E); explicit cryptographic grant per read; no default access to any data | Code: DataGrant weighted voting |
| **D**oS | .tapp exhausts fuel budget | High | Fuel metering (instruction count, NOT wall-clock); memory bound < 50MB per .tapp; CPU time cap per invocation | Code: `tc-tapp` fuel metering |
| **E**levation | .tapp gains network access | Critical | `network:external` capability PERMANENTLY BLOCKED in Host ABI struct — no negotiation possible; `send_media()` blocked when transport = BLE | Code: I-12 Host ABI block |

**Residual Risk:** Vulnerabilities in wasmtime/wasm3 runtime itself. Mitigated by: pinning exact wasmtime/wasm3 versions; monitoring CVE database; dual-engine (different implementations) reduces chance of simultaneous exploit.

## Invariant-to-Threat Coverage Matrix

| Invariant | Threat Addressed | Enforcement |
|-----------|-----------------|-------------|
| I-1 (Server never sees plaintext) | A1 Info Disclosure — relay compromise | Integration test: intercept relay traffic, assert no plaintext |
| I-2 (Private key never leaves Secure Enclave) | A2 Info Disclosure — key extraction, A1 Spoofing — device clone | CI lint `zeroize-verify` + SE/TPM binding |
| I-3 (No self-implemented crypto) | ALL — crypto implementation bugs | Dependency audit: only `ring` or `openmls` |
| I-8 (AI Local prompt rejection < 0.5%) | A2 Info Disclosure — adversarial prompt bypass | Metric: prompt rejection rate measured in Slice 6 |
| I-10 (NAS ECC Storage Authority) | A2 Tampering — silent DB corruption | Type system: compile-time |
| I-11 (BLE ≤ 500 bytes) | A1B DoS — BLE broadcast storm | Type system: `[u8; 500]` |
| I-12 (.tapp no egress) | A3 Elevation — data exfiltration | Host ABI permanent block |
| I-13 (BSL boundary immutable) | Supply chain — license tampering | CI gate `bsl-boundary-hash` |

## Security Review Cadence

| Trigger | Review Scope | Who |
|---------|-------------|-----|
| Every Slice boundary | Full STRIDE re-evaluation | Human architect + AI review |
| New crypto primitive added | Focused crypto review | Applied Cryptographer (freelance) |
| New .tapp capability added | Sandbox escape review | Security engineer |
| SOC2 Type I audit (Slice 4) | Full security assessment | Boutique auditor (~$20K) |
| SOC2 Type II (Post-Slice 6) | 6-month continuous monitoring | Vanta/Drata + auditor |

## Related Pages

- [[Invariants]] — Full invariant list with enforcement
- [[Platform Architecture]] — Module and license architecture
- [[Zero-Knowledge Architecture]] — Blind router model details
- [[Secure Enclave & AI Security]] — AI-specific security controls
- [[gap-resolution-tracker]] — GAP-A through GAP-J resolution status
