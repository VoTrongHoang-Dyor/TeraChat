# Phase 6 — Chaos, Compliance & Release Candidate

```yaml
id: "TERA-PHASE-6"
title: "Chaos Engineering, Compliance Verification & Release"
duration: "5 days (Day 31–35)"
economic_phase: "Release Candidate — All Phases"
priority: 🔴 CRITICAL — Gov/Military deployment gate
teams: [Release & Resilience Team, All 5 functional team leads]
exit_criteria:
  - All 40 chaos scenarios pass automated CI suite
  - Reproducible build + signing + audit evidence complete
  - CISO + Architect both sign Go
  - Carry-over backlog documented
```

## System Design: Phase 6 Connections

```
┌──────────────────────────────────────────────────┐
│  PHASE 6: FINAL VERIFICATION                      │
│                                                   │
│  Phase 1 (Trust Kernel) ──────────┐               │
│  Phase 2 (Dual-Sync) ─────────────┤               │
│  Phase 3 (Client Bridge) ─────────┤               │
│  Phase 4 (WASM Ecosystem) ────────┤               │
│  Phase 5 (Governance + AI) ───────┤               │
│                                    ↓               │
│  Chaos Matrix (SC-01 → SC-40)                     │
│  ├─ Layer 1: Network Failures (SC-01–07)           │
│  ├─ Layer 2: Storage & DB (SC-08–13)               │
│  ├─ Layer 3: Crypto & Keys (SC-14–21)              │
│  ├─ Layer 4: Combined Failures (SC-22–33)          │
│  └─ Layer 5: Gov/Military Gates (SC-34–40)         │
│                                    ↓               │
│  Build Verification                                │
│  ├─ Reproducible Build (Nix Flakes)               │
│  ├─ SBOM (CycloneDX 1.5 + cosign)                 │
│  ├─ EV Code Signing (Windows)                      │
│  ├─ Supply-Chain Audit                             │
│  └─ Static Analysis Full Run                       │
│                                    ↓               │
│  Release Decision                                  │
│  ├─ RC Sign-off (CISO + Architect)                 │
│  ├─ Compliance Disclosures                         │
│  ├─ Admin Runbooks                                 │
│  └─ Go/No-Go Review                                │
└──────────────────────────────────────────────────┘
```

---

## Task Box 6.1 — Chaos Matrix Automation (SC-01 → SC-40)

```yaml
task_id: "PH6-T01"
name: "Automate All 40 Chaos Scenarios in CI"
status: pending
priority: 🔴 CRITICAL
index: IDX-14-QA-CHAOS
platforms: [macOS, Windows, Linux, iPhone, Android, Oppo, Huawei]
estimated_hours: 10
```

### Code Tasks

| # | Task | Test Scenarios | Platform |
|---|------|---------------|----------|
| 1 | Layer 1 — Network Failures: SC-01 to SC-07 | `tests/chaos-mesh/network/` | All |
| 2 | Layer 2 — Storage & DB: SC-08 to SC-13 | `tests/chaos-mesh/storage/` | All |
| 3 | Layer 3 — Crypto & Keys: SC-14 to SC-21 | `tests/chaos-mesh/crypto/` | All |
| 4 | Layer 4 — Combined Failures: SC-22 to SC-33 | `tests/chaos-mesh/combined/` | All |
| 5 | Layer 5 — Gov/Military Gates: SC-34 to SC-40 | `tests/chaos-mesh/gov_mil/` | All |
| 6 | CI integration: chaos tests run nightly + on-demand for RC | `.github/workflows/chaos.yml` | All |

### Testing: Key Chaos Scenarios

| Scenario | Description | Expected | Timeout | Platform |
|----------|-------------|----------|---------|----------|
| SC-01 | Internet partition 30 min → rejoin | Zero data loss, full recovery < 120s | 2100s | All |
| SC-08 | Jetsam kill NSE mid-WAL write | WAL crash-safe replay, no notification loss | 30s | iPhone |
| SC-17 | Dead Man Switch during active call | DeadManDeferralEntry logged, lockout deferred | 180s | All |
| SC-22 | MLS Epoch rotation during BLE mesh partition + WAL bloat | All three resolve without conflict | 300s | All |
| SC-34 | ZeroizeOnDrop verification under all combined failures | Key material never survives | CI gate | All |
| SC-35 | PENDING_SECURE_CHANNEL UX correctness | Outbox TTL + user notification | 86400s | All |
| SC-36 | Burner Agent TTL + EMDP Epoch Freeze intersection | No zombie member (GAP-G) | 3600s | All |
| SC-37 | Saga recovery after 1000 kill cycles at CrdtCommitted | Idempotent recovery every time | CI gate | All |
| SC-38 | BLE 100kbps cap + 250ms RTT + 2MB file transfer | EmdpSessionTerminated < 2s | 120s | Mobile |
| SC-39 | NSE ring buffer drain with concurrent writes | No data loss | 30s | iPhone |
| SC-40 | cold_state.db corruption + rebuild from hot_dag.db | Full recovery, no data loss | 180s | All |

### Deployment

