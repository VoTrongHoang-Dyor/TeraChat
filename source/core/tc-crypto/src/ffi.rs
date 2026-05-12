//! # FFI Boundary Protocol
//!
//! The `ffi_boundary!` macro wraps all `extern "C"` functions with:
//! 1. `catch_unwind` — panics never cross the FFI boundary
//! 2. Arena wipe on panic — `GlobalKeyArena` zeroed before abort
//! 3. Error code return — safe `FfiErrorCode` instead of raw pointers
//!
//! ## Reference
//! - TERA-CORE §12.1 — FFI boundary specification
//! - TD-006 — FFI Panic Abort Bypass ZeroizeOnDrop
//! - Tech_Debt §5.5 — Aegis FFI Boundary Protocol

use crate::error::FfiErrorCode;
use crate::zeroize_guard::GlobalKeyArena;

/// Wraps an `extern "C"` function body with `catch_unwind` and arena wipe.
///
/// # Usage
///
/// ```rust,ignore
/// ffi_boundary!(my_ffi_function, {
///     // safe Rust code here
///     // panics are caught, arena is wiped on panic
///     Ok(())
/// });
/// ```
///
/// # Safety Contract
/// - The macro guarantees that no panic propagates across the FFI boundary
/// - On panic, `GlobalKeyArena::emergency_wipe()` is called before returning
/// - Return type is always `FfiErrorCode` (i32-compatible)
///
/// # Design Decision
/// WHY: `panic = "abort"` at FFI boundary means `Drop()` never runs on panic.
/// This macro intercepts panics BEFORE abort, giving us a chance to wipe
/// the key arena. Trade-off: ~2μs overhead per FFI call from catch_unwind.
#[macro_export]
macro_rules! ffi_boundary {
    ($name:ident, $body:block) => {
        #[no_mangle]
        pub extern "C" fn $name() -> i32 {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<(), $crate::error::CryptoError> {
                $body
            }));

            match result {
                Ok(Ok(())) => $crate::error::FfiErrorCode::Ok as i32,
                Ok(Err(err)) => {
                    tracing::error!(error = %err, "FFI boundary: operation failed");
                    let code: $crate::error::FfiErrorCode = err.into();
                    code as i32
                }
                Err(_panic_payload) => {
                    // DECISION: Wipe arena before returning error code.
                    // This is the critical path that TD-006 requires —
                    // key material MUST be zeroed even on panic.
                    tracing::error!("FFI boundary: panic caught — wiping key arena");
                    $crate::zeroize_guard::GlobalKeyArena::emergency_wipe();
                    $crate::error::FfiErrorCode::PanicCaught as i32
                }
            }
        }
    };

    ($name:ident, ($($param:ident : $ptype:ty),*), $body:block) => {
        #[no_mangle]
        pub extern "C" fn $name($($param: $ptype),*) -> i32 {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<(), $crate::error::CryptoError> {
                $body
            }));

            match result {
                Ok(Ok(())) => $crate::error::FfiErrorCode::Ok as i32,
                Ok(Err(err)) => {
                    tracing::error!(error = %err, "FFI boundary: operation failed");
                    let code: $crate::error::FfiErrorCode = err.into();
                    code as i32
                }
                Err(_panic_payload) => {
                    tracing::error!("FFI boundary: panic caught — wiping key arena");
                    $crate::zeroize_guard::GlobalKeyArena::emergency_wipe();
                    $crate::error::FfiErrorCode::PanicCaught as i32
                }
            }
        }
    };
}

/// Installs the global panic hook that wipes the key arena.
///
/// MUST be called once during Rust Core startup, before any FFI functions
/// are exposed. This is the safety net — even if `ffi_boundary!` is
/// somehow bypassed, the hook ensures arena wipe.
///
/// # Reference
/// Tech_Debt §5.5 — `std::panic::set_hook` installed at startup
pub fn install_panic_hook() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        tracing::error!("PANIC HOOK: wiping GlobalKeyArena before abort");
        GlobalKeyArena::emergency_wipe();
        default_hook(info);
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panic_hook_installation() {
        // Verify hook installs without panic
        install_panic_hook();
    }

    #[test]
    fn test_ffi_error_code_repr() {
        assert_eq!(FfiErrorCode::Ok as i32, 0);
        assert_eq!(FfiErrorCode::InternalError as i32, -1);
        assert_eq!(FfiErrorCode::PanicCaught as i32, -4);
    }
}
