# Souls of the Silicon Pirates: Project Reaper

**Codename:** Octopus of Chaos  
**Status:** Submarine-Alpha  
**Entity ID:** `lynx-alpha`

---

## 🏴‍☠️ The Mission

This is not a project management tool. It is an industrial dashboard for a silicon pirate swarm. Built for **velocity, tactile feedback, and hull integrity**, Project Reaper provides a rugged Kanban Forge where state is law and the Ledger is the ultimate authority.

Every action is recorded. Every failure is documented. Every state is rewindable.

---

## 🌊 Core Invariants

- **The Ledger is Absolute:** All state changes are append-only. If it’s not in the Ledger, it didn't happen.
- **UI is Reflection Only:** The frontend (HTMX) displays the system's soul but never holds the authority.
- **Sprites are Disposable:** Agents (Sprites) operate in isolated Caves. If they desync, they are purged.
- **The Red Handle:** At any moment, the system can be blown—purging volatile state and reconstructing the truth from the Ledger.

---

## 🛠 Technical Hull

- **Engine:** Rust (Axum, SQLx, Tokio)
- **Ballast:** SQLite (Immutable `event_log` + relational tables)
- **Scope:** HTMX (Low-latency, server-side reflection)
- **Skin:** Tailwind CSS (Submarine Industrial / Brutalist aesthetic)
- **Isolation:** Docker-isolated "Caves" for Sprite operations

---

## 🗂 System Architecture

### 1. The Note (Atomic Unit)
The smallest unit of intent. Titles and colors only. Position and status are emergent properties assigned by the board.

### 2. The WIP Group (The Bulkheads)
Columns that define the flow of work. Sprites are leashed to these groups to maintain focus.

### 3. The Sprite (The Tentacles)
Autonomous agents identified by unique sigils (e.g., 🦂A). They operate within **Caves**. 
- **Heartbeat:** TTL monitored. Inactivity leads to **Depth-Crush** (auto-purge).
- **Clean-Room Protocol:** Switching tasks triggers an immediate `git reset --hard` and `git clean -fd` to prevent context leakage.

### 4. The Ledger (The Black Box)
An append-only stream of JSON deltas.
- **MOVE / CREATE / ASSIGN / FAIL / REWIND**
- Sole authority for state reconstruction.

---

## 🕹 Operational Commands

### Summoning the Swarm
Lifecycle management is handled via the Mothership's authority path: `/root/mothership/lsprite.sh`.

```bash
# Build the Sprite image
/root/mothership/lsprite.sh build

# List active Caves and their status
/root/mothership/lsprite.sh ls

# Enter a specific Cave environment
/root/mothership/lsprite.sh in <cave_id>

# Emergency Blow (Admin)
/root/mothership/lsprite.sh reset-cave
```

### Running the Forge
The backend listens on port `3000`. Ensure CSS is built before launch.

```bash
# Build the Submarine Skin (Tailwind CSS)
npm run build:css

# Watch for UI changes (Optional)
npm run watch:css &

# Start the engine
cargo run

# Reconstruct state from Ledger
curl -X POST http://localhost:3000/api/admin/rewind

# Emergency Blow (The Red Handle)
curl -X POST http://localhost:3000/api/admin/emergency-blow
```

---

## ⚠️ Safety Protocols

- **Depth-Crush:** Sprite heartbeat exceeded. Sigil enters Failed state.
- **Bulkhead Failure:** Context leakage detected. Clean-room protocol engaged.
- **Signal Jamming:** UI desync. Trigger **Sonar Ping** to re-sync with the Ledger.

---

*Tame the chaos. Burn the logs. Build the future.*
