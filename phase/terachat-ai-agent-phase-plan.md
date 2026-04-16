# TeraChat AI-Orchestrated Execution Plan

```yaml
date: 2026-04-16
author_role: Principal Engineer
scope: docs/MD
output_type: phase-plan + daily-plan + team-composition + parallel-index
```

## 1. Phạm vi tài liệu đã phân tích

- `docs/MD/Introduction.md`
- `docs/MD/Arrange.md`
- `docs/MD/Tech_Debt.md`
- `docs/MD/TestMatrix.md`
- `docs/MD/Design.md`
- `docs/MD/Spec-Core-Cryptography-And-Mesh.md`
- `docs/MD/Spec-Dual-Sync-And-Local-Storage.md`
- `docs/MD/Spec-Client-IPC-And-UI-Bridge.md`
- `docs/MD/Spec-Wasm-Tapp-Runtime.md`
- `docs/MD/Spec-Identity-And-Governance.md`
- `docs/MD/Spec-Ecosystem-And-Trust-Chain.md`
- `docs/MD/Spec-Enterprise-Secure-Enclave.md`
- `docs/MD/Note.md`

## 2. Kết luận điều phối cấp kiến trúc

### 2.1 Các sự thật kiến trúc không được vi phạm

1. `Rust Core` là domain owner tuyệt đối; UI chỉ là renderer. Không được port business logic hay crypto lên Dart/JS/UI thread.  
   Nguồn: `TERA-CORE §2.1`, `TERA-CLIENT §1.2`, `Design §3`.

2. Dự án phải đi theo kiến trúc `headless daemon + unified gRPC` trước khi mở rộng UI hay .tapp.  
   Nguồn: `TERA-CORE §12.6`, `TERA-CLIENT §12.3`.

3. Sync phải tách 2 plane: `CRDT DAG` cho chat và `relational sync` cho structured data. Không được đẩy Finance/HR vào CRDT.  
   Nguồn: `TERA-SYNC §1.2`, `§8.4`.

4. Mọi tính năng AI chỉ được mở sau khi có `SanitizedPrompt`, `DomainPiiPolicy`, `ZKMemoryIndex`, và không có embedding egress.  
   Nguồn: `TERA-ENCLAVE §1.1`, `TERA-SYNC §3.4`, `§8.6`.

5. Zero-Trust được CISO nắm quyền phủ quyết. DataGrant revocation, SCIM offboarding, legal hold, kill-switch phải được xem là security channel, không phải feature phụ.  
   Nguồn: `TERA-GOV`, `TERA-ECO §8.3`, `§8.6`.

6. Test không thể đi sau. `SC-34` tới `SC-40` là deployment blockers cho Gov/Military; phải được xây cùng feature.  
   Nguồn: `TestMatrix.md`, `Tech_Debt GAP-A/B/C/E/G`.

### 2.2 Các blocker phải xử lý sớm nhất

| Mã | Nội dung | Vì sao phải làm trước |
|---|---|---|
| `TD-006` | FFI panic abort bypass zeroization | Vi phạm trực tiếp Zero-Knowledge |
| `TD-008` | BLE QoS starvation | Làm hỏng control plane trong mesh |
| `TD-009` | EMDP escrow chưa hybrid PQ | Gov/Military không thể ký |
| `TD-011` | Localhost streaming plaintext leak | Phá vỡ zero-knowledge tại client |
| `GAP-A/B/C` | SagaRecoveryGuard, WAL signals, NSE race | Không có recovery path an toàn khi khởi động |
| `GAP-E` | DataGrant quorum cho Gov-tier | Revocation/activation không đáng tin |
| `GAP-G` | Burner Agent + EMDP freeze race | Sinh zombie member, sai invariant |

### 2.3 Thứ tự triển khai đúng

1. Khóa ranh giới kiến trúc và ABI.
2. Dựng trust kernel: zeroize, monotonic time, daemon, mesh QoS, PQ escrow.
3. Dựng storage correctness: saga, WAL handshake, CoreBootSequence, CAS v2.
4. Dựng client/runtime bridge: gRPC, secure streaming, widget states, security-priority UI.
5. Mở WASM/.tapp và ecosystem chỉ sau khi runtime contracts cứng và review pipeline xong.
6. Mở AI enclave và private search sau khi governance, data grants, redaction, legal hold đã chạy.
7. Chỉ release khi chaos matrix và reproducible build đều pass.

## 3. Cơ cấu tổ chức thực thi

### 3.1 Nhóm chức năng gốc

