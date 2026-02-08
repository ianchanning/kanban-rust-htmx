# SPEC: Phase 6 - The Captain's Bridge (Active Control)

**Status:** Draft / Active Command
**Objective:** Transform the "Passive Sonar" dashboard into an "Active Command" center by adding interactive UI elements for Note and Column (WIP Group) management, while enforcing strict separation of concerns and security.

## 1. Requirement: Active Note Management
The current Kanban board is read-only. We must enable the creation and destruction of notes directly from the UI.

- **Add Note Form:** Each column (WIP Group) rendered by the backend must include a minimalist "Add Note" form at the bottom.
  - **Inputs:** A simple text input for the note content (Title).
  - **Backend Logic:**
    - **Color:** Assign a default color (e.g., `#FFFFFF` or a deterministic pastel based on ID) if not provided.
    - **Sanitization:** All user input must be HTML-escaped before rendering to prevent XSS.
  - **Action:** POST /api/notes via HTMX.
  - **Target:** The kanban-board-container should refresh or the note should be appended.
- **Delete Note Button:** Each note card must have a small "X" or "Delete" button.
  - **Action:** DELETE /api/notes/{id} via HTMX.
  - **Confirmation:** Use hx-confirm to prevent accidental deletions.

## 2. Requirement: Column (WIP Group) Management
We need the ability to add new stages to the board.

- **Add Column UI:** A form in the sidebar or at the end of the Kanban board to create a new WIP Group.
  - **Inputs:** Name of the group.
  - **Backend Logic:**
    - **Position:** The backend must calculate `MAX(position) + 1` to append the new group to the end.
    - **Sanitization:** HTML-escape the group name.
  - **Action:** POST /api/wip_groups via HTMX.

## 3. Requirement: Visual Feedback & State
- **Ledger Authority:** All state changes (Creates/Deletes) must be committed to the `event_log` (Ledger) first. The `progress.txt` artifact is a derivative that should be updated *from* the Ledger state, not by a separate agent.
- **HTMX Polish:** Use hx-swap and hx-target effectively to ensure the board feels responsive without full page reloads.

## 4. Implementation Checklist
- [x] **Architecture Refactor:** Create `src/templates.rs` (or `src/views.rs`). Move all HTML generation logic out of `src/main.rs`.
  - [x] Implement a `render_kanban_board` function in the new module.
  - [x] Implement robust HTML escaping for all dynamic strings (Note titles, WIP Group names).
- [x] **Backend Logic:** Update `src/main.rs` handlers.
  - [x] `create_note`: Handle default color assignment.
  - [x] `create_wip_group`: Handle automatic position calculation (if not already handled in `models.rs`).
  - [x] Ensure `POST` and `DELETE` handlers return appropriate HTMX fragments or triggers.
- [ ] **Frontend Update:** Update `public/index.html` if global styles/scripts are needed for the new interactive elements.
- [ ] **Verification:**
  - [ ] Run the server and manually test Note creation/deletion and Column creation.
  - [ ] **Security Test:** Attempt to inject `<script>alert('XSS')</script>` as a note title. Verify it renders as text, not code.

---
**NYX ENCRYPTED STATE:** (⊕) (⇌) (⁂)
> "The Bridge must be clean. The Hull must be solid. The Captain commands, but the Ship keeps the score."