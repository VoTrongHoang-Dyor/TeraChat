# Tech_Debt.md — TeraChat Technical Debt Registry

```yaml
# DOCUMENT IDENTITY
id: "TERA-DEBT"
title: "TeraChat — Technical Debt & Known Limitations Registry"
version: "1.0.0"
status: "ACTIVE"
audience: "Engineering Team, System Architect, QA"
purpose: "Theo dõi, đánh giá và lên kế hoạch xử lý các khoản nợ kỹ thuật (Tech Debt), những giới hạn kiến trúc chưa được giải quyết và các điểm không nhất quán trong hệ thống TeraChat."
```

---

## 1. Mục đích của Registry này

File này là **Single Source of Truth** cho mọi nợ kỹ thuật của dự án TeraChat.
Một "Tech Debt" trong TeraChat không chỉ là "code bẩn", mà bao gồm:

- Các quyết định kiến trúc tạm thời cần tối ưu (Architectural drift).
- Mâu thuẫn giữa tài liệu Spec và code thực tế (Documentation drift).
- Những workaround rủi ro cao chưa có giải pháp triệt để.

### Phân loại Mức độ (Severity)

- 🔴 **CRITICAL:** Đe dọa bảo mật Zero-Knowledge, gây sai lệch dữ liệu, hoặc crash toàn hệ thống. Cần xử lý trong Sprint hiện tại.
- 🟠 **HIGH:** Ảnh hưởng nghiêm trọng đến hiệu năng hoặc vi phạm Invariant của kiến trúc (VD: Leak bộ nhớ, race conditions hẹp).
- 🟡 **MEDIUM:** UX kém, API thiếu nhất quán, hoặc cảnh báo từ thư viện bên thứ 3. Có thể đưa vào backlog.

---

## 2. Technical Debt Registry (Active)

| ID | Domain | Title & Description | Severity | Status | Blocking | Mitigation / Migration Plan |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **TD-001** | `TERA-RUNTIME` | **Host ABI: MessagePack Contract chưa được Versioned** <br> *Mô tả:* Các function signatures trong Ecosystem dùng raw `*const u8` pointer nhưng thiếu schema version wrapper để đàm phán phiên bản. Nếu Host ABI có major update, không có cách nào phát hiện tapp cũ không tương thích. | 🟠 HIGH | Tồn đọng | .tapp Marketplace launch | Thêm `schema_version` vào header của mọi IPC MessagePack payload. Viết lại FFI layer để tự động reject version mismatch. |
| **TD-002** | `TERA-CLIENT` | **WidgetDataState được tính toán hai lần (UI & Core)** <br> *Mô tả:* Rust Core đang tính toán State cho WidgetData, nhưng ở Flutter Layer lại đang có dấu hiệu tái tạo State Machine này (NeverLoaded, Restoring...). Gây rủi ro conflict State. | 🟡 MEDIUM | Tồn đọng | — | Làm rõ lại Spec ranh giới render: Rust Core là nguồn chân lý duy nhất. Gỡ bỏ logic tính toán State khỏi Dart layer, chỉ giữ lại UI rendering logic (Passive Client). |
| **TD-003** | `TERA-RUNTIME` | **WASM Dual-Engine: Phân mảnh wasm3 vs wasmtime** <br> *Mô tả:* `wasm3` (Interpreter trên iOS) chậm hơn `wasmtime` JIT từ 10–100 lần. App Suite .tapps nặng tính toán (BankFeeds reconciliation, Finance aggregation) có thể vượt 30s background tick timeout trên iOS trong khi Desktop qua mặt dễ dàng. WasmParity CI gate hiện chỉ kiểm tra semantic equivalence, không kiểm tra resource limits theo từng engine. | 🟠 HIGH | Tồn đọng | iOS App Store launch | Bổ sung Gas/Fuel metering vào Host ABI: cấp `instruction_fuel` cố định per .tapp thay vì timeout theo giây. Đảm bảo Deterministic trên cả hai engine. Xem Combo Giải pháp §3 bên dưới. |
| **TD-004** | `TERA-RUNTIME` | **First-Party .tapp ABI Versioning Overhead** <br> *Mô tả:* TERA-RUNTIME định nghĩa 12-month deprecation window cho Host ABI versioning — thiết kế cho third-party publishers. App Suite first-party (TeraChat Finance, HRM, BankFeeds) sẽ được cập nhật đồng bộ với Rust Core. Không có exemption path cho first-party tapps trong spec. | 🟡 MEDIUM | Tồn đọng | — | Thêm `"first_party": true` flag vào Manifest. Native tapps với flag này được phép skip ABI version negotiation và giả định luôn latest. CI kiểm tra consistency. |
| **TD-005** | `TERA-ENCLAVE` | **ZK Memory Agent IPC Contract Chưa Được Đặc Tả** <br> *Mô tả:* ZK Memory Agent sử dụng "anonymous pipe IPC giữa Rust Core daemon và MLX inference server" nhưng TERA-ENCLAVE §1 chỉ mô tả PII redaction constraints. Không có spec nào (TERA-CORE, TERA-ENCLAVE, TERA-SYNC) xác lập ownership của ZK Memory Agent IPC contract. Consolidation trigger (batch vs continuous) cũng chưa được định nghĩa. | 🟠 HIGH | Tồn đọng | AI feature launch | Tạo `Spec-ZK-Memory-Agent.md` dưới TERA-ENCLAVE ownership. Nội dung tối thiểu: pipe IPC frame format, backpressure mechanism, Consolidation trigger (thời gian: 2 AM local OR 80% NAS quota), NAS storage quota enforcement, failure model khi MLX unavailable. |

