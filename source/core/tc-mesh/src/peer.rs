//! # Mesh Peer Management
//!
//! Peer discovery, election, and lifecycle for survival mesh.

use serde::{Deserialize, Serialize};

/// Peer role in the mesh network.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MeshRole {
    /// Mesh coordinator — manages routing table
    Coordinator,
    /// Regular peer — participates in store-and-forward
    Peer,
    /// Border node — bridges mesh to internet when available
    BorderNode,
}

/// Peer identity in the mesh network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshPeer {
    /// Device identity key fingerprint (derived from Secure Enclave key)
    pub device_fingerprint: [u8; 32],
    /// Current mesh role
    pub role: MeshRole,
    /// Election weight — iOS always 0 per spec invariant
    pub election_weight: u8,
    /// Whether this peer is reachable via BLE
    pub is_ble_reachable: bool,
    /// Whether this peer is reachable via Wi-Fi Direct
    pub is_wifi_direct_reachable: bool,
}

impl MeshPeer {
    /// iOS devices MUST have election_weight = 0.
    ///
    /// WHY: iOS cannot act as Mesh Dictator because AWDL is disabled
    /// when Personal Hotspot is active, making coordinator role unreliable.
    /// See README invariant #7 and XPLAT-02.
    pub fn new_ios_peer(device_fingerprint: [u8; 32]) -> Self {
        Self {
            device_fingerprint,
            role: MeshRole::Peer,
            election_weight: 0, // INVARIANT: iOS never dictator
            is_ble_reachable: true,
            is_wifi_direct_reachable: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ios_peer_election_weight_zero() {
        let peer = MeshPeer::new_ios_peer([0xAA; 32]);
        assert_eq!(peer.election_weight, 0, "iOS election_weight MUST be 0");
        assert_eq!(peer.role, MeshRole::Peer);
    }
}
