//! Storage error types.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("database open failed: {0}")]
    DatabaseOpenFailed(String),

    #[error("wal checkpoint failed")]
    WalCheckpointFailed,

    #[error("integrity check failed: {0}")]
    IntegrityCheckFailed(String),

    #[error("shadow db atomic rename failed")]
    ShadowDbRenameFailed,

    #[error("sqlcipher key derivation failed")]
    SqlCipherKeyFailed,

    #[error("query execution failed: {0}")]
    QueryFailed(String),

    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("crypto error: {0}")]
    Crypto(#[from] tc_crypto::CryptoError),
}
