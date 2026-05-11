---
type: synthesis
created: 2026-05-11
tags: [terachat, vision, redefinition, pivot, synthesis]
sources: [tera-intro, tera-core-spec, tera-eco-spec, tera-enclave-spec]
---

# Vision Redefinition — 2026-05-11

A synthesis of the TeraChat vision redefinition: what changed, why, and the implications for each phase and domain.

## What Changed

| Dimension | Before | After | Reason |
|-----------|--------|-------|--------|
| **Messaging Scope** | Internal + potential external/customer | Internal + branch companies only | Cannot force customers to switch messaging platforms |
| **Communication Model** | Open channels (Slack-like) | Hierarchical authority-based | Messages flow along org chart; compliance by design |
| **T-App Distribution** | IT-admin managed deployment | Self-service from Web Marketplace | Reduce IT bottleneck; SME-friendly |
| **T-App Scoping** | Enterprise-wide | By region, department, branch, role | Prevent t-app sprawl; need-to-know enforcement |
| **AI Strategy** | Future feature (Phase 5) | Core feature now — Gemma 4 bundled | Competitive differentiation; local AI = ZK integrity |
| **AI Provider Model** | Single bundled model | Open AI framework (bring your own) | Enterprise AI diversity; vendor flexibility |
| **Product Category** | Secure messaging platform | Work OS with messaging + AI | T-apps + local AI create a platform, not just a tool |

## What Stayed the Same

- **Zero-Knowledge architecture** — servers are blind routers, never see plaintext
- **Rust Core as domain owner** — UI is passive renderer
- **Headless daemon + gRPC** architecture
- **Dual-plane sync** — CRDT DAG for chat, Relational for structured data
- **Enterprise-only license model** — no public signup, no consumer tier
- **7 domain specs** — TERA-CORE through TERA-ECO
- **Phase execution framework** — 7 phases, Phase 0 → Phase 6

## Implications by Phase

### Phase 0 (Architecture Foundation)
- **No change.** Boundaries and contracts remain valid. Authority hierarchy is a TERA-GOV concern, already within scope.

### Phase 1 (Trust Kernel)
- **No change.** Crypto, daemon, mesh are unaffected by messaging scope.

### Phase 2 (Dual-Sync)
- **Minor impact.** Authority hierarchy adds a `reporting_line` dimension to workspace metadata but does not change the sync architecture.

### Phase 3 (Client Bridge)
- **Moderate impact.** UI must render authority hierarchy indicators (who can I message?). The PENDING_SECURE_CHANNEL widget acquires an authority scope dimension.

### Phase 4 (WASM Ecosystem)
- **Significant impact.** Self-service t-app deployment requires:
  - Simplified setup instructions (no DevOps assumed)
  - Regional/departmental scoping UI in Admin Console
  - Marketplace browsing experience
  - One-click deploy to scope target

### Phase 5 (Governance + AI)
- **Major impact.** Three new subsystems:
  1. **Hierarchical authority enforcement** — OPA policies for authority-gated workspace creation
  2. **Open AI framework** — Model registration ABI, integrity verification, provider adapters
  3. **Gemma 4 integration** — ONNX bundling, CoreML export, LoRA adapter support

### Phase 6 (Chaos + Release)
- **Moderate impact.** New chaos scenarios needed:
  - SC-41: Authority hierarchy partition (branch loses HQ connection)
  - SC-42: AI model integrity failure (BLAKE3 mismatch)
  - SC-43: T-app scoping violation attempt (region escape)

## New Concepts Created

| Concept | File | Summary |
|---------|------|---------|
| Hierarchical Authority Messaging | `hierarchical-authority-messaging.md` | Communication flows along org chart, no customer channels |
| Open AI Framework | `open-ai-framework.md` | Bring-your-own-model framework, Gemma 4 as default |

## Concepts Updated

| Concept | Changes |
|---------|---------|
| Architecture Overview | Added product redefinition table, communication model diagram, AI layer, updated domain spec map |
| WASM Tapp Runtime | Added Work OS vision, self-service deployment, regional/departmental scoping, marketplace vetting |
| Secure Enclave AI | Added Gemma 4 specification, open AI framework integration, on-device local AI model |

## Open Questions

1. **Gemma 4 RAM budget:** 4GB variant vs 8GB variant — which is the minimum bar for employee devices?
2. **Marketplace revenue model:** 30% publisher share referenced in Phase Framework — does this still hold for self-service model?
3. **Authority hierarchy depth:** How many levels of hierarchy are supported? Unlimited or capped?
4. **Inter-branch latency:** HQ-authorized inter-branch channels add governance step — acceptable latency for channel creation?
5. **AI model vetting:** What is the vetting standard for third-party models registered through the open framework?
