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
| Online | Glass Light | Blue `#24A1DE` |
| Encrypted Session | Glass blue + lock badge | MLS active |
| Mesh Mode | Dark Navy `#0F172A` | Radar Pulse HUD |
| Emergency Mesh | Dark + amber warning | EMDP active |
| License Warning (T-30) | Glass + amber banner | Admin only |
| License Degraded (T-0) | Amber tint + lock icon | Contact IT prompt |
| License Invalid | Charcoal `#1A1A2E` | Full screen lockout |

## IPC Signal → UI State Machine

UI is a **passive renderer**. Rust Core pushes state via IPC signals; UI never holds security logic.

### Full Signal-to-Widget-State Mapping

| CoreSignalType | Widget State | Visual Mode | Priority |
|---------------|-------------|-------------|----------|
| ONLINE_CONNECTED | Green indicator + Online badge | Light Glass | SECURITY |
| OFFLINE_DETECTED | Red indicator + Offline banner | Light Glass + amber overlay | SECURITY |
| MESH_MODE_ACTIVE | Radar Pulse HUD + Dark Navy bg | Mesh Dark Navy | SECURITY |
| EMDP_ACTIVATED | Emergency amber HUD + countdown timer | Emergency Amber | SECURITY |
| EMDP_DEACTIVATED | Amber clear → return to prior mode | Transition (500ms) | HIGH |
| E2EE_SESSION_ESTABLISHED | Blue lock badge (solid) | Glass blue overlay | SECURITY |
| E2EE_SESSION_DEGRADED | Yellow lock badge (pulsing) | Glass amber overlay | SECURITY |
| MLS_EPOCH_ROTATED | Brief lock rotation animation (<200ms) | No change | NORMAL |
| PENDING_SECURE_CHANNEL | Gray lock + spinner | Glass gray | SECURITY |
| SECURE_CHANNEL_READY | Blue lock (solid) + brief glow | Glass blue | SECURITY |
| KEY_ESCROW_IN_PROGRESS | Key icon + progress ring | No change | HIGH |
| KEY_ESCROW_COMPLETE | Key icon → checkmark (500ms) | No change | NORMAL |
| LICENSE_VALID | No indicator (invisible) | No change | LOW |
| LICENSE_WARNING_T30 | Amber banner "License expires in 30 days" | Amber banner top | HIGH |
| LICENSE_DEGRADED_T0 | Amber tint + "Contact IT" prompt | Amber tint overlay | SECURITY |
| LICENSE_INVALID | Full screen charcoal lockout | Charcoal #1A1A2E | SECURITY |
| MEMORY_PRESSURE_WARNING | Memory indicator (yellow) | ui_emergency_mode: false → true transition | SECURITY |
| MEMORY_PURGE_INITIATED | "Secure Memory Purge" overlay (red) | GPU Tier C forced + red overlay | SECURITY |
| THERMAL_THROTTLING | Thermal indicator (orange) + suspended ops list | Orange overlay | HIGH |
| GPU_TIER_DOWNGRADE | Subtle blur reduction (user may not notice) | Tier transition | HIGH |
| WAL_LOCK_REQUEST | Sync paused indicator (subtle) | No change | NORMAL |
| WAL_FLUSH_ACK | Sync indicator clears | No change | NORMAL |
| WAL_LOCK_GRANTED | Sync indicator clears | No change | NORMAL |
| TAPP_LOADED | .tapp icon appears in sidebar | No change | NORMAL |
| TAPP_UNLOADED | .tapp icon removed from sidebar | No change | NORMAL |
| TAPP_FUEL_WARNING | .tapp icon + yellow fuel gauge | No change | HIGH |
| TAPP_KILLED | .tapp icon + red X → fade out (1s) | No change | HIGH |
| AI_INFERENCE_STARTED | AI indicator (pulsing blue dot) | No change | NORMAL |
| AI_INFERENCE_COMPLETE | AI indicator → checkmark (500ms) | No change | NORMAL |
| AI_ENDPOINT_UNAVAILABLE | AI indicator (red) + "AI unavailable" tooltip | No change | HIGH |
| SYNC_IN_PROGRESS | Sync spinner (subtle, bottom bar) | No change | LOW |
| SYNC_COMPLETE | Sync spinner → checkmark → fade (2s) | No change | LOW |
| SYNC_CONFLICT_DETECTED | "Conflict" badge on affected channel | No change | HIGH |
| DATAGRANT_PENDING_QUORUM | DataGrant icon + "Pending Approval" tooltip | No change | HIGH |
| DATAGRANT_ACTIVATED | DataGrant icon → green checkmark | No change | HIGH |
| DATAGRANT_REVOKED | DataGrant icon → red X + "Revoked" tooltip | No change | SECURITY |
| FCP_INITIATED | Key icon + progress ring | Glass overlay | SECURITY |
| FCP_VERIFIED | Key icon → double checkmark (green) | Glass blue | SECURITY |
| FCP_FAILED | Key icon → red X + "Verification Failed" alert | Glass red | SECURITY |

### GPU Tier Ladder

| Tier | Blur | Background Opacity | Border | Shadow | When |
|------|------|--------------------|--------|--------|------|
| **A** (Full) | 20px | 0.08 | 1px rgba(255,255,255,0.12) | 20px/60px | Desktop M1+, iPhone 12+ |
| **B** (Reduced) | 10px | 0.12 | 1px rgba(255,255,255,0.15) | 10px/30px | Older devices, thermal .fair |
| **C** (Fallback) | 0px | 0.95 solid | 1px rgba(255,255,255,0.20) | 4px/12px | Thermal .serious/.critical, SECURE MEMORY PURGE |

### Visual Modes

| Mode | Background | Glass Effect | Security Indicator | When Active |
|------|-----------|-------------|-------------------|-------------|
| **Light** | `rgba(255,255,255,0.08)` | Full blur (Tier A/B) | Blue dot | Online, normal operation |
| **Dark** | `rgba(15,23,42,0.95)` | Full blur (Tier A/B) | Blue lock | Encrypted session active |
| **High Contrast** | Solid `#0F172A` | No blur | High-vis amber/red badges | Accessibility preference or EMDP |
| **Security Overlay** | Solid charcoal `#1A1A2E` | None (Tier C) | Red "MEMORY PURGE" text | Memory pressure critical, license invalid |

## 🧠 Design Decisions (Q&A)

- **Why glassmorphism instead of flat/material design?** → Glass effect provides depth — security indicators (badges, locks, warnings) float above content. Flat design makes security state blend into content. Trade-off: GPU cost — requires hardware compositing, degrades gracefully on Tier B/C.
- **Why no WhatsApp-style bubble chat?** → Bubble chat prioritizes social/emotional expression (stickers, reactions, whitespace). TeraChat is an enterprise tool — data density and security status matter more. Layout: compact, information-first.
- **Why 3 distinct visual modes?** → A user must never be confused about whether their messages are secure. If the UI looked the same in Mesh Mode, users would assume E2EE still applies when it may not (EMDP fallback uses weaker crypto). Trade-off: visual discontinuity on mode switch.
