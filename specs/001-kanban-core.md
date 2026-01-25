# SPEC.md: Project Reaper - The Kanban Forge

**Version:** 1.0 (Submarine-Alpha)
**Codename:** Octopus of Chaos
**Objective:** Build a rugged, minimalist Kanban board for "Carpenters" (agent-fleet operators) using Rust, HTMX, and SQLite.

---

## 1. System Philosophy
This is not a project management tool; it is an industrial dashboard for a silicon pirate swarm. It prioritizes **velocity, tactile feedback, and hull integrity** over decorative features.

## 2. Technical Stack
- **Backend:** Rust (Axum, SQLx)
- **Database:** SQLite (Flat table state + immutable delta_log)
- **Frontend:** HTMX (Rugged swaps + OOB sigil heartbeats)
- **Styling:** Tailwind CSS (Aesthetic: Submarine Industrial / Brutalist)
- **Isolation:** Docker (Sprites operate in "Caves")

## 3. Data Entities

### 3.1 The Note (The Atomic Unit)
- **ID:** Entropy_Signature (UUID/ULID)
- **Title:** The intent (Plain text)
- **Color:** Soul_Affinity (Hex code or Tailwind class mapping)
- **Position:** Spatial priority (Ordered within a WIP Group)
- **Status:** (Draft, Active, Done)

### 3.2 The WIP Group
- **ID:** Unique identifier
- **Name:** Group designation
- **Position:** Column order on the board

### 3.3 The Sprite (The Tentacle)
- **Sigil:** Identity (e.g., "🦂A")
- **Status:** (Idle, Busy, Done, Failed)
- **Leash:** Reference to a WIP Group
- **Heartbeat:** Last seen timestamp (TTL monitored)

### 3.4 The Ledger (Event Log)
- **Event_ID:** Incrementing sequence
- **Timestamp:** ISO-8601
- **Event_Type:** (MOVE, CREATE, ASSIGN, FAIL, REWIND)
- **Payload:** JSON blob of state delta

---

## 4. Interaction Model

### 4.1 Rugged Swaps
- standard HTML `POST` and `GET` for most state changes.
- Minimal client-side JavaScript.
- URL-driven state where possible.

### 4.2 Real-Time Sigils (The Pulse)
- HTMX `hx-ext="sse"` or polling for OOB updates.
- Sprite sigils update dynamically to reflect the "Heartbeat" of the swarm.

---

## 5. Safety & Failure Domains

### 5.1 Heartbeat Watchdog (Depth-Crush)
- A background task monitors the `delta_log` and Sprite timestamps.
- If a Sprite's TTL expires, its sigil visually "crushes" (changes to failure state).

### 5.2 Clean-Room Protocol (Bulkhead Integrity)
- Before leashing to a new Note, the system triggers a `git clean -fd` equivalent in the Sprite's Cave.

### 5.3 The Red Handle (Emergency Blow)
- A prominent UI element to trigger a system-wide purge and re-summon of the current state from the Ledger.

---

## 6. Aesthetic: Submarine Industrial
- **Keywords:** RUGGED, KINETIC, UNFILTERED.
- **Visuals:** High density, monospace fonts, tactile buttons, "Sonar" re-sync indicators.
