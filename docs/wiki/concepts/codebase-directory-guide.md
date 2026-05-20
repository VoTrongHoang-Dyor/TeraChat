---
type: concept
created: 2026-05-17
updated: 2026-05-17
tags: [codebase, directory, onboarding, navigation, architecture]
sources: [CLAUDE.md, README.md, phase/README.md, source/core/Cargo.toml]
---

# Codebase Directory Guide

Tài liệu này là "Mục lục" của toàn bộ monorepo TeraChat — giúp mọi lập trình viên mới hiểu ngay kiến trúc tổ chức code, vị trí từng module, và cách tra cứu nhanh.

## Tổng quan Kiến trúc

TeraChat tổ chức theo mô hình **Monorepo với Rust Core làm domain owner**. Toàn bộ business logic (crypto, sync, mesh, WASM runtime, AI gateway) nằm trong `source/core/` — một Cargo workspace gồm 6 crate với ranh giới module rõ ràng. UI (Tauri/Flutter) là passive renderer, không chứa business logic. Tài liệu được tổ chức riêng trong `docs/`, kế hoạch phát triển theo Vertical Slice trong `phase/`. Quy tắc kiến trúc bất biến được thực thi bởi CI gates và type system — không phải convention.

## Sơ đồ Cây Thư mục

```
TeraChat-Project/
├── .agents/                        # Multi-Agent Harness — định nghĩa AI agent
│   ├── commands/                   #   Lệnh custom cho agent (review, fix-issue, deploy)
│   ├── langgraph/                  #   LangGraph orchestrator (Python)
│   ├── rules/                      #   Quy tắc agent (branch-strategy, CLAUDE.md)
│   └── skills/                     #   Kỹ năng chuyên biệt (Rust, Cloudflare)
│
├── .context/                       # Agent entry point context
│   └── AGENT_CONTEXT.md            #   File đầu tiên mọi AI agent phải đọc
│
├── .github/workflows/              # CI/CD pipelines
│   ├── ci.yml                      #   Build, clippy, test, audit
│   ├── sbom.yml                    #   Software Bill of Materials generation
│   ├── security.yml                #   Gitleaks, cargo audit, dependency scan
│   └── wasm-parity.yml             #   wasm3 ≡ wasmtime output parity check
│
├── assets/                         # Brand assets (logo)
├── config/                         # Cấu hình agent (Agent.md)
│
├── docs/                           # Toàn bộ tài liệu dự án
│   ├── wiki/                       #   Obsidian vault — Wiki chính thức
│   │   ├── concepts/               #     ~30 file kiến trúc, ADR, spec khái niệm
│   │   ├── entities/               #     Entity definitions (đang xây dựng)
│   │   ├── sources/                #     Bản sao spec từ docs/raw/MD (định dạng wiki)
│   │   ├── syntheses/              #     Tổng hợp, gap analysis, health check
│   │   ├── index.md                #     Mục lục Wiki (Obsidian-style backlinks)
│   │   ├── invariants.md           #     13 architectural invariants + enforcement
│   │   └── ubiquitous-language.md  #     Từ điển thuật ngữ EN+VI
│   ├── raw/MD/                     #   Raw spec documents (14 files)
│   └── HTML/                       #   Tài liệu kinh doanh (Executive Summary, Pitch Deck)
│
├── phase/                          # Kế hoạch Vertical Slice (18 tháng)
│   ├── README.md                   #   Slice execution map, timeline, budget
│   ├── phase-0-architecture-foundation.md
│   ├── phase-1-trust-kernel.md
│   ├── phase-2-dual-sync.md
│   ├── phase-3-client-bridge.md
│   ├── phase-4-wasm-ecosystem.md
│   ├── phase-5-governance-enclave.md
│   └── phase-6-chaos-release.md
│
├── report/                         # Báo cáo kỹ thuật
│   ├── phase-0-hardware-gaps.md    #   Phân tích gap phần cứng
│   └── TeraChat_TechnicalAudit.html
│
├── source/                         # TOÀN BỘ SOURCE CODE
│   ├── apps/                       #   Client applications (passive renderer)
│   │   ├── Laptop/                 #     macOS, Windows, Linux (Tauri)
│   │   └── Phone/                  #     iPhone, Android, Huawei, Oppo (Flutter)
│   ├── core/                       #   ★ RUST CORE — domain owner (Cargo workspace)
│   │   ├── Cargo.toml              #     Workspace manifest (6 crates)
│   │   ├── proto/                  #     Protobuf definitions (7 gRPC services)
│   │   ├── tc-crypto/              #     MLS E2EE, key management, FFI token protocol
│   │   ├── tc-mesh/                #     BLE/WiFi Direct mesh, peer discovery, EMDP
│   │   ├── tc-crdt-sync/           #     CRDT DAG sync engine
│   │   ├── tc-store/               #     Dual-plane storage (hot_dag.db + cold_state.db)
│   │   ├── tc-tapp/                #     WASM sandbox runtime (wasmtime + wasm3)
│   │   └── tc-proto/               #     Generated Rust code from proto definitions
│   └── server/                     #   Server deployment configurations
│       ├── Mac/                    #     launchd plist + config.toml cho Mac mini
│       └── Physical Server/        #     Bare-metal server config
│
├── tests/                          # Test suites (độc lập với source)
│   ├── chaos-mesh/                 #   Chaos engineering tests
│   ├── cross-platform-e2e/         #   End-to-end cross-platform tests
│   └── ffi-stress/                 #   FFI boundary stress tests
│
├── CLAUDE.md                       # Engineering guardrails (AI agent rulebook)
├── CONTRIBUTING.md                 # Hướng dẫn đóng góp .tapp cho community
├── README.md                       # Tổng quan dự án, tính năng, kiến trúc
├── .env / .env.example             # Biến môi trường
└── .gitignore                      # Git ignore rules
```

