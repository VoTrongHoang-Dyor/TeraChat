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

## [2026-05-12] query | Brainstorm AI Gateway Architecture — Thay thế Local Proxy
- **Trigger:** Yêu cầu loại bỏ `ANTHROPIC_BASE_URL="http://localhost:8082"` local proxy pattern
- **Vấn đề xác định:** 5 rủi ro production: plaintext loopback, PII Gate bypass, thiếu audit trail, không scale, single point of failure
- **Bốn hướng đánh giá:** Native Rust SDK (H1), Unix Domain Socket (H2), TeraRelay Extension (H3), Cloudflare Gateway (H4)
- **Quyết định: Hybrid H3 → H1**
  - Phase 1: H3 TeraRelay Extension — reuse auth stack, giữ "1 binary 1 command" deployment
  - Phase 2D: H1 Native Rust SDK trong `tc-enclave` — full control, zero intermediate hop
  - H4 Cloudflare: REJECTED vĩnh viễn — vi phạm Zero-Knowledge + Offline-First invariants
- **Tài liệu tạo:** `wiki/concepts/adr-006-ai-gateway-architecture.md`
- **Index cập nhật:** Thêm ADR-006 vào Core Architecture section
- **5 câu hỏi cần lock:** AI endpoint Phase 1, ONNX PII Gate location, rate limit unit, offline behavior, audit scope
- **Key takeaway:** Local HTTP proxy là dev shortcut hợp lệ nhưng không phải production architecture. TeraRelay Extension là path ít friction nhất cho Phase 1 vì reuse existing auth infrastructure.

## [2026-05-12] refactor | Chuyển dịch thanh toán sang Web — Loại bỏ In-App Payment

- **Trigger:** Loại bỏ các thiết kế kiến trúc cũ về thanh toán ngay trên app
- **Quyết định kiến trúc:**
  - **Payment on Web Only:** Tất cả giao dịch (mua license, gia hạn, nâng cấp tier, mua .tapp) được xử lý trên **trang chủ web terachat.io**
  - **App không xử lý thanh toán:** App chỉ nhận License JWT đã cấp và xác thực — không có payment form, checkout flow, hay billing page
  - **.tapp Web Marketplace:** Mua .tapp trên web, app chỉ download, xác thực chữ ký, và chạy trong sandbox
  - **Admin Console:** Nút "Renew" được thay bằng "Manage on terachat.io" — redirect ra web dashboard
- **Nguyên tắc bất biến mới:** Payment processing không bao giờ tồn tại trong app codebase
- **Files đã cập nhật:**
  - `docs/wiki/concepts/enterprise-license-model.md` — Thêm "Payment on Web Only" section + Design Decision
  - `docs/wiki/concepts/wasm-tapp-runtime.md` — Làm rõ Web Marketplace là nơi duy nhất mua .tapp
  - `docs/raw/MD/Design.md` — License Dashboard: nút "Manage on terachat.io", Plugin Panel: link web marketplace
  - `docs/raw/MD/Introduction.md` — Cập nhật flow license-gated architecture
  - `README.md` — Cập nhật mô hình truy cập và .tapp marketplace
  - `docs/wiki/concepts/phase-framework.md` — Marketplace billing trên web
  - `docs/wiki/syntheses/deployment-automation-spec.md` — IT Admin mua license trên web
  - `docs/raw/MD/Directory-tree.md` — Web_Marketplace.html mô tả
  - `docs/HTML/Pricing_Packages.html` — Cover notice: "Payment on Web Only"
  - `phase/README.md` — Web Marketplace payment notation
- **Key takeaway:** TeraChat là enterprise platform với procurement cycle phức tạp — không thể tự động hóa qua in-app purchase. Tách payment khỏi app giữ app đơn giản và tập trung billing/compliance lên web dashboard nơi IT Admin quản lý toàn bộ quy trình tài chính.

## [2026-05-12] resolve | Technical Audit Resolution — Giải quyết toàn bộ GAPs & Tech Debt Specs

- **Trigger:** Báo cáo TeraChat_TechnicalAudit.html — 52/100 pre-prototype, 16 tech debt CRITICAL, 10 GAPs chưa giải quyết
- **Nguyên tắc:** Điều chỉnh tài liệu kỹ thuật (chưa có code) để tránh nợ và xung đột kỹ thuật trong tương lai
- **GAP Resolution — tất cả 10 GAPs đã có quyết định kiến trúc cuối cùng:**
  - GAP-A: SagaRecoveryGuard protocol — retry limit 3, manual intervention path qua Admin Console
  - GAP-B: WAL handshake signals — thêm vào CoreSignal enum, timeout 5s + exponential backoff, read-only mode silent
  - GAP-C: NSE TOCTOU — POSIX flock() + memory-mapped ring buffer, không dùng SQLite cho NSE staging
  - GAP-D: MemoryPressureWarning — đã resolved trước đó
  - GAP-E: DataGrant quorum — weighted voting (HQ=3, Regional=2, Branch=1), generation counter từ 1
  - GAP-F: Huawei disclosure — đã cập nhật Pricing_Packages.html với cảnh báo rõ ràng
  - GAP-G: Burner Agent + EMDP Freeze — queue removal, force unfreeze 2h threshold, Admin signatures
  - GAP-H: Float detection — đã resolved trước đó
  - GAP-I: Binary Transparency gossip — đã resolved trước đó
  - GAP-J: Outbox Queue TTL — đã resolved trước đó
