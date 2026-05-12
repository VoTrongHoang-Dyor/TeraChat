//! # tc-crypto — TeraChat Cryptographic Core
//!
//! Foundational cryptography crate. All key material management, encryption,
//! signing, and hardware security abstraction lives here.
//!
//! ## Invariants
//! - **No `pub` on any crypto key type** — keys are opaque handles
//! - **`ZeroizeOnDrop` on ALL key material** — SEC-01 enforced
//! - **ring-only** — no other crypto crate permitted
//! - **No raw pointer in `pub extern "C"`** — `ffi_boundary!` macro required

pub mod error;
pub mod ffi;
pub mod key_management;
pub mod zeroize_guard;

// WHY: Single public API module — all consumers go through this facade.
// Internal modules gated with pub(crate) visibility.
pub use error::CryptoError;
pub use ffi::ffi_boundary;
pub use key_management::KeyHandle;
pub use zeroize_guard::GlobalKeyArena;
