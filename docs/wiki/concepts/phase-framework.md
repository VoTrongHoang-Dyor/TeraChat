---
type: concept
created: 2026-05-10
modified: 2026-05-11
tags: [terachat, roadmap, phases, milestones, business, work-os, local-ai]
sources: [tera-intro, tera-core-spec, tera-gov-spec, tera-tech-debt]
---

# Phase Framework — Economic Milestones

TeraChat's development is organized into three economic phases. Each phase must be complete before the next begins — no parallel execution of phase-specific work.

## Phase 1 — "Enough to Sign the Pilot" (January–June)

**Goal:** An IT admin at a tier 2 bank can deploy TeraChat in the morning and demo internal messaging + one t-app to the Board in the afternoon.

### Must Work

| Component | Requirement | Spec Status |
|-----------|-------------|-------------|
| **TeraRelay Single Binary** | `curl -fsSL https://install.terachat.io \| sudo bash` → auto TLS, auto SQLite WAL, auto OPA policy → "TeraChat ready at https://your-domain.com". Target: IT admin with no DevOps background, 30-minute deploy. | ⚠️ No deployment automation spec exists |
| **MLS E2EE Internal Messaging** | E2EE group chat within organization, E2EE file transfer, epoch rotation on member leave. Authority hierarchy: employees message within their department and up/down the chain. Branch companies communicate via HQ-authorized channels. NO customer-facing messaging. | Partially in TERA-CORE §4.3 — authority hierarchy not yet mapped |
| **JWT License + DeviceIdentityKey** | License binding must be correct — this is the revenue model foundation. Wrong = customers share licenses, no revoke on expiry. | Scattered across TERA-INTRO and concepts — no unified protocol spec |
| **OIDC/SAML with Google Workspace + Azure AD** | Map SAML attributes → TeraChat roles + authority position in hierarchy. No new passwords — high friction = low adoption = failed pilot. | TERA-GOV §2.2 shows flow but NO SAML→hierarchy mapping defined |
| **One Reference .tapp** | Demonstrate the Work OS concept: one vetted .tapp (e.g., Expense Approval) that an IT admin can deploy to the Finance department in under 10 minutes. Self-service with simple instructions. | No reference .tapp spec exists |

### Explicitly NOT in Phase 1
- Survival Mesh (too complex to debug while running a pilot)
- Complete Admin Console (minimal version only)
- Mac mini HA cluster (single node only)
- .tapp Marketplace (one reference .tapp, not a marketplace)
- ZK Memory Agent
- Gemma 4 AI integration (Phase 2)
- Open AI Framework (Phase 2)
- Customer messaging (never — out of scope permanently)

## Phase 2 — "Enough to Renew and Upsell" (June–Month 18)

**Goal:** Once 2–3 pilots are converted to ARR, the question becomes: why will they renew and upgrade tier?

### Must Work

| Component | Requirement | Spec Status |
|-----------|-------------|-------------|
| **Survival Mesh** | The real USP. Manufacturing sector story: "Your factory in Binh Duong lost internet for 4 hours — TeraChat still ran." BLE 5.0 + Wi-Fi Direct, EMDP, election protocol. | TERA-CORE §4–5 extensive but AWDL risk unacknowledged |
| **Complete Admin Console** | SCIM 2.0 auto-offboarding, Remote Wipe, audit logs → PDF with Ed25519 signatures. Authority hierarchy management: define departments, branches, reporting lines. T-app deployment scoping by region/department. | Only UI patterns in TERA-DESIGN §23. No endpoint/RBAC/export pipeline spec. |
| **.tapp Web Marketplace** | Self-service: IT admin browses → purchases on web → downloads → deploys to region/department. All t-apps vetted by TeraChat. **Payment processed on terachat.io, never in app.** Simple setup instructions per t-app. Regional/departmental scoping built in. | TERA-ECO covers app signing, registry, kill-switch. Marketplace browsing/search/payment/self-service not yet specified. |
| **Mac Mini HA Cluster** | 2-Mac-mini Active-Passive, shared NAS, WAL replication, health check + failover. Required for 99.999% SLA enterprise tier. | TERA-CORE §2.4 mentions topology but no failover protocol spec. |
| **Gemma 4 Local AI (Initial)** | Bundled Gemma 4 ONNX model on employee devices. Initial use case: thread summarization, response drafting. Runs 100% local — no cloud API call. | TERA-ENCLAVE covers concept. Actual Gemma 4 integration: not yet specified. |
| **Open AI Framework (Initial)** | Model registration ABI. Enterprise can register one custom ONNX model. BLAKE3 integrity check on load. | Not yet specified — new concept. |

### Key Metric
Switching cost + value-over-time > renewal friction. Without good Admin Console + at least one AI feature that saves employees time, renewal rate drops regardless of core quality.

## Phase 3 — "Moat and Ecosystem" (Month 18+)

**Goal:** Long-term defensibility through AI features, marketplace lock-in, and Gov/Military contracts.

### Must Work

| Component | Requirement | Spec Status |
|-----------|-------------|-------------|
| **Full Open AI Framework** | Multiple model registration. Bring-your-own API key for cloud models via enterprise relay. LoRA adapter support for enterprise fine-tuning. Department-specific model deployment. | Not yet specified |
| **ZK Memory Agent** | Replace vector DB — AI features without breaking ZK invariant. Consolidation on-device via UDS. | TERA-ENCLAVE covers concept. TD-005 (IPC contract) still unresolved. |
| **.tapp Marketplace (Full)** | Third revenue stream: 30% publisher revenue share. Only valuable with 50+ enterprise customer base. Third-party publishers with full vetting pipeline. **All payment and payout processed on terachat.io.** | TERA-ECO covers app signing, registry, kill-switch. Marketplace billing/search/review/payment not yet specified. |
| **Gov/Military Tier** | Air-gapped deployment, Shamir 3-of-5, Anti-Insider Key Ceremony. Highest LTV segment. Requires ISO 27001 + 2 prior commercial references. Sales cycle 18–24 months. | TERA-CORE §2.4 mentions air-gapped tier. TERA-TEST has Gov/Military gate scenarios. |

### Key Timeline
Relationship building with Gov/Military should begin December (Month 7), close by Month 30.

## 🧠 Design Decisions (Q&A)

- **Why Mesh in Phase 2, not Phase 1?** → Debugging Mesh while running a pilot kills the team. The demo value is high but the implementation is the most complex in the stack. Phase 1 needs a stable core first.
- **Why single-node before HA?** → 99.9% SLA on one Mac mini is sufficient for pilot. 99.999% requires HA which adds complexity. Sell the pilot on the core value, upsell on reliability.
- **Why Marketplace only at 50+ customers?** → Two-sided marketplace needs publisher incentive. Publishers won't build for a platform with 5 enterprise customers. Don't invest before there's demand.
- **Why Gemma 4 in Phase 2, not Phase 1?** → Phase 1 proves the messaging core. Phase 2 adds the AI layer that makes TeraChat a Work OS, not just a chat app. Bundling AI too early distracts from proving the foundational messaging reliability.
- **Why no customer messaging — ever?** → TeraChat cannot solve the network coordination problem of forcing external customers onto a new platform. Email, phone, and support desks already serve this function. TeraChat focuses on the internal coordination problem where the enterprise controls both endpoints.
