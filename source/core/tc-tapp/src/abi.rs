//! # Host ABI Versioning
//!
//! Defines the `schema_version` header for all MessagePack payloads
//! exchanged between Host (Rust Core) and Guest (.tapp WASM module).
//!
//! ## Reference
//! - TD-001: Host ABI MessagePack Contract not versioned
//! - TERA-RUNTIME §11.4: Version negotiation handshake

use serde::{Deserialize, Serialize};

/// ABI schema version for Host ↔ Guest MessagePack payloads.
///
/// Every IPC MessagePack payload MUST carry this version in its header.
/// The host rejects payloads with incompatible versions (major mismatch).
///
/// # Version Semantics
/// - Major: Breaking change — reject if mismatched
/// - Minor: Backward-compatible addition — allow if major matches
/// - Patch: Bug fix — always compatible
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl SchemaVersion {
    /// Current ABI version.
    pub const CURRENT: Self = Self {
        major: 1,
        minor: 0,
        patch: 0,
    };

    /// Checks if this version is compatible with the `other` version.
    ///
    /// Compatibility rule: major versions must match.
    /// Minor/patch differences are tolerated (backward-compatible).
    pub fn is_compatible_with(&self, other: &Self) -> bool {
        self.major == other.major
    }
}

/// Header for all MessagePack IPC payloads.
///
/// This header is prepended to every Host ↔ Guest message.
/// The host reads this header first, rejects if version mismatch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbiHeader {
    /// Schema version of this payload
    pub schema_version: SchemaVersion,
    /// Trace ID for distributed tracing (OTEL)
    pub trace_id: [u8; 16],
    /// Workspace ID scope
    pub workspace_id: [u8; 16],
    /// Whether this is a first-party .tapp (skips version negotiation per TD-004)
    pub is_first_party: bool,
}

/// Version negotiation handshake result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NegotiationResult {
    /// Versions are compatible — proceed
    Compatible,
    /// Major version mismatch — reject .tapp
    IncompatibleMajor {
        host_major: u16,
        guest_major: u16,
    },
    /// Guest is newer minor — host should warn but allow
    GuestNewer {
        host_minor: u16,
        guest_minor: u16,
    },
}

impl SchemaVersion {
    /// Performs version negotiation between host and guest.
    pub fn negotiate(&self, guest: &Self) -> NegotiationResult {
        if self.major != guest.major {
            NegotiationResult::IncompatibleMajor {
                host_major: self.major,
                guest_major: guest.major,
            }
        } else if guest.minor > self.minor {
            NegotiationResult::GuestNewer {
                host_minor: self.minor,
                guest_minor: guest.minor,
            }
        } else {
            NegotiationResult::Compatible
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_compatibility() {
        let v1_0 = SchemaVersion { major: 1, minor: 0, patch: 0 };
        let v1_1 = SchemaVersion { major: 1, minor: 1, patch: 0 };
        let v2_0 = SchemaVersion { major: 2, minor: 0, patch: 0 };

        assert!(v1_0.is_compatible_with(&v1_1));
        assert!(!v1_0.is_compatible_with(&v2_0));
    }

    #[test]
    fn test_version_negotiation() {
        let host = SchemaVersion::CURRENT;

        // Same version
        assert_eq!(host.negotiate(&host), NegotiationResult::Compatible);

        // Guest has newer minor
        let guest_newer = SchemaVersion { major: 1, minor: 1, patch: 0 };
        match host.negotiate(&guest_newer) {
            NegotiationResult::GuestNewer { .. } => {}
            other => panic!("expected GuestNewer, got {:?}", other),
        }

        // Major mismatch
        let guest_v2 = SchemaVersion { major: 2, minor: 0, patch: 0 };
        match host.negotiate(&guest_v2) {
            NegotiationResult::IncompatibleMajor { .. } => {}
            other => panic!("expected IncompatibleMajor, got {:?}", other),
        }
    }
}