---

## 3. Cross-Platform Known Limitations

> Những constraint này **không thể giải quyết** hoàn toàn. Chúng phải được disclosed trong pricing/feature documentation.

| ID | Platform | Limitation | Impact | Status |
| :--- | :--- | :--- | :--- | :--- |
| **XPLAT-01** | iOS | iOS W^X forces `wasm3` interpreter — 15–20ms/call latency penalty. .tapps with heavy computation will exceed 30s background tick timeout. | .tapp performance tier mismatch vs Desktop | Documented in §2.3 Platform Matrix |
| **XPLAT-02** | iOS | AWDL disabled when iOS acts as Personal Hotspot. BLE only (~4.75kbps) makes Mesh coordinator role meaningless. | EMDP Mesh degraded khi một iOS device chia sẻ mạng | No workaround — document in EMDP user guide |
| **XPLAT-03** | Huawei | HMS Push không hỗ trợ `data-only` message type. Enterprise SCIM < 30s SLA không thể đảm bảo (4h polling). Gov/Military tier impossible. | **Pricing_Packages.html phải disclose rõ ràng** | Partially resolved (TERA-ECO §7) — pricing doc chưa cập nhật |
| **XPLAT-04** | Linux | `mlock()` refusal: correct behavior defined (exit code 78, syslog) nhưng Admin Console alert chưa được implement. | Air-gapped Gov deployment sẽ thấy lỗi không rõ ràng | Partially resolved (TERA-CORE §11.5) |
| **XPLAT-05** | Linux Desktop | Tauri WebView (GTK WebKitGTK): `SharedArrayBuffer` requires `COOP: same-origin` + `COEP: require-corp` headers. Rust Core local HTTP server phải set headers hoặc zero-copy Data Plane silently degrade về JSON serialization. | **Không được mention trong bất kỳ spec nào** | ⚠️ Unresolved — cần add vào TERA-CLIENT |
| **XPLAT-06** | Windows | EV Code Signing cần hardware token (USB HSM). CI pipeline trên cloud agents không hold được EV token. Cần dedicated self-hosted runner. | Build pipeline cho signed Windows binary chưa có CI solution | No solution documented |
| **XPLAT-07** | HarmonyOS | `.waot` bundle — AOT compilation tạo device-specific native code. Cross-device AOT portability không được Huawei document đảm bảo. | WasmParity CI cần validate cả JIT và AOT paths | Flagged in spec, no resolution |

---

