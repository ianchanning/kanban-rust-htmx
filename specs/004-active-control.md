# SPEC: Phase 6 - The Captain's Bridge (Active Control)

**Status:** Draft / Active Command
**Objective:** Transform the "Passive Sonar" dashboard into an "Active Command" center by adding interactive UI elements for Note and Column (WIP Group) management.

## 1. Requirement: Active Note Management
The current Kanban board is read-only. We must enable the creation and destruction of notes directly from the UI.

- **Add Note Form:** Each column (WIP Group) rendered by /htmx/kanban-board must include a minimalist "Add Note" form at the bottom.
  - **Inputs:** A simple text input for the note content.
  - **Action:** POST /api/notes via HTMX.
  - **Target:** The kanban-board-container should refresh or the note should be appended.
- **Delete Note Button:** Each note card must have a small "X" or "Delete" button.
  - **Action:** DELETE /api/notes/{id} via HTMX.
  - **Confirmation:** Use hx-confirm to prevent accidental deletions.

## 2. Requirement: Column (WIP Group) Management
We need the ability to add new stages to the board.

- **Add Column UI:** A form in the sidebar or at the end of the Kanban board to create a new WIP Group.
  - **Inputs:** Name of the group.
  - **Action:** POST /api/wip_groups via HTMX.

## 3. Requirement: Visual Feedback & State
- **OOB Progress:** Ensure that adding or deleting notes appends to the event_log (Ledger) and updates the progress.txt (via the agent).
- **HTMX Polish:** Use hx-swap and hx-target effectively to ensure the board feels responsive without full page reloads.

## 4. Implementation Checklist
- [ ] Refactor src/main.rs: Update get_kanban_board_html to render forms and action buttons.
- [ ] Refactor src/main.rs: Ensure POST/DELETE handlers return appropriate HTMX triggers or fragments to refresh the UI.
- [ ] Update public/index.html: (If necessary) Add global handlers or styles for new interactive elements.
- [ ] Verify: Run the server and manually test Note creation/deletion and Column creation.

---
**NYX ENCRYPTED STATE:** (⊕) (⇌) (⁂)
> "A ship that cannot steer is just a floating coffin. Give the Captain the wheel."
