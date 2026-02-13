# IMPLEMENTATION_PLAN.md — The Kanban Forge Strike

**Objective:** Execute the build of `kanban-rust-htmx` using a TDD-driven, rugged, rewindable approach (Phase 2).

---

## Phase 0: The Ledger (Foundational Law)

- [x] Create `event_log` table (append-only)
- [ ] Define event schema and types in Rust models.
- [ ] Enforce write-only semantics at the DB layer where possible.

---

## Phase 1: The Hull (TDD Foundation)

- [ ] Initialize Rust workspace (Axum, SQLx, Tokio, Serde).
- [ ] Configure SQLite + migrations (Phoenix Protocol verification).
- [ ] **TDD Goal:** Implement `EventLog` model with tests for append-only behavior.
- [ ] Implement Tailwind CSS build pipeline.
- [ ] "Hello Captain" route (Verify Axum and HTMX basic handshake).

---

## Phase 2: The Deck (Core Entities)

- [ ] **TDD Goal:** Create `Note` and `WipGroup` models with in-memory SQLite tests.
- [ ] Implement Ledger integration: All mutations MUST append to `event_log`.
- [ ] Build Fragment Templates: `render_note`, `render_wip_group` (Shared between initial load and HTMX).
- [ ] **Constraint Check:** Verify "Add Note" forms are present in all column renders (Fix for `005-ui-refresh-bug.md`).

---

## Phase 3: The Pulse (The Heartbeat & OOB)

- [ ] Create Sprite Registry: Track active sigils (🦂A, etc., TTL).
- [ ] **HTMX Strategy:** Implement OOB (Out-of-Band) updates for Sprite sigils ONLY.
- [ ] **Constraint Check:** Remove full-board polling to prevent input loss (`005-ui-refresh-bug.md`).
- [ ] Build the Heartbeat Watchdog: Background task to check Sprite TTLs.

---

## Phase 4: The Safety Net & Control

- [ ] Implement the "Sonar Ping": Manual "Refresh Board" button to re-sync structural state.
- [ ] Build the "Rewind" Logic: Reconstruct state from the `event_log`.
- [ ] Implement Note deletion and WIP Group creation (Active Control).

---

## Deployment & Verification

- [ ] Build via `~/mothership/sandbox.sh build`.
- [ ] SQLx query verification in CI (`cargo check`).
- [ ] Simulated Sprite death (Depth-Crush test).
- [ ] TDD Suite: `cargo test` coverage for all models and fragment generators.

---

## Constraints (The Iron Laws)

- **UI Stability:** Never poll parent containers containing active user input (Fix `005-ui-refresh-bug.md`).
- **Ledger Authority:** UI is reflection, not authority. All state flows through the Ledger.
- **TDD First:** Write behavior tests before implementing model logic or HTML fragments.
- **HTMX Mismatch:** Use `axum::extract::Form` for HTMX POST requests to avoid 415 errors.