Chaos harness in `tests/chaos-mesh/`. CI runs nightly at 02:00 UTC. JUnit XML output.

### Request / Review

| Department | Reviewer | Gate |
|------------|----------|------|
| Release & Resilience | Chaos QA Lead | All 40 pass or documented reason |
| Infra, Ops & Quality | SecOps | Observability during chaos runs |
| Architecture & Leadership | CISO | SC-34–40 all pass = blocker if fail |

### Design Requirements

- Every chaos scenario has: setup, inject failure, observe, assert, teardown
- CI output: JUnit XML, Prometheus metrics, screenshots for UX scenarios
- Nightly run: 02:00 UTC, results posted to Slack/Linear
- RC gate: ALL 40 pass within 24h of release

### Reference Documentation

- `docs/raw/MD/TestMatrix.md` — Full 40-scenario matrix
- `docs/raw/MD/Tech_Debt.md` — GAP-G (SC-36)
- `docs/raw/MD/Note.md §8` — Chaos Engineering specification

### System Design Connection

- **Input from:** Phase 1–5 (all features and fixes)
- **Output to:** Task Box 6.4 (Go/No-Go decision)
- **Connects:** Every system component → Failure injection → Assertion → CI gate

---

## Task Box 6.2 — Reproducible Build & Supply Chain Verification

```yaml
task_id: "PH6-T02"
name: "Achieve Hermetic Build + Signed Artifacts for All Platforms"
status: pending
priority: 🔴 CRITICAL
debt: XPLAT-09
platforms: [macOS, Windows, Linux, iPhone, Android, Oppo, Huawei]
estimated_hours: 8
```

### Code Tasks

| # | Task | Platform |
|---|------|----------|
| 1 | Nix Flakes: lock entire build environment (glibc, MSVC, Clang, Rustc) to exact bits | Linux (builder), All (targets) |
| 2 | Docker-based hermetic builder: `SOURCE_DATE_EPOCH=1700000000` for reproducibility | All |
| 3 | SBOM generation: CycloneDX 1.5 via `cargo cyclonedx` + `cosign sign-blob` | All |
| 4 | macOS: `.dmg` signing with Apple notarization | macOS |
| 5 | Windows: EV Code Signing via DigiCert KeyLocker + self-hosted Mac mini runner | Windows |
| 6 | Linux: `.deb` GPG signing + `.rpm` GPG signing + AppImage cosign | Linux |
| 7 | iOS: `.ipa` signing via fastlane match + App Store Connect | iPhone |
| 8 | Android: `.aab` signing via Google Play App Signing | Android, Oppo |
| 9 | Huawei: `.app` signing via Huawei AppGallery Connect | Huawei |
| 10 | Supply chain audit: `cargo audit`, `cargo vet`, `cargo-deny` — all must pass | All |

### Testing

| # | Test | Platform |
|---|------|----------|
| 1 | Build twice on same Nix Flake → bit-identical binaries | Linux, macOS |
| 2 | Build twice on different machines (same Nix lock) → bit-identical | Linux, macOS |
| 3 | SBOM hash matches build artifact hash | All |
| 4 | `cosign verify-blob` passes on all artifacts | All |
| 5 | `signtool verify /pa` passes on Windows `.exe` | Windows |
| 6 | `spctl -a -t exec -vv` passes on macOS `.app` | macOS |

### Deployment

Build pipeline: Nix Flakes → Docker → Cargo build --release --locked → Sign → SBOM → Publish.

### Request / Review

| Department | Reviewer | Gate |
|------------|----------|------|
| Infra, Ops & Quality | SRE | Reproducible build proof |
| Architecture & Leadership | CISO + Security Architect | Supply chain audit clean |
| Infra, Ops & Quality | Platform Engineer | EV signing path verified |

### Design Requirements

- Build artifacts = signed binary + SBOM + cosign attestation + Nix build log
- Missing any of 4 components = BLOCK distribution
- `SOURCE_DATE_EPOCH` = `1700000000` (frozen, not actual date)
- Self-hosted Mac mini with YubiKey FIPS for EV signing

### Reference Documentation

- `docs/raw/MD/Tech_Debt.md §5.6` — Hermetic Build Forge
- `docs/raw/MD/Tech_Debt.md` — XPLAT-09, XPLAT-06
- `docs/raw/MD/Note.md §3` — Build tools
- `docs/raw/MD/Note.md §5` — Environment variables

### System Design Connection

- **Input from:** Phase 5 (deployment models), Phase 0 (CI baseline)
- **Output to:** Task Box 6.4 (RC evidence pack)
- **Connects:** Build Forge → Signing → SBOM → Distribution → Audit

---

## Task Box 6.3 — Compliance Disclosure & Admin Runbooks

```yaml
task_id: "PH6-T03"
name: "Publish Compliance Documentation & Operational Runbooks"
status: pending
priority: 🟠 HIGH
debt: GAP-F
platforms: [All]
estimated_hours: 6
```

### Code Tasks

