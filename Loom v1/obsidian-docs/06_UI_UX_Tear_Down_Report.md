---
title: UI/UX Tear-Down Report
tags: [ui, ux, design, audit, tailwind, opendesign, minimalism]
---

# 🎨 UI/UX Tear-Down Report

*Audited using web-researched Flat Design and Minimalism principles (avoiding loud gradients).*

## The Core Problem: Visual Exhaustion
The current Loom v1 interface is extremely "loud". It relies heavily on complex gradients and glowing shadows which create a messy visual hierarchy, distracting the user from the core functional tools.

## Root Cause Analysis
1. **Aggressive Gradients (`global.css` & `tailwind.config.cjs`):**
   - The app uses sweeping radial gradients like `--app-gradient: radial-gradient(1200px circle at top right, rgb(var(--color-primary) / 0.18)...)` on the main body.
   - Modals and cards use `--skill-modal-gradient` and `--skill-card-gradient`, stacking multiple gradients on top of each other.
   - **Impact:** Gradients are currently used to provide "depth", but when overused, they clash and make the UI feel claustrophobic and dated.
2. **Programmatic Canvas Glows (`avatarRender.ts`):**
   - Avatars are drawn dynamically using `ctx.createRadialGradient()`, adding even more glowing spheres to the interface.
3. **Glowing Box Shadows:**
   - Active elements use `boxShadow: glow: '0 0 20px -5px rgb(var(--color-primary) / 0.45)'`. 

## Pro Max / Minimalist Design Recommendations
According to modern minimalist design principles, a highly functional, task-oriented interface (like a developer tool) should prioritize **Clarity, Performance, and Simplicity**.

1. **Adopt Flat Design:** Eradicate all `radial-gradient` and `linear-gradient` usage. The app background should be a solid, flat color (e.g., `#0A0A0A` or `bg-neutral-900`).
2. **Use Hairline Borders for Depth:** Instead of glows and shadows, separate panels (Sidebar vs. Chat vs. Terminal) using crisp, 1px borders (`border-white/10`).
3. **Solid-Color Avatars:** Replace the noisy gradient canvas avatars with minimalist geometric icons or solid, muted background colors.
4. **Muted Functional Colors:** Strip out the high-saturation cyans/purples. Use muted tones for the UI, reserving highly saturated colors strictly for success/error states or primary call-to-action buttons.
