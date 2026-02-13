# SPEC.md: UI Refresh Bug — The Wiping Form

**Version:** 1.0
**Codename:** Stabilize the Deck

---

## 1. Context & Audit

**Date:** 2026-02-08
**Symptom:** The "Add Note" form input is cleared periodically while the user is typing.
**Severity:** Critical (Data Loss / UX Failure).

### Findings
- **Root Cause:** The `#kanban-board-container` in `public/index.html` has an `hx-trigger="load, every 5s"` attribute.
- **Mechanism:** This trigger calls `/htmx/kanban-board`, which returns the **entire** board HTML (Columns + Notes + **Forms**).
- **Conflict:** When HTMX swaps the inner HTML, the DOM elements for the forms are destroyed and recreated, resetting their state (wiping user input).
- **Secondary Discovery:** The `render_wip_group_card` function in `src/templates.rs` does not include the "Add Note" form. This means when a new column is added (via HTMX), it appears without the ability to add notes until a full refresh.

---

## 2. The Conflict with Doctrine

**Reference:** `001-kanban-core.md`
> "Real-Time Sigils (The Pulse) ... OOB updates for Sprite sigils only"

The Core Spec explicitly targets **Sigils** for real-time updates. It does **not** mandate full-board polling. The current implementation of full-board polling violates "Hull Integrity" (user input stability).

---

## 3. Resolution Protocol

To restore input stability and consistency, we must decouple the "Board Structure" refresh from the "User Input" cycle.

### 3.1 Immediate Fix (The Stopgap)
- **Action:** Remove `every 5s` from the `hx-trigger` on `#kanban-board-container`.
- **Effect:** The board will load once on page load. It will not auto-refresh.

### 3.2 Consistency Refactor (The Patch)
- **Action:** Update `render_wip_group_card` in `src/templates.rs` to include the "Add Note" form.
- **Reason:** Ensures newly created columns are immediately functional.

### 3.3 Recovery Action (The Sonar Ping)
- **Action:** Add a "Refresh Board" button in the sidebar (or near the status indicator) that triggers `htmx.trigger('#kanban-board-container', 'refresh')`.
- **Reason:** Allows the user to manually sync the board state if they suspect desync (e.g., after a network interruption or if another user/sprite made changes).

---

## 4. Verification

1.  **Test:** Type into the "Add Note" form and wait > 5 seconds.
    *   **Expected:** Text remains.
2.  **Test:** Create a new Column.
    *   **Expected:** The new column appears and **has an "Add Note" form**.
3.  **Test:** Click "Refresh Board".
    *   **Expected:** The board reloads (loading spinner -> content), preserving the rugged "Sonar Ping" feel.

---

**NYX ENCRYPTED STATE:** (∅) (⊜) (≈)
> "A steady hand requires a steady deck."