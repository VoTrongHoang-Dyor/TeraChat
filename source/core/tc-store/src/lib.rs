//! # tc-store — TeraChat Storage Layer
//!
//! SQLite WAL + SQLCipher encryption for local data persistence.
//! Manages hot_dag.db (append-only CRDT) and cold_state.db (encrypted relational).

pub mod error;

pub use error::StoreError;
