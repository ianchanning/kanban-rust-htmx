use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool, Row};
use chrono::NaiveDateTime;

// Define EventType for ledger interaction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    NoteCreated,
    NoteUpdated,
    NoteDeleted,
    WipGroupCreated,
    WipGroupUpdated,
    WipGroupDeleted,
    SpriteCreated,
    SpriteUpdated,
    SpriteDeleted,
    // Add other event types as needed
    Unknown(String),
}


impl From<String> for EventType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "NOTE_CREATED" => EventType::NoteCreated,
            "NOTE_UPDATED" => EventType::NoteUpdated,
            "NOTE_DELETED" => EventType::NoteDeleted,
            "WIP_GROUP_CREATED" => EventType::WipGroupCreated,
            "WIP_GROUP_UPDATED" => EventType::WipGroupUpdated,
            "WIP_GROUP_DELETED" => EventType::WipGroupDeleted,
            "SPRITE_CREATED" => EventType::SpriteCreated,
            "SPRITE_UPDATED" => EventType::SpriteUpdated,
            "SPRITE_DELETED" => EventType::SpriteDeleted,
            _ => EventType::Unknown(s),
        }
    }
}

impl From<&str> for EventType {
    fn from(s: &str) -> Self {
        match s {
            "NOTE_CREATED" => EventType::NoteCreated,
            "NOTE_UPDATED" => EventType::NoteUpdated,
            "NOTE_DELETED" => EventType::NoteDeleted,
            "WIP_GROUP_CREATED" => EventType::WipGroupCreated,
            "WIP_GROUP_UPDATED" => EventType::WipGroupUpdated,
            "WIP_GROUP_DELETED" => EventType::WipGroupDeleted,
            "SPRITE_CREATED" => EventType::SpriteCreated,
            "SPRITE_UPDATED" => EventType::SpriteUpdated,
            "SPRITE_DELETED" => EventType::SpriteDeleted,
            _ => EventType::Unknown(s.to_string()),
        }
    }
}

// Implement Display for EventType
impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::NoteCreated => write!(f, "NOTE_CREATED"),
            EventType::NoteUpdated => write!(f, "NOTE_UPDATED"),
            EventType::NoteDeleted => write!(f, "NOTE_DELETED"),
            EventType::WipGroupCreated => write!(f, "WIP_GROUP_CREATED"),
            EventType::WipGroupUpdated => write!(f, "WIP_GROUP_UPDATED"),
            EventType::WipGroupDeleted => write!(f, "WIP_GROUP_DELETED"),
            EventType::SpriteCreated => write!(f, "SPRITE_CREATED"),
            EventType::SpriteUpdated => write!(f, "SPRITE_UPDATED"),
            EventType::SpriteDeleted => write!(f, "SPRITE_DELETED"),
            EventType::Unknown(s) => write!(f, "{}", s),
        }
    }
}


