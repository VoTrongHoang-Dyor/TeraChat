---
type: concept
created: 2026-05-15
updated: 2026-05-15
tags: [methodology, development, slices]
---

# Vertical Slice Development

## Problem

Traditional "horizontal layer" approach fails solo developers:

```
[Crypto] → [Sync] → [Runtime] → [Client] → [AI]
→ 2 years later: nothing runnable, no feedback, no revenue
```

Each layer must be complete before the next begins. Integration happens only at the end. Bugs discovered late. No demos possible until everything works.

## Solution: Vertical Slice

A vertical slice cuts through ALL layers but only implements enough for ONE concrete use case.

```diff
- Build all of Crypto, then all of Sync, then all of Runtime...
+ Build "send one E2EE message between two Macs" — end to end, shippable
```

Each slice is:
- **Shippable** — can demo to customers
- **Chargeable** — can generate revenue
- **Feedback-generating** — real usage informs next slice

## Slice 0-6 Overview

| Slice | Duration | Goal | Deliverable |
|-------|----------|------|-------------|
| **0: Foundation** | Week 1-2 | Repo compiles, CI green | Cargo workspace, proto scaffolding, CLAUDE.md |
| **1: Hello E2EE** | Week 3-8 | Two Macs send E2EE messages | MLS roundtrip test passing, no network, no UI |
| **2: Relay + Persistence** | Week 9-14 | Messages through relay binary | TeraRelay binary, event_log.db, license JWT |
| **3: macOS + iPhone UI** | Week 15-22 | Real devices chatting | Tauri macOS + Flutter iPhone, gRPC over UDS |
| **4: HA + Mesh Failover** | Week 23-30 | Clustered Mac mini + BLE fallback | Raft WAL replication, mesh emergency mode |
| **5: .tapp Runtime MVP** | Week 31-38 | 3 first-party .tapps running | WASM sandbox, Host ABI, fuel metering |
| **6: Local AI** | Week 39-46 | AI summarize | Qwen2.5 on-device, thermal management |

## Contrast with Horizontal Phase Plan

| Aspect | Horizontal Phase | Vertical Slice |
|--------|-----------------|----------------|
| Demo possible | After 12+ months | After every 6-8 weeks |
| Risk | Discover integration bugs at end | Integration tested continuously |
| Feedback | Delayed until launch | Immediate after each slice |
| Pivot cost | High (threw away layers) | Low (adjust next slice) |
| Solo dev feasibility | Low (context switching between layers) | High (focused on one use case) |

## Implementation Rule

**One slice at a time. No parallel slices.** Finish Slice N → demo → get feedback → adjust Slice N+1 based on feedback.

See [[Phase Framework]] for the economic model behind phase transitions.
