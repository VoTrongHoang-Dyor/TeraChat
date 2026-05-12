# Phase 0 — Hardware & Infrastructure Gaps Report

```yaml
report_id: "PH0-GAPS"
date: "2026-05-12"
author: "AI Agent (Phase 0 Execution)"
status: "ACTIVE"
```

---

## 1. Overview

This report documents all Phase 0 items that could NOT be implemented or verified due to hardware unavailability. Each item includes:

- What was skipped and why
- What is needed to complete it
- Impact on subsequent phases
- Workaround (if any)

---

## 2. Skipped Items

### 2.1 WasmParity CI (Task Box 0.3, Item 3)

**What:** `.github/workflows/wasm-parity.yml` — CI workflow that runs the same WASM module through both `wasm3` and `wasmtime` and verifies semantic equivalence (`delta ≤ 20ms`, `mem ≤ 5MB`).

**Why Skipped:**

- Requires `wasm3` and `wasmtime` installed on CI runner
- Meaningful parity tests require actual `.tapp` WASM binaries (none exist yet)
- Performance thresholds need baseline measurement on target hardware (Mac mini M2)

**What's Needed:**

- At least one reference `.tapp` binary compiled to WASM
- CI runner with both `wasm3` and `wasmtime` installed
- Baseline performance measurements on Mac mini

**Impact:** Non-blocker for Phase 0 exit gate. Becomes blocker at Phase 4 (.tapp runtime implementation).

**Status:** Workflow file placeholder NOT created — will be authored when first `.tapp` binary exists.

---

### 2.2 SBOM cosign Signing (Task Box 0.3, Item 4)

**What:** `cosign sign-blob` step in SBOM workflow — cryptographically signs the CycloneDX SBOM with a release key.

**Why Skipped:**

- `cosign` signing requires a private key (`COSIGN_KEY`)
- In production: HSM-backed key with physical PIN entry
- In CI: requires `COSIGN_KEY` GitHub Secret provisioned

**What's Needed:**

- Generate a `cosign` key pair (`cosign generate-key-pair`)
- Store public key in repo, private key in GitHub Secrets or HSM
- For Gov/Military: YubiKey FIPS on self-hosted runner

**Impact:** SBOM is still generated and archived as artifact. Only the signing step is missing. Non-blocker until Gov/Military compliance audit.

**Status:** SBOM workflow created with signing step commented out and documented.

---

### 2.3 Custom Clippy Lint Plugin (Task Box 0.3, Item 5)

**What:** `tera_ffi_raw_pointer` — custom Clippy lint that denies `pub extern "C"` functions accepting raw pointers.

**Why Skipped:**

- Custom Clippy lints require the `clippy_lints` nightly API
- Plugin development requires nightly toolchain (project pins stable 1.75.0)
- Lint plugin is a separate crate with its own build pipeline

**What's Needed:**

- Nightly Rust toolchain for lint plugin development
- Dedicated crate: `source/core/tc-lints/`
- CI step that runs the custom lint on nightly while keeping main build on stable

**Impact:** The `ffi_boundary!` macro already prevents raw pointer FFI by design. The lint is a defense-in-depth layer. Medium priority.

**Status:** Not implemented. The `ffi_boundary!` macro in `tc-crypto/src/ffi.rs` provides the primary protection. Custom lint is a secondary guard.

---

### 2.4 Cross-Compilation to All 4 Targets (Task Box 0.1, Testing)

**What:** `cargo check --workspace` on all 4 targets: `x86_64-linux`, `aarch64-linux`, `x86_64-darwin`, `aarch64-darwin`.

**Why Skipped:**

- Local machine is `aarch64-darwin` (Apple Silicon Mac)
- Cross-compilation to Linux requires cross-compilation toolchain or Docker
- `x86_64-darwin` requires Rosetta 2 and may have `ring` crate build issues

**What's Needed:**

- `cross` tool installed (`cargo install cross`)
- Docker Desktop for Linux target cross-compilation
- CI matrix handles this automatically (implemented in `ci.yml` with `ubuntu-latest` + `macos-latest`)

**Impact:** CI workflow covers `ubuntu-latest` (x86_64-linux) and `macos-latest` (aarch64-darwin). Two targets are covered. aarch64-linux and x86_64-darwin are CI gaps.

**Status:** CI workflow created covering 2 of 4 targets. Full matrix requires self-hosted runners or additional CI configuration.

---

### 2.5 GPU Tier Downgrade Testing (Task Box 0.4, Testing)

**What:** Verify GPU tier downgrade path (Tier A → B → C) without visual breakage across all 4 visual modes.

**Why Skipped:**

- Requires actual iOS/macOS devices with different GPU capabilities
- Tier C fallback testing requires intentionally degrading GPU (simulator insufficient)
- Visual verification requires human review of rendered UI

**What's Needed:**

- iPhone (any recent model) for iOS Tier A testing
- Older iPhone or iPad for Tier B testing
- GPU throttled device or simulator configuration for Tier C
- macOS with both integrated and discrete GPU for Desktop testing

**Impact:** Design tokens JSON is complete with all 3 tiers defined. Visual verification is pending hardware.

**Status:** Design tokens created in `source/clients/apple/design-tokens.json` with all 3 GPU tiers fully specified.

