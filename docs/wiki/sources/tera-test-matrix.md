---
type: source
created: 2026-05-10
tags: [terachat, testing, chaos-engineering, qa, gov-military]
sources: [raw/MD/TestMatrix.md]
depends_on: [tera-core-spec, tera-sync-spec, tera-client-spec]
---

# TeraChat Chaos Engineering & Test Matrix (TERA-TEST)

Source: `raw/MD/TestMatrix.md` — v0.3.7, 2026-03-23.

## What It Covers

Comprehensive test scenarios for enterprise and Gov/Military deployment: 40 chaos engineering scenarios across 5 layers, pre-production CI gates, and platform-specific coverage requirements.

## Key Content

- **40 Test Scenarios** across 5 layers:
  - Layer 1 (SC-01–07): Network Failures — partition, ALPN block, TURN failover, DNS attack
  - Layer 2 (SC-08–13): Storage & DB — Jetsam kill, power loss, migration fail, WAL bloat
  - Layer 3 (SC-14–21): Crypto & Key — MLS version mismatch, Doze/ZeroizeOnDrop, Dead Man Switch
  - Layer 4 (SC-22–25): Runtime & WASM — sandbox panic, XPC OOM, NetworkProfile race
  - Layer 5 (SC-26–40): Combined Failures — Gov/Military gate scenarios
- **3 Core Test Principles:** Zero data loss, zero key material exposure, automatic recovery
- **CI Gates:** 14 automated checks (clippy, miri, audit, gitleaks, trivy, nextest, wasm_parity, etc.)
- **Pre-Gov Go-Live Checklist:** 13 manual verification items
- **Platform Coverage:** iOS (100% SC-01–21, SC-34–38), Android, Huawei, macOS, Windows, Linux

## Related

- [[tera-tech-debt]] — Gaps tracked here become test scenarios
- [[tera-core-spec]]
- [[Survival Mesh Networking]]
