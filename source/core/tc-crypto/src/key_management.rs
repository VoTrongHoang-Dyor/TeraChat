//! # Key Management — Opaque Key Handles
//!
//! All key material is managed through opaque `KeyHandle` references.
//! Private keys are NEVER exposed as public types.
//!
//! ## Invariants
//! - No `pub` on any crypto key type
//! - Keys stored in `GlobalKeyArena` with `ZeroizeOnDrop`
//! - Key material never serialized to disk without SQLCipher

use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Opaque handle to a key stored in the `GlobalKeyArena`.
///
/// This is the ONLY type consumers can use to reference keys.
/// The actual key material is never exposed across crate boundaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyHandle {
    /// Internal arena slot index — not meaningful outside tc-crypto
    slot: u32,
    /// Generation counter — prevents use-after-free of recycled slots
    generation: u32,
}

impl KeyHandle {
    /// Creates a new key handle pointing to an arena slot.
    pub(crate) fn new(slot: u32, generation: u32) -> Self {
        Self { slot, generation }
    }

    /// Returns the arena slot index (crate-internal only).
    pub(crate) fn slot(&self) -> u32 {
        self.slot
    }

    /// Returns the generation counter (crate-internal only).
    pub(crate) fn generation(&self) -> u32 {
        self.generation
    }
}

/// Internal key material — NEVER pub.
///
/// All variants carry `ZeroizeOnDrop` to guarantee cleanup.
#[derive(Zeroize, ZeroizeOnDrop)]
pub(crate) enum KeyMaterial {
    /// AES-256-GCM symmetric key (32 bytes)
    Symmetric([u8; 32]),
    /// Ed25519 signing key (32 bytes seed)
    Signing([u8; 32]),
    /// X25519 key agreement (32 bytes)
    KeyAgreement([u8; 32]),
    /// HKDF-derived key material (variable length, max 64 bytes)
    Derived(Vec<u8>),
}

/// Key purpose tag — determines which operations are allowed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum KeyPurpose {
    /// MLS session encryption
    SessionEncryption,
    /// Message signing (Ed25519)
    MessageSigning,
    /// Key agreement (X25519)
    KeyAgreement,
    /// Key derivation (HKDF root)
    Derivation,
    /// Audit trail signing
    AuditSigning,
}

/// Metadata associated with a key in the arena.
pub(crate) struct KeyEntry {
    pub(crate) material: KeyMaterial,
    pub(crate) purpose: KeyPurpose,
    pub(crate) generation: u32,
    /// Whether this key is hardware-backed (Secure Enclave / StrongBox / TPM)
    pub(crate) is_hardware_backed: bool,
}

impl Drop for KeyEntry {
    fn drop(&mut self) {
        // WHY: Explicit drop ensures ZeroizeOnDrop on material fires.
        // The generation counter is also cleared to prevent stale-handle attacks.
        self.generation = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_handle_opacity() {
        let handle = KeyHandle::new(42, 1);
        assert_eq!(handle.slot(), 42);
        assert_eq!(handle.generation(), 1);
    }

    #[test]
    fn test_key_material_zeroize_on_drop() {
        let key = KeyMaterial::Symmetric([0xAB; 32]);
        // Verify key exists
        match &key {
            KeyMaterial::Symmetric(bytes) => assert_eq!(bytes[0], 0xAB),
            _ => panic!("wrong variant"),
        }
        // Drop will zeroize — we can't inspect after drop, but this
        // confirms the ZeroizeOnDrop derive compiles correctly.
        drop(key);
    }
}
