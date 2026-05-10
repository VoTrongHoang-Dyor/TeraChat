---
type: source
created: 2026-05-10
tags: [terachat, design, ui, ux, glassmorphism]
sources: [raw/MD/Design.md]
depends_on: [tera-client-spec, tera-core-spec]
---

# TeraChat Design System (TERA-DESIGN)

Source: `raw/MD/Design.md` — v0.3.7, 2026-03-23.

## What It Covers

The UI/UX design contract for TeraChat: Glassmorphism spec, visual modes, animation timing, IPC signal-to-UI mappings, and admin console patterns. Binding constraints for all frontend implementation.

## Key Claims

- **Non-Negotiable Design Rules:** Violating any = design reject.
- **3 Visual Modes:** Online (Glass Light / Blue), Mesh Mode (Dark Navy / Radar Pulse), License Invalid (Charcoal / Amber banner).
- **Glassmorphism Spec:** `backdrop-filter: blur(20px)`, `rgba(255,255,255,0.08)` background, `0.12` border opacity.
- **Typography:** Inter (body), JetBrains Mono (mono), System (headings). No system font for body text.
- **Security-First:** UI is a passive renderer — Rust Core pushes state via IPC signals. UI never holds crypto logic.
- **IPC Signal → UI Mapping:** 10 signals defined (session_established, mesh_mode_active, memory_zeroize, crypto_shred, etc.)
- **Animation Timing:** 120ms (message send) to 500ms (mesh mode switch).
- **GPU Fallback Tiers:** A (hardware compositing) → B (software) → C (flat solid).
- **20 IPC signal-to-UI mappings** defined.

## Related Concepts

- [[Glassmorphism Design System]]
- [[Terachat Architecture Overview]]
- [[Zero-Knowledge Architecture]]
