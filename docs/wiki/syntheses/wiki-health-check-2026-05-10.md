---
type: synthesis
created: 2026-05-10
tags: [terachat, lint, health-check, audit, gaps]
sources: [tera-intro, tera-core-spec, tera-gov-spec, tera-client-spec, tera-sync-spec, tera-runtime-spec, tera-enclave-spec, tera-eco-spec, tera-tech-debt, tera-note]
---

# Wiki Health Check — 2026-05-10

Comprehensive lint of wiki data integrity, factual accuracy, completeness, and cross-reference consistency. Prioritized against the three-phase economic framework.

## 1. Factual Issues Found

### 1.1 BLE 5.0 Data Rate — Incorrect Figure
- **Where:** `[[Survival Mesh Networking]]`, TERA-CORE, TERA-TEST
- **What:** Claims BLE 5.0 Coded PHY provides "~4.75 kbps"
- **Reality:** BLE 5.0 Coded PHY (S=8) delivers ~125 kbps at PHY layer. 4.75 kbps may reflect application-layer throughput after BLE MTU fragmentation, GATT overhead, and Ed25519 signature wrapping — but this is not stated. The raw PHY number should be clarified to avoid confusion.
- **Severity:** MEDIUM — misrepresents BLE capability to technical evaluators.
- **Action:** Add clarifying note distinguishing PHY rate from app-layer throughput.

### 1.2 Wi-Fi Direct Bandwidth — PHY vs Real-World
- **Where:** `[[Survival Mesh Networking]]`
- **What:** Claims "Up to 250 Mbps"
- **Reality:** Achievable at 802.11ac PHY layer only. Real-world throughput is 30–80 Mbps due to protocol overhead, contention, and half-duplex constraints.
- **Severity:** MEDIUM — could cause unrealistic performance expectations in pilot evaluations.
- **Action:** Qualify as "PHY layer up to 250 Mbps; real-world 30–80 Mbps."

### 1.3 AWDL — No Public API (Unacknowledged Risk)
- **Where:** `raw/MD/Note.md` lists `NWPathMonitor` for "AWDL detection" and `MultipeerConnectivity / AWDL adapter` as iOS dependencies
- **Reality:** AWDL is Apple's private framework with NO public API. It is used by AirDrop/AirPlay internally. Third-party apps cannot programmatically use AWDL. The OWL project reverse-engineered it, but Apple can break it with any iOS update.
- **Severity:** HIGH — if the mesh strategy depends on AWDL for iOS P2P communication, this is a technical dead end.
- **Action:** Clarify in TERA-CORE whether the iOS mesh path uses BLE-only or if there's a dependency on AWDL. If AWDL is required, this is a Phase 2 blocker.

### 1.4 MLS RFC Number — Needs Verification
- **Where:** TERA-CORE references "MLS RFC 9420"
- **Status:** Could not verify externally (web search blocked). Training data cutoff may not include final RFC number. If the number is incorrect, it undermines cryptographic credibility in security audits.
- **Action:** Verify RFC number against IETF datatracker. Consider using "MLS (IETF RFC 9420)" or "MLS Protocol (messaginglayersecurity.org)" to be safe if number is uncertain.

### 1.5 ML-KEM / FIPS 203 — Needs Verification
- **Where:** TERA-CORE references "ML-KEM-768 (chuẩn NIST FIPS 203)"
- **Status:** Could not verify externally. ML-KEM was in draft FIPS 203 as of 2024. Whether it's been published as final standard affects Gov/Military compliance claims.
- **Action:** Verify FIPS 203 publication status. If still draft, qualify as "NIST FIPS 203 (draft)" in Gov-tier documentation.

## 2. Critical Gaps — Phase 1 ("Sign the Pilot")

