# AGENT_CONTEXT.md — TeraChat Agent Entry Point

```yaml
id: "TERA-AGENT-CTX"
version: "2.1.0"
date: "2026-05-18"
slice: "Slice 0 — Foundation"
status: "Pre-code documentation"
```

## Project Overview

TeraChat is an **enterprise Work OS** for internal and branch-company communication: Zero-Knowledge E2EE (MLS RFC 9420), self-hosted on Mac mini + NAS ECC, local AI inference (Qwen2.5/MLX), and a .tapp WASM ecosystem — all under a blind router architecture where servers never see plaintext.

## Current State — Slice 0 (Week 1–2)

**Status:** Pre-code. Repo structure established, CI scaffolding, documentation finalization.
**No real code written yet** — this is the optimal time to adjust architecture.

## Reading Order (Mandatory)

Every agent MUST read these files in order before writing any code:

| # | File | What It Contains | Time |
|---|------|-----------------|------|
| 1 | `AGENT_CONTEXT.md` | This file — project overview, current priority | 2 min |
| 2 | `docs/wiki/ubiquitous-language.md` | Shared vocabulary (EN + VI) — use these terms | 5 min |
| 3 | `docs/wiki/invariants.md` | 13 non-negotiable architectural rules with enforcement | 10 min |
| 4 | `CLAUDE.md` | Engineering guardrails, forbidden patterns, AI compatibility matrix | 10 min |
| 5 | `phase/README.md` | Current slice + timeline + deliverables | 5 min |
| 6 | `docs/wiki/concepts/platform-architecture.md` | License tiers (CLOSED/BSL 1.1/MIT), module diagram, flywheel | 5 min |
| 7 | `docs/wiki/concepts/threat-model.md` | STRIDE for 3+ attack vectors, BLE identity commitment, PQ-KEM | 8 min |
| 8 | `docs/wiki/concepts/hardware-specification.md` | Compute/Storage/AI Node separation, tier hardware tables | 5 min |
| 9 | `docs/wiki/concepts/teralink-fallback-network.md` | TeraLink 3-tier fallback, Floor Subnet, RAM budget | 8 min |

## Current Priority — Slice 0 Deliverables

Before any code is written (this is the P0 checklist from the Architecture Update v2.0):

1. **PLATFORM_ARCHITECTURE.md** — Done. License tiers, BSL boundary, module diagram, 5-step flywheel
2. **THREAT_MODEL.md** — Done. STRIDE for 3 attack vectors + BLE identity commitment + PQ-KEM
3. **INVARIANTS.md** — Done. 13 invariants with enforcement mechanisms
4. **HARDWARE_SPEC.md** — Done. Updated hardware table with Compute/Storage/AI node separation
5. **Observability from Slice 0** — Add OpenTelemetry traces requirement to CI pipeline

## What NOT To Do

- Do NOT write new technical specs — 80+ documents exist, sufficient for 18 months
- Do NOT run slices in parallel — finish Slice 0 before Slice 1
- Do NOT add platforms (Android, Windows) before 3+ paying customers
- Do NOT start coding before the 5 P0 tasks above are complete

## Key Architecture Decisions (v2.0)

These decisions from the Architecture Update report override prior assumptions:

1. **Mac mini = Compute Node only** — never primary DB writer. NAS ECC = sole Storage Authority
2. **Gov/Military = HPE hardware** — not Mac mini. Apple fails FIPS 140-3. Software identical.
3. **BLE → 3-tier TeraLink Fallback Network** — T1 (LAN), T2 (mDNS/Multipeer), T3 (BLE emergency only)
4. **AI Node = separate SKU** — not bundled with Compute Node. Optional add-on.
5. **BSL + MIT SDK split** — tc-crypto/HA/engine = CLOSED, core modules = BSL 1.1, SDKs = MIT
6. **Pricing based on org size, hardware sized for concurrent sessions** — different metrics
7. **Full 32-byte Ed25519 fingerprint in BLE beacons** — not 8-byte truncated hash (mitigates brute-force)

## Quick Commands

```bash
cargo test --workspace     # Must pass (currently 0 tests, compiles)
cargo clippy -- -D warnings  # Must be 0 warnings
buf lint                     # Proto validation
```

## Agent File Scope

Each agent type has strict boundaries:

| Agent | Scope | Must Read |
|-------|-------|-----------|
| Rust Agent | `source/core/` — tc-crypto, tc-crdt-sync, tc-mesh, tc-store, tc-ai, tc-tapp | invariants.md + CLAUDE.md |
| Test Agent | `source/core/*/tests/` — integration + property-based tests | invariants.md + threat-model.md |
| Security Agent | TC-ENCLAVE, tc-crypto, tc-gov — crypto + policy review | threat-model.md + invariants.md |
| Doc Agent | `docs/wiki/` — updates only | ubiquitous-language.md |
| Flutter Agent | `source/apps/flutter/` — UI only, no business logic | CLAUDE.md forbidden patterns |
