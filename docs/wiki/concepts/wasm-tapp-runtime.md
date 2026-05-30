---
type: concept
created: 2026-05-10
updated: 2026-05-30
tags: [terachat, wasm, tapp, sandbox, runtime, trust-tier, work-os, marketplace, self-service]
sources: [tera-runtime-spec, tera-eco-spec, tera-core-spec]
---

# WASM Tapp Runtime & Work OS Marketplace

The dual-engine WebAssembly sandbox for executing enterprise mini-applications (.tapp) within TeraChat. T-apps transform TeraChat from a messaging platform into a **Work OS** — employees run business tasks directly inside the application. T-apps are distributed through the TeraChat Web Marketplace with a self-service model: businesses browse vetted t-apps, **purchase them on the web**, download them, and set them up following simple instructions. **All .tapp payment processing happens on terachat.io — the app only downloads, verifies, and runs .tapps.**

## Work OS Vision

```
┌───────────────────────────────────────────────────────┐
│                   TERACHAT WORK OS                     │
│                                                       │
│  ┌─────────┐ ┌──────────┐ ┌────────┐ ┌────────────┐  │
│  │ Finance │ │    HR    │ │ Project│ │ Supply Chain│  │
│  │ .tapp   │ │  .tapp   │ │ .tapp  │ │   .tapp    │  │
│  │(invoice,│ │(onboard, │ │(gantt, │ │(inventory, │  │
│  │ expense)│ │ time-off)│ │ sprint)│ │ ordering)  │  │
│  └─────────┘ └──────────┘ └────────┘ └────────────┘  │
│                                                       │
│  ┌────────────────────────────────────────────────┐   │
│  │           TERACHAT WEB MARKETPLACE              │   │
│  │  Browse → Select → Download → Self-Setup        │   │
│  │  All t-apps vetted & declared by TeraChat       │   │
│  └────────────────────────────────────────────────┘   │
│                                                       │
│  Deployment Scope: By Region · By Department          │
└───────────────────────────────────────────────────────┘
```

## Self-Service T-App Model

### Lifecycle

```
1. IT Admin visits terachat.io → browses Web Marketplace
2. Selects t-apps relevant to their business
3. Purchases t-app on website (payment via web, not app)
4. Downloads t-app package (.tapp bundle, signed)
5. Follows simple setup instructions (no DevOps needed)
6. Deploys to specific REGIONS or DEPARTMENTS via Admin Console
7. Employees in scope see the t-app in their workspace
8. T-app runs in WASM sandbox with Host ABI access
```

### Deployment Scoping

T-apps are not enterprise-wide by default. They are opened by:

| Scope | Example | Control |
|-------|---------|---------|
| **By Region** | "Inventory .tapp only for APAC branches" | Region-level Admin toggle |
| **By Department** | "Expense .tapp only for Finance department" | Department-level role gate |
| **By Branch** | "Local compliance .tapp for Branch B only" | Branch-level deployment |
| **By Role** | "Approval .tapp only for Managers+" | Role-based access (OPA policy) |

This prevents t-app sprawl — employees only see t-apps relevant to their work.

## Dual Engine Architecture

| Engine | Platform | Type | Performance |
|--------|----------|------|-------------|
| `wasmtime` | Android, Desktop, Huawei | JIT compiler | Fast (native speed) |
| `wasm3` | iOS | Interpreter | 10-100x slower, +15-20ms/call latency |

**Reason:** iOS enforces W^X (write-xor-execute) — memory cannot be both writable and executable. This blocks all JIT compilers. `wasm3` interprets WASM bytecode without generating native code.

## Trust Tier Model & Resource Limits

> **Cập nhật 2026-05-30:** Thay thế flat limits cũ (64MB RAM, 2MB Outbox) bằng **Trust Tier Model** phân cấp theo mức độ tin cậy của .tapp.

### Tier 0 — System/Crypto (First-Party Only)

Chỉ dành cho Core Modules hệ thống nội bộ TeraChat. Không có third-party .tapp nào đạt Tier 0.

| Resource | Limit |
|----------|-------|
| RAM | **50MB hard kill** |
| Float arithmetic | **Cấm hoàn toàn** (`f32`/`f64`) |
| Network egress | **Không có** |
| Language | Rust tĩnh (biên dịch sang WASM) |
| Background | 10MB, fuel metering |

### Tier 1 — Enterprise Standard (Third-Party + Internal)

Dành cho .tapp Marketplace và Enterprise Side-loading. Linh hoạt hơn để hỗ trợ đa dạng developer.

| Resource | Mobile | Desktop | Background |
|----------|--------|---------|------------|
| RAM | **256MB** hard kill | **512MB** hard kill | 10MB (OS suspend) |
| Float math | ✅ Được phép | ✅ Được phép | — |
| Egress | Declarative Proxy | Declarative Proxy | Không có network I/O |
| Language | JS/TS qua Javy/Extism, Rust, Go | JS/TS qua Javy/Extism, Rust, Go | — |
| OOM behavior | Kill .tapp, show "Reload Plugin" | Kill .tapp, show "Reload Plugin" | — |