| Nhóm | Vai trò chính |
|---|---|
| `Architecture & Leadership` | Khóa boundaries, trade-off, sequencing |
| `Core Mesh & Cryptography` | MLS, HKMS, PQ-KEM, mesh, EMDP |
| `State, Runtime & Client UI` | dual-sync, runtime, IPC, UI |
| `Private Data & AI Ecosystem` | ZKMemory, search, enclave AI, encrypted storage |
| `Infra, Ops & Quality` | daemon, build forge, SRE, SecOps, chaos |

### 3.2 Mô hình team lai theo phase

| Team lai | Thành phần kết hợp | Nhiệm vụ |
|---|---|---|
| `Trust Kernel Team` | Software Architect, CISO/Lead Security, Applied Cryptography, Distributed Systems, Systems Runtime, SecOps | khóa trust boundaries, zeroize, PQ, daemon |
| `State Integrity Team` | State & CRDT, DBA, Distributed Systems, Tech Lead, Chaos QA | dual-plane correctness, saga, WAL, recovery |
| `Client Bridge Team` | Systems/WASM Runtime, Product UI, Design, Engineering Manager, Security Architect | gRPC bridge, streaming, signals, UI states |
| `Governance & Ecosystem Team` | CISO, Lead Security Architect, Platform Engineer, IT Admin rep, Review lead | OPA, SCIM, DataGrant, trust chain, kill-switch |
| `Private AI & Enclave Team` | AI/ML Enclave, Private Search, Data Pipeline & DBA, Enclave/SRE, Compliance | redaction, enclave routing, ZKMemory, on-prem AI |
| `Release & Resilience Team` | SecOps, Chaos QA, SRE, Review lead, Engineering Manager | SC-01..40, build reproducibility, release gates |

## 4. Mô hình điều phối AI Agents

### 4.1 Vai trò mặc định

| Agent | Trách nhiệm bắt buộc |
|---|---|
| `AI Agent` | đọc spec, sinh context pack, tách task, đối chiếu invariant, theo dõi dependency |
| `Code Agent` | implement bounded slice với write scope rõ ràng |
| `Test Agent` | viết test, chaos harness, fixtures, benchmark, negative cases |
| `Review Agent` | review security, regression, spec compliance, merge gate |

### 4.2 Quy trình chuẩn cho mọi ticket

1. `AI Agent` tạo `Context Pack`: mục tiêu, spec refs, file scope, acceptance criteria, scenario test bắt buộc.
2. `Code Agent` chỉ sửa trong write scope được giao, không đụng scope chung nếu chưa khóa contract.
3. `Test Agent` map ticket vào `TestMatrix` và thêm test tự động trước merge.
4. `Review Agent` kiểm tra 4 điểm: vi phạm invariant, drift khỏi spec, regression, thiếu test.

### 4.3 Luật song song hóa

- Không giao 2 `Code Agent` cùng write scope.
- Shared contracts phải khóa trước ở Day 1-2 của phase.
- Mọi ticket phải gắn ít nhất 1 spec reference và 1 scenario test.
- `CISO / Review Agent` có quyền chặn merge nếu vi phạm Zero-Trust, legal hold, hoặc signed audit trail.

## 5. Chỉ mục song song cho AI Agents

