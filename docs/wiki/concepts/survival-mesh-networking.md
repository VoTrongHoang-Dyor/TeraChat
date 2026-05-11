---
type: concept
created: 2026-05-10
tags: [terachat, mesh, p2p, ble, wifi-direct, survival, emdp]
sources: [tera-core-spec, tera-test-matrix, tera-tech-debt]
---

# Survival Mesh Networking

TeraChat's offline communication capability: when internet connectivity is lost, devices automatically form a P2P mesh using BLE 5.0 and Wi-Fi Direct. Designed for disaster scenarios, air-gapped environments, and tactical operations.

## How It Works

```
Internet Available                    Internet Lost
─────────────────                    ─────────────
Client ↔ TeraRelay (TLS 1.3)        Client ↔ Client (BLE 5.0 / Wi-Fi Direct)
                                     ↓
                                  Mesh Mode Activated
                                  • Dark Navy UI + Radar Pulse HUD
                                  • Store-and-Forward CRDT
                                  • P2P encrypted
```

## Communication Layers

| Priority | Protocol | Range | Bandwidth | Use Case |
|----------|----------|-------|-----------|----------|
| P0 (Critical) | BLE 5.0 Coded PHY | ~200m (open air) | ~125 kbps PHY / ~5 kbps app-layer | Control signals, EMDP, KillDirective |
| P1 (Standard) | Wi-Fi Direct | ~50m | 30–80 Mbps (real-world) | Messages, presence |
| P2 (Bulk) | Wi-Fi Direct | ~50m | 30–80 Mbps (real-world) | File transfers, media |

## EMDP (Emergency Mesh Disaster Protocol)

When all internet paths fail and a Border Node (device with internet) is lost:
- EMDP activates < 30s
- Text-only mode
- Key escrow transfers to surviving nodes
- 60min TTL before requiring reconnection to a Border Node

## BLE QoS (TD-008)

`MeshMultiplexer` enforces priority:
- P0 packets never suspended, never queued behind P2
- Dynamic Backpressure: RTT > 200ms → suspend P2 immediately
- P2 resumes when RTT < 100ms sustained for 5s

## 🧠 Design Decisions (Q&A)

- **Why BLE instead of just Wi-Fi Direct?** → BLE 5.0 Coded PHY has ~200m range and very low power. Wi-Fi Direct is faster but range-limited (~50m) and power-hungry. BLE keeps the mesh alive when devices are spread out. Trade-off: BLE app-layer throughput is very low (~5 kbps after protocol overhead), so only control signals go over it.
- **What is the AWDL risk on iOS?** → AWDL (Apple Wireless Direct Link) is Apple's private framework for AirDrop/AirPlay with NO public API. Third-party apps cannot legally use AWDL. If the iOS mesh strategy depends on AWDL for P2P Wi-Fi, this is a technical dead end. The fallback is BLE-only mesh on iOS, which is much slower. This risk is not yet acknowledged in TERA-CORE.
- **Why EMDP TTL of 60 minutes?** → Prevents indefinite offline access. If a device is lost/stolen while in mesh mode, access expires. Trade-off: legitimate field operations need a Border Node within 60 min.
- **Why no QoS initially (TD-008)?** → BLE channel is so narrow that a single 2MB file transfer can saturate it for minutes, starving control signals. This is a CRITICAL gap for Gov deployment — MeshMultiplexer is the fix but not yet implemented.
