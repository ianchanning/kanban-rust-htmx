-- Create event_log table
CREATE TABLE event_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TEXT NOT NULL,
    event_type TEXT NOT NULL,
    payload TEXT NOT NULL
);

-- Enforce append-only semantics
CREATE TRIGGER prevent_update_event_log
BEFORE UPDATE ON event_log
BEGIN
    SELECT RAISE(FAIL, 'event_log is append-only');
END;

CREATE TRIGGER prevent_delete_event_log
BEFORE DELETE ON event_log
BEGIN
    SELECT RAISE(FAIL, 'event_log is append-only');
END;