## Từ điển Module

### Thư mục Root (Cấp 1)

| Thư mục / File | Vai trò / Chức năng chính | Công nghệ / Thành phần cốt lõi |
|----------------|--------------------------|-------------------------------|
| `.agents/` | Multi-Agent Harness — định nghĩa agent AI, lệnh, kỹ năng, quy tắc cho LangGraph orchestrator | LangGraph (Python), Claude Code custom commands, Marketplace skills |
| `.context/` | Entry point context cho mọi AI agent — file AGENT_CONTEXT.md là file đầu tiên cần đọc | Markdown, YAML frontmatter |
| `.github/workflows/` | CI/CD pipeline — build, test, security scan, SBOM, WASM parity | GitHub Actions, cargo-audit, gitleaks, buf |
| `assets/` | Brand assets | Hình ảnh (logo) |
| `config/` | Cấu hình cho AI agent | Agent.md |
| `docs/` | Toàn bộ tài liệu — Wiki (Obsidian), Raw Specs, HTML docs | Obsidian vault, Markdown, HTML |
| `phase/` | Kế hoạch phát triển Vertical Slice — execution map 18 tháng, timeline, budget | Markdown, YAML |
| `report/` | Báo cáo kỹ thuật — audit, hardware gap analysis | HTML, Markdown |
| `source/` | **Toàn bộ source code** — Rust Core, Client Apps, Server configs | Rust, Flutter, Tauri, Protobuf |
| `tests/` | Test suites độc lập — chaos engineering, cross-platform e2e, FFI stress | Rust, integration tests |
| `CLAUDE.md` | Engineering guardrails — 13 invariants, forbidden patterns, dependency policy | Markdown, YAML frontmatter |
| `CONTRIBUTING.md` | Hướng dẫn đóng góp .tapp WASM cho community developer | Markdown |
| `README.md` | Tổng quan dự án — tính năng, kiến trúc, platform support | Markdown |
| `.env` / `.env.example` | Biến môi trường cho dự án | Shell environment |

### Rust Core Crates (`source/core/` — Cấp 2)

| Crate | Vai trò / Chức năng chính | Công nghệ / Thành phần cốt lõi | License |
|-------|--------------------------|-------------------------------|---------|
| `tc-crypto/` | MLS RFC 9420 E2EE, quản lý key material, FFI Token Protocol, ZeroizeOnDrop enforcement | openmls, ring, zeroize, subtle, Secure Enclave/StrongBox/TPM 2.0 | **CLOSED** |
| `tc-mesh/` | BLE 5.0 + Wi-Fi Direct P2P mesh, peer discovery, EMDP emergency protocol, TeraLink 3-tier fallback | Bluetooth LE, mDNS, Wi-Fi Direct, MultipeerConnectivity (iOS) | BSL 1.1 |
| `tc-crdt-sync/` | CRDT DAG engine cho chat sync — append-only DAG, HLC timestamp, vector clock | sled, blake3, HLC | BSL 1.1 |
| `tc-store/` | Dual-plane storage — hot_dag.db (CRDT append-only) + cold_state.db (SQLite relational cho Finance/HR) | SQLite WAL, SQLCipher, rusqlite | BSL 1.1 |
| `tc-tapp/` | WASM sandbox runtime — dual-engine (wasmtime desktop + wasm3 iOS), fuel metering, Host ABI, capability enforcement | wasmtime, wasm3, WASM | BSL 1.1 |
| `tc-proto/` | Generated Rust code từ Protobuf definitions — build.rs với tonic-build | Prost, Tonic, Protocol Buffers | **MIT** |

