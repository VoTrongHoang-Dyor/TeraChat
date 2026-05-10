# Spec-Enterprise-Secure-Enclave.md — TeraChat Enterprise Platform

```yaml
# DOCUMENT IDENTITY
id: "TERA-ENCLAVE"
title: "TeraChat — Enterprise Secure Enclave & AI Security Specification"
version: "1.0.0"
status: "ACTIVE"
audience: "Security Engineer, AI Architect, Enterprise Compliance Officer"
purpose: "Đặc tả kỹ thuật các cơ chế bảo mật cấp độ Enclave, bảo vệ dữ liệu PII và giới hạn quyền hạn của AI khi hoạt động nội bộ hệ thống."
depends_on: ["TERA-CORE", "TERA-GOV"]
```

## §1 — ARCHITECTURAL INVARIANTS & AUDIT RESOLUTIONS (SECURE ENCLAVE)

### 1.1 AI PII Redaction vs. BankFeeds / CRM Data

**Constraint:** AI Micro-NER models often fail to identify regional-specific Private Identifiable Information (PII) like localized banking formats, posing an injection vulnerability against the underlying AI analysis sessions.
**Resolution:** The `SanitizedPrompt` newtype strictly guarantees non-reversible PII redaction prior to AI payload submission. The embedded NER model’s entity dictionary is configured exclusively via tenant-specific `DomainPiiPolicy` schemas (e.g., specialized regex sets for Regional Account Syntax). Consequently, Zero-Knowledge compliance is structurally enforced across all LLM inference operations.

## §2 — LOCAL APPLIANCE MODEL & ZK MEMORY AGENT

### 2.1 Enterprise Edge Appliance Architecture
TeraChat shifts from cloud-hosted AI to the **Local Appliance Model**. The hardware stack in the customer's office defines the trust boundary:
- **Control Plane (Mac Node):** M4 Pro Mac mini handling TeraRelay routing, DAG synchronization, and hosting the ZK Memory Agent.
- **Compute Plane (RTX Node - Optional):** Dedicated inference servers for massive RAG tasks, connected securely via LAN.
- **Data Plane (NAS Node):** High-capacity TrueNAS storage housing encrypted Blobs and Vector Indices.

This physical isolation completely severs dependencies on third-party cloud AI vendors.

### 2.2 ZK Memory Agent IPC Contract
The `ZK Memory Agent` runs as a completely decoupled local daemon (`mlx-server`) on the Mac Node. It interfaces with TeraRelay strictly through a **Unix Domain Socket (UDS)**, eliminating LAN port-scan surface.

```rust
// Request Format over UDS (TeraRelay -> ZK Agent)
pub struct ZkMemoryQuery {
    pub session_id: SessionId,
    pub masked_context: Vec<u8>, // Already redacted by PII Layer
    pub query_type: ZkQueryType, // "Summary", "Search", "Suggest"
    pub max_tokens: u32,
}

// Response Format over UDS (ZK Agent -> TeraRelay)
pub struct ZkMemoryResponse {
    pub session_id: SessionId,
    pub response_tokens: Vec<u8>,
    pub metadata: ResponseMetadata,
}
```
pub enum ZkQueryType { Summary, Search, Suggest }
```

### 2.3 Consolidation Triggers and Resource Constraints
Maintaining a massive Vector Embeddings database (ZK Memory Index) is compute-intensive for Apple Silicon. Continuous background vectorization causes thermal saturation.
**Consolidation Trigger Rules:**
- The indexer runs in batch mode, triggered explicitly at **02:00 AM Local Time**, OR when the accumulated message delta un-indexed queue exceeds 80% NAS buffer capacity.
- This ensures thermal decay time and extends hardware lifespan.

### 2.4 MLX Graceful Degradation (Failure Model)
In the event that the `mlx-server` crashes, becomes overloaded, or OOM (Out of Memory):
- **Fallback Protocol:** TeraRelay catches UDS timeouts (`Err(NetworkTimeout)`). It avoids 500 crashes and emits an analytic error.
- Inference requests seamlessly degrade: Large document summaries fall back to basic text retrieval, or queries are offloaded to on-device NPUs (like iPhone native whisper/tiny models) as an emergency measure to preserve business continuity.
