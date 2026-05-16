# Contributing to TeraChat

Build .tapp WASM mini-apps for the TeraChat Work OS.

## Ubiquitous Language

Before coding, learn these core concepts:

| Term | Definition | NOT |
|------|-----------|-----|
| .tapp | WASM mini-app running in TeraChat sandbox | Plugin, Extension |
| TappContext | Read-only view of Core state injected into .tapp | State, Props |
| Host ABI | Contract between .tapp WASM and Rust Core | API, SDK |
| Fuel | Instruction budget — deterministic resource limit | Timeout |
| DataGrant | Cryptographic permission to read data | Permission, Role |
| CoreSignal | Event from Rust Core pushed to .tapp | Webhook, Callback |

See `docs/wiki/ubiquitous-language.md` for the full vocabulary.

## TDD Workflow (Required)

All .tapp contributions follow Test-Driven Development:

1. **Write the test** — describe the behavior you want
2. **Run the test** → FAIL (expected)
3. **Implement minimum code** to make it pass
4. **Refactor** if needed
5. **Submit PR** with test + implementation

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

## Validation

Run the validator locally before submitting:

```bash
# Install validator
cargo install terachat-tapp-validator

# Validate your .tapp
terachat-tapp validate ./my-tapp.wasm
```

The validator checks:
- Ed25519 signature validity
- manifest.json completeness
- Fuel budget within limits
- No f32/f64 in financial code
- wasm3 ≡ wasmtime output parity
- Memory bound (< 50MB peak)
- Network egress schema compliance

## PR Checklist

- [ ] Tests pass: `cargo test`
- [ ] Validator passes: `terachat-tapp validate ./my-tapp.wasm`
- [ ] No `unwrap()` in .tapp code
- [ ] Fuel budget declared in manifest
- [ ] Follows ubiquitous language conventions
- [ ] README or inline docs explain the .tapp purpose

## Directory Structure

```
my-tapp/
├── Cargo.toml
├── manifest.json          # .tapp metadata + fuel budget
├── src/
│   └── lib.rs             # Tapp trait implementation
└── tests/
    └── integration_test.rs
```

## manifest.json

```json
{
  "name": "my-tapp",
  "version": "0.1.0",
  "publisher": "your-org-id",
  "computation": {
    "instruction_fuel": 10000000
  },
  "permissions": {
    "storage": true,
    "events": false,
    "crypto": false,
    "ai": false
  }
}
```

## Getting Help

- Architecture: `docs/wiki/concepts/tapp-community-framework.md`
- SDK docs: `terachat-tapp-sdk/`
- Examples: `terachat-tapp-sdk/examples/`
- Spec: `docs/raw/MD/Spec-Wasm-Tapp-Runtime.md`