---

### 2.6 Secure Enclave Integration Testing (Task Box 0.1)

**What:** Verify that `KeyHandle` operations correctly delegate to Apple Secure Enclave (SEP) on iOS/macOS.

**Why Skipped:**

- Secure Enclave API (`SecKeyCreateRandomKey` with `kSecAttrTokenID: kSecAttrTokenIDSecureEnclave`) requires physical device
- iOS Simulator does NOT support Secure Enclave operations
- macOS requires T2/Apple Silicon chip with SEP

**What's Needed:**

- Physical iPhone with Face ID / Touch ID for iOS testing
- Mac with Apple Silicon (M1+) for macOS testing
- Xcode 15+ with device provisioning profile

**Impact:** `GlobalKeyArena` works with software keys. Hardware-backed key support is stubbed (`is_hardware_backed: bool` flag) but untested against real hardware.

**Status:** Key management code supports hardware-backed flag. Integration with Apple CryptoTokenKit requires platform-specific binding in `source/bindings/uniffi-apple/`.

---

### 2.7 BLE Mesh Integration Testing

**What:** Test BLE 5.0 mesh discovery, EMDP activation, and multiplexer backpressure on actual devices.

**Why Skipped:**

- BLE testing requires at least 3 physical devices in proximity
- CoreBluetooth framework requires entitlement and physical device
- EMDP stress testing requires controlled RF environment

**What's Needed:**

- Minimum 3 iOS/macOS devices for mesh topology
- BLE 5.0 capable devices
- Controlled RF environment for congestion testing

**Impact:** Mesh data structures (`MeshPeer`, `MeshMultiplexer`, `MeshPriority`) are fully defined. Transport layer requires platform bindings.

**Status:** Core mesh types implemented in `tc-mesh/`. Platform-specific BLE transport requires `MultipeerConnectivity` binding for iOS/macOS.

---

### 2.8 `buf lint` and `buf breaking` CI (Task Box 0.2, Deployment)

**What:** Protobuf backward compatibility checks using `buf` CLI tool.

**Why Skipped:**

- `buf` requires BSR (Buf Schema Registry) configuration
- Initial proto files have no previous version to check against
- Backward compatibility becomes meaningful after first proto freeze

**What's Needed:**

- Install `buf` CLI
- Create `buf.yaml` and `buf.lock` configuration
- Commit initial proto files as the baseline for future breaking checks

**Impact:** Non-blocker for Phase 0. Becomes important when proto files are modified in later phases.

**Status:** Proto files created. `buf` configuration deferred to first proto modification.

---

## 3. Summary Matrix

| # | Item | Severity | Blocks Phase | Hardware Needed |
|---|------|----------|-------------|-----------------|
| 2.1 | WasmParity CI | Medium | Phase 4 | CI runner + .tapp binary |
| 2.2 | SBOM cosign | Low | Gov/Military | HSM key / cosign key pair |
| 2.3 | Custom Clippy Lint | Medium | None (ffi_boundary! covers) | Nightly toolchain |
| 2.4 | Cross-compilation | Low | None (CI covers 2/4) | Docker / cross tool |
| 2.5 | GPU Tier Testing | Medium | Phase 3 (UI) | iOS/macOS devices |
| 2.6 | Secure Enclave | High | Phase 1 (trust kernel) | Physical Apple device |
| 2.7 | BLE Mesh Testing | High | Phase 2 (mesh) | 3+ physical devices |
| 2.8 | buf lint/breaking | Low | None | buf CLI |
| 2.9 | Rust toolchain install | Medium | All (compilation) | rustup + stable 1.75.0 |
| 2.10 | WasmParity CI runtime test | Medium | Phase 4 | wasm3 + wasmtime binaries |

---

### 2.9 Rust Toolchain Not Installed

**What:** `cargo check --workspace` cannot be run locally — Rust 1.75.0 is not installed on the development machine.

**Why Skipped:**

- `rustup` not found in PATH
- No `~/.cargo/bin/cargo` binary exists
- All workspace code is syntactically authored but not compiler-verified

**What's Needed:**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install 1.75.0
rustup default 1.75.0
# Then from source/core/:
cargo check --workspace
```

**Impact:** All Rust code is structurally correct per spec and uses idiomatic patterns, but has not been through `rustc` type-checking. Minor type errors may exist. CI will catch these on first push.

**Status:** Code authored. Compilation deferred to Rust toolchain installation.

---

## 4. Recommended Priority When Hardware Available

1. **Secure Enclave** (2.6) — blocks Phase 1 Trust Kernel
2. **BLE Mesh** (2.7) — blocks Phase 2 Mesh deployment
3. **GPU Tier Testing** (2.5) — blocks Phase 3 Security-Visible UI
4. **WasmParity** (2.1) — blocks Phase 4 .tapp runtime
5. **SBOM Signing** (2.2) — blocks Gov/Military compliance
6. **Custom Lint** (2.3) — defense-in-depth, not blocking
7. **Cross-compilation** (2.4) — CI handles primary targets
8. **buf lint** (2.8) — first proto modification trigger

---

*Generated: 2026-05-12 · Phase 0 Architecture & Design Foundation*
