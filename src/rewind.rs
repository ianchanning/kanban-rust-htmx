use sqlx::{SqlitePool, Executor};
use crate::ledger::{self, EventLog};
use crate::models::{EventType, Note, CreateNote, UpdateNote, WipGroup, CreateWipGroup, UpdateWipGroup, Sprite, CreateSprite, UpdateSpriteStatus};
use serde_json::Value;
use tracing::info;

pub async fn truncate_tables(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    info!("Truncating application tables...");
    sqlx::query!("DELETE FROM notes").execute(pool).await?;
    sqlx::query!("DELETE FROM wip_groups").execute(pool).await?;
    sqlx::query!("DELETE FROM sprites").execute(pool).await?;
    info!("Application tables truncated.");
    Ok(())
}

pub async fn replay_events(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    info!("Replaying events from ledger...");

    let events = sqlx::query_as!(
        EventLog,
        r#"SELECT id, timestamp, event_type, payload FROM event_log ORDER BY id"#
    )
    .fetch_all(pool)
    .await?;

    for event in events {
        match event.kind() {
            EventType::NoteCreated => {
                let note: Note = serde_json::from_str(&event.payload)
                    .map_err(|e| sqlx::Error::Decode("JSON deserialize error".into()))?;
                // Re-insert the note. We need to handle potential conflicts if IDs are not auto-incremented,
                // or if we rely on the DB to generate IDs. For SQLite, if we specify the ID, it will use it.
                // However, the original `Note::create` uses a sequence for position, and does not accept `id`.
                // This means we need a special "replay create" function or just raw SQL.
                // For now, let's use raw SQL for direct insertion with existing ID.
                // Note: This bypasses the `append_event` from the model's create.
                sqlx::query!(
                    r#"
                    INSERT INTO notes (id, title, color, wip_group_id, position, status, created_at, updated_at)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                    "#,
                    note.id,
                    note.title,
                    note.color,
                    note.wip_group_id,
                    note.position,
                    note.status,
                    note.created_at,
                    note.updated_at,
                )
                .execute(pool)
                .await?;
            },
            EventType::NoteUpdated => {
                let note: Note = serde_json::from_str(&event.payload)
                    .map_err(|e| sqlx::Error::Decode("JSON deserialize error".into()))?;
                sqlx::query!(
                    r#"
                    UPDATE notes
                    SET title = ?, color = ?, wip_group_id = ?, position = ?, status = ?, updated_at = ?
                    WHERE id = ?
                    "#,
                    note.title,
                    note.color,
                    note.wip_group_id,
                    note.position,
                    note.status,
                    note.updated_at,
                    note.id,
                )
                .execute(pool)
                .await?;
            },
            EventType::NoteDeleted => {
                let note: Note = serde_json::from_str(&event.payload)
                    .map_err(|e| sqlx::Error::Decode("JSON deserialize error".into()))?;
                sqlx::query!(r#"DELETE FROM notes WHERE id = ?"#, note.id)
                    .execute(pool)
                    .await?;
            },
            EventType::WipGroupCreated => {
                let wip_group: WipGroup = serde_json::from_str(&event.payload)
                    .map_err(|e| sqlx::Error::Decode("JSON deserialize error".into()))?;
                sqlx::query!(
                    r#"
                    INSERT INTO wip_groups (id, name, position, created_at, updated_at)
                    VALUES (?, ?, ?, ?, ?)
                    "#,
                    wip_group.id,
                    wip_group.name,
                    wip_group.position,
                    wip_group.created_at,
                    wip_group.updated_at,
                )
                .execute(pool)
                .await?;
            },
            EventType::WipGroupUpdated => {
                let wip_group: WipGroup = serde_json::from_str(&event.payload)
                    .map_err(|e| sqlx::Error::Decode("JSON deserialize error".into()))?;
                sqlx::query!(
                    r#"
                    UPDATE wip_groups
                    SET name = ?, position = ?, updated_at = ?
                    WHERE id = ?
                    "#,
                    wip_group.name,
                    wip_group.position,
                    wip_group.updated_at,
                    wip_group.id,
                )
                .execute(pool)
                .await?;
            },
            EventType::WipGroupDeleted => {
                let wip_group: WipGroup = serde_json::from_str(&event.payload)
                    .map_err(|e| sqlx::Error::Decode("JSON deserialize error".into()))?;
                sqlx::query!(r#"DELETE FROM wip_groups WHERE id = ?"#, wip_group.id)
                    .execute(pool)
                    .await?;
            },
            EventType::SpriteCreated => {
                let sprite: Sprite = serde_json::from_str(&event.payload)
                    .map_err(|e| sqlx::Error::Decode("JSON deserialize error".into()))?;
                sqlx::query!(
                    r#"
                    INSERT INTO sprites (id, sigil, status, wip_group_id, last_seen, created_at, updated_at)
                    VALUES (?, ?, ?, ?, ?, ?, ?)
                    "#,
                    sprite.id,
                    sprite.sigil,
                    sprite.status,
                    sprite.wip_group_id,
                    sprite.last_seen,
                    sprite.created_at,
                    sprite.updated_at,
                )
                .execute(pool)
                .await?;
            },
            EventType::SpriteUpdated => {
                let sprite: Sprite = serde_json::from_str(&event.payload)
                    .map_err(|e| sqlx::Error::Decode("JSON deserialize error".into()))?;
                sqlx::query!(
                    r#"
                    UPDATE sprites
                    SET sigil = ?, status = ?, wip_group_id = ?, last_seen = ?, updated_at = ?
                    WHERE id = ?
                    "#,
                    sprite.sigil,
                    sprite.status,
                    sprite.wip_group_id,
                    sprite.last_seen,
                    sprite.updated_at,
                    sprite.id,
                )
                .execute(pool)
                .await?;
            },
            EventType::SpriteDeleted => {
                let sprite: Sprite = serde_json::from_str(&event.payload)
                    .map_err(|e| sqlx::Error::Decode("JSON deserialize error".into()))?;
                sqlx::query!(r#"DELETE FROM sprites WHERE id = ?"#, sprite.id)
                    .execute(pool)
                    .await?;
            },
            EventType::Unknown(s) => {
                eprintln!("Unknown event type encountered during replay: {}", s);
            }
        }
    }
    info!("Events replayed successfully.");
    Ok(())
}

pub async fn rewind_state(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    truncate_tables(pool).await?;
    replay_events(pool).await?;
    Ok(())
}