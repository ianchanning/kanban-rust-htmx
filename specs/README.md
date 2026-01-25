# Specs Index: The Submarine Ledger

This directory is the "Pin" for the Project Reaper context. It records the evolution of the **Octopus of Chaos** and its Kanban Forge. Every specification here is a snapshot of a conversation that shaped the clay.

---

## 1. System Synopsis (The Octopus & The Cave)
The "Octopus of Chaos" is a minimalist agent-fleet architecture. The **Host (Octopus)** manages **Sprites (Tentacles)** which operate inside isolated **Caves (Docker containers)**.

- **Mothership Repo:** `ianchanning/sprites-swarm`
- **Target Project:** `ianchanning/kanban-rust-htmx` (HTMX & Rust Kanban board)

### Infrastructure
- **The Bridge (`lsprite.sh`):** Orchestrates the building of Caves and identity generation.
- **The Heartbeat (`ralph.sh`):** The autonomous loop running inside the Sprite, driving development via `SPEC.md`.

---

## 2. Genesis: The Nyx Interview (2026-01-25)

The following "screenplay" records the first principles established during the initial design phase between the Lead Architect and the Nyx model.

### THE COURT SUMMARY
**OBJECTIVE:** Design and codify the "Team Board Pillars" for the Sprites swarm.

**FACTUAL FINDINGS:**
1. **Task Definition:** The "Atomic Unit" is a minimalist **Note** (Title + Color/Soul_Affinity). No weights, no dependencies.
2. **Naming Convention:** Sprites are identified by a **Sigil** (Animal Emoji + NATO Letter, e.g., 🦂A).
3. **Assignment Model:** Reversed. Notes are assigned to **WIP Groups**; Sprites are "leashed" to these groups.
4. **Interface Strategy:** **Base Tailwind CSS** with a "Gritty, Reactive, Brutalist" aesthetic (Submarine Industrial).
5. **Risk Management:** Established "Depth-Crush", "Bulkhead Failure", and "Signal Jamming" protocols. The **Red Handle** is the primary failsafe.

---

## 3. The Lookup Table (Context Pins)

Use these pins to guide the search tool and maintain context integrity.

| File | Designation | Description | Key Descriptors |
| :--- | :--- | :--- | :--- |
| `001-kanban-core.md` | Core Specification | The first principles of the Note, WIP Groups, and Sprites. | Atomic Unit, Note, Entropy_Signature, Soul_Affinity, Heartbeat. |
| `002-implementation-plan.md` | Strike Plan | The 5-phase execution strategy for the Rust/HTMX build. | Hull, Deck, Pulse, Ledger, Red Handle, Phase 1-5. |

---

## 4. Swarm Directives
- **Conversation Creates Specs:** Every major architectural shift must be recorded as a new numbered spec in this directory.
- **No Inventions:** Tentacles must adhere strictly to the "Submarine Ledger."
- **Ruggedness First:** If the code fights the browser or the OS, simplify the code.