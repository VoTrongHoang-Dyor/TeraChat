---
type: concept
created: 2026-05-15
updated: 2026-05-15
tags: [ios, mesh, storage, ble, device]
---

# iOS Mesh Storage Tiers

Tiered buffer architecture for mesh-mode storage on resource-constrained devices. Automatically selects tier based on available RAM and device class.

## Tier Definitions

| Tier | Device Class | Max Buffer | Content Allowed |
|------|-------------|------------|-----------------|
| **Minimal** | iPhone 12, 13 (4GB) | 32MB | Text only |
| **Standard** | iPhone 14, 15 (6-8GB) | 64MB | Text + small files (< 5MB) + voice |
| **Enhanced** | iPad Pro (8-16GB) | 128MB | Text + media + voice |
| **Full** | Mac mini (32GB+) | 2GB | Everything (relay capability) |

## Auto-Detection

```rust
impl BufferTier {
    pub fn detect() -> Self {
        let available_ram = system_available_ram();
        let device_class = DeviceClass::current();

        match (device_class, available_ram) {
            (DeviceClass::MacMini, _) => Self::Full {
                max_bytes: 2 * 1024 * 1024 * 1024  // 2GB
            },
            (_, ram) if ram >= 6 * GB => Self::Enhanced {
                max_bytes: 128 * MB
            },
            (_, ram) if ram >= 4 * GB => Self::Standard {
                max_bytes: 64 * MB
            },
            _ => Self::Minimal {
                max_bytes: 32 * MB
            },
        }
    }
}
```

## Content Type Gating

```rust
pub struct AllowedContent {
    text: bool,
    small_files: bool,  // < 5MB
    media: bool,        // images, video
    voice: bool,        // voice messages
}

impl BufferTier {
    pub fn allowed_content(&self) -> AllowedContent {
        match self {
            Self::Minimal { .. } => AllowedContent {
                text: true, small_files: false, media: false, voice: false,
            },
            Self::Standard { .. } => AllowedContent {
                text: true, small_files: true, media: false, voice: true,
            },
            Self::Enhanced { .. } | Self::Full { .. } => AllowedContent {
                text: true, small_files: true, media: true, voice: true,
            },
        }
    }
}
```

## Eviction Policy

LRU with priority protection — critical messages are never evicted.

```rust
pub struct EvictionPolicy {
    max_age: Duration,
    priority_protected: HashSet<MessageId>,
}

impl EvictionPolicy {
    pub fn evict_if_needed(&mut self, buffer: &mut MeshStorage) {
        let current = buffer.total_bytes();
        let max = buffer.tier.max_bytes() as u64;

        // No action below 90%
        if current <= max * 9 / 10 { return; }

        // Eviction order: oldest non-priority first
        let candidates = buffer.entries()
            .filter(|e| !self.priority_protected.contains(&e.id))
            .filter(|e| e.age() > self.max_age)
            .sorted_by_key(|e| e.timestamp);

        for entry in candidates {
            // Stop at 80% target
            if buffer.total_bytes() <= max * 8 / 10 { break; }
            buffer.remove(entry.id);
        }
    }
}
```

## iOS Mesh Constraints

| Constraint | Value | Reason |
|-----------|-------|--------|
| Election weight | 0 | iPhone never becomes mesh coordinator |
| Thermal critical | Suspend mesh sync | Preserve battery + prevent crash |
| Max BLE throughput | 100 kbps | BLE 5.0 physical limit |
| Max BLE RTT | 250ms | Acceptable for P0 messages |
| Background refresh | 15 min interval | iOS background task limit |
| Mesh buffer flush | On network restore or buffer > 90% | Batch sync to relay |

## MeshBuffer Interface

As a deep module, only 5 public methods:

```rust
impl MeshBuffer {
    pub fn new(tier: BufferTier) -> Self;
    pub fn store(&mut self, msg: Message) -> Result<MessageId>;
    pub fn store_priority(&mut self, msg: Message) -> Result<MessageId>;
    pub fn retrieve(&self, id: MessageId) -> Option<&Message>;
    pub fn total_bytes(&self) -> u64;
}
```

Interior: tier detection, content filtering, eviction policy, thermal-aware throttling — all hidden.

## Related Pages

- [[Survival Mesh Networking]] — BLE/WiFi Direct mesh protocol
- [[Deep Module Design]] — Why MeshBuffer has only 5 public methods
- [[AI Inference Offloading]] — Thermal monitor shared with inference
