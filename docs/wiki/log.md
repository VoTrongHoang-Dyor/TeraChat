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
