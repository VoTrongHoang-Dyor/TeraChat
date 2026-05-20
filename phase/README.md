# TeraChat — Vertical Slice Execution Map

```yaml
id: "TERA-SLICE-MAP"
version: "5.0.0"
date: "2026-05-18"
principle: "Vertical Slice — mỗi slice là shippable, demo được, charge được"
philosophy: "Deep Modules + Multi-Agent Harness"
timeline: "18 months (46 weeks)"
```

## Mental Model: Vertical Slice > Horizontal Layer

```
❌ WRONG (Horizontal Layer):
[Crypto] → [Sync] → [Runtime] → [Client] → [AI]
2 years later: nothing runnable

✅ RIGHT (Vertical Slice):
Slice 1: "E2EE message giữa 2 Mac" (6 weeks) → demo
Slice 2: "iPhone → Mac" (6 weeks) → demo on real devices
Slice 3: "Relay + TeraLink fallback" (6 weeks) → HA demo
Slice 4: ".tapp đầu tiên" (8 weeks) → Work OS demo
Slice 5: "AI summarize thread" (8 weeks) → AI demo
```

Every slice is **shippable** — can demo, can charge, can get feedback. Each slice adjusts based on previous feedback.

---

## Tổng quan Timeline (18 tháng, 46 tuần)

```
SLICE 0: Foundation (Tuần 1-2)
  │  Repo compiles, CI green, proto scaffolding
  │
  ├─ SLICE 1: "Hello E2EE" (Tuần 3-8) — 6 tuần
  │    │  MLS roundtrip test. Không UI, không network.
  │    │  Deliverable: cargo test --test mls_roundtrip pass
  │    │
  │    ├─ SLICE 2: "Relay + Persistence" (Tuần 9-14) — 6 tuần
  │    │    │  Messages through relay. hot_dag.db append-only.
  │    │    │  Deliverable: terachat-relay binary + integration test
  │    │    │
  │    │    ├─ SLICE 3: "macOS + iPhone UI" (Tuần 15-22) — 8 tuần
  │    │    │    │  Real devices chatting. Tauri + Flutter.
  │    │    │    │  Deliverable: E2EE message between iPhone and Mac
  │    │    │    │
  │    │    │    ├─ SLICE 4: "HA + Mesh Failover" (Tuần 23-30) — 8 tuần
  │    │    │    │    │  2× Mac mini cluster + BLE emergency fallback
  │    │    │    │    │  Deliverable: 99.99% SLA demo
  │    │    │    │    │
  │    │    │    │    ├─ SLICE 5: ".tapp Runtime MVP" (Tuần 31-38) — 8 tuần
  │    │    │    │    │    │  3 first-party .tapps running
  │    │    │    │    │    │  Deliverable: Expense Approval .tapp works
  │    │    │    │    │    │
  │    │    │    │    │    └─ SLICE 6: "Local AI" (Tuần 39-46) — 8 tuần
  │    │    │    │    │         │  Qwen2.5 on-device, PII gate
  │    │    │    │    │         │  Deliverable: AI summarize with zero PII leaks
  │    │    │    │    │
  Mỗi slice = demonstrable, feedbackable, chargeable.
```

---

## Slice Details

### Slice 0: Foundation (Week 1-2)

**Goal:** Repository compiles, CI green, no real code yet.

```bash
cargo test --workspace    # 0 tests but compiles
buf lint                  # proto files valid
cargo clippy -- -D warnings  # 0 warnings
```

**Human work:** Review CI config, sign off CLAUDE.md, create `BSL_BOUNDARY.sha256`, setup secrets.
**AI work:** CI/CD pipeline, buf.yaml, proto scaffolding.

**Pre-code documents (produced in Slice 0 — before any code):**
- `PLATFORM_ARCHITECTURE.md` — License tiers, BSL boundary, module diagram
- `THREAT_MODEL.md` — STRIDE for 3 attack vectors
- `HARDWARE_SPEC.md` — Updated hardware table with Compute/Storage/AI node separation
- `INVARIANTS.md` updated — 13 invariants with enforcement mechanisms

