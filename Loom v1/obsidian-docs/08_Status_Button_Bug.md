---
title: Status Button Bug Report
tags: [bug, css, ui, overflow, root-cause]
---

# 🐞 Status Button Bug Report

**Investigated via Systematic Debugging Protocol**

## 1. The Symptom
The user reported that clicking the Status button (located in the top left corner of the `SidebarNav.vue` avatar) either "does nothing or doesn't show properly."

## 2. Root Cause Investigation
After tracing the DOM structure and Vue reactivity in `SidebarNav.vue`, I confirmed the reactivity (`statusMenuOpen = true`) and event listeners (`handleClickOutside`) were perfectly functional. 

The bug was entirely CSS-based: **CSS Stacking Context & Clipping**.

1. The `<nav>` container element had the Tailwind class `md:overflow-y-auto`.
2. The Status dropdown menu was rendered inside this `<nav>` using `absolute` positioning, instructed to render outside the bounds of the sidebar (`left-full`).
3. **The Collision:** Whenever a parent container has `overflow-y-auto` (or `overflow-hidden`), any child element that overflows the horizontal bounds is automatically clipped by the browser engine. 

When the user clicked the button, the menu *was* opening, but it was being rendered in an invisible, clipped region outside the `88px` wide navigation bar.

## 3. The Fix
I updated `SidebarNav.vue` and removed the `md:overflow-y-auto` constraint. 
I replaced it with `md:overflow-visible`, which explicitly allows absolutely positioned children (like tooltips and dropdowns) to break out of the flex container without being violently cropped by the browser.

**Status:** Fixed and committed to the working directory.
