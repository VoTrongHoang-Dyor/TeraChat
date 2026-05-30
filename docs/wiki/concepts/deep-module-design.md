---
type: concept
created: 2026-05-15
updated: 2026-05-15
tags: [design, architecture, rust, matt-pocock]
---

# Deep Module Design

Applied from Matt Pocock's principle: **simple interfaces, complex interiors.**

## The Principle

> "The best modules are those whose interfaces are much simpler than their implementations." — John Ousterhout, *A Philosophy of Software Design*

Translated for TeraChat Rust codebase:

| Aspect | Deep Module | Shallow Module |
|--------|-------------|----------------|
| Public API | ≤ 5 items | > 7 items |
| Interior | Complex, hidden | Exposed to caller |
| Cognitive load | Low (caller sees simple interface) | High (caller must understand internals) |
| Change safety | High (interior changes don't break callers) | Low (callers depend on internals) |
| AI agent friendly | Yes (clear contract) | No (ambiguous boundaries) |

## CI Enforcement

```rust
// CI gate: public items per module ≤ 7
// Violations trigger refactor requirement

fn check_module_depth(crate_path: &Path) -> Result<()> {
    for file in rust_files(crate_path) {
        let public_items = count_public_items(&file);
        if public_items > 7 {
            return Err(anyhow!(
                "{}: exposes {} public items (max 7). \
                 Refactor into sub-modules with deep interfaces.",
                file.display(),
                public_items
            ));
        }
    }
    Ok(())
}
```

## Example: InferenceGateway

```rust
// PUBLIC INTERFACE — simple, stable, AI agents can use easily
// Only 3 methods exposed. All complexity hidden.
pub trait InferenceGateway: Send + Sync {
    async fn complete(&self, request: InferenceRequest) 
        -> Result<InferenceResponse, InferenceError>;
    
    async fn stream(&self, request: InferenceRequest)
        -> Result<InferenceStream, InferenceError>;
    
    fn health(&self) -> GatewayHealth;
}

// INTERIOR — complex but completely hidden from callers
pub struct TeraInferenceGateway {
    scheduler: Arc<InferenceScheduler>,     // decides where to run
    thermal: Arc<ThermalMonitor>,           // polls OS thermal state
    pii_gate: Arc<PiiRedactionGate>,        // strips PII before inference
    // ... hidden complexity
}

impl InferenceGateway for TeraInferenceGateway {
    async fn complete(&self, request: InferenceRequest) 
        -> Result<InferenceResponse, InferenceError> 
    {
        // 1. PII redaction — mandatory, cannot bypass
        let sanitized = self.pii_gate.redact(request)?;
        
        // 2. Thermal check
        if self.thermal.is_critical() {
            return Err(InferenceError::ThermalThrottle {
                retry_after: self.thermal.estimated_recovery(),
            });
        }
        
        // 3. Schedule to best endpoint
        let target = self.scheduler.decide(&sanitized);
        target.execute(sanitized).await
    }
}
```

Callers see only `complete()`, `stream()`, `health()`. They don't know about thermal monitoring or scheduling algorithms.

## Example: MeshBuffer

```rust
// Simple interface
pub struct MeshBuffer { /* ... */ }

impl MeshBuffer {
    pub fn new(tier: BufferTier) -> Self;
    pub fn store(&mut self, msg: Message) -> Result<MessageId>;
    pub fn retrieve(&self, id: MessageId) -> Option<&Message>;
    pub fn total_bytes(&self) -> u64;
    pub fn health(&self) -> BufferHealth;
}
// 5 public methods. Interior: tier detection, LRU eviction,
// priority protection, content filtering — all hidden.
```

## Example: TappSDK

```rust
// Contributors only need to know this trait
pub trait Tapp {
    fn on_start(&mut self, ctx: &TappContext) -> Result<()>;
    fn on_action(&mut self, action: Action, ctx: &TappContext) 
        -> Result<ActionResult>;
    fn on_tick(&mut self, ctx: &TappContext) -> Result<()> {
        Ok(()) // default no-op
    }
}
// 3 methods. Interior: WASM engine, fuel metering,
// Host ABI, sandbox isolation — all hidden.
```

## Anti-Pattern: Shallow Module

```rust
// ❌ WRONG: Exposes everything
pub struct MeshBuffer {
    pub tier: BufferTier,
    pub storage: MeshStorage,
    pub eviction: EvictionPolicy,
    pub config: MeshConfig,
}
// Caller must understand ALL fields to use correctly.
// Changes to any field break all callers.
```

## When to Split a Module

If a module exceeds 7 public items:
1. Identify a sub-concept that can stand alone
2. Extract it into a sub-module with its own deep interface
3. The parent module delegates to the sub-module internally

```rust
// Instead of 15 methods on MeshBuffer:
mod mesh_buffer {
    mod storage;      // 4 public items
    mod eviction;     // 3 public items  
    mod tier;         // 3 public items
}

// Each sub-module is deep. MeshBuffer facade delegates internally.
```

## Related Pages

- [[Multi-Agent Harness]] — How deep modules enable effective AI agents
- [[Tapp Community Framework]] — Deep modules applied to .tapp SDK
- [[Ubiquitous Language]] — Shared vocabulary for naming deep modules