---

### Slice 1: "Hello E2EE" (Week 3-8)

**Goal:** Two processes on same Mac send/receive E2EE messages via MLS. No UI, no network, no database.

```rust
#[tokio::test]
async fn mls_roundtrip() {
    let alice = MlsClient::new_test().await;
    let bob = MlsClient::new_test().await;

    let group = alice.create_group("test-group").await?;
    group.add_member(&bob.key_package()).await?;

    let ciphertext = group.encrypt(b"hello bob").await?;
    let plaintext = bob.decrypt(&ciphertext).await?;
    assert_eq!(plaintext, b"hello bob");

    // Bob leaves → new epoch → Bob can't decrypt
    group.remove_member(&bob.identity()).await?;
    let new_cipher = group.encrypt(b"bob is gone").await?;
    assert!(bob.decrypt(&new_cipher).await.is_err());
}
```

**Key Deliverables:**
- `MlsClient`, `MlsGroup` with openmls wrapper
- `mls_roundtrip` test passing
- ZeroizeOnDrop on all key types verified
- `cargo miri test --test ffi_boundary_zeroize` pass
- `SanitizedPrompt` type implemented (compile-time PII enforcement)
- OpenTelemetry traces for all MLS operations (not retrofitted later)

**Agent assignment:** Rust Agent writes MlsClient/MlsGroup, Test Agent writes roundtrip test, Review Agent verifies invariants.

**External hire:** Applied Cryptographer ($15,000–30,000 freelance) reviews MLS implementation.

---

### Slice 2: "Relay + Persistence" (Week 9-14)

**Goal:** Messages through relay binary, persisted in SQLite WAL on NAS ECC storage path.

```
Client A ──TLS 1.3──> TeraRelay ──TLS 1.3──> Client B
                         │
                    NAS ECC Storage
                    (SQLite WAL — ciphertext only, relay sees nothing)
```

**Key Deliverables:**
- `terachat-relay` binary — single command deploy
- `hot_dag.db` — append-only CRDT events on NAS ECC path
- NAS-aware storage path config (I-10 enforcement)
- Async SQLCipher rekey (double-buffer, no UI block)
- License JWT validation
- Health check endpoint
- Integration test: 1000 messages → 0 loss, kill relay mid-send → reconnect → message delivered
- **Vanta/Drata setup** — begin SOC2 evidence collection ($5,000–8,000/yr)

**Milestone:** 1 paying pilot customer by end of Slice 2.

---

### Slice 3: "macOS + iPhone UI" (Week 15-22)

**Goal:** Two real devices chatting with each other.

**Stack:** Tauri (macOS) + Flutter (iPhone) + gRPC over UDS for IPC.

**UI — minimal but correct:**
- Channel list
- E2EE indicator (from CoreSignal)
- Send/receive message
- Glassmorphism basic

**FFI constraint:** `flutter_rust_bridge` with strict ownership transfer — Dart never holds raw Rust pointer > 1 frame.

**Platform constraint:** macOS + iOS first. Android only when 3+ paying customers request it.

**Human work:** UX decisions, testing on real devices.
**AI work:** Tauri commands, Flutter screens, FFI bindings.

---

### Slice 4: "HA + TeraLink Fallback Network" (Week 23-30)

**Goal:** Enterprise-grade HA with automatic TeraLink 3-tier fallback. This is the SLA argument for customer deals.

```
Normal (T1): Devices → TeraRelay (Compute Node) → NAS ECC Storage

Compute Node fail (auto-detect within 5s):
  T2 → mDNS/Multipeer activated (server down, LAN intact)
  T3 → BLE emergency if LAN also unavailable (text only, ≤ 500 bytes)
  Store-and-forward CRDT
  When primary returns: auto-sync all tiers → T1
```

