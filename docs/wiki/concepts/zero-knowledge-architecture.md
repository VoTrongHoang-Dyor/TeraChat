---
type: concept
created: 2026-05-10
tags: [terachat, zero-knowledge, e2ee, architecture, security]
sources: [tera-intro, tera-core-spec, tera-sync-spec, tera-enclave-spec]
---

# Zero-Knowledge Architecture

The foundational security model of TeraChat: servers are **blind routers** that never possess plaintext or decryption keys. This is a structural property, not a configuration option.

## Core Principle

```
Client Device                         TeraRelay (Blind Router)
─────────────                        ─────────────────────────
Encrypt(message, session_key)  ───→  Sees only:
                                     • destination_device_id
                                     • blob_size
                                     • timestamp
                                     Routes ciphertext blindly
```

## Why This Model

- **Mathematical guarantee, not policy promise:** Even if TeraChat Inc. servers are compromised, attacker gets only ciphertext blobs.
- **Enterprise compliance:** Satisfies GDPR data residency, HIPAA, and Gov data sovereignty without trusting a cloud provider.
- **Contrast with Signal/WhatsApp:** They use ZK for messages but still require phone numbers and central servers for identity. TeraChat extends ZK to identity, storage, and plugins.

## Key Mechanisms

1. **Key Material Never Leaves Chip:** Private keys generated and stored permanently in Secure Enclave (Apple), StrongBox (Android), or TPM 2.0 (Desktop). No export path exists.
2. **License Entanglement:** License JWT bound to DeviceIdentityKey via KDF — wrong license = wrong key = database becomes garbage.
3. **Blind Storage:** All blobs (files, media) encrypted client-side. Server stores ciphertext with no access to symmetric keys.
4. **Streaming Decryption Proxy:** Media decrypted on-device, streamed to 127.0.0.1 loopback — never writes plaintext to disk.

## Trade-offs

- **No server-side search:** Full-text search must use client-side FTS5 with ZK indexes. This is computationally heavier on device.
- **No admin plaintext access:** IT Admin cannot read employee messages even with legal justification — requires Legal Hold + employee device cooperation.
- **Key recovery impossible:** If all user devices are lost, data cannot be recovered. No backdoor exists.

## 🧠 Design Decisions (Q&A)

- **Why not use standard PKI with server-held keys?** → Server-held keys make the server a target. With ZK, server compromise yields zero plaintext. Trade-off: harder key management, no server-side recovery.
- **Why bind license to device key?** → Prevents license sharing. If license JWT is copied to unauthorized device, key derivation produces wrong key → database is unreadable. Trade-off: device migration requires explicit re-enrollment.
- **What case does the blind router model not cover?** → Traffic analysis. An attacker watching the relay can see who communicates when and message sizes. Mitigated by padding and timing obfuscation (TERA-CORE).