| Index | Scope | Spec refs chính | Human lead | Agent flow | Dependency |
|---|---|---|---|---|---|
| `IDX-01-CORE-FFI` | `GlobalKeyArena`, `ffi_boundary!`, panic hook | `TERA-CORE §12.1` | Applied Crypto | AI -> Code -> Test -> Review | none |
| `IDX-02-CORE-MASK-TIME` | XOR masking, monotonic tick, composite key derivation | `TERA-CORE §12.2`, `§12.5`, `§11.7` | Applied Crypto + Security Architect | AI -> Code/Test song song -> Review | `IDX-01` |
| `IDX-03-CORE-MESH` | MeshMultiplexer, PQ escrow, EMDP freeze | `TERA-CORE §11.4`, `§12.3`, `§12.4` | Distributed Systems | AI -> Code -> Test -> Review | `IDX-01` |
| `IDX-04-CORE-DAEMON` | headless daemon, gRPC core service | `TERA-CORE §12.6`, `TERA-CLIENT §12.3` | Architect + Systems Runtime | AI -> Code -> Review | contract lock |
| `IDX-05-SYNC-SAGA` | dual-plane schema, saga journal, boot recovery | `TERA-SYNC §8.4`, `TERA-CORE §11.8` | State & CRDT | AI -> Code/Test -> Review | `IDX-04` |
| `IDX-06-SYNC-CAS` | tenant-salted dual-hash CAS, adaptive vacuum | `TERA-SYNC §9.1`, `§9.2`, `§9.3` | DBA + Data Pipeline | AI -> Code -> Test -> Review | `IDX-05` partial |
| `IDX-07-CLIENT-STREAM` | UDS/Named Pipes, OTST, secure stream | `TERA-CLIENT §12.1` | Product UI + Systems Runtime | AI -> Code/Test -> Review | `IDX-04` |
| `IDX-08-CLIENT-UX` | `PENDING_SECURE_CHANNEL`, widget states, security priority channel | `TERA-CLIENT §11.3-11.5`, `Design §8-17` | Product UI + Design | AI -> Code -> Test -> Review | `IDX-04` |
| `IDX-09-RUNTIME-ABI` | MessagePack schema version, cursor protocol, quota enforcement | `TERA-RUNTIME §11.4`, `§11.6`, `Tech_Debt TD-001` | Systems/WASM Runtime | AI -> Code/Test -> Review | `IDX-04`, `IDX-05` |
| `IDX-10-RUNTIME-FUEL` | gas/fuel metering, fixed-point enforcement, webhook contract | `TERA-RUNTIME §11.3`, `§11.5`, `§11.8` | Systems/WASM Runtime | AI -> Code/Test -> Review | `IDX-09` |
| `IDX-11-GOV-DATAGRANT` | OPA, SCIM, audit log, DataGrant revocation/quorum | `TERA-GOV`, `TERA-ECO §8.3-8.7` | CISO + Security Architect | AI -> Code/Test -> Review | `IDX-05`, `IDX-09` |
| `IDX-12-ECO-TRUST` | signing chain, registry, kill-switch, EV signing | `TERA-ECO §2-4`, `§8.8`, `§8.9` | Platform Engineer | AI -> Code/Test -> Review | `IDX-10`, `IDX-11` |
| `IDX-13-ENCLAVE-AI` | SanitizedPrompt, DomainPiiPolicy, ZKMemoryIndex | `TERA-ENCLAVE §1.1`, `TERA-SYNC §3.4`, `§4.5` | AI/ML Enclave | AI -> Code/Test -> Review | `IDX-11` |
| `IDX-14-QA-CHAOS` | SC-01..40 automation, release gates | `TestMatrix.md` | Chaos QA + SecOps | AI -> Test -> Review | mọi index trước |

## 6. Roadmap theo phase và từng ngày

> Giả định: 35 ngày làm việc, triển khai theo style vibe coding nhưng có contract cứng, branch song song, review gate nghiêm ngặt.

---

## Phase 1. Architecture Lock & Delivery Skeleton

**Mục tiêu phase**

- Khóa boundaries giữa `Core`, `Sync`, `Runtime`, `Client`, `Gov`, `Eco`, `Enclave`.
- Chốt work partition cho AI agents để tránh đụng scope.
- Khóa CI/review gate từ ngày đầu.

**Team kết hợp**

- `Architecture & Leadership`
- `Core Mesh & Cryptography`
- `State, Runtime & Client UI`
- `Infra, Ops & Quality`

| Ngày | Nội dung | Mục tiêu ngày | Bộ phận lead | AI agent split |
|---|---|---|---|---|
| `Day 1` | Chốt domain map, dependency graph, task index `IDX-01..14` | không còn task mơ hồ hoặc chồng scope | Architect + Tech Lead | AI Agent tạo context packs; Review Agent khóa boundaries |
| `Day 2` | Chốt contract `gRPC/Protobuf`, ABI versioning, state ownership | không còn tranh cãi FFI vs gRPC ở phase sau | Architect + Systems Runtime | Code Agent scaffold proto; Review Agent kiểm invariants |
| `Day 3` | Dựng CI baseline: audit, gitleaks, nextest, wasm parity, SBOM hooks | mọi PR từ đây đều bị gate tự động | Infra + SecOps | Code Agent dựng CI; Test Agent kiểm smoke |
| `Day 4` | Chốt signal-to-UI map, widget states, design security overlays | UI phase sau chỉ cần render đúng signal | Product UI + Design | AI Agent sinh state catalog; Review Agent chốt naming |
| `Day 5` | Freeze ADRs, phase entry checklist, branch strategy cho agents | phase 2 có thể chạy song song mà không drift | Architect + EM + CISO | Review Agent phát hành sign-off checklist |

