//! # tc-proto — TeraChat Protobuf Definitions
//!
//! Generated Rust stubs for gRPC services, signals, commands, and errors.
//! This crate is auto-generated from `.proto` files via `tonic-build`.

/// Core service definitions (7 TERA domains)
pub mod terachat {
    tonic::include_proto!("terachat");
}

/// Error codes covering all 7 domains
pub mod errors {
    tonic::include_proto!("terachat.errors");
}

/// CoreSignal — IPC signals from Rust Core → UI
pub mod signals {
    tonic::include_proto!("terachat.signals");
}

/// UICommand — pull-based commands from UI → Rust Core
pub mod commands {
    tonic::include_proto!("terachat.commands");
}
