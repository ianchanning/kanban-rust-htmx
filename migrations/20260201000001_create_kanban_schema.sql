-- Create wip_groups table
CREATE TABLE wip_groups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE, -- Added UNIQUE constraint as was in my custom migration
    position INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create notes table
CREATE TABLE notes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    color TEXT NOT NULL DEFAULT '#cccccc',
    position INTEGER NOT NULL DEFAULT 0,
    wip_group_id INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT 'active', -- 'active', 'archived', 'deleted'
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(wip_group_id) REFERENCES wip_groups(id) ON DELETE CASCADE
);

-- Index for faster lookups by group
CREATE INDEX idx_notes_wip_group ON notes(wip_group_id);

-- Triggers for updated_at timestamps (added for consistency)
CREATE TRIGGER update_wip_groups_updated_at
AFTER UPDATE ON wip_groups
FOR EACH ROW
BEGIN
    UPDATE wip_groups SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;

CREATE TRIGGER update_notes_updated_at
AFTER UPDATE ON notes
FOR EACH ROW
BEGIN
    UPDATE notes SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;