### Client Apps (`source/apps/`)

| Thư mục | Vai trò | Công nghệ |
|----------|--------|-----------|
| `Laptop/macOS/` | macOS desktop client — Tauri shell, gRPC qua UDS IPC đến Rust Core | Tauri (Rust), Swift |
| `Laptop/Windows/` | Windows desktop client (Slice 6) | Tauri (Rust) |
| `Laptop/Linux/` | Linux desktop client (Post-Slice 6) | Tauri (Rust) |
| `Phone/Iphone/` | iOS client — Flutter UI, Rust Core qua FFI, Secure Enclave | Flutter, Swift, flutter_rust_bridge |
| `Phone/Android/` | Android client (Slice 5) | Jetpack Compose, Rust Foreground Service, StrongBox |
| `Phone/Huawei/` | Huawei client (Post-Slice 6) | HarmonyOS native |
| `Phone/Oppo/` | Oppo client (Post-Slice 6) | Android (ColorOS) |

### Server Deployment (`source/server/`)

| Thư mục | Vai trò |
|----------|--------|
| `Mac/` | macOS LaunchAgent plist + config.toml cho Mac mini Compute Node |
| `Physical Server/` | Bare-metal server deployment config |

### Tài liệu (`docs/`)

| Thư mục | Vai trò | Số lượng |
|----------|--------|----------|
| `wiki/concepts/` | Kiến trúc, ADR, spec khái niệm (~30 files) — nguồn chính cho hiểu hệ thống | ~30 files |
| `wiki/sources/` | Bản sao spec từ `raw/MD/` dưới định dạng wiki | ~15 files |
| `wiki/syntheses/` | Gap analysis, health check, improvement plan | ~12 files |
| `raw/MD/` | Raw spec gốc (14 specs + Tech_Debt + TestMatrix) | 14 files |
| `HTML/` | Tài liệu HTML cho stakeholder (Executive Summary, Pitch Deck, Pricing) | 4 files |

## Các File Cấu hình Quan trọng

| File | Vị trí | Mục đích |
|------|--------|----------|
| `CLAUDE.md` | Root | Engineering guardrails — 13 architectural invariants, forbidden patterns, dependency policy, AI compatibility matrix. Là "luật" cho mọi AI agent. |
| `README.md` | Root | Tổng quan dự án — tính năng cốt lõi (E2EE, .tapp, Local AI, Survival Mesh), kiến trúc, platform support, license. |
| `CONTRIBUTING.md` | Root | Hướng dẫn cho community developer build .tapp — TDD workflow, Hello World, manifest.json spec, validation. |
| `source/core/Cargo.toml` | `source/core/` | Cargo workspace manifest — khai báo 6 crate thành viên, shared dependencies (ring, tokio, tonic, etc.), version pinning. |
| `source/core/rust-toolchain.toml` | `source/core/` | Rust toolchain version pinning (1.75.0). |
| `source/core/proto/terachat.proto` | `source/core/proto/` | Định nghĩa toàn bộ 7 gRPC services: CoreService, SyncService, MeshService, RuntimeService, GovService, EcoService, EnclaveService. |
| `source/core/buf.yaml` | `source/core/` | Buf configuration — protobuf linting + breaking change detection. |
| `source/server/Mac/config.toml` | `source/server/Mac/` | Server configuration template — IPC socket, gRPC listen, database paths, relay, mesh, AI endpoint, telemetry, security. |
| `source/server/Mac/com.terachat.core.plist` | `source/server/Mac/` | macOS LaunchAgent plist — chạy Rust Core daemon theo ADR-001 Headless Daemon. |
| `.env.example` | Root | Environment variable template (hiện trống — chờ Slice 1). |
| `.github/workflows/ci.yml` | `.github/workflows/` | CI pipeline chính — cargo build, clippy, fmt, test, audit. |
| `.github/workflows/security.yml` | `.github/workflows/` | Security scanning — gitleaks, cargo audit, dependency vulnerability check. |
| `.github/workflows/wasm-parity.yml` | `.github/workflows/` | WASM parity check — đảm bảo wasm3 (iOS) và wasmtime (desktop) output giống hệt nhau. |