#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Note {
    pub id: i64,
    pub title: String,
    pub color: String,
    pub wip_group_id: i64,
    pub position: i64,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct WipGroup {
    pub id: i64,
    pub name: String,
    pub position: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateNote {
    pub title: String,
    pub color: String,
    pub wip_group_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateNote {
    pub title: Option<String>,
    pub color: Option<String>,
    pub wip_group_id: Option<i64>,
    pub position: Option<i64>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReorderNote {
    pub new_position: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateWipGroup {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateWipGroup {
    pub name: Option<String>,
    pub position: Option<i64>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Sprite {
    pub id: String, // ULID
    pub sigil: String,
    pub status: String,
    pub wip_group_id: Option<String>,
    pub last_seen: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateSprite {
    pub id: String,
    pub sigil: String,
    pub wip_group_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateSpriteStatus {
    pub status: String,
}

// CRUD operations for Sprite
impl Sprite {
    pub async fn create(
        pool: &SqlitePool,
        new_sprite: CreateSprite,
        event_type: EventType,
    ) -> Result<Sprite, sqlx::Error> {
        let mut tx = pool.begin().await?;
        let res = sqlx::query_as::<_, Sprite>(
            r#"
            INSERT INTO sprites (id, sigil, wip_group_id)
            VALUES (?, ?, ?)
            RETURNING id, sigil, status, wip_group_id, last_seen, created_at, updated_at
            "#
        )
        .bind(new_sprite.id)
        .bind(new_sprite.sigil)
        .bind(new_sprite.wip_group_id)
        .fetch_one(&mut *tx)
        .await?;

        // Append event to ledger
        crate::ledger::append_event(&mut *tx, &event_type, &res).await?;

        tx.commit().await?;
        Ok(res)
    }

    pub async fn find_all(pool: &SqlitePool) -> Result<Vec<Sprite>, sqlx::Error> {
        sqlx::query_as::<_, Sprite>(
            r#"
            SELECT id, sigil, status, wip_group_id, last_seen, created_at, updated_at
            FROM sprites
            "#
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_id<'e, E>(executor: E, id: &str) -> Result<Option<Sprite>, sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
    {
        sqlx::query_as::<_, Sprite>(
            r#"
            SELECT id, sigil, status, wip_group_id, last_seen, created_at, updated_at
            FROM sprites
            WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_optional(executor)
        .await
    }

    pub async fn update_status(
        pool: &SqlitePool,
        id: &str,
        status: String,
        event_type: EventType,
    ) -> Result<Option<Sprite>, sqlx::Error> {
        let mut tx = pool.begin().await?;
        
        let res = sqlx::query_as::<_, Sprite>(
            r#"
            UPDATE sprites
            SET status = ?, last_seen = CURRENT_TIMESTAMP
            WHERE id = ?
            RETURNING id, sigil, status, wip_group_id, last_seen, created_at, updated_at
            "#
        )
        .bind(status)
        .bind(id)
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(ref sprite) = res {
            // Append event to ledger
            crate::ledger::append_event(&mut *tx, &event_type, sprite).await?;
        }

        tx.commit().await?;
        Ok(res)
    }

    pub async fn update_heartbeat(pool: &SqlitePool, id: &str, event_type: EventType) -> Result<Option<Sprite>, sqlx::Error> {
        let mut tx = pool.begin().await?;

        let res = sqlx::query_as::<_, Sprite>(
            r#"
            UPDATE sprites
            SET last_seen = CURRENT_TIMESTAMP
            WHERE id = ?
            RETURNING id, sigil, status, wip_group_id, last_seen, created_at, updated_at
            "#
        )
        .bind(id)
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(ref sprite) = res {
            crate::ledger::append_event(&mut *tx, &event_type, sprite).await?;
        }

        tx.commit().await?;
        Ok(res)
    }

    pub async fn find_by_wip_group_id(
        pool: &SqlitePool,
        wip_group_id: i64,
    ) -> Result<Vec<Sprite>, sqlx::Error> {
        let wip_group_id_str = wip_group_id.to_string();
        sqlx::query_as::<_, Sprite>(
            r#"
            SELECT id, sigil, status, wip_group_id, last_seen, created_at, updated_at
            FROM sprites
            WHERE wip_group_id = ?
            "#
        )
        .bind(wip_group_id_str)
        .fetch_all(pool)
        .await
    }
}

// CRUD operations for Note
impl Note {
    pub async fn create(
        pool: &SqlitePool,
        new_note: CreateNote,
        event_type: EventType,
    ) -> Result<Note, sqlx::Error> {
        let mut tx = pool.begin().await?;
        let res = sqlx::query_as::<_, Note>(
            r#"
            INSERT INTO notes (title, color, wip_group_id, position)
            VALUES (?, ?, ?, (SELECT COALESCE(MAX(position), 0) + 1 FROM notes WHERE wip_group_id = ?))
            RETURNING id, title, color, wip_group_id, position, status, created_at, updated_at
            "#
        )
        .bind(new_note.title)
        .bind(new_note.color)
        .bind(new_note.wip_group_id)
        .bind(new_note.wip_group_id)
        .fetch_one(&mut *tx)
        .await?;

        // Append event to ledger
        crate::ledger::append_event(&mut *tx, &event_type, &res).await?;

        tx.commit().await?;
        Ok(res)
    }

    pub async fn find_all(pool: &SqlitePool) -> Result<Vec<Note>, sqlx::Error> {
        sqlx::query_as::<_, Note>(
            r#"
            SELECT id, title, color, wip_group_id, position, status, created_at, updated_at
            FROM notes
            ORDER BY position
            "#
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_id<'e, E>(executor: E, id: i64) -> Result<Option<Note>, sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
    {
        sqlx::query_as::<_, Note>(
            r#"
            SELECT id, title, color, wip_group_id, position, status, created_at, updated_at
            FROM notes
            WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_optional(executor)
        .await
    }

    pub async fn update(
        pool: &SqlitePool,
        id: i64,
        update_note: UpdateNote,
        event_type: EventType,
    ) -> Result<Option<Note>, sqlx::Error> {
        let mut tx = pool.begin().await?;
        
        let res = sqlx::query_as::<_, Note>(
            r#"
            UPDATE notes
            SET
                title = COALESCE(?, title),
                color = COALESCE(?, color),
                wip_group_id = COALESCE(?, wip_group_id),
                position = COALESCE(?, position),
                status = COALESCE(?, status)
            WHERE id = ?
            RETURNING id, title, color, wip_group_id, position, status, created_at, updated_at
            "#
        )
        .bind(update_note.title)
        .bind(update_note.color)
        .bind(update_note.wip_group_id)
        .bind(update_note.position)
        .bind(update_note.status)
        .bind(id)
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(ref note) = res {
            // Append event to ledger
            crate::ledger::append_event(&mut *tx, &event_type, note).await?;
        }

        tx.commit().await?;
        Ok(res)
    }

    pub async fn delete(pool: &SqlitePool, id: i64, event_type: EventType) -> Result<bool, sqlx::Error> {
        let mut tx = pool.begin().await?;
        let deleted_note = Self::find_by_id(&mut *tx, id).await?;
        let deleted_note = match deleted_note {
            Some(note) => note,
            None => return Ok(false),
        };

        let result = sqlx::query(
            r#"
            DELETE FROM notes
            WHERE id = ?
            "#
        )
        .bind(id)
        .execute(&mut *tx)
        .await?;

        if result.rows_affected() == 1 {
            // Append event to ledger
            crate::ledger::append_event(&mut *tx, &event_type, &deleted_note).await?;
            tx.commit().await?;
            Ok(true)
        } else {
            tx.rollback().await?;
            Ok(false)
        }
    }

    pub async fn reorder(
        pool: &SqlitePool,
        id: i64,
        new_position: i64,
        event_type: EventType,
    ) -> Result<Option<Note>, sqlx::Error> {
        let mut tx = pool.begin().await?;

        // 1. Get the current note to find its wip_group_id and current position
        let current_note = Self::find_by_id(&mut *tx, id).await?;
        let current_note = match current_note {
            Some(note) => note,
            None => return Ok(None),
        };
        let old_position = current_note.position;
        let wip_group_id = current_note.wip_group_id;

        // Ensure the new_position is non-negative
        let new_position = new_position.max(0);

        // 2. Adjust positions of other notes in the same wip_group
        if new_position < old_position {
            // Moving note up (smaller position number)
            sqlx::query(
                r#"
                UPDATE notes
                SET position = position + 1
                WHERE wip_group_id = ? AND position >= ? AND position < ? AND id != ?
                "#
            )
            .bind(wip_group_id)
            .bind(new_position)
            .bind(old_position)
            .bind(id)
            .execute(&mut *tx)
            .await?;
        } else if new_position > old_position {
            // Moving note down (larger position number)
            sqlx::query(
                r#"
                UPDATE notes
                SET position = position - 1
                WHERE wip_group_id = ? AND position > ? AND position <= ? AND id != ?
                "#
            )
            .bind(wip_group_id)
            .bind(old_position)
            .bind(new_position)
            .bind(id)
            .execute(&mut *tx)
            .await?;
        }

        // 3. Update the position of the target note
        let updated_note = sqlx::query_as::<_, Note>(
            r#"
            UPDATE notes
            SET position = ?
            WHERE id = ?
            RETURNING id, title, color, wip_group_id, position, status, created_at, updated_at
            "#
        )
        .bind(new_position)
        .bind(id)
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(ref note) = updated_note {
            // Append event to ledger
            crate::ledger::append_event(&mut *tx, &event_type, note).await?;
        }

        tx.commit().await?;
        Ok(updated_note)
    }

    pub async fn find_by_wip_group_id(
        pool: &SqlitePool,
        wip_group_id: i64,
    ) -> Result<Vec<Note>, sqlx::Error> {
        sqlx::query_as::<_, Note>(
            r#"
            SELECT id, title, color, wip_group_id, position, status, created_at, updated_at
            FROM notes
            WHERE wip_group_id = ?
            ORDER BY position
            "#
        )
        .bind(wip_group_id)
        .fetch_all(pool)
        .await
    }
}

// CRUD operations for WipGroup
impl WipGroup {
    pub async fn create(
        pool: &SqlitePool,
        new_wip_group: CreateWipGroup,
        event_type: EventType,
    ) -> Result<WipGroup, sqlx::Error> {
        let mut tx = pool.begin().await?;
        let res = sqlx::query_as::<_, WipGroup>(
            r#"
            INSERT INTO wip_groups (name, position)
            VALUES (?, (SELECT COALESCE(MAX(position), 0) + 1 FROM wip_groups))
            RETURNING id, name, position, created_at, updated_at
            "#
        )
        .bind(new_wip_group.name)
        .fetch_one(&mut *tx)
        .await?;

        // Append event to ledger
        crate::ledger::append_event(&mut *tx, &event_type, &res).await?;

        tx.commit().await?;
        Ok(res)
    }

    pub async fn find_all(pool: &SqlitePool) -> Result<Vec<WipGroup>, sqlx::Error> {
        sqlx::query_as::<_, WipGroup>(
            r#"
            SELECT id, name, position, created_at, updated_at
            FROM wip_groups
            ORDER BY position
            "#
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_id<'e, E>(executor: E, id: i64) -> Result<Option<WipGroup>, sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
    {
        sqlx::query_as::<_, WipGroup>(
            r#"
            SELECT id, name, position, created_at, updated_at
            FROM wip_groups
            WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_optional(executor)
        .await
    }

    pub async fn update(
        pool: &SqlitePool,
        id: i64,
        update_wip_group: UpdateWipGroup,
        event_type: EventType,
    ) -> Result<Option<WipGroup>, sqlx::Error> {
        let mut tx = pool.begin().await?;
        
        let res = sqlx::query_as::<_, WipGroup>(
            r#"
            UPDATE wip_groups
            SET
                name = COALESCE(?, name),
                position = COALESCE(?, position)
            WHERE id = ?
            RETURNING id, name, position, created_at, updated_at
            "#
        )
        .bind(update_wip_group.name)
        .bind(update_wip_group.position)
        .bind(id)
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(ref wip_group) = res {
            // Append event to ledger
            crate::ledger::append_event(&mut *tx, &event_type, wip_group).await?;
        }

        tx.commit().await?;
        Ok(res)
    }

    pub async fn delete(pool: &SqlitePool, id: i64, event_type: EventType) -> Result<bool, sqlx::Error> {
        let mut tx = pool.begin().await?;
        let deleted_wip_group = Self::find_by_id(&mut *tx, id).await?;
        let deleted_wip_group = match deleted_wip_group {
            Some(wip_group) => wip_group,
            None => return Ok(false),
        };

        let result = sqlx::query(
            r#"
            DELETE FROM wip_groups
            WHERE id = ?
            "#
        )
        .bind(id)
        .execute(&mut *tx)
        .await?;

        if result.rows_affected() == 1 {
            // Append event to ledger
            crate::ledger::append_event(&mut *tx, &event_type, &deleted_wip_group).await?;
            tx.commit().await?;
            Ok(true)
        } else {
            tx.rollback().await?;
            Ok(false)
        }
    }
}