//! # Global Key Arena — Centralized Key Material Storage
//!
//! All key material lives in a single `GlobalKeyArena` that can be
//! wiped atomically on panic or security event.
//!
//! ## Reference
//! - TD-006: FFI Panic Abort Bypass ZeroizeOnDrop
//! - Tech_Debt §5.5: GlobalKeyArena + Panic Hook

use std::sync::{Mutex, OnceLock};

use crate::error::CryptoError;
use crate::key_management::{KeyEntry, KeyHandle, KeyMaterial, KeyPurpose};

/// Maximum number of concurrent keys in the arena.
///
/// DECISION: 256 slots is sufficient for enterprise messaging —
/// typical session requires ~10 keys (session, signing, agreement,
/// derived chain keys). 256 allows for 25 concurrent MLS groups
/// with headroom for .tapp keys.
const MAX_ARENA_SLOTS: usize = 256;

/// Singleton arena instance.
static ARENA: OnceLock<Mutex<ArenaInner>> = OnceLock::new();

/// Internal arena state.
struct ArenaInner {
    slots: Vec<Option<KeyEntry>>,
    next_generation: u32,
}

impl ArenaInner {
    fn new() -> Self {
        let mut slots = Vec::with_capacity(MAX_ARENA_SLOTS);
        slots.resize_with(MAX_ARENA_SLOTS, || None);
        Self {
            slots,
            next_generation: 1,
        }
    }

    /// Wipe all slots — called on panic or emergency.
    fn wipe(&mut self) {
        for slot in self.slots.iter_mut() {
            // Drop triggers ZeroizeOnDrop on KeyMaterial
            *slot = None;
        }
        self.next_generation = 1;
    }
}

/// Centralized key material storage with atomic wipe capability.
///
/// The arena is a singleton — initialized once during Rust Core startup.
/// All key operations go through `KeyHandle` references that index into
/// this arena. On panic (caught by `ffi_boundary!`), the entire arena
/// is wiped before the process terminates.
pub struct GlobalKeyArena;

impl GlobalKeyArena {
    /// Initializes the global arena. Must be called once at startup.
    ///
    /// Subsequent calls are no-ops (OnceLock guarantees).
    pub fn init() {
        ARENA.get_or_init(|| Mutex::new(ArenaInner::new()));
    }

    /// Stores key material in the arena, returning an opaque handle.
    ///
    /// # Errors
    /// Returns `ArenaCapacityExhausted` if all slots are occupied.
    pub fn store(
        material: KeyMaterial,
        purpose: KeyPurpose,
        is_hardware_backed: bool,
    ) -> Result<KeyHandle, CryptoError> {
        let arena = ARENA
            .get()
            .ok_or(CryptoError::KeyNotFound)?;
        let mut inner = arena
            .lock()
            .map_err(|_| CryptoError::KeyGenerationFailed)?;

        let slot_idx = inner
            .slots
            .iter()
            .position(|s| s.is_none())
            .ok_or(CryptoError::ArenaCapacityExhausted)?;

        let generation = inner.next_generation;
        inner.next_generation = inner.next_generation.wrapping_add(1);

        inner.slots[slot_idx] = Some(KeyEntry {
            material,
            purpose,
            generation,
            is_hardware_backed,
        });

        Ok(KeyHandle::new(slot_idx as u32, generation))
    }

    /// Removes a key from the arena (triggers ZeroizeOnDrop).
    ///
    /// # Errors
    /// Returns `KeyNotFound` if the handle is stale or invalid.
    pub fn remove(handle: KeyHandle) -> Result<(), CryptoError> {
        let arena = ARENA
            .get()
            .ok_or(CryptoError::KeyNotFound)?;
        let mut inner = arena
            .lock()
            .map_err(|_| CryptoError::KeyGenerationFailed)?;

        let slot_idx = handle.slot() as usize;
        if slot_idx >= MAX_ARENA_SLOTS {
            return Err(CryptoError::KeyNotFound);
        }

        match &inner.slots[slot_idx] {
            Some(entry) if entry.generation == handle.generation() => {
                // Drop triggers ZeroizeOnDrop
                inner.slots[slot_idx] = None;
                Ok(())
            }
            _ => Err(CryptoError::KeyNotFound),
        }
    }

    /// Emergency wipe — zeros ALL key material in the arena.
    ///
    /// Called by:
    /// - `ffi_boundary!` macro on caught panic
    /// - Global panic hook installed by `install_panic_hook()`
    /// - Security event handler (e.g., tamper detection)
    ///
    /// This function MUST NOT panic. It uses a best-effort approach —
    /// if the mutex is poisoned, we still attempt to wipe via
    /// the poisoned guard.
    pub fn emergency_wipe() {
        if let Some(arena) = ARENA.get() {
            // WHY: Use lock() even on poisoned mutex — we need to wipe
            // regardless of what caused the poison.
            match arena.lock() {
                Ok(mut inner) => inner.wipe(),
                Err(poisoned) => poisoned.into_inner().wipe(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arena_store_and_remove() {
        GlobalKeyArena::init();

        let handle = GlobalKeyArena::store(
            KeyMaterial::Symmetric([0x42; 32]),
            KeyPurpose::SessionEncryption,
            false,
        )
        .expect("store should succeed");

        assert!(GlobalKeyArena::remove(handle).is_ok());
        // Second remove should fail — slot is empty
        assert!(GlobalKeyArena::remove(handle).is_err());
    }

    #[test]
    fn test_arena_emergency_wipe() {
        GlobalKeyArena::init();

        let _handle = GlobalKeyArena::store(
            KeyMaterial::Signing([0xFF; 32]),
            KeyPurpose::MessageSigning,
            false,
        )
        .expect("store should succeed");

        GlobalKeyArena::emergency_wipe();

        // All handles should now be invalid
        let stale = KeyHandle::new(0, 1);
        assert!(GlobalKeyArena::remove(stale).is_err());
    }

    #[test]
    fn test_arena_capacity() {
        GlobalKeyArena::init();
        // Wipe first to ensure clean state
        GlobalKeyArena::emergency_wipe();

        // Fill all slots
        let mut handles = Vec::new();
        for _ in 0..MAX_ARENA_SLOTS {
            let handle = GlobalKeyArena::store(
                KeyMaterial::Symmetric([0x00; 32]),
                KeyPurpose::SessionEncryption,
                false,
            );
            match handle {
                Ok(h) => handles.push(h),
                Err(_) => break,
            }
        }

        // Next store should fail
        let result = GlobalKeyArena::store(
            KeyMaterial::Symmetric([0x00; 32]),
            KeyPurpose::SessionEncryption,
            false,
        );
        assert!(result.is_err());

        // Cleanup
        GlobalKeyArena::emergency_wipe();
    }
}