- **Tech Debt Resolution Specs — thêm 6 protocol specs vào Tech_Debt.md §5:**
  - §5.8: EMDP Hybrid PQ-KEM Escrow (TD-009) — Hybrid ECIES + ML-KEM-768, RaptorQ FEC fragmentation
  - §5.9: Hardware Monotonic Clock (TD-010) — mach_absolute_time, clock rollback detection, TimeSource trait
  - §5.10: Secure Streaming Protocol (TD-011) — UDS + SO_PEERCRED (Desktop), OTST (Mobile), stream encryption
  - §5.11: Duress PIN + Exponential Backoff Tarpit (TD-012) — exponential backoff, Duress PIN, remote wipe
  - §5.12: Tenant-Salted CAS Dedup (TD-013 + TD-014) — workspace-salted hash, dual-algorithm for Gov tier
  - §5.13: Thermal Budget Management Protocol (TD-016) — ThermalStateMonitor, 4 throttling levels, recovery
- **Infrastructure Specs Created:**
  - `ci-cd-pipeline-spec.md` — Progressive CI gates (Phase 0 → Phase 2+), secrets management, hermetic builds, runner infrastructure
  - `deployment-automation-spec.md` (enhanced) — Backup/recovery procedures, monitoring stack (Prometheus+Loki+Grafana), staging environment, configuration reference, infrastructure cost model
  - `platform-limitation-registry.md` — 10 XPLAT items với disclosure requirements, platform SLA matrix, testing requirements per platform
- **AI-Vibe Coding Guardrails:**
  - `CLAUDE.md` — 8 architectural invariants, 8 forbidden patterns, AI compatibility matrix, code review checklist, crypto code requirements
  - `.claude/settings.local.json` — invariants, forbidden_patterns, security_critical_paths, pre_commit hooks
- **Phase Plan Update:**
  - `phase/README.md` V3.1 — Solo founder reality analysis, hire triggers, realistic budget per phase, burn-out warnings, minimum viable day rule
  - Budget estimates: Prototype ~$100-200, Phase 1 ~$200-400, full path ~$30-55K (6 tháng) hoặc $200-500K/năm (Phase 3B)
- **Pricing Update:**
  - `Pricing_Packages.html` — Thêm Huawei disclaimer (GAP-F): Huawei chỉ Standard tier, SLA 4h, không Enterprise/Gov
- **Files changed:** 10 files (4 created, 6 updated)
- **Key takeaway:** Tất cả 10 GAPs đã có quyết định kiến trúc cuối cùng. Tất cả 16 tech debt items đã có protocol resolution cụ thể. Project sẵn sàng để bắt đầu prototype mà không có unresolved architectural questions.

## [2026-05-15] restructure | Reframe Wiki to Vertical Slice + Multi-Agent Harness Architecture

Major documentation restructure to align with new development philosophy: Vertical Slice over Horizontal Layer, Deep Modules (Matt Pocock), and Multi-Agent Harness with LangGraph.

**Core agent files created:**
- `AGENT_CONTEXT.md` (project root) — First file every agent reads: project overview, reading order, file scope per agent
- `docs/wiki/ubiquitous-language.md` — Shared vocabulary EN+VI, anti-patterns, code conventions
- `docs/wiki/invariants.md` — Non-negotiable architectural rules with code examples

**New concept pages (8):**
- `docs/wiki/concepts/vertical-slice-development.md` — Shippable slices every 6-8 weeks, Slice 0-6 overview
- `docs/wiki/concepts/multi-agent-harness.md` — LangGraph orchestrator, agent types, daily workflow
- `docs/wiki/concepts/deep-module-design.md` — Matt Pocock principle applied to Rust, CI enforcement
- `docs/wiki/concepts/ai-inference-offloading.md` — InferenceScheduler, ThermalMonitor, ModelTiers, PII gate
- `docs/wiki/concepts/mac-mini-ha-cluster.md` — One-touch setup, mDNS discovery, Raft consensus, SLA tiers
- `docs/wiki/concepts/tapp-community-framework.md` — .tapp SDK, TappValidator, contribution flow
- `docs/wiki/concepts/ios-mesh-storage-tiers.md` — BufferTier, auto-detection, eviction policy
- `docs/wiki/concepts/openmls-self-healing.md` — AI debug loop, ErrorContext sanitization, fine-tuning

**Agent orchestrator:**
- `.agents/langgraph/terachat_graph.py` — LangGraph StateGraph: grooming → TDD → implement → invariant check → security → integration
- `.agents/grooming-template.md` — Design-first checklist for task assignment

**Updated files:**
- `CLAUDE.md` — v2.0.0: Added Deep Module Principle, Vertical Slice, Multi-Agent Harness, Model Tiers, iOS constraints
- `phase/README.md` — v4.0.0: Replaced Phase 1-6 with Slice 0-6, 18-month timeline, updated budget/hire triggers
- `docs/wiki/index.md` — Added all new pages, updated product definition
- `CONTRIBUTING.md` (new) — .tapp developer guide with TDD workflow
- `.claude/settings.local.json` — Removed invalid `hooks.pre_commit` (not a valid Claude Code hook event)
- `.git/hooks/pre-commit` (new) — Bash hook: cargo fmt + clippy + gitleaks

**Key takeaway:** Documentation now reflects the solo-dev reality: vertical slices for fast feedback, deep modules for AI agent compatibility, multi-agent harness for scaling output. All architectural modules (AI inference, HA, .tapp, mesh storage, MLS self-healing) documented as concept pages.
