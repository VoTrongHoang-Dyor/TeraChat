//! # CRDT DAG Node
//!
//! Core data structure for the append-only message DAG.
//! Each node references parent(s), forming a directed acyclic graph.
//!
//! ## Invariants
//! - hot_dag.db is APPEND-ONLY — no UPDATE or DELETE
//! - UUID v7 time-ordered primary keys
//! - BLAKE3 hash for content addressing

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A node in the CRDT DAG.
///
/// Each message, edit, or tombstone is a node. Nodes reference
/// their parent(s) to form a DAG that can be merged without
/// central coordination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagNode {
    /// UUID v7 — time-ordered, globally unique
    pub id: Uuid,
    /// Parent node IDs — empty for root nodes
    pub parents: Vec<Uuid>,
    /// BLAKE3 hash of the node content (used for deduplication)
    pub content_hash: [u8; 32],
    /// Workspace ID this node belongs to
    pub workspace_id: Uuid,
    /// Node type discriminator
    pub node_type: DagNodeType,
    /// Encrypted payload — plaintext never stored
    pub encrypted_payload: Vec<u8>,
    /// Timestamp (monotonic, not wall-clock — see TD-010)
    pub monotonic_tick: u64,
}

/// Type discriminator for DAG nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DagNodeType {
    /// Chat message
    Message,
    /// Message edit (references original via parent)
    Edit,
    /// Tombstone (soft delete — original content retained for audit)
    Tombstone,
    /// Presence update
    Presence,
    /// System event (MLS epoch rotation, key update)
    SystemEvent,
}

impl DagNode {
    /// Creates a new DAG node with UUID v7 and BLAKE3 content hash.
    pub fn new(
        workspace_id: Uuid,
        parents: Vec<Uuid>,
        node_type: DagNodeType,
        encrypted_payload: Vec<u8>,
        monotonic_tick: u64,
    ) -> Self {
        let content_hash = blake3::hash(&encrypted_payload);
        Self {
            id: Uuid::now_v7(),
            parents,
            content_hash: *content_hash.as_bytes(),
            workspace_id,
            node_type,
            encrypted_payload,
            monotonic_tick,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dag_node_creation() {
        let workspace = Uuid::now_v7();
        let payload = b"encrypted content".to_vec();
        let node = DagNode::new(workspace, vec![], DagNodeType::Message, payload.clone(), 1);

        assert!(!node.id.is_nil());
        assert_eq!(node.parents.len(), 0);
        assert_eq!(node.workspace_id, workspace);
        assert_eq!(node.node_type, DagNodeType::Message);
        // Verify BLAKE3 hash matches
        let expected_hash = blake3::hash(&payload);
        assert_eq!(&node.content_hash, expected_hash.as_bytes());
    }

    #[test]
    fn test_dag_node_with_parents() {
        let workspace = Uuid::now_v7();
        let parent = Uuid::now_v7();
        let node = DagNode::new(
            workspace,
            vec![parent],
            DagNodeType::Edit,
            b"edited".to_vec(),
            2,
        );

        assert_eq!(node.parents, vec![parent]);
        assert_eq!(node.node_type, DagNodeType::Edit);
    }
}
