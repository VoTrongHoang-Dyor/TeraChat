//! CRDT sync error types.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SyncError {
    #[error("dag merge conflict: {0}")]
    DagMergeConflict(String),

    #[error("tombstone vacuum failed")]
    TombstoneVacuumFailed,

    #[error("vector clock divergence detected")]
    VectorClockDivergence,

    #[error("saga recovery guard failed: orphaned committed saga")]
    SagaRecoveryFailed,

    #[error("wal integrity check failed")]
    WalIntegrityFailed,

    #[error("crypto error: {0}")]
    Crypto(#[from] tc_crypto::CryptoError),

    #[error("storage error: {0}")]
    Storage(String),
}
