-- Create sprites table
CREATE TABLE IF NOT EXISTS sprites (
    id TEXT PRIMARY KEY NOT NULL,
    sigil TEXT UNIQUE NOT NULL,
    status TEXT NOT NULL DEFAULT 'Idle',
    wip_group_id TEXT,
    last_seen TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (wip_group_id) REFERENCES wip_groups(id) ON DELETE SET NULL
);

-- Create a trigger to update the updated_at column on each row update
CREATE TRIGGER IF NOT EXISTS update_sprites_updated_at
AFTER UPDATE ON sprites
FOR EACH ROW
BEGIN
    UPDATE sprites SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;