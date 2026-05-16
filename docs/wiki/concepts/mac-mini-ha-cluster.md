---
type: concept
created: 2026-05-15
updated: 2026-05-15
tags: [infrastructure, ha, deployment, mac-mini, raft]
---

# Mac Mini HA Cluster

Zero-ops high availability for self-hosted deployments. Customers are not DevOps — everything must work with one command.

## One-Touch Setup

```bash
curl -fsSL https://install.terachat.io | bash
```

This single script handles:
- Platform detection (macOS ARM64/x86_64, Linux)
- Binary download and signature verification
- Auto-discovery of peer Mac minis via mDNS
- TLS certificate provisioning (Let's Encrypt)
- Service installation (launchd / systemd)
- Health check + admin console URL

## Cluster Discovery

Two Mac minis find each other automatically via mDNS — no configuration needed. Plug both into the same switch and they discover each other.

```rust
pub struct ClusterDiscovery {
    node_id: NodeId,
    mdns: MdnsService,
}

impl ClusterDiscovery {
    pub async fn run(&self) -> ClusterTopology {
        // 1. Broadcast our presence
        self.mdns.register(ServiceRecord {
            name: format!("_terachat._tcp.{}", self.node_id),
            port: 7474,
            txt: vec![
                ("role", "primary_candidate"),
                ("version", env!("CARGO_PKG_VERSION")),
                ("raft_port", "7475"),
            ],
        }).await;

        // 2. Discover peers
        let peers = self.mdns.browse("_terachat._tcp")
            .timeout(Duration::from_secs(10))
            .collect()
            .await;

        // 3. Decide topology
        if peers.is_empty() {
            ClusterTopology::Standalone { am_primary: true }
        } else {
            self.raft_election(peers).await
        }
    }
}
```

## Raft Consensus (Simplified)

Not full Raft — only what TeraChat needs: leader election + WAL replication.

```rust
pub struct RaftNode {
    id: NodeId,
    state: Arc<RwLock<RaftState>>,
    log: Arc<RaftLog>,
    peers: Vec<RaftPeer>,
}

pub enum RaftState {
    Follower { leader: NodeId, lease_expires: Instant },
    Candidate { votes: HashSet<NodeId> },
    Leader { followers: Vec<NodeId>, next_index: HashMap<NodeId, u64> },
}

impl RaftNode {
    pub async fn replicate_wal_entry(&self, entry: WalEntry) 
        -> Result<Quorum, RaftError> 
    {
        // Primary → secondary replication
        // Majority quorum: 1/1 or 2/2 follower acks
    }
}
```

## SLA Tiers

| Configuration | Uptime | Downtime/Year | Cost |
|--------------|--------|---------------|------|
| 1× Mac mini | 99.5% | ~4 hours | ~$800 |
| 2× Mac mini HA | 99.95% | ~4 minutes | ~$2,000 |
| 2× Mac mini + Mesh fallback | 99.99% | ~1 minute | ~$2,000 |

**99.99% is enterprise contract grade.** Mesh bridges the gap when primary fails by enabling device-to-device relay during failover.

## Hardware Requirements per Tier

| Tier | Users | Primary | Storage | Total |
|------|-------|---------|---------|-------|
| Starter | ≤ 50 | Mac mini M2 (8GB) $699 | External SSD 1TB $80 | ~$800 |
| Business | ≤ 500 | Mac mini M2 Pro (32GB) $1,299 | Synology DS223+ 2×4TB $600 | ~$2,000 |
| Enterprise HA | ≤ 2000 | 2× Mac mini M4 Pro (48GB) $4,000 | Synology DS923+ 4×8TB $1,300 | ~$5,500 |

## Customer ROI Argument

> "You pay $X/month for SaaS. After 12 months, you've paid enough to buy the hardware. From month 13, you pay $Y/month for TeraChat license — 80% cheaper. And data stays in your office."

| Current | Monthly Cost | 12-Month Total | TeraChat ROI |
|---------|-------------|----------------|--------------|
| Slack (50 users) | ~$2,500 | $30,000 | Payback < 1 month |
| Teams (500 users) | ~$6,250 | $75,000 | Payback < 1 month |

## Install Script Design

```bash
#!/bin/bash
set -euo pipefail

TERA_VERSION="1.0.0"
TERA_DATA="/var/lib/terachat"

detect_platform() { /* macos-arm64 | macos-x86_64 | linux-x86_64 */ }

# Download + verify signature
curl -fsSL "https://dl.terachat.io/${TERA_VERSION}/terachat-$(detect_platform).tar.gz" \
    | tar xz -C /usr/local/bin/

# Bootstrap — auto-discover peers
terachat bootstrap --auto-discover --auto-tls --data-dir "$TERA_DATA"

# Start service
terachat service install && terachat start

# Print status
terachat status
```

## Related Pages

- [[Survival Mesh Networking]] — Emergency BLE/WiFi Direct fallback
- [[AI Inference Offloading]] — Cluster as inference tier
- [[Vertical Slice Development]] — Slice 4: HA + Mesh Failover
