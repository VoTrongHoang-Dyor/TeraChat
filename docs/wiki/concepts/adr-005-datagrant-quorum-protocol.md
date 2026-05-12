---
type: concept
created: 2026-05-12
updated: 2026-05-12
tags: [adr, datagrant, quorum, governance, phase-0]
sources: [tera-eco-spec, tera-gov-spec]
---

# ADR-005: DataGrant Quorum Protocol

## Status

**ACCEPTED** — 2026-05-12

## Context

DataGrant is TeraChat's mechanism for granting scoped data access to `.tapp` applications. A DataGrant activation must be verified by multiple nodes to prevent:

1. **Rogue admin**: Single compromised admin grants unrestricted access
2. **Offline grant injection**: Offline node fabricates a DataGrant with stale generation counter
3. **Split-brain activation**: Partitioned network results in conflicting grant states

### GAP-E Resolution

The `generation` counter alone cannot distinguish "grant never seen" (gen 0) from "revoked grant" (gen > 0) when a node was offline during grant issuance.

## Decision

**Majority quorum of `election_weight > 0` nodes must confirm DataGrant activation via `Hash_Frontier` gossip.**

### Protocol

```
1. Admin initiates DataGrant request
2. Request broadcast to all nodes with election_weight > 0
3. Each node validates:
   - License JWT valid
   - OPA policy allows the grant
   - Grant generation > local known generation
4. Node signs vote with its Ed25519 device key
5. Quorum collector waits for majority (⌈N/2⌉ + 1) votes
6. If quorum reached → DataGrant status = ACTIVE
7. If quorum not reached → status = PENDING_QUORUM (data NOT served)
```

### Gov-Tier Requirements

- **Standard tier**: Simple majority (3/5 nodes)
- **Gov/Military tier**: Supermajority (3/5 with at least 2 from different physical locations)
- **Unconfirmed grants**: Return `PENDING_QUORUM`, never serve data without quorum

### Generation Counter Semantics

| Generation | Meaning |
|-----------|---------|
| 0 | Grant never created |
| N (odd) | Active grant (version N) |
| N (even) | Revoked grant (version N) |

Offline nodes receiving a grant with gen > local gen must request quorum verification before serving.

## Consequences

### Positive
- ✅ No single point of grant authority
- ✅ Offline nodes cannot silently accept fabricated grants
- ✅ Gov/Military compliance: multi-location quorum
- ✅ Generation counter provides clear active/revoked semantics

### Negative
- ❌ Grant activation latency increases (quorum collection)
- ❌ Network partition can block grant activation (by design — fail secure)
- ❌ Quorum voting adds network overhead

## Related

- [[Enterprise Identity Governance]] — OPA policy enforcement
- GAP-E — DataGrant generation counter ambiguity
- TERA-ECO §8.3 — DataGrant specification