### 2.1 Deployment Automation — COMPLETELY MISSING
- **User's Phase 1 requirement:** `curl -fsSL https://install.terachat.io | sudo bash` → prompts for domain, license, admin email → auto-generates TLS, SQLite WAL, OPA policy → "TeraChat ready at https://your-domain.com"
- **Wiki status:** TERA-CORE §2.4 lists deployment topologies but has ZERO detail on the install script, auto-configuration, TLS cert generation (Let's Encrypt? self-signed?), or the bootstrap OPA policy.
- **Severity:** CRITICAL — this is the #1 conversion mechanism from "interested" to "pilot." No spec = no implementation.
- **Recommendation:** Create `Spec-Deployment-Automation.md` as a Phase 1 gate document.

### 2.2 SAML Attribute → Role Mapping — UNDEFINED
- **User's Phase 1 requirement:** Map SAML attributes from Google Workspace and Azure AD to TeraChat roles.
- **Wiki status:** TERA-GOV §2.2 shows the Identity Federation Flow with Keycloak/Dex and §3.1 defines `OidcCapabilityToken` as "Maps SAML assertion → TeraChat role." But **no mapping table exists** — which Azure AD group maps to "IT Admin"? Which Google Workspace OU maps to "Member"?
- **Severity:** CRITICAL — without this mapping, OIDC/SAML integration cannot be implemented. IT admins cannot configure the bridge.
- **Recommendation:** Add § to TERA-GOV with explicit mapping table for Google Workspace and Azure AD.

### 2.3 JWT License Claims — SCATTERED, NO UNIFIED SPEC
- **User's Phase 1 requirement:** JWT license + DeviceIdentityKey binding must work correctly — it's the revenue model foundation.
- **Wiki status:** TERA-INTRO mentions `{tenant_id, domain, max_seats, tier, valid_until, features}`. TERA-CORE has zero JWT license detail. The binding mechanism (KDF(License JWT, DeviceIdentityKey)) is mentioned in concepts but has no protocol specification.
- **Severity:** HIGH — the mechanism is described in prose but has no implementable spec (token format, validation flow, error codes, renewal protocol).
- **Recommendation:** Add `LicenseProtocol` section to TERA-CORE or create a dedicated spec.

### 2.4 Admin Console — MENTIONED BUT UNSPECIFIED
- **User's Phase 1 mention:** "expose the Admin Console via browser"
- **User's Phase 2 requirement:** "A complete Admin Console" with SCIM offboarding, Remote Wipe, audit log PDF export with Ed25519 signatures.
- **Wiki status:** TERA-DESIGN §23 has UI patterns for Admin Console (License Status Dashboard, Plugin Registry Panel). TERA-GOV has SCIM offboarding protocol. But there is NO dedicated Admin Console spec covering: what endpoints it exposes, authentication, RBAC for admin actions, the PDF export pipeline, or the Remote Wipe protocol.
- **Severity:** HIGH — Admin Console is referenced as working in Phase 1 but the spec gap means it won't exist.
- **Recommendation:** Phase 1: spec the minimal Admin Console (license status, user list). Phase 2: expand to full spec.

## 3. Critical Gaps — Phase 2 ("Renew and Upsell")

### 3.1 Mac Mini HA Cluster — Partially Specified
- **Wiki status:** TERA-CORE §2.4 shows "2 Mac mini M4 Pro 48GB (cluster)" for Enterprise tier and mentions "Mesh-consensus HA (1 Mac = 99.9%, 2 Mac = 99.999%)." But the failover protocol, WAL replication mechanism, and health check specifics are not detailed.
- **Severity:** MEDIUM — concept exists but lacks implementation-level detail needed for Phase 2 build.

### 3.2 Survival Mesh — Detailed but AWDL Risk
- **Wiki status:** Extensive specification in TERA-CORE §4, §5.1, §12. But the iOS AWDL dependency risk (see §1.3 above) is unacknowledged.
- **Severity:** HIGH if AWDL-dependent; MEDIUM if BLE-only fallback is sufficient.

## 4. Cross-Reference Consistency

### 4.1 Unresolved Wikilinks (Legacy)
- `[[OpenAI GPT-4 Technical Report 2023]]` — from legacy `llm-overview.md`
- `[[Vaswani et al. 2017 Attention Is All You Need]]` — from legacy `transformer-architecture.md`
- **Action:** Either create stub pages or remove these links. They pollute the unresolved list.

### 4.2 Tag Hygiene
- **Fixed:** Hex colors `#0F172A`, `#1A1A2E`, `#24A1DE` were leaking as tags from CSS code in Glassmorphism page — wrapped in backticks.
- **Remaining issues:**
  - `#survival` (1) and `#mesh` (2) should consolidate to `#survival-mesh`
  - `#data-sovereignty` (1) and `#sovereignty` (1) should consolidate
  - `#e2ee` (2) should also have `#encryption` or consolidate
  - `#llm` (3) on legacy pages unrelated to TeraChat

## 5. Missing Concept Pages

These topics deserve concept pages based on the phase framework:

| Topic | Phase | Status |
|-------|-------|--------|
| Phase Framework (economic milestones) | All | **Missing** |
| TeraRelay Deployment | Phase 1 | **Missing** |
| JWT License Binding Protocol | Phase 1 | **Missing** |
| OIDC/SAML Federation Bridge | Phase 1 | Partially in TERA-GOV source |
| Admin Console | Phase 1/2 | Only UI patterns exist |
| Mac Mini HA Cluster | Phase 2 | **Missing** |
| ZK Memory Agent | Phase 3 | **Missing** (TD-005 unresolved) |
| .tapp Marketplace | Phase 3 | Partially in TERA-ECO |

## 6. Summary

| Category | Count |
|----------|-------|
| Factual issues | 5 |
| Critical Phase 1 gaps | 4 |
| Phase 2 gaps | 2 |
| Broken wikilinks (legacy) | 2 |
| Tag issues | 4 |
| Missing concept pages | 8 |

**Bottom line:** The wiki accurately documents the architectural vision but has zero representation of the deployment and operational reality needed to sign a pilot. The Phase 1 gaps (deployment automation, SAML mapping, JWT license spec, Admin Console) are all implementation-blocking — without them, an IT admin cannot deploy TeraChat in 30 minutes as required.
