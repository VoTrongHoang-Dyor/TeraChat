---
type: synthesis
created: 2026-05-12
tags: [terachat, ci-cd, infrastructure, devops, quality-gate]
sources: [tech-debt-registry, terachat-technical-audit-2026, prototype-first-model]
status: active
resolves: "Thiếu CI/CD pipeline — critical path cho mọi phase"
---

# CI/CD Pipeline Specification

**Quyết định:** CI/CD pipeline phải được setup TRƯỚC khi có dòng code đầu tiên. Mọi PR phải pass tất cả gate trước khi merge.

## Nguyên tắc

1. **Shift-left security:** Audit + secret scan chạy trên mọi commit, không chỉ release
2. **Zero warning policy:** Clippy warnings = CI fail (không có "fix later")
3. **Hermetic builds:** Build trong Docker container frozen dependencies
4. **Progressive gates:** Prototype có gate đơn giản, Phase 3 có gate đầy đủ

---

## Phase 0 — Baseline CI (Setup trước Prototype)

```yaml
# .github/workflows/ci-baseline.yml
name: CI Baseline
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
        with:
          toolchain: "1.75.0"
          components: clippy, rustfmt
      - run: cargo clippy --all-targets --all-features -- -D warnings
      - run: cargo fmt --all -- --check

  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo audit --deny warnings
      - uses: gitleaks/gitleaks-action@v2
        with:
          config: .gitleaks.toml

  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
        with:
          toolchain: "1.75.0"
      - run: cargo nextest run --all-features
      - run: cargo test --test integration -- --test-threads=4

  protobuf-lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: bufbuild/buf-setup-action@v1
      - uses: bufbuild/buf-lint-action@v1
      - uses: bufbuild/buf-breaking-action@v1
        with:
          against: "https://github.com/terachat/terachat.git#branch=main,subdir=proto"
```

---

## Phase 1 — Extended CI (Sau Prototype)

Thêm vào baseline:

```yaml
  wasm-parity:
    runs-on: ubuntu-latest
    steps:
      - run: cargo test --test wasm_parity -- --engine both
        # Test vector chạy trên cả wasm3 và wasmtime
        # Assert: semantic equivalence, delta ≤ 20ms

  memory-safety:
    runs-on: ubuntu-latest
    steps:
      - run: cargo miri test --test ffi_boundary_zeroize
        # Chỉ test FFI boundary — không chạy full test suite
        timeout-minutes: 30

  float-detection:
    runs-on: ubuntu-latest
    steps:
      - run: ./scripts/detect-floats.sh
        # Block merge nếu f32/f64 xuất hiện trong financial .tapp
        # Whitelist: crypto constants, test files

  property-test:
    runs-on: ubuntu-latest
    steps:
      - run: cargo test --test proptest -- --ignored
        # Property-based tests cho crypto (proptest)
        # Chạy với --ignored vì slow — schedule nightly, không block PR
```

---

## Phase 2+ — Full CI/CD

Thêm vào:

```yaml
  chaos-test:
    runs-on: self-hosted  # Mac mini on-premise
    steps:
      - run: cargo test --test chaos -- --scenarios SC-01,SC-34..SC-40
        timeout-minutes: 120

  fuzz:
    runs-on: ubuntu-latest
    steps:
      - run: cargo fuzz run ffi_boundary -- -max_total_time=3600
        # Nightly only — không block PR

  hermetic-build:
    runs-on: self-hosted
    steps:
      - uses: nixbuild/nixbuild-action@v14
      - run: nix build .#terachat-all-platforms
      - run: cosign sign-blob --key hardware-token:// -y ./result/*
      - run: syft ./result/* -o spdx-json > sbom.json

  container-scan:
    runs-on: ubuntu-latest
    steps:
      - uses: aquasecurity/trivy-action@master
        with:
          scan-type: "fs"
          scan-ref: "."
          severity: "CRITICAL,HIGH"
```

---

## Quality Gates Per Phase

| Gate | Prototype | Phase 1 | Phase 2+ |
|------|-----------|---------|----------|
| `cargo clippy -D warnings` | ✅ Required | ✅ Required | ✅ Required |
| `cargo fmt --check` | ✅ Required | ✅ Required | ✅ Required |
| `cargo audit --deny` | ✅ Required | ✅ Required | ✅ Required |
| `gitleaks` | ✅ Required | ✅ Required | ✅ Required |
| `buf lint` | ✅ Required | ✅ Required | ✅ Required |
| `buf breaking` | — | ✅ Required | ✅ Required |
| `cargo nextest` | ✅ Required | ✅ Required | ✅ Required |
| `cargo miri` (FFI) | — | ✅ Required | ✅ Required |
| `wasm_parity` test | — | ✅ Required | ✅ Required |
| `float_detection` | — | ✅ Required | ✅ Required |
| `proptest` (crypto) | — | Nightly | ✅ Required |
| `chaos_test` (SC-01..40) | — | — | ✅ Required |
| `cargo fuzz` | — | — | Nightly |
| `hermetic_build` + SBOM | — | — | ✅ Required |
| `trivy` container scan | — | — | ✅ Required |
| EV Code Signing | — | — | ✅ Required (Windows) |

---

## Secrets Management

- **Development:** Doppler (free tier) hoặc GitHub Secrets
- **Production:** HashiCorp Vault (self-hosted trên Mac mini)
- **CI Variables:** Không dùng `.env` file — tất cả secrets qua CI secrets manager
- **Pre-commit hook:** `gitleaks detect --source .` ngăn commit chứa secrets

---

## Build Artifacts

Mỗi release build phải tạo ra:

1. **Signed binary** (`.app`, `.ipa`, `.apk`, `.exe`, `.AppImage`)
2. **SBOM** (SPDX JSON format — `sbom.json`)
3. **Cosign attestation** (signed by hardware token)
4. **Nix build log** (reproducible build proof)

Thiếu bất kỳ artifact nào → **block distribution**.

---

## Runner Infrastructure

| Runner | Purpose | Phase |
|--------|---------|-------|
| GitHub Actions (ubuntu-latest) | Lint, audit, unit test, security scan | Phase 0 |
| GitHub Actions (macos-latest) | macOS build + test | Phase 0 |
| Self-hosted Mac mini | iOS build, EV signing, chaos test, hermetic build | Phase 2+ |

---

*CI-CD-SPEC v1.0.0 · 2026-05-12 · Created from Technical Audit recommendations*
