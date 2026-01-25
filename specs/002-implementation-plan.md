# IMPLEMENTATION_PLAN.md: The Kanban Forge Strike

**Objective:** Execute the build of `kanban-rust-htmx` in five logical phases.

---

## Phase 1: The Hull (Foundation)
- [ ] Initialize Rust workspace with `axum`, `sqlx`, and `tokio`.
- [ ] Configure SQLite with SQLx migrations.
- [ ] Setup `delta_log` table for light event-sourcing.
- [ ] Implement Tailwind CSS build pipeline (standalone CLI or Node-based).
- [ ] Basic "Hello Captain" route to verify the stack.

## Phase 2: The Deck (The Board)
- [ ] Create Database Schema: `notes`, `wip_groups`, `event_log`.
- [ ] Implement CRUD for `Notes` (Titles and Colors only).
- [ ] Build the Main Dashboard: Multi-column layout with Tailwind (Submarine Vibe).
- [ ] Implement Spatial Reordering: Simple reordering logic (move up/down).

## Phase 3: The Pulse (The Heartbeat)
- [ ] Create Sprite Registry: Track active sigils (🦂A, etc.).
- [ ] Implement HTMX OOB Updates: Endpoints for sigil status fragments.
- [ ] Build the Heartbeat Watchdog: Background task to check Sprite TTLs.
- [ ] Integrate Sigil UI: Display live status next to WIP Groups.

## Phase 4: The Ledger (The Safety Net)
- [ ] Implement Event Middleware: Every state change must append to `event_log`.
- [ ] Build the "Rewind" Logic: Functionality to reconstruct state from the ledger.
- [ ] Implement the "Sonar Ping": UI mechanism to re-sync state on reconnect.

## Phase 5: The Red Handle (Emergency Blow)
- [ ] Implement the "Red Handle" Trigger: UI button to reset state.
- [ ] Integration: Connect the Red Handle to `lsprite.sh` logic (if possible via environment/shell).
- [ ] Clean-Room Protocol: Automated `git reset --hard` hooks when switching notes.

---

## Deployment & Verification
- [ ] Execute `lsprite.sh build` to ensure image parity.
- [ ] Verify SQLx query safety in CI.
- [ ] Perform a "Depth-Crush" test (simulating Sprite failure).
