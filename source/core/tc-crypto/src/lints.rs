//! # Custom Lint — `tera_ffi_raw_pointer`
//!
//! Deny `pub extern "C"` functions that take or return raw pointers.
//! All FFI functions must use the Token Protocol (opaque `u64` token)
//! or the `ffi_boundary!` macro.
//!
//! ## Reference
//! - TERA-CORE §12.1 — FFI boundary specification
//! - Tech_Debt §5.5 — Aegis FFI Boundary Protocol
//!
//! ## Design Decision
//! A full clippy lint requires `#![feature(rustc_private)]` and a separate
//! dylib crate. For Phase 0, we use a build-time static analysis scan
//! driven by a script executed in CI. The script scans all `pub extern "C"`
//! signatures and rejects any containing `*const` or `*mut`.
//!
//! Full clippy plugin implementation deferred to Phase 1 per the spec
//! (custom_proc_macro crate with rustc_private).

use std::fs;
use std::path::Path;

/// Scans a Rust source file for `pub extern "C"` functions with raw pointers.
///
/// Returns `true` if the file passes (no violations).
pub fn check_file(path: &Path) -> Result<bool, std::io::Error> {
    let content = fs::read_to_string(path)?;

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        // Detect: pub extern "C" fn with *const or *mut in signature
        if trimmed.contains("pub extern \"C\"") || trimmed.contains("pub extern \"C-unwind\"") {
            if trimmed.contains("*const") || trimmed.contains("*mut") {
                eprintln!(
                    "❌ VIOLATION: {}:{} — raw pointer in FFI boundary",
                    path.display(),
                    line_num + 1
                );
                eprintln!("   {}", trimmed);
                eprintln!("   Use `ffi_boundary!` macro or Token Protocol (opaque u64)");
                return Ok(false);
            }
        }

        // Detect: Option<*const> or Option<*mut> (still raw pointer)
        if (trimmed.contains("pub extern \"C\"") || trimmed.contains("pub extern \"C-unwind\""))
            && (trimmed.contains("Option<*const") || trimmed.contains("Option<*mut"))
        {
            eprintln!(
                "❌ VIOLATION: {}:{} — Option<*const/*mut> in FFI boundary",
                path.display(),
                line_num + 1
            );
            eprintln!("   {}", trimmed);
            return Ok(false);
        }
    }

    Ok(true)
}

/// Recursively scans all `.rs` files in a directory for FFI raw pointer violations.
///
/// Returns the count of files with violations (0 = pass).
pub fn scan_directory(dir: &Path, violations: &mut u32) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() && path.file_name().map_or(false, |n| n != "target") {
            scan_directory(&path, violations)?;
        } else if path.extension().map_or(false, |ext| ext == "rs") {
            match check_file(&path) {
                Ok(true) => {} // Pass
                Ok(false) => *violations += 1,
                Err(e) => eprintln!("Error reading {}: {}", path.display(), e),
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_check_file_clean() {
        let dir = std::env::temp_dir();
        let path = dir.join("test_clean.rs");
        let clean_code = r#"
pub use crate::ffi::ffi_boundary;

ffi_boundary!(my_func, {
    Ok(())
});
"#;
        fs::write(&path, clean_code).unwrap();
        assert!(check_file(&path).unwrap());
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_check_file_violation() {
        let dir = std::env::temp_dir();
        let path = dir.join("test_violation.rs");
        let violation_code = r#"
#[no_mangle]
pub extern "C" fn bad_func(ptr: *const u8, len: usize) -> i32 {
    0
}
"#;
        fs::write(&path, violation_code).unwrap();
        assert!(!check_file(&path).unwrap());
        fs::remove_file(&path).unwrap();
    }
}
