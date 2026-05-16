---
type: concept
created: 2026-05-15
updated: 2026-05-15
tags: [tapp, wasm, community, sdk, contributors]
---

# .tapp Community Framework

Framework for third-party developers to build WASM mini-apps inside TeraChat. Designed for community contribution with deep modules and mandatory TDD.

## SDK Structure

```
terachat-tapp-sdk/
├── CONTRIBUTING.md          # How to build a .tapp
├── examples/
│   ├── hello-world/         # Minimal .tapp (~50 lines)
│   ├── expense-approval/    # Real-world example
│   └── document-signing/    # Crypto usage example
├── sdk/
│   ├── src/lib.rs           # Public SDK — stable API
│   └── tests/
│       ├── abi_contract.rs  # Tests AI agents must pass
│       └── fuel_budget.rs   # Performance tests
└── validator/
    └── src/main.rs          # CLI: terachat-tapp validate ./my.tapp
```

## Tapp Trait — The Only Interface Contributors Need

```rust
/// Entry point for every .tapp
/// Implement this trait — that's all you need
pub trait Tapp {
    /// Called when .tapp is loaded
    fn on_start(&mut self, ctx: &TappContext) -> Result<()>;

    /// Called when user triggers an action
    fn on_action(&mut self, action: Action, ctx: &TappContext)
        -> Result<ActionResult>;

    /// Called every background_tick_interval_s seconds
    fn on_tick(&mut self, ctx: &TappContext) -> Result<()> {
        Ok(()) // Default: no-op
    }
}

/// Context injected into .tapp — read-only view of Core state
pub struct TappContext {
    storage: TappStorage,   // Key-value store, scoped per .tapp
    events: EventBus,       // Pub/sub with other tapps
    crypto: CryptoService,  // Sign/verify — no raw key access
    ai: AiService,          // AI inference — PII redacted automatically
}
```

## Hello World .tapp

```rust
use terachat_sdk::{Tapp, TappContext, Action, ActionResult};

struct HelloTapp;

impl Tapp for HelloTapp {
    fn on_start(&mut self, _ctx: &TappContext) -> Result<()> {
        println!("Hello from .tapp!");
        Ok(())
    }

    fn on_action(&mut self, action: Action, ctx: &TappContext)
        -> Result<ActionResult>
    {
        match action.name.as_str() {
            "greet" => Ok(ActionResult::message("Hello, World!")),
            _ => Ok(ActionResult::noop()),
        }
    }
}

terachat_sdk::export!(HelloTapp);
```

## TappValidator CLI

```rust
/// terachat-tapp validate ./my-tapp.wasm
///
/// Runs validation suite before Registry submission.
/// Contributors run locally. CI runs automatically.
pub struct TappValidator {
    checks: Vec<Box<dyn ValidationCheck>>,
}

impl TappValidator {
    pub fn default_suite() -> Self {
        Self {
            checks: vec![
                Box::new(SignatureCheck),      // Ed25519 valid?
                Box::new(ManifestCheck),       // manifest.json complete?
                Box::new(FuelBudgetCheck),     // Within fuel limit?
                Box::new(FloatDetectCheck),    // No f32/f64 in finance?
                Box::new(WasmParityCheck),     // wasm3 == wasmtime output?
                Box::new(MemoryBoundCheck),    // < 50MB peak RAM?
                Box::new(EgressSchemaCheck),   // Network calls valid?
            ],
        }
    }
}
```

## Host ABI (Minimal for MVP)

Only implement what the first 3 .tapps need:

```rust
extern "C" {
    fn host_storage_get(key: *const u8, key_len: usize,
                        out: *mut u8, out_max: usize) -> i32;
    fn host_storage_set(key: *const u8, key_len: usize,
                        val: *const u8, val_len: usize) -> i32;
    fn host_ed25519_sign(key_id: u64, msg: *const u8,
                         msg_len: usize, sig_out: *mut u8) -> i32;
    fn host_event_publish(event: *const u8, event_len: usize) -> i32;
}
```

Deferred: egress network, AI inference, SQLite virtual tables, cross-tapp IPC. Add after 3 first-party .tapps have real usage.

## First 3 .tapps (Slice 5)

| .tapp | Use Case | Complexity |
|-------|----------|------------|
| Expense Approval | Manager approve/reject expenses with digital signature | Medium |
| Document Signing | Multi-party Ed25519 document signing workflow | Medium — crypto |
| Task Assignment | Create, assign, track tasks with due dates | Simple |

## Contribution Flow

```
Contributor writes .tapp
    → cargo test (local)
    → terachat-tapp validate (local)
    → PR to tapp-registry repo
    → CI auto-validation
    → Human review (security + quality)
    → Published to marketplace
```

## Fuel Metering

Required from the start:

```toml
# .tapp manifest
[computation]
instruction_fuel = 10_000_000
```

Why now? Adding fuel metering after .tapps are running is a breaking change. Build it in from the start even if not strictly needed yet.

## Related Pages

- [[WASM Tapp Runtime]] — Dual-engine sandbox specification
- [[Deep Module Design]] — Why the Tapp trait has only 3 methods
- [[Ubiquitous Language]] — Terminology for .tapp developers
- [[CONTRIBUTING]] — Full contributor guide
