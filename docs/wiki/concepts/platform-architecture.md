---
type: concept
created: 2026-05-16
updated: 2026-05-18
tags: [platform, licensing, bsl, open-source, architecture, modules, flywheel]
sources: [CLAUDE.md, invariants.md]
---

# Platform Architecture

TeraChat dùng mô hình 3 tầng license (CLOSED / BSL 1.1 / MIT) để cân bằng giữa bảo vệ doanh thu, community trust, và ecosystem growth.

## Triết lý

```
CLOSED:  Bảo vệ revenue model — crypto, license engine, HA orchestration
BSL 1.1: Bảo vệ competitive advantage — source-readable, no fork, auto-MIT after 4 years
MIT:     Ecosystem growth — SDKs, proto, tooling — community tự do đóng góp
```

Bài học từ HashiCorp 2023: license bait-and-switch phá hủy community trust. TeraChat commit: module MIT hôm nay sẽ mãi mãi là MIT (enforced by I-13).

## License Tiers

### CLOSED (Proprietary — Mãi Mãi)

| Module | Lý do đóng |
|--------|-----------|
| `tc-crypto` | MLS RFC 9420 implementation — core security IP |
| License Engine | Revenue model — JWT validation, hardware fingerprint binding |
| HA Orchestration | Cluster failover logic — competitive moat |

Các module này không bao giờ được public source. Phân phối dưới dạng binary trong TeraChat release.

### BSL 1.1 (Business Source License)

| Module | Auto-Convert |
|--------|-------------|
| `tc-crdt-sync` | → Apache 2.0 sau 4 năm |
| `tc-mesh` | → Apache 2.0 sau 4 năm |
| `tc-store` | → Apache 2.0 sau 4 năm |
| `tc-ai` | → Apache 2.0 sau 4 năm |
| `tc-tapp` | → Apache 2.0 sau 4 năm |
| Server Core | → Apache 2.0 sau 4 năm |

**BSL 1.1 Competitive Use Clause:**
"Competitive use" được định nghĩa là: cung cấp TeraChat-compatible messaging service cho third-party organization as a service.

- **Được phép:** Tự deploy nội bộ, build .tapp bán trên marketplace, integrate AI adapter, academic research, personal projects
- **Không được phép:** Fork server + rebrand thành sản phẩm cạnh tranh, SaaS reselling TeraChat server

### MIT (Luôn Mở)

| Module | Mục đích |
|--------|---------|
| `tapp-sdk` | Public SDK cho .tapp developers — kéo ecosystem |
| `ai-adapter-sdk` | Public SDK cho AI adapter developers |
| `tc-proto` | Protobuf definitions — interoperability |

Community được tự do fork, modify, distribute các module MIT. Đây là "mồi câu" cho ecosystem flywheel.

## Module Architecture Diagram

```
source/core/                          License
├── tc-crypto/         MLS E2EE, Key Management      [CLOSED]
├── tc-crdt-sync/      CRDT DAG + Offline Sync       [BSL 1.1]
├── tc-mesh/           TeraLink Fallback Network      [BSL 1.1]
├── tc-store/          SQLite WAL + Blob Storage      [BSL 1.1]
├── tc-ai/             TeraAiAdapter [BSL 1.1]
├── tc-tapp/           WASM Engine, Host ABI          [BSL 1.1]
├── tc-proto/          Protobuf + gRPC Definitions    [MIT]
│
source/sdk/
├── tapp-sdk/          Public .tapp Developer SDK     [MIT]
├── ai-adapter-sdk/    Public AI Adapter SDK          [MIT]
```

## Ecosystem Flywheel

```
Bước 1: TeraChat build 3 reference .tapp nội bộ
        (task manager, expense report, leave request)
        ↓
Bước 2: Publish tapp-sdk + ai-adapter-sdk (MIT) lên GitHub
        ↓
Bước 3: Enterprise thấy ecosystem, mua license
        ↓
Bước 4: Third-party developer build thêm .tapp vì có khách hàng
        ↓
Bước 5: Marketplace hoa hồng 20% bắt đầu có ý nghĩa
        (chỉ meaningful sau 500+ org customers)
```

## Upgrade Path (3 Giai Đoạn)

| Giai Đoạn | Command | Target |
|-----------|---------|--------|
| **1. 1-Click Install** | `curl -fsSL https://install.terachat.io \| bash` | Pilot customer — 1 Mac mini, 12 phút end-to-end |
| **2. Linux Migration** | `terachat migrate --target linux` | Production enterprise — NAS ECC + Linux server |
| **3. HA Cluster** | `terachat cluster init` | Enterprise/Gov — 2× Compute + NAS + AI Node |

Mỗi bước là **upgrade**, không phải reinstall. Data và client configuration không bị ảnh hưởng khi migrate.

## Pricing (4-Tier License)

| Tier | Price/Year | Org Size | Hardware Concurrent Capacity | Key Features |
|------|-----------|----------|------------------------------|--------------|
| **Starter** | $900/yr | ≤50 users | 80–120 concurrent | 1-click install, basic E2EE, community support |
| **Business** | $2,400/yr | ≤500 users | 300–500 concurrent | TeraLink Fallback, .tapp support, email support |
| **Enterprise** | $6,000/yr | ≤5,000 users | 1,500–2,000 concurrent | HA Cluster, AI Node, priority support, SLA |
| **Gov/Military** | $15,000+/yr | Negotiate | Negotiate | HPE hardware, FIPS 140-3, air-gapped, HSM, dedicated support |

**Quan trọng:** 'User' trong pricing = tổng tài khoản trong tổ chức. 'Concurrent' = số session hoạt động đồng thời tại peak. Một org 500 nhân viên với peak 200 concurrent → cần Business license (≤500 users) + Business hardware (300–500 concurrent capacity).

Rẻ hơn 40–70% so với Slack/Teams tính per-org. TCO Calculator trên `terachat.io` so sánh 3-year cost vs Slack/Teams là conversion tool chính.

## Invariant Enforcement (Platform)

| Invariant | Enforcement |
|-----------|------------|
| I-13 (BSL boundary immutable) | CI gate `bsl-boundary-hash`: mỗi `git tag` CI hash `LICENSE` so với `.github/BSL_BOUNDARY.sha256`. Mismatch = block |
| I-10 (NAS ECC Storage) | `tc-store` write path compile cho NAS target, không cho Mac mini |
| I-12 (.tapp no egress) | `network:external` capability blocked permanently in Host ABI |

## Related Pages

- [[Hardware Specification]] — Hardware tiers mapped to license tiers
- [[TeraLink Fallback Network]] — Network architecture using these modules
- [[Enterprise License Model]] — Original license model (pre-v2.1)
- [[Tapp Community Framework]] — .tapp SDK and community model
- [[Invariants]] — I-13 (BSL boundary)
- [[Threat Model]] — Security analysis of this architecture