| # | Task | Output |
|---|------|--------|
| 1 | Huawei limitation disclosure: HMS Push no data-only, CRL ≤ 4h polling, no Gov/Military tier | `docs/wiki/concepts/` |
| 2 | Platform limitation matrix: XPLAT-01 to XPLAT-10 clearly disclosed per tier | `docs/wiki/concepts/` |
| 3 | Fallback semantics: what degrades when (BLE only, no AWDL, no PQ, etc.) | `docs/wiki/concepts/` |
| 4 | Admin runbook: deployment, SCIM setup, OPA policy authoring, audit export, remote wipe | `docs/wiki/concepts/` |
| 5 | Security disclosure: known attack vectors, threat model, penetration test results | `docs/wiki/concepts/` |
| 6 | Pricing/Packaging update: reflect platform limitations in tier descriptions | `docs/HTML/Pricing_Packages.html` |

### Testing

| # | Test |
|---|------|
| 1 | Admin runbook: fresh IT admin follows steps → TeraChat running < 30 min |
| 2 | Compliance reviewer reads disclosures → no surprise limitations |

### Deployment

Documentation published to `docs/wiki/` and embedded in Admin Console help.

### Request / Review

| Department | Reviewer | Gate |
|------------|----------|------|
| Governance & Compliance | Compliance Officer | No undisclosed limitations |
| Architecture & Leadership | CISO | Security disclosure completeness |
| Governance & Compliance | Product Manager | Pricing reflects reality |

### Design Requirements

- Huawei: explicitly NOT in Enterprise SLA tier
- All platform limitations disclosed per deployment tier (Solo, Team, Enterprise, Gov/Military)
- Fallback behavior clearly documented: what degrades, what stops, what continues
- Admin runbooks: step-by-step, no assumed DevOps knowledge

### Reference Documentation

- `docs/raw/MD/Tech_Debt.md §3` — XPLAT-01 to XPLAT-10
- `docs/raw/MD/Tech_Debt.md` — GAP-F
- `docs/HTML/Pricing_Packages.html`
- `docs/wiki/concepts/enterprise-license-model.md`

### System Design Connection

- **Input from:** All phases (known limitations), Phase 5 (deployment models)
- **Output to:** Task Box 6.4 (Go/No-Go review)
- **Connects:** Documentation → Compliance → Pricing → Customer Expectation

---

## Task Box 6.4 — RC Go/No-Go Review & Sign-off

```yaml
task_id: "PH6-T04"
name: "Release Candidate Decision Gate"
status: pending
priority: 🔴 CRITICAL
platforms: [All]
estimated_hours: 4
```

### Code Tasks

N/A — decision gate based on evidence from Task Boxes 6.1–6.3.

### Evidence Pack Required

| Evidence | Source | Gate |
|----------|--------|------|
| All 40 chaos scenarios pass | Task Box 6.1 CI output | MUST pass |
| Reproducible build verified | Task Box 6.2 Nix build logs | MUST pass |
| SBOM + cosign signatures | Task Box 6.2 artifacts | MUST present |
| Supply chain audit clean | Task Box 6.2 cargo audit/vet/deny | MUST pass |
| Compliance disclosures complete | Task Box 6.3 documents | MUST present |
| Admin runbook validated | Task Box 6.3 runbook | MUST validate |
| All ADRs signed off | Phase 0 ADR documents | MUST sign |
| All CRITICAL debt resolved or accepted | Tech_Debt.md registry | MUST document |
| Static analysis clean (all tools) | Note.md §9 checklist | MUST pass |
| Security review: no invariant violations | Review Agent gate log | MUST verify |

### Go/No-Go Decision Makers

| Role | Vote | Criteria |
|------|------|----------|
| **System Architect** | Veto | Architecture invariants intact |
| **CISO** | Veto | Zero-Knowledge guarantees verified |
| **Engineering Manager** | Vote | All exit criteria met, carry-over documented |
| **Chaos QA Lead** | Vote | All 40 scenarios pass or accepted risk |
| **Product Manager** | Vote | Feature completeness for target tier |

### Carry-Over Backlog

| ID | Description | Target Phase |
|----|-------------|-------------|
| TD-002 | WidgetDataState dual computation | Phase 7 (post-RC) |
| XPLAT-05 | Linux Tauri SharedArrayBuffer headers | Phase 7 (post-RC) |
| XPLAT-07 | HarmonyOS .waot AOT portability | Phase 7 (post-RC) |
| Any non-blocking MEDIUM debt | — | Phase 7 backlog |

### Design Requirements

- Go decision requires: CISO + Architect both sign + all CRITICAL evidence present
- No-Go: documented reason, carry-over plan, re-evaluation date
- Carry-over: nothing CRITICAL or HIGH may be carried over without explicit CISO acceptance

### Reference Documentation

- All phase files (Phase 0–6)
- `phase/README.md` — Invariant checklist
- `docs/raw/MD/Tech_Debt.md` — Full debt registry

### System Design Connection

- **Input from:** Everything (Phase 0 through Task Box 6.3)
- **Output to:** Production deployment or No-Go plan
- **Connects:** All teams → Evidence pack → Decision → Release or Remediate
