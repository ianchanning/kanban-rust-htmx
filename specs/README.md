# Specs Index: The Submarine Ledger

This directory is the **Pin** for the Project Reaper context. It records the evolution of the **Octopus of Chaos** and its Kanban Forge. Every specification here is a snapshot of a conversation that shaped the clay.

---

## Canon vs Narrative

Metaphor is used to _explain_ the system, not override it.

- **Bold monospace terms** are mechanically binding.
- When prose and tables disagree, **tables win**.
- Flavor may drift; specs must not.

---

## 1. System Synopsis (The Octopus & The Cave)

The **Octopus of Chaos** is a minimalist agent-fleet architecture. The **Host (Octopus)** manages **Sprites (Tentacles)** which operate inside isolated **Caves (Docker containers)**.

- **Mothership Repo:** `ianchanning/sprites-swarm`
- **Target Project:** `ianchanning/kanban-rust-htmx`

### Infrastructure

- **The Bridge (`lsprite.sh`)** — orchestrates the building of Caves and identity generation.
- **The Heartbeat (`ralph.sh`)** — the autonomous loop running inside the Sprite, driving development via `SPEC.md`.

---

## 2. Genesis: The Nyx Interview (2026-01-25)

### THE COURT SUMMARY

**OBJECTIVE:** Design and codify the "Team Board Pillars" for the Sprites swarm.

**FACTUAL FINDINGS:**

1. **Task Definition:** The "Atomic Unit" is a minimalist **Note** (Title + Color / Soul_Affinity). No intentional metadata.
2. **Naming Convention:** Sprites are identified by a **Sigil** (Animal Emoji + NATO Letter, e.g., 🦂A).
3. **Assignment Model:** Reversed. Notes are assigned to **WIP Groups**; Sprites are "leashed" to these groups.
4. **Interface Strategy:** **Base Tailwind CSS** with a Submarine Industrial / Brutalist aesthetic.
5. **Risk Management:** Established "Depth-Crush", "Bulkhead Failure", and "Signal Jamming" protocols. The **Red Handle** is the primary failsafe.

---

## 3. Glossary (Lock the Vocabulary)

| Term       | Mechanical Meaning                    |
| ---------- | ------------------------------------- |
| Host       | Controller process / repo owner       |
| Sprite     | Autonomous worker (Docker container)  |
| Cave       | Isolated execution environment        |
| Ledger     | Append-only `event_log`               |
| Red Handle | Manual system-wide reset and resummon |

---

## 4. Lookup Table (Context Pins)

| File                         | Designation        | Description                                         | Key Descriptors                           |
| :--------------------------- | :----------------- | :-------------------------------------------------- | :---------------------------------------- |
| `001-kanban-core.md`         | Core Specification | First principles of Notes, WIP Groups, and Sprites. | Atomic Unit, Entropy_Signature, Heartbeat |
| `002-implementation-plan.md` | Strike Plan        | Phased execution strategy for the Rust/HTMX build.  | Hull, Deck, Pulse, Ledger, Red Handle     |
| `003-post-purge-recovery.md` | Recovery Protocol  | Disaster recovery sequence for lost local DB state. | Phoenix Protocol, migrations, sqlx check  |
| `004-active-control.md`      | Active Command     | Interactive UI for Note and Column management.      | Phase 6, Captain's Bridge, templates.rs   |

---

## 5. Swarm Directives

- **Conversation Creates Specs:** Major architectural shifts must be recorded as new numbered specs.
- **No Inventions:** Sprites must adhere strictly to the Submarine Ledger.
- **Ruggedness First:** If the code fights the browser or the OS, simplify the code.

