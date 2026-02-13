-- Create event_log table for append-only audit trail
CREATE TABLE IF NOT EXISTS event_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_type TEXT NOT NULL,
    payload TEXT NOT NULL, -- JSON representation of the change
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
