---
type: concept
created: 2026-05-16
updated: 2026-05-30
tags: [teralink, fallback, mesh, ble, mDNS, multipeer, network, floor-subnet, dms, dynamic-mesh-score]
sources: [CLAUDE.md, invariants.md, Spec-Core-Cryptography-And-Mesh.md]
---

# TeraLink Fallback Network

Mạng dự phòng 3 tầng cho TeraChat khi TeraRelay không khả dụng. Thay thế hoàn toàn khái niệm "BLE Mesh" trước đây — BLE chỉ là T3 emergency, không phải toàn bộ fallback strategy.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│ T1: LAN / Wi-Fi (Normal Operation)                          │
│   Devices → TeraRelay (Compute Node) → NAS ECC Storage      │
│   Full features: messages, files, AI, .tapp                  │
│   Detection: Relay health check green                        │
├─────────────────────────────────────────────────────────────┤
│ T2: mDNS / Multipeer (Server Down)                          │
│   Devices discover each other via mDNS (macOS) or            │
│   MultipeerConnectivity (iOS)                                │
│   Text messages + presence only                              │
│   Detection: Relay health check fails (3 consecutive)        │
├─────────────────────────────────────────────────────────────┤
│ T3: BLE Emergency (No LAN / No Wi-Fi)                       │
│   BLE 5.0 Coded PHY, ~200m range                             │
│   Text only, ≤ 500 bytes per message                         │
│   Floor Subnet Architecture: ≤ 50 devices per subnet         │
│   Detection: mDNS/Multipeer returns 0 peers for 10s          │
└─────────────────────────────────────────────────────────────┘
```

## DMS — Dynamic Mesh Score (Phân hạng Thiết bị)

> **M-9 Fix 2026-05-30:** Cơ chế phân hạng thiết bị được gọi là **DMS (Dynamic Mesh Score)**, không phải EMDP. EMDP đã deprecated.

Định kỳ mỗi 60 giây, tầng `tc-mesh` chạy ngầm luồng đánh giá DMS để tự xếp hạng thiết bị vào đồ thị định tuyến:

| Hạng (Tier) | Loại thiết bị | Điều kiện | Vai trò trong Mesh |
|-------------|--------------|-----------|--------------------|
| **Tier 1** | Desktop/Laptop cắm sạc | AC power, RAM dồi dào, CPU đa nhân | **Super Relay (Lõi):** Store-and-Forward full bandwidth, buffer 10,000 msgs |
| **Tier 2** | Laptop chạy pin | Pin > 50%, Wi-Fi LAN/Direct active | **Active Relay:** Tiếp sóng thông thường. Tự hạ xuống Leaf khi pin < 40% |
| **Tier 3** | Android | Pin > 50%, RAM trống > 50% | **Edge Relay (Biên):** Gossip local cluster, buffer 2,000 msgs |
| **Tier 4** | iPhone (iOS) | RSSI mạnh, app đang mở hoặc background < 3 phút | **Passive Relay / Leaf:** Chỉ tiếp sóng khi không có Tier 1-3 gần đó. Buffer 300 msgs. `election_weight = 0` |

**Tại sao iOS `election_weight = 0`:** iOS áp đặt cơ chế tự đình chỉ cổng BLE GATT khi khóa màn hình. iOS không bao giờ được bầu làm Floor Gateway.

**iOS Micro-Daemon Mode:** Khi iPhone chuyển sang Background T3, Flutter UI bị đóng băng và giải phóng RAM (Zero-UI footprint). Chỉ duy trì headless `tc-mesh` Rust Core với **< 15MB RAM** — an toàn dưới ngưỡng Jetsam 50MB.

## Tier Transition Logic

| Transition | Trigger | Detection Time | Impact |
|-----------|---------|---------------|--------|
| T1 → T2 | Relay health check fails (3 consecutive pings) | < 5s | Text only, no files/media, no AI |
| T2 → T3 | mDNS/Multipeer discovery returns 0 peers for 10s | < 15s | BLE text only, 500-byte limit |
| T2 → T1 | Relay health check passes (2 consecutive pings) | < 3s | Full feature restore |
| T3 → T2 | mDNS/Multipeer rediscovers ≥ 1 peer | < 10s | Text + presence restored |
| T3 → T1 | Relay health check passes (2 consecutive pings) | < 3s | Full feature restore (skip T2) |

Transition logic is implemented in `tc-mesh` with `TeraLinkStateMachine`. Each transition is logged as an event for observability.

## PQ-KEM Offline Disable (T3 BLE)

> **M-5 Fix 2026-05-30:** PQ-KEM (ML-KEM-768/Kyber768) tự động disable khi T3 BLE mode. Không phải "luôn active".

ML-KEM-768 (Kyber768) ciphertext chiếm ~1,100 bytes — gần gấp đôi MTU BLE 500 bytes, gây nghẽn mạng nghiêm trọng nếu đẩy qua BLE.

**Khi T3 BLE Emergency active:**
- PQ-KEM tự động disable (`KemMode::Classical` thay vì `KemMode::Hybrid`)
- Chỉ duy trì E2EE tiêu chuẩn **Curve25519/X25519** để bảo toàn bandwidth
- MLS Epoch Rotation bị đóng băng (Mesh-Freeze mode): chỉ dùng Static Session Keys
- PQ-KEM tự động re-enable khi quay về T1/T2

**Lý do chấp nhận:** Trong emergency offline, priority tuyệt đối là duy trì liên lạc. Curve25519 vẫn cung cấp E2EE security đủ mạnh cho communication ngắn hạn trong môi trường khẩn cấp.

### Activation Workflow

- **Auto-detect:** Grace period 30 giây sau khi mất internet trước khi chuyển sang T2 — tránh flicker khi mạng thoáng mất
- **Manual:** Nút "Bật TeraLink" trong Settings — dành cho scenario cố ý (họp ngoài trời, vùng tín hiệu yếu)
- **Permission pop-up:** Chỉ lần đầu tiên per device. Sau đó lưu preference. Cần giải thích rõ: TeraLink dùng BLE và Wi-Fi Direct/Multipeer, có thể ảnh hưởng pin
- **UI indicator:** Hiển thị rõ ràng khi đang ở TeraLink mode (Dark Navy background theo Design System) — user cần biết latency và throughput khác bình thường
- **Sync khi restore:** Khi Compute Node restore, CRDT merge delta tự động. Không cần user action. Thời gian sync tỷ lệ với số message trong window offline — thường < 5 giây

### Unified Transport Abstraction

Để tránh duplicate logic giữa các transport path, `tc-mesh` dùng unified abstraction:

```rust
trait MeshTransport: Send + Sync {
    async fn send(&self, peer: &PeerId, payload: &[u8]) -> Result<()>;
    async fn recv(&self) -> Result<(PeerId, Vec<u8>)>;
    fn estimated_throughput_kbps(&self) -> u32;
}

