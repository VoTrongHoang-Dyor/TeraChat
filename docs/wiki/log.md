# Wiki Log

## [2026-05-06] init | Initialized LLM Wiki
- Created folder structure: `raw/`, `wiki/`
- Wrote `CLAUDE.md` schema
- Set up `index.md` and `log.md`
- Ready for first ingest

## [2026-05-06] ingest | LLM Overview Sample
- Source: `raw/articles/llm-overview.md`
- Created pages:
  - `wiki/sources/llm-overview.md` (source summary)
  - `wiki/concepts/large-language-models.md` (new concept)
  - `wiki/concepts/transformer-architecture.md` (new concept)
- Updated `wiki/index.md` with 3 new pages
- Key takeaway: LLMs use Transformer architecture, have defined capabilities/limitations

## [2026-05-10] ingest | TeraChat Full Documentation Ingest
- Sources: 15 files from `raw/MD/` — all TeraChat project documentation
- Created 15 source summary pages in `wiki/sources/`:
  - Gateway: `tera-intro`, `tera-design`, `tera-arrange`, `tera-note`
  - Quality: `tera-tech-debt`, `tera-test-matrix`
  - Core Specs (7): `tera-core-spec`, `tera-sync-spec`, `tera-runtime-spec`, `tera-enclave-spec`, `tera-gov-spec`, `tera-client-spec`, `tera-eco-spec`
  - Additional Specs: `tera-migration-spec`, `tera-export-spec`
- Created 8 concept pages in `wiki/concepts/`:
  - `terachat-architecture-overview` — system layers, spec dependency graph
  - `zero-knowledge-architecture` — blind router model, key isolation
  - `enterprise-license-model` — license-gated access, deployment tiers
  - `crdt-dual-sync` — two-plane sync architecture
  - `wasm-tapp-runtime` — dual-engine WASM sandbox
  - `survival-mesh-networking` — BLE/Wi-Fi Direct P2P, EMDP
  - `enterprise-identity-governance` — DID, OPA, RBAC, SCIM
  - `secure-enclave-ai` — on-premise AI, PII redaction
  - `data-sovereignty-export` — SPF format, streaming export
  - `glassmorphism-design-system` — security-visible UI design
- Updated `wiki/index.md` with full catalog
- Key takeaway: TeraChat is a Zero-Knowledge E2EE enterprise messaging platform with 7 core domain specs, enterprise-only license model, and offline-first survival mesh
- Cross-references: all pages linked via wikilinks between sources and concepts

## [2026-05-10] lint | Wiki Health Check Audit
- Ran automated checks: `obsidian unresolved`, `obsidian tags counts`, full-text searches
- Found 5 factual issues:
  - BLE 5.0 data rate: corrected from "4.75 kbps" to "~125 kbps PHY / ~5 kbps app-layer" in `survival-mesh-networking.md`
  - Wi-Fi Direct bandwidth: corrected from "250 Mbps" to "30–80 Mbps (real-world)" in `survival-mesh-networking.md`
  - AWDL iOS risk: flagged as HIGH — Apple private framework, no public API, could block iOS mesh strategy
  - MLS RFC 9420 number: needs external verification (web search blocked)
  - ML-KEM FIPS 203: needs external verification — may still be draft
- Fixed tag hygiene: hex colors `#0F172A`, `#1A1A2E`, `#24A1DE` escaped with backticks to prevent Obsidian tag parsing
- Identified 4 critical Phase 1 gaps: deployment automation missing, SAML attribute mapping undefined, JWT license spec scattered, Admin Console unspecified
- Identified 2 Phase 2 gaps: Mac Mini HA failover protocol not detailed, AWDL risk unacknowledged
- Created pages:
  - `wiki/syntheses/wiki-health-check-2026-05-10.md` — comprehensive health check report
  - `wiki/concepts/phase-framework.md` — three-phase economic milestone framework
- Updated `wiki/index.md`
- Remaining: tag consolidation (`#survival`+`#mesh`→`#survival-mesh`, `#data-sovereignty`+`#sovereignty`), legacy broken wikilinks (`OpenAI GPT-4`, `Vaswani et al.`)

## [2026-05-11] redefine | TeraChat Vision Redefinition
- **Trigger:** Founder redefined TeraChat's scope, communication model, and AI strategy
- **Scope change:** Internal + branch company messaging ONLY. Customer-facing messaging permanently out of scope — cannot force customers to switch platforms.
- **Communication model:** Hierarchical authority-based. Messages flow along org chart (vertical up/down, horizontal peer, inter-branch via HQ authorization).
- **Product category:** Work OS — not just a messaging app. Business tasks run through .tapp mini-applications.
- **T-app model:** Self-service from TeraChat Web Marketplace. Vetted by TeraChat, businesses download and set up themselves. Deployable by region or department.
- **AI strategy:** Gemma 4 (Google open model) as bundled default for on-device local AI. Open AI framework for enterprises to bring their own models.
- **AI goal:** Bring data to the customer. Bring local AI to the machine to help employees automate tasks.
- **Created concepts:**
  - `hierarchical-authority-messaging.md` — authority-gated communication model, messaging scope matrix
  - `open-ai-framework.md` — Gemma 4 default model, model registration ABI, bring-your-own-model architecture
