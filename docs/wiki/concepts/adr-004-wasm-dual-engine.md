---
type: concept
created: 2026-05-12
updated: 2026-05-12
tags: [adr, wasm, dual-engine, wasmtime, wasm3, ios, phase-0]
sources: [tera-runtime-spec, tera-core-spec, tech-debt]
---

# ADR-004: WASM Dual-Engine (wasmtime + wasm3)

## Status

**ACCEPTED** — 2026-05-12

## Context

TeraChat runs `.tapp` mini-applications in a WASM sandbox. iOS enforces W^X (write-xor-execute) — memory pages cannot be simultaneously writable and executable. This fundamentally blocks all JIT compilers, including `wasmtime`'s Cranelift backend.

| Engine | Type | Performance | iOS Compatible |
|--------|------|-------------|---------------|
| `wasmtime` | JIT compiler (Cranelift) | Near-native speed | ❌ W^X blocked |
| `wasm3` | Pure interpreter | 10-100x slower, +15-20ms/call | ✅ No JIT needed |
| `wasmer` (Singlepass) | Single-pass compiler | Medium | ❌ Also needs W^X |

## Decision

**Dual-engine: `wasmtime` for Desktop/Android, `wasm3` for iOS.**

### Metering Strategy

**Gas/Fuel metering instead of wall-clock timeout** (TD-003 resolution):

- Each `.tapp` invocation receives a fixed `instruction_fuel` budget
- When fuel is exhausted, the .tapp is forcibly suspended — deterministic regardless of hardware
- This prevents the scenario where a .tapp passes on a fast Desktop but exceeds the 30s background tick on iOS

```
wasmtime: set_fuel(10_000_000) → executes 10M instructions, deterministic
wasm3:    set_fuel(10_000_000) → executes same 10M instructions, slower but identical budget
```

### Parity Guarantee

WasmParity CI gate ensures semantic equivalence:
- Same WASM module, same inputs → same outputs on both engines
- `delta ≤ 20ms` tolerance for timing differences
- `mem ≤ 5MB` ceiling for resource usage

## Consequences

### Positive
- ✅ iOS App Store compatible — no JIT, no W^X violation
- ✅ Deterministic resource limits via fuel metering
- ✅ Same .tapp binary runs on all platforms
- ✅ WasmParity CI catches semantic drift

### Negative
- ❌ iOS .tapps are 10-100x slower than Desktop
- ❌ Two engine codepaths to maintain and test
- ❌ Fuel metering adds ABI complexity
- ❌ Finance .tapps doing reconciliation may hit fuel limits on iOS

## Related

- [[WASM Tapp Runtime]] — full runtime architecture
- TD-003 — WASM Dual-Engine fragmentation
- XPLAT-01 — iOS W^X constraint
- Tech_Debt §5.3 — Gas/Fuel Metering
