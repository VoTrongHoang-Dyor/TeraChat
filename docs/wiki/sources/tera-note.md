---
type: source
created: 2026-05-10
tags: [terachat, devsecops, dependencies, build, security]
sources: [raw/MD/Note.md]
---

# TeraChat Engineering Notes (TERA-NOTE)

Source: `raw/MD/Note.md`.

## What It Covers

A mixed engineering reference covering: DevSecOps code review process, Prompt Injection analysis (Direct/Indirect), and detailed technical Q&A on dependencies, build tools, environment variables, databases, credentials, testing, and linting.

## Key Technical Details

- **Rust toolchain:** Fixed at 1.75.0 via `rust-toolchain.toml`
- **Crypto:** `ring` crate mandated — no other crypto crate allowed
- **WASM:** `wasmtime` (most platforms) + `wasm3` (iOS, W^X constraint)
- **Databases:** SQLite WAL (hot_dag.db, cold_state.db), PostgreSQL HA (≥100k users), MinIO/R2 for blobs
- **CI Gates:** `cargo clippy`, `cargo miri`, `cargo audit`, `gitleaks`, `trivy` — all blockers
- **No Maven, no web frontend framework** — Rust-first, Flutter/Tauri UI
- **Prompt Injection:** Documented Direct (Jailbreak) and Indirect (data poisoning) attack vectors for LLM integration

## Related

- [[tera-tech-debt]]
- [[tera-test-matrix]]
- [[WASM Tapp Runtime]]
- [[Zero-Knowledge Architecture]]
