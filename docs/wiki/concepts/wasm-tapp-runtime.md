---
type: concept
created: 2026-05-10
modified: 2026-05-11
tags: [terachat, wasm, tapp, sandbox, runtime, work-os, marketplace, self-service]
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

## Security Boundaries

- **No direct network access:** `wasi-sockets` stripped. All I/O goes through Host ABI → Rust Core OPA policy check.
- **Crypto delegation:** WASM code cannot perform cryptographic operations. Must call Host ABI which delegates to Rust Core (hardware-backed keys).
- **Manifest-declared permissions:** All capabilities declared at install time, not runtime. No permission escalation possible.
- **Region/department sandbox:** T-app cannot access data outside its deployed scope.
- **Resource limits:** 10MB RAM max (background), 2MB Egress_Outbox hard limit, instruction_fuel metering.

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

## 🧠 Design Decisions (Q&A)

- **Why self-service from Web Marketplace instead of in-app purchase?** → The original model assumed IT Admin manages everything including procurement. Self-service on web means a department head can browse, purchase, and set up a t-app without opening a ticket — but payment still goes through the organization's procurement cycle on terachat.io. Trade-off: requires t-apps to be foolproof in setup. App never handles payment.

- **Why regional/departmental scoping?** → A finance t-app shouldn't appear on a factory worker's screen. Scoping reduces cognitive load and enforces need-to-know. Trade-off: more complex deployment UI in Admin Console.

- **Why two engines instead of just wasm3 everywhere?** → wasm3 is 10-100x slower. Finance .tapps doing reconciliation would timeout on the 30s background tick on Desktop if limited to interpreter. Trade-off: must maintain WasmParity CI gate ensuring semantic equivalence across both engines.

- **Why Gas/Fuel metering instead of timeout?** → Timeouts favor powerful hardware — a .tapp that passes on Desktop might exceed 30s on iOS. Instruction_fuel is deterministic: same fuel = same limit regardless of hardware. Trade-off: more complex ABI.
