//! # tc-mesh — TeraChat Survival Mesh Network
//!
//! BLE 5.0 + Wi-Fi Direct mesh networking with EMDP emergency protocol.
//!
//! ## Dependency Direction
//! tc-mesh depends on tc-crypto ONLY (CORE → MESH).

pub mod error;
pub mod multiplexer;
pub mod peer;

pub use error::MeshError;
