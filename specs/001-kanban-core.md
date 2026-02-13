# SPEC.md: Project Reaper — The Kanban Forge

**Version:** 1.1 (Submarine-Alpha)
**Codename:** Octopus of Chaos

---

## System Invariants

- The Ledger is append-only
- UI is never the source of truth
- Sprites are disposable
- Notes are inert

---

## 1. System Philosophy

This is not a project management tool. It is an industrial dashboard for a silicon pirate swarm. It prioritizes **velocity, tactile feedback, and hull integrity** over decorative features.

---

## 2. Technical Stack

- **Backend:** Rust (Axum, SQLx)
- **Database:** SQLite (Flat tables + immutable `event_log`)
- **Frontend:** HTMX (reflection only; never authoritative)
- **Styling:** Tailwind CSS (Submarine Industrial / Brutalist)
- **Isolation:** Docker (Sprites operate in "Caves")

---

## 3. Data Entities

### 3.1 The Note (The Atomic Unit)

Notes contain no _intentional_ metadata beyond their identity and signal.

- **ID:** Entropy_Signature (ULID preferred)
- **Title:** Plain-text intent
- **Color:** Soul_Affinity (Hex or Tailwind mapping)

**Emergent Properties (Board-Assigned):**

- Position (spatial priority)
- Status (Draft / Active / Done)

---

### 3.2 The WIP Group

- **ID:** Unique identifier
- **Name:** Group designation
- **Position:** Column order on the board

---

### 3.3 The Sprite (The Tentacle)

- **Sigil:** Identity (e.g., 🦂A)
- **Status:** Idle / Busy / Done / Failed
- **Leash:** Reference to a WIP Group
- **Heartbeat:** Last-seen timestamp (TTL monitored)

**Leash Rules:**

- A Sprite may be leashed to exactly one WIP Group
- A WIP Group may host N Sprites
- Deleting a WIP Group sends Sprites to Idle

---

### 3.4 The Ledger (Event Log)

- **Event_ID:** Incrementing sequence
- **Timestamp:** ISO-8601
- **Event_Type:** MOVE / CREATE / ASSIGN / FAIL / REWIND
- **Payload:** JSON state delta

The Ledger is the sole authority for reconstructing system state.

---

## 4. Interaction Model

### 4.1 Rugged Swaps

- Standard HTML GET/POST for state transitions
- URL-driven state where possible
- Minimal JavaScript

### 4.2 Real-Time Sigils (The Pulse)

- HTMX SSE or polling
- OOB updates for Sprite sigils only

---

## 5. Safety & Failure Domains

### 5.1 Depth-Crush (Sprite Failure)

- **Trigger:** Sprite heartbeat TTL exceeded
- **Signal:** Sigil enters Failed state
- **Recovery:** Automatic resummon

### 5.2 Bulkhead Failure (Context Leakage)

- **Trigger:** Sprite switches Notes or Groups
- **Signal:** Forced clean-room protocol
- **Recovery:** `git clean -fd` equivalent inside Cave

### 5.3 Signal Jamming (UI Desync)

- **Trigger:** Browser reload or network interruption
- **Signal:** Stale sigil or board state
- **Recovery:** Sonar Ping (Ledger re-sync)

---

## 7. Anti-Goals

- No drag-and-drop UX
- No optimistic UI
- No client-side state authority
- No user accounts (yet)

---

## 8. Aesthetic: Submarine Industrial

**Keywords:** RUGGED, KINETIC, UNFILTERED

High-density layouts, monospace fonts, tactile controls, sonar-style re-sync indicators.