**TeraLink 3-Tier Implementation:**
- T1: LAN/Wi-Fi — gRPC to Compute Node, full features
- T2: mDNS peer discovery + Wi-Fi Direct / iOS Multipeer Connectivity — text + presence
- T3: BLE 5.0 Coded PHY — text only, ≤ 500 bytes, Floor Subnet Architecture (≤50 devices/subnet)

**Approach:** Apple `MultipeerConnectivity` (iOS) + `mDNS + TCP` (macOS) for T2. Floor Gateway: Raspberry Pi 4 pre-configured image per floor.

**SLA:**
- 1 Compute Node + External SSD: 99.5% (~4h downtime/year)
- 2 Compute Node HA + NAS ECC: 99.95% (~4 min downtime/year)
- 2 Compute Node + NAS ECC + TeraLink Fallback: **99.99%** (~1 min downtime/year) — enterprise contract grade

**Milestone:** SOC2 Type I audit with boutique auditor (~$20K total).

---

### Slice 5: ".tapp Runtime MVP" (Week 31-38)

**Goal:** Not a marketplace — just 3 first-party .tapps running.

| .tapp | Use Case |
|-------|----------|
| Expense Approval | Manager approve/reject with digital signature |
| Document Signing | Multi-party Ed25519 document signing |
| Task Assignment | Create, assign, track tasks |

**WASM runtime:**
- wasmtime (desktop) + wasm3 (iOS) dual-engine
- Host ABI: storage_get/set, ed25519_sign, event_publish
- Fuel metering from day 1 (not retrofitted)
- Capability enforcement at Host ABI syscall layer (I-12)
- `network:external` syscall permanently blocked (I-12 enforcement)
- `send_media()` blocked when transport = BLE (I-11 enforcement)

**Milestone:** Publish `tapp-sdk` (MIT) on GitHub. Write `TAPP_SPEC.md` public specification.

---

### Slice 6: "Local AI" (Week 39-46)

**Goal:** AI summarize thread content, running on AI Inference Node (separate from Compute).

**Stack:**
- iPhone: Qwen2.5-0.5B (MLX, ~400MB)
- AI Node (Mac mini M4 Max): Qwen2.5-7B or Gemma2-9B (MLX, ~5GB)
- PII redaction mandatory before any inference (I-5 type system enforcement)
- ThermalMonitor gates inference during thermal stress

**TeraAiAdapter implementations:**
- Ollama local (default)
- Azure OpenAI private endpoint (enterprise)
- ONNX air-gapped bundle (Gov/Military)

**Milestone:** Publish `ai-adapter-sdk` (MIT). Measure prompt rejection rate < 0.5%. Begin SOC2 Type II observation period.

**Why not Gemma 4?** Gemma 4 doesn't have stable MLX export. Qwen2.5 and Gemma 2 do. Swap when stable.

---

## Platform Rollout (Progressive)

| Slice | Platforms | Why |
|-------|-----------|-----|
| **0-2** | macOS only | No UI, test-only |
| **3** | macOS + iPhone | Same Apple ecosystem, same Secure Enclave |
| **4** | macOS + iPhone | Mesh test on same ecosystem |
| **5** | + Android | .tapp needs broader test base |
| **6** | + Windows | Enterprise AI demand |

**Rule:** Only add platform when 3+ paying customers request it.

---

## Platform Coverage Matrix

| Platform | Core Engine | UI Framework | Secure Enclave | Slice |
|----------|------------|--------------|----------------|-------|
| **macOS** | Rust Core (native) | Tauri | Secure Enclave | 3 |
| **iPhone** | Rust Core (FFI) | Flutter | Secure Enclave | 3 |
| **Android** | Rust Core (Foreground Svc) | Jetpack Compose | StrongBox | 5 |
| **Windows** | Rust Core (native) | Tauri | TPM 2.0 | 6 |
| **Linux** | Rust Core (native) | Tauri | TPM 2.0 | Post-6 |
| **Mac Server** | TeraRelay binary | — | Secure Enclave | 2 |

