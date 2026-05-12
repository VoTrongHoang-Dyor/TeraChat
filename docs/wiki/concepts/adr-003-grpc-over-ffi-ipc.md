---
type: concept
created: 2026-05-12
updated: 2026-05-12
tags: [adr, grpc, ffi, ipc, phase-0]
sources: [tera-client-spec, tera-core-spec]
---

# ADR-003: gRPC over FFI for IPC

## Status

**ACCEPTED** — 2026-05-12

## Context

TeraChat's Rust Core communicates with UI layers (Flutter/SwiftUI/Tauri) across an IPC boundary. Two approaches were evaluated:

| Approach | Description | Verdict |
|----------|-------------|---------|
| **Direct FFI** | `extern "C"` functions called via Dart FFI / Swift interop | ❌ REJECTED for control plane |
| **gRPC** | Protobuf-defined services over UDS/localhost | ⭐ ACCEPTED |

### Why FFI Was Rejected for Control Plane

1. **Fragmented contracts** — each platform (Dart, Swift, Kotlin) requires separate FFI bindings with different calling conventions
2. **Raw pointer risk** — `*const u8` across FFI boundary violates `tera_ffi_raw_pointer` lint (TD-006)
3. **No built-in versioning** — ABI changes break silently
4. **No streaming** — FFI is request/response; gRPC supports server-streaming for CoreSignals

### Where FFI Is Still Used

- **Data Plane (bulk)**: `SharedArrayBuffer` / mapped memory for zero-copy media transfer (Tier 1)
- **Key Material**: Hardware Secure Enclave access (platform-specific native API)
- **Performance-critical**: Sub-microsecond operations where gRPC overhead is unacceptable

## Decision

**gRPC (Protobuf over UDS) for the control plane.** FFI retained only for the data plane and hardware security.

### Architecture

```
UI (Flutter/SwiftUI/Tauri)
    │
    ├─ Control Plane: gRPC over UDS ────► Rust Core daemon
    │   (CoreSignals, UICommands, RPC)
    │
    └─ Data Plane: SharedArrayBuffer ───► Rust Core daemon
        (Bulk media, file chunks)
```

### Protocol Details

- All RPCs carry `TraceId` + `WorkspaceId` in metadata
- Error responses MUST NOT include key material paths
- `CoreSignal` pushed via server-streaming RPC
- `UICommand` sent as unary RPC

## Consequences

### Positive
- ✅ Single protobuf contract serves all platforms
- ✅ Built-in versioning and backward compatibility (protobuf)
- ✅ Server-streaming for real-time CoreSignals
- ✅ No raw pointers across the primary IPC boundary

### Negative
- ❌ ~50μs overhead per gRPC call vs ~1μs for direct FFI
- ❌ Protobuf serialization cost for small payloads
- ❌ Requires `protoc` in build toolchain

## Related

- [[Terachat Architecture Overview]] — IPC boundary layer
- [[ADR-001]] — daemon architecture (gRPC client is the UI)
- [[ADR-006]] — AI Gateway reuses the same gRPC channel
