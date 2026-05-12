//! # tc-crypto error types
//!
//! Unified error enum for all cryptographic operations.
//! Error messages MUST NOT include key material paths (security invariant).

use thiserror::Error;

/// Top-level error type for all cryptographic operations.
///
/// # Security
/// Error variants intentionally omit key paths, key IDs, and any material
/// that could aid an attacker in reconstructing the key hierarchy.
#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("key generation failed")]
    KeyGenerationFailed,

    #[error("encryption failed")]
    EncryptionFailed,

    #[error("decryption failed: ciphertext integrity check failed")]
    DecryptionFailed,

    #[error("signature verification failed")]
    SignatureVerificationFailed,

    #[error("key derivation failed")]
    KeyDerivationFailed,

    #[error("secure enclave unavailable")]
    EnclaveUnavailable,

    #[error("key not found in arena")]
    KeyNotFound,

    #[error("arena capacity exhausted")]
    ArenaCapacityExhausted,

    #[error("ffi boundary violation: {0}")]
    FfiBoundaryViolation(String),

    #[error("zeroization verification failed")]
    ZeroizationFailed,

    #[error("hardware root of trust not available on this platform")]
    HardwareRootUnavailable,
}

/// FFI-safe error code enum for cross-boundary error reporting.
///
/// These codes are returned via `extern "C"` functions wrapped by
/// the `ffi_boundary!` macro. They carry no sensitive information.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfiErrorCode {
    /// Operation succeeded
    Ok = 0,
    /// Generic internal error — caller should retry or escalate
    InternalError = -1,
    /// Key not found in the global arena
    KeyNotFound = -2,
    /// Invalid argument passed across FFI boundary
    InvalidArgument = -3,
    /// Panic caught at FFI boundary — arena wiped
    PanicCaught = -4,
    /// Buffer too small for output
    BufferTooSmall = -5,
    /// Hardware enclave not available
    EnclaveUnavailable = -6,
}

impl From<CryptoError> for FfiErrorCode {
    fn from(err: CryptoError) -> Self {
        match err {
            CryptoError::KeyNotFound => FfiErrorCode::KeyNotFound,
            CryptoError::EnclaveUnavailable => FfiErrorCode::EnclaveUnavailable,
            CryptoError::ArenaCapacityExhausted => FfiErrorCode::InternalError,
            _ => FfiErrorCode::InternalError,
        }
    }
}
