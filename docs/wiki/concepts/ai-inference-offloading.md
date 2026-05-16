---
type: concept
created: 2026-05-15
updated: 2026-05-15
tags: [ai, inference, thermal, rust, device]
---

# AI Inference Offloading

Distributed inference with Rust thermal/RAM management. Dynamically routes AI requests to the best endpoint based on device capability, thermal state, and network availability.

## Problem

```
iPhone 15 Pro: 8GB RAM, A17 Pro Neural Engine
User active: MLS E2EE (crypto) + BLE mesh (networking) + UI rendering
Available for AI: ~1.5-2GB

Gemma 2B: needs ~2.4GB → OOM
Qwen2.5-0.5B: needs ~600MB → acceptable, low quality
Qwen2.5-1.5B: needs ~1.8GB → borderline, thermal throttle risk
```

## Architecture

```
┌─────────────────────────────────────────────────┐
│              InferenceGateway                    │
│                                                  │
│  complete(request) → Response                    │
│  stream(request) → Stream                        │
│  health() → GatewayHealth                        │
│                                                  │
│  ┌────────────────────────────────────────────┐ │
│  │         InferenceScheduler                  │ │
│  │                                              │ │
│  │  Decision Engine:                            │ │
│  │  ┌────────┐ ┌────────┐ ┌────────┐ ┌──────┐ │ │
│  │  │ Device │ │Mac mini│ │Cluster │ │Reject│ │ │
│  │  │ NPU    │ │ Local  │ │Primary │ │(OOM) │ │ │
│  │  │ ≤500tok│ │ ≤4096  │ │≤32768  │ │      │ │ │
│  │  └────────┘ └────────┘ └────────┘ └──────┘ │ │
│  └────────────────────────────────────────────┘ │
│                                                  │
│  PiiRedactionGate → ThermalMonitor              │
└─────────────────────────────────────────────────┘
```

## ThermalMonitor

Background monitor polling OS thermal state and RAM pressure. Consumers only call `is_critical()` — interior complexity hidden.

```rust
pub struct ThermalMonitor { /* ... */ }

impl ThermalMonitor {
    pub fn spawn(config: ThermalConfig) -> Arc<Self>;

    /// Non-blocking — called on hot path
    pub fn is_critical(&self) -> bool;

    /// Returns estimated recovery time in seconds
    pub fn estimated_recovery(&self) -> Duration;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalState {
    Nominal,                                  // All operations allowed
    Fair { throttle_factor: f32 },            // Reduce batch size
    Serious,                                  // Only ≤ 256 tokens, suspend mesh sync
    Critical,                                 // No inference, only E2EE messaging
}
```

### Platform-Specific Polling

```rust
#[cfg(target_os = "ios")]
fn poll_os_thermal() -> RawThermalLevel {
    // FFI to Swift ProcessInfo.thermalState
    unsafe { ios_get_thermal_state() }.into()
}

#[cfg(target_os = "macos")]
fn poll_os_thermal() -> RawThermalLevel {
    // IOKit thermal sensors
    iokit_read_thermal_sensors()
}
```

## InferenceScheduler

Decision tree — explicit, testable, no magic:

```rust
impl InferenceScheduler {
    pub fn decide(&self, req: &SanitizedRequest) -> Arc<dyn InferenceEndpoint> {
        match (self.thermal.state(), self.network.state(), req.estimated_tokens()) {
            // Critical: reject non-essential inference
            (ThermalState::Critical, _, _) => self.null_endpoint(),

            // Device NPU: small, fast, private — highest priority
            (_, _, t) if t <= 512 => self.device_npu_endpoint(),

            // Mac mini local: medium complexity, on network
            (_, NetworkState::Connected, t) if t <= 4096 => self.mac_mini_endpoint(),

            // Cluster: complex requests, good network
            (_, NetworkState::Connected, t) if t <= 32768 => self.cluster_endpoint(),

            // Offline fallback: smallest model on device
            (_, NetworkState::Offline, _) => self.offline_fallback_endpoint(),

            // Too large: reject with clear message
            _ => self.rejection_endpoint(RejectionReason::TooLarge),
        }
    }
}
```

## Model Tiers

| Tier | Model | RAM | Targets |
|------|-------|-----|---------|
| Tiny | Qwen2.5-0.5B (~600MB) | 4GB+ | iPhone 12, old Android |
| Small | Qwen2.5-1.5B (~1.8GB) | 8GB+ | iPhone 15 Pro, high-end Android |
| Medium | Qwen2.5-7B (~5GB) | 32GB+ | Mac mini M2 |
| Large | Qwen2.5-14B (~10GB) | 48GB+ | Mac mini M4 Pro |
| XLarge | Qwen2.5-32B (~24GB) | 64GB+ | Mac mini M4 Max cluster |

## PII Redaction

Always runs BEFORE any inference. Simple, fast, auditable regex patterns:

```rust
pub struct PiiRedactor {
    patterns: Vec<(Regex, &'static str)>,
}

impl PiiRedactor {
    pub fn new_for_region(region: Region) -> Self { /* ... */ }

    pub fn redact(&self, text: &str) -> String {
        // Replace CCCD (VN national ID): 9-12 digits
        // Replace phone: 10 digits
        // Replace email
        // Replace credit card patterns
        // Extensible per vertical (healthcare, legal, finance)
    }
}
```

Upgrade to ONNX NER after collecting real user data for training.

## Why Not Gemma 4?

Gemma 4 doesn't have stable MLX export yet. Qwen2.5 and Gemma 2 have production-ready MLX format, running on Apple Silicon today. Swap when Gemma 4 MLX is stable.

## Related Pages

- [[Mac Mini HA Cluster]] — The cluster inference tier
- [[Secure Enclave & AI Security]] — PII redaction details
- [[Open AI Framework]] — Multi-model support
- [[Deep Module Design]] — Why InferenceGateway has only 3 public methods
