//! # tc-tapp — TeraChat .tapp WASM Runtime
//!
//! Dual-engine WASM sandbox (wasmtime for Desktop/Android, wasm3 for iOS).
//! Host ABI, event bus, and resource metering.

pub mod abi;
pub mod error;

pub use abi::SchemaVersion;
pub use error::TappError;