// WifiDirectTransport impl MeshTransport  (T2, < 60m)
// BleRelayTransport impl MeshTransport    (T3, multi-hop)
```

MLS E2EE và CRDT sync layer gọi vào `MeshTransport` mà không cần biết transport phía dưới — Deep Module principle: tránh coupling giữa crypto layer và network transport.

### RAM Budget Khi TeraLink Active

Đây là điểm nghẽn thực tế nhất trên iOS. Jetsam sẽ kill process nếu vượt ngưỡng RAM:

| Component | RAM Budget | Status |
|-----------|-----------|--------|
| BLE GATT stack + Control Plane | ~5 MB | Active — bắt buộc |
| MLS session state (group keys) | ~10-20 MB | Active — cached per group |
| CRDT hot_dag delta buffer | ~20-50 MB | Active — theo giới hạn device |
| Wi-Fi Direct / Multipeer session | ~15-30 MB | On-demand — foreground only |
| WASM .tapp sandbox | 0 MB | SUSPEND hoàn toàn khi TeraLink active |
| AI model | 0 MB | KHÔNG load trong background |
| **Tổng target** | **< 120 MB** | An toàn khỏi Jetsam threshold |

Trade-off bắt buộc: WASM .tapp và AI inference PHẢI bị suspend khi TeraLink mode active. UI cần thông báo rõ: "TeraLink mode đang hoạt động — một số tính năng tạm thời không khả dụng."

## Floor Subnet Architecture

Mỗi tầng vật lý = 1 BLE subnet riêng biệt. Không broadcast cross-floor qua BLE.

```
Floor 3:  [Device] [Device] [Device] ── Floor Gateway 3 ──┐
Floor 2:  [Device] [Device] [Device] ── Floor Gateway 2 ──┼── Backbone LAN
Floor 1:  [Device] [Device] [Device] ── Floor Gateway 1 ──┘
```

**Rules:**
- Max 50 devices per BLE subnet
- TTL = 2 trong mỗi subnet (ngăn broadcast storm)
- 1 Floor Gateway được bầu per tầng (highest uptime + strongest BLE signal)
- Floor Gateway bridge BLE subnet → backbone LAN (Ethernet/Wi-Fi)
- iOS devices: `election_weight = 0` — không bao giờ làm Floor Gateway
- iOS devices: không relay khi màn hình tắt (background BLE restriction)

**Floor Gateway Hardware:** Raspberry Pi 4 Model B 8GB, pre-configured image. Hardware SKU add-on: $150–200.

## Platform-Specific Implementation

| Platform | T2 Discovery | T3 BLE | Notes |
|----------|-------------|--------|-------|
| **macOS** | mDNS (Bonjour) + TCP | CoreBluetooth | Full BLE central + peripheral |
| **iOS** | MultipeerConnectivity | CoreBluetooth | No relay when screen off, election_weight = 0 |
| **Android** | NSD (Network Service Discovery) | BluetoothLe | Foreground service required for BLE |
| **Windows** | mDNS (native) | WinRT BLE | Limited peripheral mode |
| **Linux** | Avahi mDNS | BlueZ | Depends on hardware BLE support |

## Invariant Enforcement in TeraLink

| Invariant | How Enforced | Layer |
|-----------|-------------|-------|
| I-10 (NAS ECC Storage) | `tc-store` write path only compiles for NAS target | Compile-time |
| I-11 (BLE ≤ 500 bytes) | `BlePayload` is `[u8; 500]` fixed array | Type system |
| I-12 (.tapp no egress) | `network:external` permanently blocked in Host ABI | WASM sandbox |
| iOS election_weight = 0 | `ElectionWeight::zero()` hardcoded for iOS targets | Compile-time |
| No iOS relay when screen off | Background task assertion check in `tc-mesh` | Runtime |

## Observability

- `teralink.tier_transition` — OpenTelemetry event on every tier change (T1→T2, T2→T3, etc.)
- `teralink.floor_gateway_election` — event log for Floor Gateway elections
- `teralink.ble_subnet.device_count` — gauge: current device count per subnet
- `teralink.message.bytes` — histogram: message sizes on each tier (alert if > 500 on T3)

## Related Pages

- [[Hardware Specification]] — Hardware nodes that run TeraLink
- [[Invariants]] — I-10, I-11, I-12, iOS election_weight
- [[Survival Mesh Networking]] — Legacy BLE mesh concept (pre-v2.1)
- [[Mac Mini HA Cluster]] — HA setup that prevents T1→T2 transitions
