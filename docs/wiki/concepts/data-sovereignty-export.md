---
type: concept
created: 2026-05-10
tags: [terachat, data-sovereignty, export, compliance, gdpr, spf]
sources: [tera-export-spec, tera-migration-spec, tera-sync-spec]
---

# Data Sovereignty & Export

TeraChat's guarantee that organizations own their data and can leave at any time with a complete, verifiable export — no vendor lock-in. Contrasts with Slack/Teams where exports are server-side, partial, and plaintext to the provider.

## The Challenge

Since all data is E2EE and stored as CRDT DAGs:
- Servers cannot produce an export (they hold only ciphertext)
- Direct DB dump produces unreadable blobs
- Must decrypt and transform client-side while preserving hierarchy

## Export Pipeline

```
Export Request + DID Signature
         ↓
OPA Policy Check (can this user export this scope?)
         ↓
Streaming Decryption (CRDT DAG → plaintext chunks → format)
         ↓
Sovereign Portability Format (SPF) Packaging
  • JSON (messages, threads)
  • CSV (structured data)
  • HTML (human-readable archive)
  • manifest.sig (Ed25519 cryptographic proof of integrity)
         ↓
ZIP Archive → delivered to user
```

## Migration Path (Ingest)

The reverse direction: importing from Slack/Teams/Google Chat:
1. OAuth 2.0 / Admin API keys → fetch legacy data
2. Transform centralized roles → DID profiles + OPA policies
3. Re-encrypt all messages client-side before entering tc-store
4. Drop all memory buffers (ZeroizeOnDrop)

## 🧠 Design Decisions (Q&A)

- **Why streaming decryption instead of batch?** → Gigabyte-scale workspaces would OOM the client device. Streaming processes data in fixed-size windows. Trade-off: more complex pipeline, must handle partial failures gracefully.
- **Why SPF instead of standard formats like Takeout?** → Google Takeout and similar formats lose organizational structure (channels, threads, permissions). SPF preserves the full hierarchy. Trade-off: SPF is TeraChat-specific — no existing tooling reads it.
- **Why must export respect OPA policies?** → A user who was removed from a channel should not be able to export its contents. "Encryption as authorization" — if their DID key can't decrypt it, they can't export it. Trade-off: former members who had access at the time can still decrypt historical data.