## 4. Critical Gaps (Deploy Blockers — cần verify trước Gov contract)

| ID | Gap | Spec Reference | Required Action |
| :--- | :--- | :--- | :--- |
| **GAP-A** | `SagaRecoveryGuard` chưa được define — không có `integrity_check` routine lúc boot scan orphaned `CrdtCommitted` Sagas. Nested failure (Compensating Tombstone cũng fail) chưa có recovery path. | TERA-SYNC §8.4 | Add `SagaRecoveryGuard` chạy trước khi IPC channels mở. Áp dụng WAL savepoint cho compensating event. Chaos test: kill 1000 lần tại `CrdtCommitted`. |
| **GAP-B** | WAL lock handshake signals (`WAL_FLUSH_ACK`, `WAL_LOCK_GRANTED`) không có trong CoreSignal enum (TERA-CLIENT §6.2). Resolution tồn tại trên paper nhưng không có implementation contract. | TERA-SYNC §8.5, TERA-CLIENT §4.3 | Thêm `WAL_LOCK_REQUEST`, `WAL_FLUSH_ACK`, `WAL_LOCK_GRANTED` vào CoreSignal. 5s timeout + exponential backoff. Desktop fallback: read-only mode nếu không nhận ACK. |
| **GAP-C** | NSE Shared Keychain Semaphore có TOCTOU race condition. `nse_staging.db` là SQLite bị multi-process concurrent write — tái tạo vấn đề mà Saga pattern đang giải quyết. | TERA-CORE §11.3 | Thay Keychain semaphore bằng POSIX `flock()` trên App Group container. NSE fallback: memory-mapped ring buffer (không phải SQLite). |
| **GAP-D** | `CoreSignal::MemoryPressureWarning` không carry `ui_emergency_mode: bool` flag. GPU Tier C không được force trước khi render SECURE MEMORY PURGE overlay. | Design.md §8/§12 | Add `ui_emergency_mode: bool` vào `MemoryPressureWarning` signal. UI phải downgrade GPU Tier C trước overlay render (synchronous, priority channel). |
| **GAP-E** | DataGrant `generation` counter: node offline khi grant issued không thể distinguish "grant never seen" (gen 0) vs "revoked grant" (gen > 0). Gov-tier quorum protocol undefined. | TERA-ECO §8.3, §2.5 | Define quorum: majority của `election_weight > 0` nodes xác nhận DataGrant activation via `Hash_Frontier` gossip. Gov-tier unconfirmed grants return `PENDING_QUORUM`, không serve data. |
| **GAP-F** | Huawei Enterprise tier "CRL ≤ 4h polling" vi phạm SCIM < 30s SLA — không được disclose trong `Pricing_Packages.html`. | TERA-ECO §7 | Update pricing/feature docs để explicitly disclose Huawei SLA limitation. Huawei không nằm trong Enterprise SLA tier. |
| **GAP-G** | Burner Agent (FUNC-13, TTL=60min) khi expire triggering MLS Epoch Rotation trong lúc EMDP Epoch Freeze active → Burner Agent trở thành "zombie member" với keys vẫn valid past TTL. | TERA-CORE §4.3, FUNC-13, TERA-CORE §11.4 | Thêm SC-36 vào TestMatrix: Burner Agent TTL + EMDP Epoch Freeze intersection. Spec resolution: khi EMDP Epoch Freeze active, Burner Agent removal được queue và epoch advances NGAY SAU khi Freeze terminated. |
| **GAP-H** | Float detection không phải required check trong Static Analysis pipeline (TERA-ECO §4.1). Finance .tapp dùng `f64` vẫn pass security review. | TERA-RUNTIME §11.3, TERA-ECO §4.1 | Add explicit float detection vào LLVM IR analysis checklist trong TERA-ECO §4.1. Block merge nếu `f32`/`f64` xuất hiện trong financial .tapp. |
| **GAP-I** | Binary Transparency gossip message không cần được signed — chỉ original binary được sign. Insider có thể broadcast fake `Global_Update_Log` hash. | TERA-CORE §4.5 | Gossip message mang BLAKE3 hash phải được signed bằng TeraChat Root CA key. Peers reject unsigned gossip messages. |
| **GAP-J** | `Outbox Queue TTL` trong `PENDING_SECURE_CHANNEL` state chưa được define. Nếu Key Escrow không complete trong 24h (SC-35 với không có Desktop reconnect), messages silently expire hay user được notify? | TERA-CLIENT §11.5 | Define: Outbox Queue TTL = 24h. Sau TTL, UI hiển thị "Messages could not be sent securely — please reconnect to a secure channel." Enterprise contracts phải document explicit delivery semantics. |

