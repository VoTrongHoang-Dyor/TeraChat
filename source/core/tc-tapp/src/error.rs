//! .tapp runtime error types.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TappError {
    #[error("wasm module instantiation failed: {0}")]
    InstantiationFailed(String),

    #[error("abi version mismatch: host={host_major}, guest={guest_major}")]
    AbiVersionMismatch { host_major: u16, guest_major: u16 },

    #[error("instruction fuel exhausted")]
    FuelExhausted,

    #[error("memory limit exceeded: {used_bytes} > {limit_bytes}")]
    MemoryLimitExceeded { used_bytes: usize, limit_bytes: usize },

    #[error("egress outbox limit exceeded: {used_bytes} > {limit_bytes}")]
    EgressOutboxExceeded { used_bytes: usize, limit_bytes: usize },

    #[error("host abi call denied by policy: {0}")]
    PolicyDenied(String),

    #[error("manifest capability not declared: {0}")]
    CapabilityNotDeclared(String),

    #[error("sandbox isolation violation")]
    SandboxViolation,

    #[error("crypto error: {0}")]
    Crypto(#[from] tc_crypto::CryptoError),

    #[error("storage error: {0}")]
    Store(#[from] tc_store::StoreError),
}