**OOM Isolation:** Nếu .tapp Tier 1 vượt RAM limit, TeraChat kill tiến trình .tapp và hiển thị "Reload Plugin" — tuyệt đối không ảnh hưởng ứng dụng chat chính.

## Marketplace Vetting

Before a t-app appears on the TeraChat Web Marketplace, TeraChat:

1. **Audits the source** (or .wasm binary for third-party)
2. **Verifies capability declarations** — no undeclared permissions
3. **Runs security scan** — float detection for financial t-apps, memory analysis
4. **Signs with TeraChat Root CA** — Ed25519 signature
5. **Publishes with setup instructions** — step-by-step, no DevOps assumed
6. **Records in transparency log** — Merkle leaf for every published version

Third-party t-app publishers follow the same pipeline. The 30% publisher revenue share applies — all revenue collection and publisher payouts are processed on terachat.io, never in the app.

## Host ABI

The set of functions Rust Core exposes to .tapp WASM modules:

- **Storage** (get/set/delete, scoped to .tapp + region/department)
- **Crypto** (encrypt, sign, verify — delegated)
- **Network** (proxied egress with OPA check)
- **Event Bus** (publish/subscribe to local events)
- **AI Inference** (host_ai_invoke — through Open AI Framework)

## Security Boundaries

- **No direct network access** (Tier 0): `wasi-sockets` stripped. Không có egress.
- **Declarative Proxy** (Tier 1): .tapp gọi `host.fetch(url)` — Core đối chiếu với domain allowlist trong manifest, push qua DLP proxy của enterprise trước khi ra internet.
- **Manifest-declared permissions:** Mọi capability khai báo tại install time, không có runtime escalation.
- **Crypto delegation:** WASM code không tự thực hiện crypto. Phải gọi Host ABI → Rust Core (hardware-backed keys).
- **Region/department sandbox:** .tapp không truy cập data ngoài scope triển khai.
- **Fuel metering (thay vì timeout):** `instruction_fuel` bất kể engine/hardware — deterministit tuyệt đối.

## Egress Model: Declarative Proxy

Thay thế 2MB hard Outbox limit — model mới linh hoạt hơn cho enterprise use cases:

```yaml
# manifest.json (.tapp Tier 1)
capabilities:
  network_egress:
    domains:
      - "api.github.com"
      - "hr-service.company.internal"
    max_body_bytes: 10485760    # 10MB per request
    dlp_required: true          # Enterprise: qua DLP proxy trước khi ra ngoài
```

```rust
// .tapp gọi qua Host ABI — không gọi network trực tiếp
extern "C" {
    fn host_fetch(
        url_ptr: *const u8, url_len: usize,
        method_ptr: *const u8, method_len: usize,
        body_ptr: *const u8, body_len: usize,
    ) -> FetchHandle;  // async, non-blocking
}
```

**Kiểm soát:** Core đối chiếu URL với domain allowlist manifest. ở Enterprise tier, request tiếp tục qua hệ thống DLP proxy của công ty trước khi ra internet. .tapp không có callback ngược (data diode model).

## 🧠 Design Decisions (Q&A)

- **Tại sao Trust Tier thay vì flat limits?** → Một chính sách bảo mật cào bằng (one-size-fits-all) tự bóp nghẹt hệ sinh thái. Core Modules hệ thống cần giới hạn khắt khe (50MB, không float, không network). Enterprise .tapp cần linh hoạt hơn (256MB, float OK, declarative egress). Trade-off: phức tạp hơn trong implementation và review.

- **Tại sao self-service từ Web Marketplace?** → IT Admin quản lý procurement trên web. Department head có thể browse, purchase, setup .tapp không cần mở ticket. Payment luôn trên terachat.io, app chỉ download và run.

- **Tại sao regional/departmental scoping?** → Finance .tapp không xuất hiện trên màn hình công nhân nhà máy. Scoping giảm cognitive load và enforce need-to-know.

- **Tại sao hai engine (wasmtime + wasm3)?** → wasm3 là 10-100x chậm hơn. Finance .tapps làm reconciliation sẽ timeout trên iOS nếu chỉ dùng interpreter. Trade-off: phải maintain WasmParity CI gate.

- **Tại sao Fuel Metering thay vì Timeout?** → Timeout thiên vị hardware mạnh — .tapp pass trên Desktop có thể exceed 30s timeout trên iOS. `instruction_fuel` là deterministic: cùng fuel = cùng limit bất kể hardware. Trade-off: ABI phức tạp hơn.

- **Tại sao cho phép JS/TS (Javy/Extism) ở Tier 1?** → Mở rộng developer pool đáng kể. Phần lớn enterprise developer biết JS/TS. Lập trình viên không phải học Rust chỉ để build .tapp. OOM Isolation đảm bảo nếu .tapp leak memory thì chỉ kill .tapp, không kill chat app chính.