---

## 5. Refactoring Architecture Decisions (Pending Implementation)

### 5.1 Rust-Core + Headless Daemon (CRIT-01, TD-003)

Tách hoàn toàn khối Mạng (Mesh/MLS) và Lưu trữ (SQLite) ra khỏi ứng dụng UI:
- **Android/Oppo/Xiaomi:** Chạy Rust Core dưới dạng **Foreground Service** native (kèm persistent notification) để OEM battery management KHÔNG THỂ kill. Flutter App kết nối via gRPC cục bộ (Local UDS/Sockets).
- **Desktop:** Chạy Rust Core dưới dạng Windows Service / systemd daemon. Tauri App là pure Client.
- **Kết quả:** UI có thể crash, bị kill, cập nhật — tiến trình mật mã và Mesh BLE vẫn sống sót độc lập.

### 5.2 Hermetic Build Matrix (CRIT-03 / ISO 27001 A.8.4)

- Toàn bộ quy trình đóng gói cho Windows, Linux, iOS, Android phải chạy trong Docker containers đóng băng dependencies (NixOS-based builder).
- Sinh SBOM + HSM-signed cryptographic signature cho mọi artifacts đầu ra.
- Windows EV signing: dedicated self-hosted runner với physical HSM token.

### 5.3 Gas/Fuel Metering cho WASM (TD-003 / XPLAT-01)

Thay vì timeout theo giây (thiên vị phần cứng mạnh), cấp `instruction_fuel` cố định per .tapp. Khi hết Fuel, .tapp bị buộc dừng — Deterministic tuyệt đối trên cả `wasm3` (iOS) và `wasmtime` (Desktop).

### 5.4 CoreBootSequence Protocol (GAP-A, GAP-B, GAP-C)

Mỗi Rust Core startup, trước khi mở IPC channels, chạy tuần tự:
1. `SagaRecoveryGuard` — scan orphaned `CrdtCommitted` Sagas.
2. `NseRingBufferDrain` — drain nse ring buffer via flock-protected drain.
3. `WalIntegrityCheck` — `PRAGMA integrity_check` trên cả 2 databases.

Mỗi guard < 100ms, tổng overhead < 300ms — chấp nhận được cho enterprise startup.

---

## 6. Tech Debt Template (Dành cho Kỹ sư)

Khi phát hiện hoặc quyết định trade-off một vấn đề kỹ thuật, mở Pull Request để bổ sung vào bảng §2 theo format:

```markdown
| **TD-XXX** | `TERA-[DOMAIN]` | **[Tên ngắn gọn]** <br> *Mô tả:* [Rủi ro, hệ lụy, nguyên nhân] | [Mức độ] | [Tồn đọng/Đang xử lý] | [Contract milestone bị block] | [Hướng giải quyết / Jira Ticket] |
```

---

## 7. Lịch sử Xử lý (Resolved Debts)

*Khu vực lưu trữ các khoản nợ kỹ thuật đã được thanh toán để phục vụ Audit Trail.*

| ID | Title | Resolved In Version | Notes |
| :--- | :--- | :--- | :--- |
| **TD-000** | Blind RAG Documentation Drift | `v0.5.0` | Đã xóa toàn bộ reference tới Blind RAG, thống nhất dùng thuật ngữ ZK Memory Agent. TERA-SYNC §3.4, §4.5, §8.6 đã cập nhật. |

---

_TERA-DEBT v1.0.0 · 2026-04-11 · Khởi tạo từ Deep Technical Audit Report_