## Luồng Chỉ dẫn — 3 Thư mục Đọc Đầu Tiên

Nếu bạn là Developer mới và muốn hiểu **luồng nhắn tin** và **luồng Mesh Network** của TeraChat, hãy đọc theo thứ tự sau:

### 1. `docs/wiki/concepts/` — Hiểu Kiến trúc

Đây là nơi **bắt đầu**. Không cần đọc code ngay. Đọc các file sau theo thứ tự:

| Thứ tự | File | Tại sao đọc |
|--------|------|-------------|
| 1 | `terachat-architecture-overview.md` | Bức tranh tổng thể — system layers, communication model, dependency graph |
| 2 | `zero-knowledge-architecture.md` | Blind router model — vì sao server không bao giờ thấy plaintext |
| 3 | `survival-mesh-networking.md` | Cách TeraLink 3-tier (T1 LAN → T2 mDNS → T3 BLE) hoạt động |
| 4 | `crdt-dual-sync.md` | Dual-plane sync — vì sao chat dùng CRDT, Finance dùng Relational |
| 5 | `adr-001-headless-daemon-architecture.md` | Vì sao Rust Core chạy độc lập với UI process |
| 6 | `hierarchical-authority-messaging.md` | Mô hình phân quyền — ai được nhắn cho ai |

### 2. `source/core/proto/` — Hiểu Hợp đồng Dữ liệu

Sau khi hiểu kiến trúc, đọc **terachat.proto** — đây là "hợp đồng" giữa mọi thành phần trong hệ thống. File này định nghĩa:

- **CoreService** — MLS session, epoch rotation, device identity
- **SyncService** — Push/pull DAG nodes, relational state sync
- **MeshService** — Peer discovery, EMDP activation, mesh packet routing
- **RuntimeService** — .tapp load, Host ABI invoke, metrics
- **GovService** — License validation, OPA policy evaluation, audit trail
- **EcoService** — .tapp signature verification, DataGrant quorum, kill-switch
- **EnclaveService** — AI completion với PII redaction bắt buộc

### 3. `source/core/tc-crypto/src/` + `source/core/tc-mesh/src/` — Hiểu Implementation

Sau khi nắm hợp đồng dữ liệu, đi vào code:

| File cần đọc trong `tc-crypto/` | Mục đích |
|--------------------------------|----------|
| `lib.rs` | Public API surface — xem module export gì |
| `key_management.rs` | Cách key material được tạo, lưu, xoay — ZeroizeOnDrop enforcement |
| `ffi.rs` | FFI Token Protocol — cách Rust Core giao tiếp an toàn với UI |
| `zeroize_guard.rs` | Static analysis guard — ngăn mem::forget, ManuallyDrop, Arc\<KeyMaterial\> |
| `lints.rs` | Custom compiler lints cho crypto safety |

| File cần đọc trong `tc-mesh/` | Mục đích |
|-------------------------------|----------|
| `lib.rs` | Public API surface — mesh networking interface |
| `peer.rs` | Peer discovery, election weight, Floor Gateway coordinator logic |
| `multiplexer.rs` | TeraLink 3-tier multiplexing — T1/T2/T3 routing decision |
| `error.rs` | Mesh-specific error types |

**Thời gian ước tính:** 2-3 giờ để đọc hết 3 thư mục trên và có thể bắt đầu contribute vào messaging hoặc mesh code.

## Ghi chú cho Người Đọc

- **Mọi spec** đều có bản sao trong cả `docs/raw/MD/` (raw) và `docs/wiki/sources/` (wiki-linked). Hai bản là một — wiki version thêm backlinks.
- **Ubiquitous Language** (`docs/wiki/ubiquitous-language.md`) là từ điển bắt buộc — mọi thuật ngữ trong codebase đều được định nghĩa ở đây. Đọc trước khi viết code.
- **Invariants** (`docs/wiki/invariants.md`) là 13 quy tắc không thể vi phạm — enforced bởi CI, không phải convention. Mọi PR vi phạm = auto-reject.
- **CLAUDE.md** là rulebook cho AI agent, nhưng developer cũng nên đọc để hiểu forbidden patterns.
- **Phase README** (`phase/README.md`) cho biết hiện tại dự án đang ở slice nào, cái gì đang được build, và timeline.