**Exit criteria**

- Có ADR cho daemon, gRPC, dual-sync, DataGrant, .tapp runtime.
- Mọi workstream có `spec refs`, `file scope`, `acceptance test`, `owner`.

---

## Phase 2. Trust Kernel & Survivability Foundation

**Mục tiêu phase**

- Trả nợ các điểm đe dọa trực tiếp Zero-Knowledge.
- Dựng daemon để core không chết theo UI.
- Khóa các invariant về memory, time, mesh control plane.

**Team kết hợp**

- `Trust Kernel Team`
- `Release & Resilience Team`

| Ngày | Nội dung | Mục tiêu ngày | Bộ phận lead | AI agent split |
|---|---|---|---|---|
| `Day 6` | Implement `GlobalKeyArena`, panic hook, `ffi_boundary!` | xử lý `TD-006` | Applied Crypto | Code Agent `IDX-01`; Test Agent thêm zeroize tests |
| `Day 7` | Implement iOS XOR RAM masking + Linux `mlock()` hard fail | đóng `TD-007`, `XPLAT-04` | Applied Crypto + SecOps | Code/Test song song; Review Agent audit memory paths |
| `Day 8` | Implement composite key derivation + monotonic tick clock | khóa `SECURITY-TIME-01` và defense-in-depth | Security Architect | Code Agent `IDX-02`; Test Agent thêm clock rollback tests |
| `Day 9` | Implement `MeshMultiplexer` P0/P1/P2 + hybrid PQ escrow | đóng `TD-008`, `TD-009` | Distributed Systems + Crypto | Code Agent `IDX-03`; Test Agent map `SC-38` |
| `Day 10` | Scaffold `headless Rust daemon` cho Android/Desktop + gRPC service skeleton | dời project sang kiến trúc sống sót độc lập UI | Architect + Systems Runtime | Code Agent `IDX-04`; Review Agent kiểm scope phá vỡ cũ |

**Exit criteria**

- `TD-006`, `TD-008`, `TD-009` có implementation path rõ và test đầu tiên chạy.
- Core có daemon shell + gRPC service skeleton.

---

## Phase 3. Dual-Sync Correctness & Recovery Plane

**Mục tiêu phase**

- Chống sai lệch dữ liệu giữa chat plane và app-state plane.
- Dựng startup recovery chuẩn để client crash không thành data corruption.
- Chốt CAS v2 và vacuum policy để scale.

**Team kết hợp**

- `State Integrity Team`
- `Trust Kernel Team`

| Ngày | Nội dung | Mục tiêu ngày | Bộ phận lead | AI agent split |
|---|---|---|---|---|
| `Day 11` | Chốt schema `hot_dag.db` / `cold_state.db` + `SagaEntry` | dual-plane write có contract chuẩn | State & CRDT | Code Agent `IDX-05`; Review Agent kiểm immutability |
| `Day 12` | Implement `SagaRecoveryGuard`, `NseRingBufferDrain`, `WalIntegrityCheck` | xử lý `GAP-A/C`, boot recovery < 300ms | State & CRDT + iOS runtime | Code/Test song song với `SC-37` |
| `Day 13` | Implement WAL handshake signals và replay ordering | xử lý `GAP-B`, `CRIT-05` | State & CRDT + Client bridge | Code Agent `IDX-05`; Test Agent race tests |
| `Day 14` | Implement tenant-salted dual-hash CAS migration | đóng `TD-013`, `TD-014` | DBA + Data Pipeline | Code Agent `IDX-06`; Test Agent blob migration tests |
| `Day 15` | Implement adaptive vacuum + legal-hold-aware retention + ZKMemory cleanup | ngăn WAL bloat và doc drift | DBA + Gov liaison | Test Agent bench/vacuum; Review Agent retention policy |

**Exit criteria**

- `SC-37`, `SC-39`, `SC-40` có harness đầu tiên.
- Dual-plane write đã có idempotent recovery path.

---

## Phase 4. Client Bridge, Secure Streaming & Security-Visible UX

**Mục tiêu phase**

- Cắt dần FFI/localhost cũ, chuyển sang bridge an toàn hơn.
- Đảm bảo UI biểu đạt đúng trạng thái bảo mật nhưng không sở hữu logic.
- Chốt các trạng thái “degraded nhưng an toàn”.

**Team kết hợp**

- `Client Bridge Team`
- `Trust Kernel Team`