---

## Solo Founder Reality — Budget & Hire Triggers

### Phân tích nguồn lực

| Scenario | Team Size | Timeline |
|----------|-----------|----------|
| Full senior team | 8-12 engineers | 12-18 months |
| **Solo + AI agents (khuyến nghị)** | **1 founder + AI harness** | **18 months** |
| Solo without AI agents | 1 engineer | 6-8 years (not feasible) |

### Hire Triggers

| Trigger | Role | Est. Cost | Slice |
|---------|------|-----------|-------|
| MLS implementation review | Applied Cryptographer (freelance) | $15,000-30,000 | 1 |
| 3+ pilots active, need support | Solutions Engineer | $60-80K/year | 3 |
| PQ-KEM implementation | Applied Cryptographer (freelance) | $15,000-30,000 | Post-6 |
| BLE mesh protocol tuning | Distributed Systems Engineer | $80-120K/year | 4 |
| ISO 27001 preparation | Compliance Consultant | $20,000-40,000 | Post-6 |
| 50+ enterprise customers | SRE + Support team | $100-150K/person/year | Post-6 |
| Revenue > $15K MRR | CTO/VP Engineering (full-time) | $120-180K + equity | 5+ |

### Budget per Slice

| Slice | Duration | Infra/month | External Cost | Total |
|-------|----------|-------------|---------------|-------|
| **0: Foundation** | 2 weeks | $30-50 | $0 | **~$50** |
| **1: Hello E2EE** | 6 weeks | $30-50 | $0 | **~$100** |
| **2: Relay** | 6 weeks | $50-100 | $0 | **~$150** |
| **3: UI** | 8 weeks | $50-100 | $0 | **~$200** |
| **4: HA + Mesh** | 8 weeks | $100-200 | $0 (solo) | **~$400** |
| **5: .tapp** | 8 weeks | $200-500 | $0 (solo + AI) | **~$1,000** |
| **6: AI** | 8 weeks | $300-800 | $0 (solo + AI) | **~$1,600** |

### Financial Principles

1. **No hire before revenue** — except Applied Cryptographer (freelance) for MLS review
2. **Hire one at a time** — never two people simultaneously
3. **Revenue thresholds:** $15K MRR → first engineer, $50K MRR → second engineer
4. **Pilot revenue target:** 3 pilots × $500-1,500/month = $1,500-4,500 MRR

---

## System Design: What Connects to What

```
tc-crypto (MLS E2EE)  →  tc-mesh (BLE/WiFi Direct)
                      →  tc-store (encryption keys)
                      →  Hardware (Secure Enclave / TPM)

tc-mesh               →  tc-crypto (session keys)
                      →  tc-crdt-sync (offline queue)
                      →  UI HUD (CoreSignal renderer)

tc-crdt-sync          →  tc-store (hot_dag.db)
                      →  tc-tapp (WASM state)
                      →  Relay (WAL replication)

tc-store              →  tc-crypto (encryption)
                      →  tc-crdt-sync (read/write)
                      →  FFI data path

tc-tapp (WASM)        →  tc-store (transient state)
                      →  tc-crypto (ABI key delegation)
                      →  AI Module (host_ai_invoke)

AI Module             →  tc-tapp (Host ABI boundary)
                      →  SanitizedPrompt (PII guard)
                      →  ThermalMonitor (resource gate)
                      →  MLX Runtime (local execution)

tc-ai (AI adapter)     →  SanitizedPrompt (PII type guard)
                      →  TeraAiAdapter trait (Ollama/Azure/ONNX)
                      →  AI Inference Node (optional, dedicated)

tapp-sdk (MIT)        →  Public WASM SDK for .tapp developers
                      →  Tapp trait (on_start, on_action, on_tick)
                      →  Manifest format + signing toolchain

ai-adapter-sdk (MIT)  →  Public SDK for AI adapter developers
                      →  TeraAiAdapter trait implementation guide

Relay                 →  All clients (mTLS/WSS)
                      →  RaftNode (WAL replication)
                      →  Object Storage (MinIO/R2)
```

