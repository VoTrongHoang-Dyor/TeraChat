//! # Mesh Priority Multiplexer
//!
//! Implements P0/P1/P2 priority-tagged packet scheduling with
//! dynamic backpressure per TD-008 and Tech_Debt §5.7.
//!
//! ## Priority Levels
//! - P0 (Critical): `EmdpSessionTerminated`, `KillDirective` — NEVER suspended
//! - P1 (Standard): Chat messages, state sync
//! - P2 (Bulk): File chunks, media transfers — suspended when RTT > 200ms

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Packet priority level for mesh multiplexing.
///
/// P0 packets are NEVER queued behind P2 packets.
/// This prevents control plane starvation per TD-008.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(u8)]
pub enum MeshPriority {
    /// Critical control signals — never suspended, never queued behind bulk
    Critical = 0,
    /// Standard messaging — normal priority
    Standard = 1,
    /// Bulk data transfer — suspended under backpressure
    Bulk = 2,
}

/// Backpressure state for the mesh multiplexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackpressureState {
    /// Normal operation — all priorities transmit
    Normal,
    /// P2 suspended — RTT exceeded 200ms threshold
    /// P2 resumes when RTT < 100ms sustained for 5s
    BulkSuspended {
        /// When backpressure was engaged
        since_ms: u64,
    },
}

/// Configuration for the mesh multiplexer.
pub struct MultiplexerConfig {
    /// RTT threshold to engage backpressure (default: 200ms per TD-008)
    pub backpressure_engage_rtt: Duration,
    /// RTT threshold to disengage backpressure (default: 100ms)
    pub backpressure_release_rtt: Duration,
    /// Sustained low-RTT duration before releasing backpressure (default: 5s)
    pub backpressure_release_sustain: Duration,
}

impl Default for MultiplexerConfig {
    fn default() -> Self {
        Self {
            backpressure_engage_rtt: Duration::from_millis(200),
            backpressure_release_rtt: Duration::from_millis(100),
            backpressure_release_sustain: Duration::from_secs(5),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_ordering() {
        assert!(MeshPriority::Critical < MeshPriority::Standard);
        assert!(MeshPriority::Standard < MeshPriority::Bulk);
    }

    #[test]
    fn test_default_config() {
        let config = MultiplexerConfig::default();
        assert_eq!(config.backpressure_engage_rtt, Duration::from_millis(200));
        assert_eq!(config.backpressure_release_rtt, Duration::from_millis(100));
    }
}
