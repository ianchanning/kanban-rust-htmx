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

- [x] Create Database Schema: `notes`, `wip_groups`.
- [x] Implement CRUD for `Notes` (Titles and Colors only).
- [x] Build the Main Dashboard: Multi-column layout with Tailwind (Submarine Vibe).
- [x] Implement Spatial Reordering: Simple reordering logic (move up/down).

---

## Phase 3: The Pulse (The Heartbeat)

- [x] Create Sprite Registry: Track active sigils (🦂A, etc., TTL)
- [x] Implement HTMX OOB Updates: Endpoints for sigil status fragments.
- [x] Build the Heartbeat Watchdog: Background task to check Sprite TTLs.
- [x] Integrate Sigil UI: Display live status next to WIP Groups.

---

## Phase 4: The Safety Net

- [x] Implement Event Middleware: Every state change must append to `event_log`.
- [x] Build the "Rewind" Logic: Functionality to reconstruct state from the ledger.
- [x] Implement the "Sonar Ping": UI mechanism to re-sync state on reconnect.

---

## Phase 5: The Red Handle (Emergency Blow)

- [x] Implement the "Red Handle" Trigger: UI button to reset state.
- [x] Backend Emergency Blow (stateless rebuild)
- [x] Sprite Clean-Room Protocol: Automated `git reset --hard` hooks when switching notes.
- [x] Optional Integration: Connect the Red Handle to `lsprite.sh` logic (if possible via environment/shell).

---

## Known Issues & Blockers (Submarine-Alpha) - RESOLVED

### B-001: Structural Integrity Breach (Compilation) - FIXED
The codebase is now fully functional and compiles correctly.
- [x] **SQLx Overrides Required:** Resolved by using `query_as::<_, T>` for runtime type mapping.
- [x] **Logic Gaps:** `ReorderNote` and other missing models implemented in `models.rs`.
- [x] **Async/Trait Friction:** `append_event` helper adjusted to accept `Executor`.
- [x] **Model/Schema Sync:** Aligned `Sprite` models with `TEXT` `wip_group_id` in DB.

### B-002: Mothership Desync (Infrastructure) - DOCUMENTED
- [x] **Path Desync:** Authoritative path documented as `/root/mothership/lsprite.sh`.
- [x] **Resolution Path:** `admin_emergency_blow` integrated with `LSPRITE_SH_PATH` env var.

---

## Deployment & Verification

- [x] Build via `lsprite.sh build`
- [ ] SQLx query verification in CI
- [ ] Simulated Sprite death (Depth-Crush test)
- [ ] Browser reload + network interruption tests

---

## Constraints

- HTMX must never perform primary state transitions
- UI is reflection, not authority
- All recovery paths must flow through the Ledger
