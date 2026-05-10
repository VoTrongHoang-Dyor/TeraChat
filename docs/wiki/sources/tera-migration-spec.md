---
type: source
created: 2026-05-10
tags: [terachat, migration, slack, teams, google-chat, data-ingestion]
sources: [raw/MD/Spec-Third-Party-Migration-And-Sync.md]
---

# Third-Party Migration & Sync (TERA-MIGRATION)

Source: `raw/MD/Spec-Third-Party-Migration-And-Sync.md` — added 2026-05-05.

## What It Covers

Technical architecture for ingesting data from Slack, Microsoft Teams, and Google Chat into TeraChat's Zero-Knowledge, CRDT-based system. The hardest problem: preserving hierarchical permission structures from centralized paradigms into OPA Policy Engine and cryptographic access controls.

## Key Requirements

- **Input:** OAuth 2.0 / Admin API keys or Data Export Archives from Slack/Teams/Google Workspace
- **Output:** Complete migration of Workspaces, Channels, Threads, DMs with centralized roles transformed into DID profiles and OPA policies
- **Zero-Knowledge Constraint:** Migration server operates within Secure Enclave (HSM) or entirely client-side. All memory buffers ZeroizeOnDrop after transform.
- **CRDT Compatibility:** Legacy sequential data mapped into DAG structure
- **Scale:** Must handle millions of messages without locking main thread

## Edge Cases

- Unmapped users (legacy users without TeraChat DIDs yet)
- Orphaned threads (parent message deleted)
- Circular/nested permissions (Slack private channels shared externally)

## Related Concepts

- [[Data Sovereignty & Export]]
- [[CRDT Dual-Sync Pattern]]
- [[Enterprise Identity & Governance]]
