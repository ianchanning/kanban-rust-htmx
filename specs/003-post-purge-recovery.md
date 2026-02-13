# SPEC.md: Post-Purge Recovery

**Version:** 1.0
**Codename:** Phoenix Protocol

---

## 1. Context & Audit

**Date:** 2026-02-07
**Event:** Workspace purge following container recreation.
**Symptom:** `cargo run` fails with `error returned from database: (code: 14) unable to open database file`.

### Findings
- The project relies on `sqlx` for compile-time query verification.
- `sqlx` requires a live database connection (defined in `.env` as `sqlite:kanban.db`) to expand macros.
- The `kanban.db` file was located in the volatile workspace and was lost during the purge.
- `migrations/` directory is intact, allowing for full schema reconstruction.

---

## 2. Recovery Protocol

To restore the development environment, the following sequence must be executed:

1.  **Initialize Database:**
    - Command: `sqlx database create`
    - Effect: Creates an empty `kanban.db` file.

2.  **Apply Migrations:**
    - Command: `sqlx migrate run`
    - Effect: Reconstructs the schema (Event Log, Kanban Core, Sprite Registry) from the `migrations/` directory.

3.  **Verify Integrity:**
    - Command: `cargo check`
    - Effect: Confirms that `sqlx` macros can successfully connect to the DB and verify queries.

---

## 3. Resilience Notes

- **Volatile State:** The SQLite database is local and ephemeral in this container setup. It is not persisted across container recreations unless mounted from the host (which it currently appears not to be, or the mount was cleared).
- **The Ledger:** Since the database is the only source of truth (Ledger), losing it means losing all application history.
- **Automation:** This recovery sequence should be part of the standard `~/mothership/sandbox.sh` initialization if not already present.
