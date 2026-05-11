---
type: concept
created: 2026-05-11
tags: [terachat, messaging, hierarchical-authority, internal-communication, branch-company]
sources: [tera-intro, tera-core-spec]
---

# Hierarchical Authority Messaging

TeraChat's communication model is built on **organizational hierarchy**, not open social networking. Messages flow along authority lines — within the company and between branch companies — never to external customers.

## Core Principle

```
                    ┌──────────────────┐
                    │   HEADQUARTERS    │
                    │   (Root Authority)│
                    └──────┬───────────┘
                           │
           ┌───────────────┼───────────────┐
           │               │               │
    ┌──────▼──────┐ ┌──────▼──────┐ ┌──────▼──────┐
    │  BRANCH A   │ │  BRANCH B   │ │  BRANCH C   │
    │  (Region 1) │ │  (Region 2) │ │  (Region 3) │
    └──────┬──────┘ └──────┬──────┘ └──────┬──────┘
           │               │               │
    ┌──────▼──────┐       ...             ...
    │ DEPARTMENT  │
    │  Finance    │
    └─────────────┘
```

Communication flows are **authority-gated**:

- **Vertical (Up/Down):** Employees communicate with managers within their department. Managers escalate to branch leadership. Branch leadership reports to headquarters.
- **Horizontal (Peer):** Employees within the same department and authority tier communicate freely. Cross-department requires shared workspace authorization.
- **Inter-Branch:** Branch A communicates with Branch B through headquarters-authorized channels. No direct branch-to-branch bypass.
- **External:** TeraChat does **NOT** support customer-facing messaging. We cannot force customers to switch messaging platforms.

## Why This Model

- **Authority mirrors org structure:** The messaging graph matches the company org chart. An employee's communication scope is bounded by their position in the hierarchy.
- **No shadow IT:** Cross-branch communication is visible to headquarters. No hidden backchannels form.
- **Compliance by design:** Legal hold, audit, and data retention map directly to organizational units. Governance follows the same hierarchy.
- **Customer reality:** Enterprises already have customer communication channels (email, phone, support desks). TeraChat doesn't compete with those — it replaces internal Slack/Teams while respecting the boundary between internal ops and customer touchpoints.

## Messaging Scope Matrix

| Communication | Supported | Scope Boundary |
|---------------|-----------|----------------|
| Employee → Manager (same dept) | Yes | Department authority chain |
| Manager → Employee (same dept) | Yes | Department authority chain |
| Peer → Peer (same dept) | Yes | Department workspace |
| Dept A → Dept B (same branch) | Yes | Branch workspace + authorization |
| Branch A → Branch B | Yes | HQ-authorized inter-branch channel |
| Branch → Headquarters | Yes | Root authority channel |
| Employee → Customer | **No** | Out of scope |
| Branch → External Partner | **No** | Use existing customer channels |
| Public / Anonymous | **No** | Not supported |

## Authority-Gated Workspace Creation

Workspaces (channels, groups) inherit authority from their creator:

- A department head creates a workspace → scope is their department + subordinates
- A branch director creates a workspace → scope is the entire branch
- Headquarters creates a workspace → scope can span multiple branches (inter-branch)
- Authority scope CANNOT be widened after creation (immutable authority ceiling)

## 🧠 Design Decisions (Q&A)

- **Why no customer messaging?** → TeraChat cannot solve the network coordination problem of forcing external customers onto a new platform. Email, phone, and support desks already serve this function. TeraChat focuses on the internal coordination problem where the enterprise controls both endpoints. Trade-off: enterprises still need existing customer-facing tools — TeraChat complements, doesn't replace them.

- **Why authority-gated workspaces instead of open channels?** → Open channels (like Slack #general) create information sprawl and compliance risk. Authority-gating ensures every communication path is traceable to an organizational decision. Trade-off: less spontaneous cross-team interaction, but higher governance confidence.

- **Why inter-branch through headquarters?** → Direct branch-to-branch communication without HQ visibility creates shadow governance. HQ-authorized inter-branch channels ensure multi-branch enterprises maintain a single source of truth about who communicates with whom. Trade-off: adds latency to inter-branch channel creation.
