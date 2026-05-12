//! # tc-crdt-sync — TeraChat CRDT DAG Sync
//!
//! Offline-first dual-plane synchronization:
//! - Message Sync: CRDT DAG (append-only hot_dag.db)
//! - App State Sync: Vector-Clock Relational (cold_state.db)

pub mod error;
pub mod dag;

pub use error::SyncError;
