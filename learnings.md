# Learnings & Decision Trace: Kanban Forge (Phase 1)

This document tracks the architectural decisions, pitfalls, and technical debt identified during the initial implementation (archived in `archive-master`). These learnings inform the TDD-driven second attempt.

## 1. Architectural Pitfalls

### 1.1 UI Refresh Conflict (The "Wiping Form" Bug)
- **Decision:** Used `hx-trigger="every 5s"` on the entire board container to provide "real-time" updates.
- **Outcome:** Catastrophic UX failure. Every 5 seconds, the entire board HTML was replaced, wiping out any text the user was currently typing into "Add Note" forms.
- **Learning:** Never replace parent containers that hold active user input via polling. Use **Out-of-Band (OOB) updates** for specific, non-input elements or move to a manual "Sonar Ping" refresh model for structural changes.

### 1.2 Data Persistence & The Ledger
- **Decision:** SQLite was used as the "Ledger" (source of truth).
- **Outcome:** The `kanban.db` was stored in the volatile workspace and lost during container resets.
- **Learning:** The system must be able to reconstruct state from migrations (Phoenix Protocol). For the second attempt, ensure the database initialization is robust and consider how to persist or safely recreate state.

### 1.3 HTMX Content-Type Mismatches
- **Decision:** Used standard Axum `Json` extractors for all POST/PUT routes.
- **Outcome:** HTMX sends form data as `application/x-www-form-urlencoded` by default, leading to `415 Unsupported Media Type` errors.
- **Learning:** Explicitly use `axum::extract::Form` for HTMX-driven form submissions.

## 2. Technical Debt

### 2.1 Lack of Testing
- **Observation:** Zero unit or integration tests were present in Phase 1.
- **Risk:** Regressions were common, especially in HTML fragment rendering and SQL queries.
- **Requirement for Phase 2:** Strictly follow **TDD**. Define expectations for model logic and HTML fragments before implementation.

### 2.2 Direct HTML String Interpolation
- **Observation:** Templates were built using raw `format!` macros with HTML strings.
- **Risk:** Difficult to maintain, prone to XSS (if not manually escaped), and hard to test.
- **Improvement:** Consider a more structured templating approach or at least unit test the HTML output of every fragment generator.

### 2.3 Tight Coupling of DB and Business Logic
- **Observation:** CRUD operations in `models.rs` were tightly coupled with `sqlx` and event logging.
- **Learning:** Abstracting the "Ledger" and "State" might make testing easier.

## 3. The "Sprite" Protocol
- **Discovery:** The concept of "Sprites" (autonomous agents) updating status via heartbeats was partially implemented but buggy.
- **Bug:** New columns wouldn't show the "Add Note" form because the `render_wip_group_card` function was inconsistent between the initial load and the HTMX fragment return.
- **Learning:** Fragments used by HTMX *must* be the exact same ones used in the initial page load to ensure consistency.

## 4. Implementation Strategy for Phase 2 (TDD Focus)

1.  **Test-First Models:** Write tests for `Note` and `WipGroup` logic (reordering, state transitions) using an in-memory SQLite database.
2.  **Fragment Testing:** Write tests that verify the generated HTML fragments contain the necessary `hx-` attributes and IDs.
3.  **Endpoint Testing:** Use `axum::test` (or similar) to verify routes return correct status codes and headers for HTMX.
4.  **Decoupled Polling:** Implement polling only for volatile "Sigil" elements (like Sprite statuses) using OOB swaps, leaving the board structure static until manual refresh or specific user actions.