- **Updated concepts:**
  - `terachat-architecture-overview.md` — added product redefinition table, communication model diagram, AI layer, updated domain spec map with AI ABI
  - `phase-framework.md` — added Gemma 4 to Phase 2, open AI framework to Phase 3, explicit "no customer messaging" in all phases, reference .tapp for Phase 1
  - `wasm-tapp-runtime.md` — added Work OS vision, self-service deployment model, regional/departmental scoping, marketplace vetting pipeline
  - `secure-enclave-ai.md` — added Gemma 4 specification, open AI framework integration, on-device local AI model diagram, updated design decisions
- **Created synthesis:**
  - `syntheses/vision-redefinition-2026-05-11.md` — comprehensive change log: what changed, why, implications per phase, open questions
- **Architecture impact:** TERA-RUNTIME now includes AI inference ABI. TERA-ENCLAVE now owns Gemma 4 + open AI framework. TERA-GOV now owns authority hierarchy enforcement. TERA-ECO now owns Web Marketplace self-service.
- **Phase impact:** Phase 4 (WASM) gets self-service deployment + scoping. Phase 5 (AI) gets Gemma 4 + open framework as core deliverables, not future features. AI is now Phase 2+ concern, not just Phase 5.
- **Open questions:** Gemma 4 RAM budget (4GB vs 8GB), marketplace revenue model confirmation, authority hierarchy depth limit, inter-branch channel creation latency target, third-party AI model vetting standard.
- **Key takeaway:** TeraChat is now explicitly a Work OS with internal hierarchical messaging + local AI, not a general-purpose secure chat app. Customer messaging is permanently excluded.

## [2026-05-11] improve | Systematic Improvement & Restructuring
- **Trigger:** Comprehensive evaluation (65/100) identified 14 weaknesses across structure, goals, technical, requirements, feasibility, market fit, and approach
- **Created Improvement Hub:** `syntheses/improvement-plan-2026-05-11.md` — central navigation: 14 weaknesses → 14 solutions, step-by-step execution order, status tracking
- **Created 9 synthesis documents** addressing specific weaknesses:
  - `narrowed-phase-1-mvp.md` — Phase 1 scope reduction: MLS + License + OIDC/SAML + 1 ref .tapp. Timeline 3-4 months (not 5 days). PQ-KEM, mesh, marketplace, AI deferred to Phase 2+
  - `gap-resolution-tracker.md` — All 10 GAPs (A-J) with proposed resolutions, concrete decisions needed, current status (4 resolved, 6 pending)
  - `platform-rollout-phasing.md` — Platform strategy: macOS+iPhone first (Apple ecosystem) → Android/Oppo → Windows → Linux/Huawei. Each new platform adds 1 month minimum
  - `prototype-first-model.md` — Build MLS E2EE chat prototype in 4-6 weeks before more specs. Progressive complexity: 1 subsystem per phase
  - `deployment-automation-spec.md` — The missing spec: 1-command deploy, 30-minute IT admin target, health checks, failure recovery
  - `quantitative-phase-metrics.md` — Hard metrics for every phase (3 signed pilots, ≥ 80% retention, NPS ≥ 40, ≥ $15K MRR, ≥ $1M ARR)
  - `ai-independent-workstream.md` — AI decoupled from messaging via Host ABI boundary. AI ships independently, fails independently, has separate pricing
  - `security-review-requirements.md` — Review gates per phase: internal review → Security Architect → external Applied Cryptographer (Phase 2A) → third-party audit (Phase 3A)
- **Updated phase/README.md to V3:**
  - Timeline: 18-24 months (replaced 35-day plan)
  - Platform rollout phasing integrated
  - Quantitative gate metrics per phase
  - 1 subsystem per phase (Progressive Complexity)
  - Prototype Phase inserted before Phase 1
  - Links to all synthesis documents
- **Created phase/prototype-phase.md** — 4 task boxes: MLS Core, macOS+iPhone UI, OIDC+Deploy, Hardening. Target: demo to 5+ enterprises in 4-6 weeks
- **Rewrote phase/phase-1-trust-kernel.md** — Narrowed to 5 task boxes: Production MLS, License JWT, OIDC/SAML, Deployment Automation, Reference .tapp. Deferred PQ-KEM, mesh, marketplace, AI. Phase 1 gate: 7 quantitative metrics
- **Updated wiki/index.md** — Added 10 synthesis files under "Improvement & Restructuring" section
- **Key structural changes:**
  - Prototype-first: working code before more specs
  - Progressive complexity: 1 subsystem per phase, no parallel subsystem builds
  - Quantitative gates: no phase transition without hitting metrics
  - AI decoupled: separate workstream, separate pricing, separate failure domain
  - Platform discipline: only add platforms when customers demand them
  - Security review budgeted: $15K-$100K depending on phase
- **Open questions remaining:** Gemma 4 RAM budget, marketplace revenue model, authority hierarchy depth, inter-branch latency, AI model vetting standard
- **Key takeaway:** TeraChat architecture is excellent but was over-scoped and under-validated. The restructuring provides a realistic 18-24 month path focused on proving value incrementally rather than building everything at once.
