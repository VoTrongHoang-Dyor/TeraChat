//! Mesh network error types.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MeshError {
    #[error("peer discovery failed")]
    PeerDiscoveryFailed,

    #[error("ble transport unavailable")]
    BleUnavailable,

    #[error("wifi direct transport unavailable")]
    WifiDirectUnavailable,

    #[error("emdp session terminated")]
    EmdpSessionTerminated,

    #[error("mesh multiplexer: priority queue overflow")]
    MultiplexerOverflow,

    #[error("mesh multiplexer: backpressure engaged (RTT > 200ms)")]
    BackpressureEngaged,

    #[error("crypto error: {0}")]
    Crypto(#[from] tc_crypto::CryptoError),
}
