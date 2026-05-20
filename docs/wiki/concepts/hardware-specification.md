---
type: concept
created: 2026-05-16
updated: 2026-05-18
tags: [hardware, infrastructure, deployment, compute-node, storage-node, ai-node, hpe, nas-ecc]
sources: [CLAUDE.md, invariants.md]
---

# Hardware Specification

TeraChat hardware architecture separates roles into distinct physical nodes. This separation is an architectural invariant (I-10), not an operations recommendation.

## Node Type Separation

| Node Type | Hardware | ECC RAM | Role | Example |
|-----------|----------|---------|------|---------|
| **Compute Node** | Mac mini M4 / M4 Pro | **No** (non-ECC) | TeraChat Core, Relay, gRPC server, client-facing logic | Mac mini M4 Pro 48GB |
| **Storage Node** | NAS with ECC RAM | **Yes** (required) | Primary DB writer, SQLite WAL journal, blob storage | Synology DS923+ 32GB ECC |
| **AI Inference Node** | Mac mini M4 Max / HPE MicroServer | Optional | Inference only, MLX runtime, AI model execution | Mac mini M4 Max 64GB |
| **Gov/Military Compute** | HPE MicroServer Gen10+ | **Yes** (ECC) | FIPS 140-3 validated compute + storage | HPE ProLiant MicroServer Gen10+ |
| **Floor Gateway** | Raspberry Pi 4 (pre-configured) | No | BLE inter-subnet bridge, one per physical floor | Raspberry Pi 4 Model B 8GB |

## Why ECC RAM for Storage?

Non-ECC RAM can experience silent single-bit errors — a bit flip in the WAL journal before it flushes to disk corrupts the entire database. ECC RAM detects and corrects single-bit errors. Mac mini uses non-ECC consumer RAM, so it cannot be the primary database writer. The NAS with ECC RAM is the sole storage authority.

## Tier Definitions

All tiers use **Concurrent Active Sessions** — peak simultaneous online users, not total registered accounts. Pricing tiers limit total registered users; hardware tiers size for peak concurrent. An organization with 500 employees and 200 peak concurrent needs a Business license (≤500 users) with Business hardware (300–500 concurrent capacity).

### Starter — ≤50 Org Users / 80–120 Concurrent

| Component | Specification | Est. Cost |
|-----------|--------------|-----------|
| Compute Node | Mac mini M4 Pro 24GB | ~$1,400 |
| Storage Node | Synology DS423+ 4TB ECC | ~$600 |
| AI Node | Mac mini M4 8GB + Ollama Q4 (optional) | ~$600 |
| Floor Gateway | None (single floor only) | — |
| **Total Hardware** | | **~$2,000–2,600** |

**Capacity:** 80–120 concurrent active sessions. TeraChat Core + relay chạy trên Compute Node. NAS ECC là storage authority. AI Node là SKU add-on tùy chọn — TeraChat Core hoạt động không cần AI Node.

### Business — ≤500 Org Users / 300–500 Concurrent

| Component | Specification | Est. Cost |
|-----------|--------------|-----------|
| Compute Node | 2× Mac mini M4 Pro 48GB (HA active-passive) | ~$3,600 |
| Storage Node | Synology DS923+ 16TB ECC | ~$1,200 |
| AI Node | Mac Studio M2 Max 32GB + Ollama (optional) | ~$2,000 |
| Floor Gateway | Up to 3× Raspberry Pi 4 (optional) | ~$450 |
| **Total Hardware** | | **~$5,000–7,250** |

**Capacity:** 300–500 concurrent active sessions. HA cluster với automatic failover giữa 2 Compute Node. NAS ECC là storage authority bắt buộc. AI Node là SKU add-on tùy chọn riêng biệt.

### Enterprise — ≤5,000 Org Users / 1,500–2,000 Concurrent

| Component | Specification | Est. Cost |
|-----------|--------------|-----------|
| Compute Node | 2× Mac mini M4 Pro 64GB (HA active-passive) | ~$4,000 |
| Relay Node | 1× Mac Studio M2 Ultra (dedicated relay) | ~$4,000 |
| Storage Node | Synology RS2423+ Rackmount ECC 32TB | ~$2,500 |
| AI Node | Mac Studio M2 Ultra 64GB + Ollama (optional) | ~$4,000 |
| Floor Gateway | Up to 10× Raspberry Pi 4 | ~$1,500 |
| **Total Hardware** | | **~$10,000–16,000** |

**Capacity:** 1,500–2,000 concurrent active sessions. 5,000 total accounts với 30% online cùng lúc là feasible. Dedicated relay node (Mac Studio M2 Ultra) cho connection scaling. Rackmount NAS ECC cho storage enterprise-grade. AI Node recommended but optional.

### Gov/Military — Negotiate (Air-Gapped)

| Component | Specification | Est. Cost |
|-----------|--------------|-----------|
| Compute Node | 2× HPE MicroServer Gen10+ (ECC, IPMI, FIPS 140-3) | ~$8,000 |
| Storage Node | HPE MSA Storage ECC + RAID | ~$5,000 |
| AI Node | On-device ONNX Phi-3-mini Q4 | ~$6,000 |
| HSM | YubiHSM2 FIPS 140-2 Level 3 | ~$800 |
| Floor Gateway | Custom ARM SBC (FIPS validated) | ~$2,000 |
| **Total Hardware** | | **~$25,000+** |

**Why HPE, not Mac mini:** Apple hardware does not pass FIPS 140-3 validation required for US/EU government procurement. TeraChat Rust binary compiles cross-platform — software is identical, only hardware changes. HPE MicroServer Gen10+ provides ECC RAM, IPMI remote management, and FIPS 140-3 validated cryptographic module. AI dùng ONNX bundle air-gapped, không có network call ra ngoài.

## Hardware Constraints (Non-Negotiable)

1. **Compute Node ≠ Storage Authority** — Mac mini non-ECC RAM must never be the primary database writer. I-10 enforcement: `tc-store` write path compiles only for NAS target.
2. **NAS ECC Required for All Tiers** — NAS với ECC RAM là storage authority bắt buộc từ Starter tier trở lên. Không có ngoại lệ.
3. **AI Node is Add-On** — AI inference is optional for TeraChat Core. Separating it avoids coupling AI hardware requirements to messaging reliability.
4. **Floor Gateway is Separate SKU** — Not bundled with any tier. Purchased per floor as needed. Pre-configured Raspberry Pi 4 image available.
5. **Concurrent = Peak Active, Not Total Registered** — Always size hardware for peak concurrent sessions, not total user accounts.

## Related Pages

- [[TeraLink Fallback Network]] — 3-tier fallback that uses these hardware nodes
- [[Platform Architecture]] — License tiers and module organization
- [[Mac Mini HA Cluster]] — HA cluster setup details
- [[Invariants]] — I-10 (NAS ECC Storage Authority), I-11 (BLE ≤ 500 bytes)
