---
type: concept
created: 2026-05-15
updated: 2026-05-15
tags: [agents, ai, methodology, langgraph]
---

# Multi-Agent Harness

## Philosophy

> "You are the strategic architect. AI is the tactical programmer." — Matt Pocock

With one developer, the only way to cover TeraChat's scope is **AI agents doing the bulk of code generation, human doing architecture + review + integration.**

## Harness Architecture

```
┌──────────────────────────────────────────┐
│           HUMAN (Strategic Layer)         │
│  Architecture │ Security Review │ Customer│
└──────────────────┬───────────────────────┘
                   │ orchestrate
┌──────────────────▼───────────────────────┐
│       LangGraph Orchestrator              │
│  Grooming → TDD → Implement → Check → Sec│
└──┬──────────┬──────────┬─────────────────┘
   │          │          │
┌──▼──┐  ┌───▼──┐  ┌────▼──┐  ┌──────────┐
│Rust │  │ Test │  │Security│  │   Doc    │
│Agent│  │Agent │  │ Agent  │  │  Agent   │
│     │  │      │  │        │  │          │
│tc-  │  │cargo │  │invariant│  │wiki/     │
│crypto│  │nextest│ │ check  │  │update    │
└─────┘  └──────┘  └────────┘  └──────────┘
```

## Agent Types & Scope Boundaries

| Agent | Scope | Tools | Constraint |
|-------|-------|-------|------------|
| **Rust Agent** | `source/core/tc-*/**` | cargo, clippy | Cannot cross crate boundaries without review |
| **Test Agent** | `tests/`, `*_test.rs` | cargo nextest, proptest | Must follow TDD contract |
| **Security Agent** | All code | cargo miri, cargo audit, gitleaks | Veto power on invariant violations |
| **Doc Agent** | `docs/wiki/` | Obsidian CLI | Append-only on log.md |
| **Proto Agent** | `source/core/proto/**` | buf | Must pass buf breaking |
| **Review Agent** | All PRs | Git diff | Checks CLAUDE.md compliance |

## Workflow

### Grooming (Design First)
AI interviews to clarify design before writing code. Checks:
- Ubiquitous language consistency
- Deep module interface (≤ 5 public items)
- Invariant impact
- Spec reference
- TDD contract (3 key tests)

### TDD Contract (Tests Second)
Tests are written BEFORE implementation. Defines:
- Input/output types
- Invariant behavior
- Edge cases
- Performance bounds

### Implement (Code Third)
Agent implements within strict file scope. Must:
- Pass the TDD contract
- Compile with zero warnings
- Not introduce new dependencies

### Invariant Check (Gate)
Automated verification against CLAUDE.md:
- ZeroizeOnDrop on key structs
- No raw pointers in FFI
- No unwrap() in pub functions
- No SystemTime for TTL
- Module depth ≤ 7

### Security Review
Automated then human-reviewed:
- `cargo miri` for unsafe code
- `cargo audit` for dependencies
- `gitleaks` for secrets
- Constant-time comparison verification

## Example Daily Workflow

```
09:00 — Review PRs from yesterday (30 min)
        Claude Code: "Review PR #47, check CLAUDE.md compliance"

09:30 — Assign new task
        "Task: implement MlsGroup::add_member()
         Spec: TERA-CORE §4.3
         Output: tc-crypto/src/mls.rs
         Test: cargo test --test mls_add_member"

10:00 — Human does customer/business work
        Claude Code runs in background

12:00 — Review agent output (1 hour)
        Verify invariants, security, test quality

13:00 — Fix issues, request changes

14:00 — Merge if passing, assign next task

15:00 — Customer development

18:00 — End-of-day review, update wiki/log.md, plan tomorrow
```

## Human Role

- **Architecture decisions** that span multiple crates
- **Security sign-off** on crypto and key management
- **Customer development** and feedback collection
- **Integration testing** on real devices
- **Priority setting** — which slice, which feature next

AI agents implement within clear boundaries. Human keeps strategic control.

## Related Pages

- [[Deep Module Design]] — Interface design principle for agent-friendly code
- [[Vertical Slice Development]] — How slices map to agent tasks
- [[LangGraph Orchestrator]] — Python implementation of the agent graph
