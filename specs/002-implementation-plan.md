# IMPLEMENTATION_PLAN.md — The Kanban Forge Strike

**Objective:** Execute the build of `kanban-rust-htmx` in rugged, rewindable phases.

---

## Phase 0: The Ledger (Foundational Law)

- [x] Create `event_log` table (append-only)
- [x] Define event schema and types
- [x] Enforce write-only semantics at the DB layer

---

## Phase 1: The Hull (Foundation)

- [x] Initialize Rust workspace (Axum, SQLx, Tokio)
- [x] Configure SQLite + migrations
- [x] Verify Ledger integration on all mutations
- [x] Implement Tailwind CSS build pipeline
- [x] "Hello Captain" route

---

## Phase 2: The Deck (The Board)

- [ ] Create Database Schema: `notes`, `wip_groups`.
- [ ] Implement CRUD for `Notes` (Titles and Colors only).
- [ ] Build the Main Dashboard: Multi-column layout with Tailwind (Submarine Vibe).
- [ ] Implement Spatial Reordering: Simple reordering logic (move up/down).

---

## Phase 3: The Pulse (The Heartbeat)

- [ ] Create Sprite Registry: Track active sigils (🦂A, etc., TTL)
- [ ] Implement HTMX OOB Updates: Endpoints for sigil status fragments.
- [ ] Build the Heartbeat Watchdog: Background task to check Sprite TTLs.
- [ ] Integrate Sigil UI: Display live status next to WIP Groups.

---

## Phase 4: The Safety Net

- [ ] Implement Event Middleware: Every state change must append to `event_log`.
- [ ] Build the "Rewind" Logic: Functionality to reconstruct state from the ledger.
- [ ] Implement the "Sonar Ping": UI mechanism to re-sync state on reconnect.

---

## Phase 5: The Red Handle (Emergency Blow)

- [ ] Implement the "Red Handle" Trigger: UI button to reset state.
- [ ] Backend Emergency Blow (stateless rebuild)
- [ ] Sprite Clean-Room Protocol: Automated `git reset --hard` hooks when switching notes.
- [ ] Optional Integration: Connect the Red Handle to `lsprite.sh` logic (if possible via environment/shell).

---

## Deployment & Verification

- [ ] Build via `lsprite.sh build`
- [ ] SQLx query verification in CI
- [ ] Simulated Sprite death (Depth-Crush test)
- [ ] Browser reload + network interruption tests

---

## Constraints

- HTMX must never perform primary state transitions
- UI is reflection, not authority
- All recovery paths must flow through the Ledger
