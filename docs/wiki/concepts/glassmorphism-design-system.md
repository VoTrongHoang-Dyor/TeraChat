---
type: concept
created: 2026-05-10
tags: [terachat, design, ui, glassmorphism, security-visible]
sources: [tera-design, tera-client-spec]
---

# Glassmorphism Design System

TeraChat's UI design language: security-visible, information-dense, zero-noise. The glass effect is not cosmetic — it carries security state information.

## Core Spec

```css
backdrop-filter: blur(20px);
background: rgba(255, 255, 255, 0.08);
border: 1px solid rgba(255, 255, 255, 0.12);
box-shadow: 0 20px 60px rgba(0, 0, 0, 0.25);
```

## Security-Visible Design

The UI explicitly visualizes cryptographic state at all times:

| Mode | Background | Indicator |
|------|-----------|-----------|
| Online | Glass Light | Blue #24A1DE |
| Encrypted Session | Glass blue + lock badge | MLS active |
| Mesh Mode | Dark Navy #0F172A | Radar Pulse HUD |
| Emergency Mesh | Dark + amber warning | EMDP active |
| License Warning (T-30) | Glass + amber banner | Admin only |
| License Degraded (T-0) | Amber tint + lock icon | Contact IT prompt |
| License Invalid | Charcoal #1A1A2E | Full screen lockout |

## IPC Signal → UI State Machine

UI is a **passive renderer**. Rust Core pushes state via IPC signals; UI never holds security logic:

```
Rust Core: CoreSignal::mesh_mode_active
          ↓
UI: Switch to Dark Navy + Radar HUD
```

## 🧠 Design Decisions (Q&A)

- **Why glassmorphism instead of flat/material design?** → Glass effect provides depth — security indicators (badges, locks, warnings) float above content. Flat design makes security state blend into content. Trade-off: GPU cost — requires hardware compositing, degrades gracefully on Tier B/C.
- **Why no WhatsApp-style bubble chat?** → Bubble chat prioritizes social/emotional expression (stickers, reactions, whitespace). TeraChat is an enterprise tool — data density and security status matter more. Layout: compact, information-first.
- **Why 3 distinct visual modes?** → A user must never be confused about whether their messages are secure. If the UI looked the same in Mesh Mode, users would assume E2EE still applies when it may not (EMDP fallback uses weaker crypto). Trade-off: visual discontinuity on mode switch.