| Ngày | Nội dung | Mục tiêu ngày | Bộ phận lead | AI agent split |
|---|---|---|---|---|
| `Day 16` | Bật `gRPC` non-breaking cho màn hình/signal mới | bắt đầu migration khỏi IPC phân mảnh | Systems Runtime + Product UI | Code Agent `IDX-04/08`; Review Agent kiểm backward compatibility |
| `Day 17` | Thay `localhost proxy` bằng `UDS/Named Pipes` desktop + `OTST` mobile | đóng `TD-011` | Systems Runtime + Security | Code Agent `IDX-07`; Test Agent unauthorized access tests |
| `Day 18` | Implement `PENDING_SECURE_CHANNEL`, outbox guard, widget states | xử lý `SC-35` UX đúng | Product UI + Design | Code Agent `IDX-08`; Test Agent UI state contract tests |
| `Day 19` | Render Mesh HUD, memory purge overlay, FCP, E2EE indicators, GPU fallback | security visible mọi lúc | Product UI + Design | Code Agent UI lane; Review Agent spec-to-design check |
| `Day 20` | Hoàn tất reconnect/restart flows cho daemon, Binder/XPC, Windows lock serialization | UI crash không giết core; lock contention giảm | Systems Runtime + Desktop lead | Test Agent stress desktop/mobile reconnect |

**Exit criteria**

- `TD-011` được thay thế bằng secure stream path.
- UI chỉ render signals/snapshots, không còn tự tính domain state mới.

---

## Phase 5. WASM Runtime, .tapp Contracts & Ecosystem Safety

**Mục tiêu phase**

- Chỉ mở cửa .tapp sau khi runtime contracts đủ cứng.
- Chặn OOM, float drift, quota races, schema drift, unsafe egress.
- Dựng review pipeline và signing chain cho ecosystem.

**Team kết hợp**

- `Governance & Ecosystem Team`
- `Client Bridge Team`

| Ngày | Nội dung | Mục tiêu ngày | Bộ phận lead | AI agent split |
|---|---|---|---|---|
| `Day 21` | MessagePack `schema_version`, cursor protocol, host ABI error codes | đóng `TD-001`, giảm boundary leak | Systems/WASM Runtime | Code Agent `IDX-09`; Test Agent parity/cursor tests |
| `Day 22` | Fuel metering, fixed-point gate, float detection required CI | xử lý `TD-003`, `HIGH-3` | Systems/WASM Runtime + Review lead | Code/Test song song; Review Agent LLVM gate |
| `Day 23` | Per-endpoint circuit breaker, webhook ACK contract, quota synchronous enforcement | runtime không tự bắn vào chân | Systems/WASM Runtime | Code Agent `IDX-10`; Test Agent webhook timeout tests |
| `Day 24` | Registry review pipeline, transparency log, publisher trust tiers, kill-switch path | ecosystem có zero-trust supply chain | Platform Engineer | Code Agent `IDX-12`; Review Agent security scan checklist |
| `Day 25` | DataGrant prefetch, revocation gossip, quorum activation Gov-tier | khóa `GAP-E`, `CRIT-03` | CISO + Platform + Gov | Code Agent `IDX-11`; Test Agent revocation partition tests |

**Exit criteria**

- `.tapp` runtime có deterministic execution.
- Registry/MDM/kill-switch path có trust chain hoàn chỉnh.

---

## Phase 6. Governance, Private Data & Secure Enclave

**Mục tiêu phase**

- Biến policy, audit, approval, redaction thành first-class primitives.
- Chốt đường chạy AI nội bộ mà không phá Zero-Knowledge.
- Hoàn thiện on-prem topology cho enterprise/gov.

**Team kết hợp**

- `Governance & Ecosystem Team`
- `Private AI & Enclave Team`

| Ngày | Nội dung | Mục tiêu ngày | Bộ phận lead | AI agent split |
|---|---|---|---|---|
| `Day 26` | Implement OPA bundle lifecycle, `host_opa_check`, signed audit append path | governance decisions chạy ngay tại boundary | CISO + Security Architect | Code Agent `IDX-11`; Review Agent formal policy checklist |
| `Day 27` | Implement SCIM offboarding, approval signatures, legal hold gates | xử lý SLA `<30s` và non-repudiation | Security Architect + IT Admin rep | Code/Test song song với revoke tests |
| `Day 28` | Implement `SanitizedPrompt`, `DomainPiiPolicy`, tenant PII redaction | AI chỉ thấy dữ liệu đã khử PII theo tenant | AI/ML Enclave | Code Agent `IDX-13`; Review Agent privacy audit |
| `Day 29` | Implement `ZKMemoryIndex`, private search limits, local indexing budgets | private search không leak, không treo client | Private Search + DBA | Code/Test bench search memory |
| `Day 30` | Chốt Mac mini + NAS + RTX topology, EV signing runner, air-gapped packaging path | enterprise deployment path có thật, không nói suông | Enclave/SRE + SecOps | AI Agent tạo deploy runbook; Review Agent kiểm signing chain |

