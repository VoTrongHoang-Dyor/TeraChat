---
type: source
created: 2026-05-10
tags: [terachat, tech-debt, security, architecture, gaps]
sources: [raw/MD/Tech_Debt.md]
---

# TeraChat Technical Debt Registry (TERA-DEBT)

Source: `raw/MD/Tech_Debt.md` — v1.0.0, 2026-04-11.

## What It Covers

Single Source of Truth for all technical debt: architectural drift, documentation drift, high-risk workarounds. Severity-classified (CRITICAL/HIGH/MEDIUM), domain-tagged, with mitigation plans.

## Key Debt Items

- **TD-006 (CRITICAL):** FFI Panic Abort bypasses ZeroizeOnDrop — key material stays in RAM. Gov/Military blocker.
- **TD-008 (CRITICAL):** BLE Mesh no QoS — control plane starvation during file transfers. EMDP split-brain risk.
- **TD-009 (CRITICAL):** EMDP Key Escrow uses Curve25519 only — Quantum Harvest risk. Needs Hybrid PQ-KEM upgrade.
- **TD-011 (CRITICAL):** Localhost Streaming Proxy no process-level auth — plaintext exfiltration vector.
- **TD-016 (CRITICAL):** No Thermal Budget Architecture for mobile — overheating risk from combined E2EE/BLE/WASM.
- 10 Cross-Platform Known Limitations (XPLAT-01 through XPLAT-10) — iOS W^X, Huawei HMS, Linux mlock, Android OEM kills.
- 10 Critical Gaps (GAP-A through GAP-J) — missing SagaRecoveryGuard, WAL lock signals, TOCTOU races, etc.
- 7 Refactoring Architecture Decisions pending — Headless Daemon, Hermetic Builds, Gas/Fuel Metering, CoreBootSequence, Aegis FFI, Hermetic Build Forge, BLE Mesh Priority.

## Related Concepts

- [[Zero-Knowledge Architecture]]
- [[Survival Mesh Networking]]
- [[WASM Tapp Runtime]]
