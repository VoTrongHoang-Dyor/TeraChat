---
type: concept
created: 2026-05-10
tags: [terachat, wasm, tapp, sandbox, runtime, plugin]
sources: [tera-runtime-spec, tera-eco-spec, tera-core-spec]
---

# WASM Tapp Runtime

The dual-engine WebAssembly sandbox for executing enterprise mini-applications (.tapp) within TeraChat. Provides a secure, deterministic runtime with hardware-adapted execution engines.

## Dual Engine Architecture

| Engine | Platform | Type | Performance |
|--------|----------|------|-------------|
| `wasmtime` | Android, Desktop, Huawei | JIT compiler | Fast (native speed) |
| `wasm3` | iOS | Interpreter | 10-100x slower, +15-20ms/call latency |

**Reason:** iOS enforces W^X (write-xor-execute) — memory cannot be both writable and executable. This blocks all JIT compilers. `wasm3` interprets WASM bytecode without generating native code.

## Security Boundaries

- **No direct network access:** `wasi-sockets` stripped. All I/O goes through Host ABI → Rust Core OPA policy check.
- **Crypto delegation:** WASM code cannot perform cryptographic operations. Must call Host ABI which delegates to Rust Core (hardware-backed keys).
- **Manifest-declared permissions:** All capabilities declared at install time, not runtime. No permission escalation possible.
- **Resource limits:** 10MB RAM max (background), 2MB Egress_Outbox hard limit, instruction_fuel metering.

## Host ABI

The set of functions Rust Core exposes to .tapp WASM modules:
- Storage (get/set/delete, scoped to .tapp)
- Crypto (encrypt, sign, verify — delegated)
- Network (proxied egress with OPA check)
- Event Bus (publish/subscribe to local events)

## 🧠 Design Decisions (Q&A)

- **Why two engines instead of just wasm3 everywhere?** → wasm3 is 10-100x slower. Finance .tapps doing BankFeeds reconciliation would timeout on 30s background tick on Desktop if limited to interpreter. Trade-off: must maintain WasmParity CI gate ensuring semantic equivalence across both engines.
- **Why strip wasi-sockets instead of filtering?** → Filtering implies a network stack in the sandbox, which is attack surface. Complete removal means even a sandbox escape can't reach the network. Trade-off: all network I/O must go through slower Host ABI path.
- **Why Gas/Fuel metering instead of timeout?** → Timeouts favor powerful hardware — a .tapp that passes on Desktop might exceed 30s on iOS. Instruction_fuel is deterministic: same fuel = same limit regardless of hardware. Trade-off: more complex ABI.
