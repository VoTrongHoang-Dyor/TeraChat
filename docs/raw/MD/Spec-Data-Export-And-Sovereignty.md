# Spec-Data-Export-And-Sovereignty

## 1. Problem Statement

As a "Sovereign Work OS," TeraChat guarantees users absolute ownership and control over their data, strictly prohibiting vendor lock-in. To fulfill this promise, TeraChat must provide a robust Data Export technical architecture.

However, since all data in TeraChat is End-to-End Encrypted (E2EE) and stored as Decentralized CRDT DAGs (Directed Acyclic Graphs), traditional server-side database dumps are impossible. The challenge is to orchestrate a secure, client-side (or Secure Enclave) decryption and transformation pipeline that produces human-readable and standard machine-readable formats without breaking the Zero-Knowledge constraint or violating OPA (Open Policy Agent) access rules.

## 2. Expected Behavior

- **Input:** An export request triggered by a user or Organization Admin, accompanied by a cryptographic proof of identity (DID signature).
- **Output:**
  - A structured ZIP archive containing standard formats (JSON, CSV, HTML).
  - A Cryptographic Manifest (`manifest.sig`) proving the authenticity and integrity of the exported data.
  - Retention of the hierarchical structure (Organization -> Mesh -> Channels -> Threads).
- **Edge Cases:**
  - Exporting media attachments (handling large blobs stored in IPFS/S3-compatible encrypted buckets).
  - Handling revoked access (users cannot export data from channels they were removed from before the export was initiated).
  - Device storage limitations during the local decryption process.

## 3. Non-Goals

- Server-side plaintext export (this violates the Zero-Knowledge architecture).
- Exporting data that the user's current DID cryptographic keys cannot mathematically decrypt (enforcing "encryption as authorization").

## 4. Constraints

- **Zero-Knowledge Requirement:** The TeraChat infrastructure servers never possess the plaintext data or the symmetric keys required to generate the export.
- **Memory Efficiency:** Exporting gigabytes of history requires a streaming decryption pipeline to prevent Out-Of-Memory (OOM) crashes on client devices.
- **Cryptographic Enforcement:** OPA policies must be evaluated at the edge. A user cannot export an entire organization's data unless they hold the `Organization Admin` cryptographic DataGrant.

## 5. Architectural Design & Implementation Strategy

### 5.1. Authorization via DataGrants and OPA

1. **Initiation:** The user invokes the `Export .tapp` (running in the WASM Sandbox).
2. **Policy Evaluation:** The local OPA engine evaluates the user's DID against the requested export scope (e.g., "Export All My Direct Messages" vs "Export Entire Mesh Network").
3. **Key Retrieval:** If authorized, the system retrieves the necessary symmetric `Message Keys` from the local Secure Hardware (HSM/Secure Enclave) using the user's Private Key.

### 5.2. Streaming Decryption Pipeline (`tc-store` + `tc-crypto`)

To handle large datasets without crashing the client:

1. **Cursor-based DAG Traversal:** The `tc-store` reads the local `hot_dag.db` CRDT nodes in chunks.
2. **Decryption Stream:** `tc-crypto` decrypts each node on-the-fly in a background Web Worker or Rust thread.
3. **Transformation:** The raw CRDT operations (insert, delete, update) are logically squashed to represent the final state of messages.
4. **Serialization:** The final state is serialized into TeraChat's Sovereign Portability Format (SPF) — a standardized JSON schema.

### 5.3. Media Handling and Verifiability

- **Blob Decryption:** Encrypted media attachments are streamed from local cache or downloaded dynamically, decrypted, and saved into an `attachments/` folder within the export ZIP.
- **Cryptographic Signature:** Once the archive is built, the client signs the SHA-256 hash of the archive using the user's Ed25519 Private Key. This provides non-repudiation and proves the data was exported by that specific DID.

## 6. Acceptance Criteria

- [ ] Export process runs entirely on the client or within a WASM Secure Enclave.
- [ ] Successfully decrypts and transforms CRDT DAG structures into standard, hierarchical JSON.
- [ ] Implements a streaming pipeline capable of exporting >10GB of data without exceeding 512MB of RAM overhead.
- [ ] Enforces OPA policies cryptographically—users can only export data they have the keys to decrypt.
- [ ] Exported archives include a valid Ed25519 cryptographic signature.