**Exit criteria**

- SCIM, audit, legal hold, DataGrant, AI redaction cùng hoạt động không conflict.
- Có deploy model rõ cho `On-Prem`, `Air-Gapped`, `Hybrid`.

---

## Phase 7. Chaos, Compliance & Release Candidate

**Mục tiêu phase**

- Chứng minh hệ thống chịu được combined failures.
- Chốt release candidate bằng test và audit, không bằng niềm tin.

**Team kết hợp**

- `Release & Resilience Team`
- Toàn bộ lead của 5 nhóm chức năng

| Ngày | Nội dung | Mục tiêu ngày | Bộ phận lead | AI agent split |
|---|---|---|---|---|
| `Day 31` | Tự động hóa `SC-34` tới `SC-40` và kết nối vào CI/nightly | combined-failure thành test thật | Chaos QA + SecOps | Test Agent `IDX-14`; Review Agent kiểm observability |
| `Day 32` | Chạy full matrix `SC-01..40`, fix regressions blocker | gom defect trước RC | Chaos QA + Tech Lead | Code/Test feedback loop nhanh |
| `Day 33` | Reproducible build, SBOM, signing, supply-chain, port scan, static analysis full run | release artifacts có thể audit | Infra + SecOps | Test Agent verify build gates |
| `Day 34` | Hoàn tất disclosures/risk docs: Huawei limits, fallback semantics, admin runbooks | không có compliance surprise | CISO + Compliance + Product | AI Agent doc pack; Review Agent legal/compliance check |
| `Day 35` | Go/No-Go review, RC sign-off, carry-over backlog | ra quyết định dựa trên evidence | Architect + CISO + EM | Review Agent final gate; AI Agent compile evidence pack |

**Exit criteria**

- `All 40 chaos scenarios pass automated CI suite`.
- Reproducible build + signing + audit evidence đầy đủ.
- CISO và Architect cùng ký `Go`.

## 7. Mục tiêu đầu ra cho từng phase

| Phase | Đầu ra bắt buộc |
|---|---|
| `Phase 1` | ADRs, task index, CI gates, signal catalog |
| `Phase 2` | secure core primitives, daemon skeleton, mesh/QoS base |
| `Phase 3` | dual-sync correctness, boot recovery, CAS v2, vacuum |
| `Phase 4` | secure bridge, stream path, security-visible UI |
| `Phase 5` | safe .tapp runtime, registry trust chain, DataGrant orchestration |
| `Phase 6` | governance enforcement, AI enclave, private search, deployment path |
| `Phase 7` | chaos evidence, compliance pack, release candidate |

## 8. Definition of Done cho dự án vibe-coded này

Một workstream chỉ được xem là hoàn tất khi có đủ:

1. Code chạy được trên write scope đã giao.
2. Có test map về `TestMatrix` hoặc negative case tương ứng.
3. Có review security/spec compliance từ `Review Agent`.
4. Không vi phạm các invariant: zeroize, append-only audit, no plaintext egress, no UI-owned business logic, no cross-tenant leakage.
5. Có artifact rõ: ADR, proto/schema, benchmark, chaos log, SBOM, hoặc runbook.

## 9. Nhận định cuối cùng

TeraChat không phải bài toán “xây chat app rồi thêm bảo mật sau”. Theo toàn bộ bộ spec hiện tại, thứ tự đúng là:

1. dựng `trust kernel`,
2. dựng `correctness plane`,
3. dựng `bridge/runtime`,
4. rồi mới mở `ecosystem + AI`.

Nếu đảo thứ tự, team sẽ tạo ra rất nhiều code nhìn có vẻ chạy được nhưng sẽ xung đột trực tiếp với `TERA-CORE`, `TERA-SYNC`, `TERA-GOV`, `TERA-ECO`, và toàn bộ `TestMatrix`.

Vì vậy, mô hình tốt nhất cho dự án này là: **kiến trúc khóa cứng ở đầu phase, implementation song song theo index, test song hành từng ngày, review có quyền phủ quyết thực sự**.