---

## Pricing (License Tiers)

| Tier | Price/Year | Org Size | Concurrent Capacity | Key Features |
|------|-----------|----------|---------------------|--------------|
| **Starter** | $900/yr | ≤50 users | 80–120 | 1-click install, basic E2EE, community support |
| **Business** | $2,400/yr | ≤500 users | 300–500 | TeraLink Fallback, .tapp support, email support |
| **Enterprise** | $6,000/yr | ≤5,000 users | 1,500–2,000 | HA Cluster, AI Node, priority support, SLA |
| **Gov/Military** | $15,000+/yr | Negotiate | Negotiate | FIPS 140-3, air-gapped, HSM, Shamir 3-of-5, dedicated support |

**Ghi chú:** 'User' trong pricing = tổng tài khoản trong tổ chức. 'Concurrent' = số session hoạt động đồng thời tại peak. Một org 500 nhân viên với peak 200 concurrent → cần Business license (≤500 users) + Business hardware (300–500 concurrent capacity).

Pricing rẻ hơn 40–70% so với Slack/Teams tính per-org. TCO Calculator trên `terachat.io` so sánh 3-year cost là conversion tool chính.

## SOC2 Compliance Roadmap

| Giai đoạn | Slice | Scope | Cost |
|-----------|-------|-------|------|
| **G1: Preparation** | Slice 2 | Vanta/Drata setup, evidence collection, policy drafting | ~$5,000–8,000/yr |
| **G2: Type I Audit** | Slice 4 | Point-in-time assessment by boutique auditor | ~$20,000 total |
| **G3: Type II Audit** | Post-Slice 6 | 6-month continuous monitoring period | ~$30,000+ |

GDPR DPA template và HIPAA BAA template được chuẩn bị trong G2. FedRAMP để sau năm 3.

---

## Invariants — Never Violated

1. **Rust Core is domain owner** — UI is passive renderer only (CI enforced)
2. **ZeroizeOnDrop on ALL key material** — no `mem::forget`, `ManuallyDrop`, or `Arc<KeyMaterial>` (CI enforced)
3. **No raw pointer in `pub extern "C"`** — Token Protocol only (CI enforced)
4. **Dual-plane sync** — CRDT for chat, Relational for structured data (CI lint enforced)
5. **AI only after SanitizedPrompt** — PII redaction + type system enforced, compile-time guarantee
6. **Headless daemon + gRPC** before UI expansion (integration test enforced)
7. **Test never trails** — every slice has passing tests before demo (CI enforced)
8. **One slice at a time** — Progressive complexity, no parallel slices (process enforced)
9. **iOS election_weight = 0** — iPhone never Floor Gateway coordinator (CI enforced)
10. **NAS ECC sole storage authority** — Mac mini never primary DB writer (type system enforced)
11. **BLE text only ≤ 500 bytes** — no file/media over TeraLink T3 (type system enforced)
12. **.tapp no external network egress** — capability permanently blocked (Host ABI enforced)
13. **BSL boundary immutable** — LICENSE hash in git tag (CI enforced)

---

## Risk Burnout — Guardrails

- **Spec limit:** Don't write new specs when old specs have no running code. Rule: "1 spec → 1 prototype → validate → next spec."
- **Analysis paralysis:** 80+ docs is enough for 18 months of development. No more specs before prototype.
- **Minimum viable day:** At least 1 commit or 1 test passed per day. No "research only" days.
- **AI does the typing:** Human does architecture + review. Claude Code writes the code.
