# Spec-Third-Party-Migration-And-Sync

## 1. Problem Statement

TeraChat targets enterprise customers transitioning from legacy, centralized communication platforms (Slack, Microsoft Teams, Google Chat) to a Sovereign Work OS. The critical challenge is achieving seamless data ingestion and identity synchronization while strictly adhering to TeraChat's Zero-Knowledge, E2EE (End-to-End Encryption), and CRDT-based offline-first architecture.

The hardest technical requirement is **preserving the exact hierarchical permission structure (Threads, Channels, Roles)** from central paradigms into TeraChat's OPA (Open Policy Agent) Policy Engine and cryptographic access controls without exposing plaintext data to intermediate transit servers.

## 2. Expected Behavior

- **Input:** OAuth 2.0 / Admin API keys or Data Export Archives from Slack/Teams/Google Workspace.
- **Output:**
  - Complete migration of Workspaces, Channels, Threads, and Direct Messages.
  - Transformation of centralized Roles (Admin, Member, Guest) into DID (Decentralized Identifier) profiles and OPA policies.
  - All historical messages re-encrypted on the client/migration enclave before entering TeraChat's `tc-store` (SQLite CRDT).
- **Edge Cases:**
  - Unmapped users (users in legacy systems who haven't created TeraChat DIDs yet).
  - Orphaned threads (threads where the parent message was deleted).
  - Circular or deeply nested permissions (e.g., Slack private channels shared externally).

## 3. Non-Goals

- Real-time continuous bi-directional sync (this is a one-way migration to enforce data sovereignty, though a transitional bridge might be supported temporarily).
- Migrating third-party app integrations (TeraChat uses `.tapp` WASM runtime, which requires separate ecosystem bridging).

## 4. Constraints

- **Zero-Knowledge Requirement:** The server performing the migration must operate within a Secure Enclave (HSM) or entirely client-side, and drop all memory buffers immediately after transforming data (ZeroizeOnDrop).
- **CRDT Compatibility:** Legacy sequential data must be mapped into TeraChat's DAG (Directed Acyclic Graph) CRDT structure.
- **Data Volume:** Must handle millions of messages without locking the main execution thread.

## 5. Architectural Design & Implementation Strategy

### 5.1. Identity & Role Translation (The Hardest Problem)

Centralized platforms use database joins to check if `User A` is in `Channel B`. TeraChat uses cryptographically verifiable structured DataGrants and OPA Policies.

1. **Shadow DID Mapping:** Generate placeholder "Shadow DIDs" for all imported users. When the user logs in for the first time via Enterprise SSO/SAML, they claim their Shadow DID and generate their device's Kyber768/Ed25519 keypair.
2. **Role to OPA Policy Conversion:**
   - *Workspace Admin* -> TeraChat `Organization Admin` (Global OPA Policy).
   - *Private Channel Member* -> Encrypted channel keys are wrapped for the specific DID.
   - *Guest/External* -> Scoped DataGrant restricting access to specific CRDT branches.

### 5.2. Hierarchy Transformation (Channel & Thread Mapping)

1. **Workspace/Teams:** Maps to a TeraChat **Mesh Network / Organization ID**.
2. **Channels:** Maps to a TeraChat **CRDT Topic / Room**. A unique symmetric encryption key (Message Key) is generated for each channel and distributed via PQ-KEM to authorized DIDs.
3. **Threads:** Maps to a **DAG Sub-branch** tied to the parent message ID. In CRDT, threads are simply child nodes with a `parent_hash` reference, maintaining structural integrity across offline syncs.

### 5.3. Migration Workflow (Secure Enclave Bridge)

1. **Extraction:** An authorized `Migration .tapp` (running in the WASM Sandbox) securely calls the Slack/Teams export API or parses a provided ZIP archive.
2. **Transformation & Encryption:**
   - The `.tapp` parses JSON exports.
   - For each channel, it generates TeraChat cryptographic structures.
   - Messages are encrypted using the channel's newly generated symmetric key.
3. **Ingestion:** The encrypted payloads are injected into the local `hot_dag.db` as CRDT operations.
4. **Synchronization:** The client syncs these encrypted blobs to the TeraChat network. The infrastructure only sees encrypted DAG nodes, maintaining Zero-Knowledge.

## 6. Acceptance Criteria

- [ ] Connects to Slack/Teams/Google Chat APIs using secure OAuth/Tokens or processes raw archive files.
- [ ] Correctly translates legacy Roles into TeraChat OPA Policies.
- [ ] Maps Threads and Channels accurately to the CRDT DAG structure without data loss.
- [ ] Migration process operates entirely client-side or within a Secure Enclave, ensuring Zero-Knowledge.
- [ ] Shadow DIDs are successfully claimed by users upon first SSO login, seamlessly granting access to historical encrypted data.
