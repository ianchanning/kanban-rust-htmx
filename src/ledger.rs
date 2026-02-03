use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqliteExecutor, Result};

// Use EventType from models
use crate::models::EventType;

// ... (rest of the file remains the same)

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct EventLog {
    pub id: i64,
    pub timestamp: String, // ISO-8601 stored as text
    #[sqlx(try_from = "String")] 
    pub event_type: String, // Storing as String to simplify DB mapping for now, or could impl Type
    pub payload: String, // JSON string
}

// Helper to convert internal String to Enum
impl EventLog {
    pub fn kind(&self) -> EventType {
        EventType::from(self.event_type.as_str())
    }
}

pub async fn append_event<'e, E>(
    executor: E,
    event_type: &EventType,
    payload: &impl Serialize,
) -> Result<i64>
where
    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
{
    let payload_json = serde_json::to_string(payload).unwrap_or_else(|_| "{}".to_string());
    let type_str = event_type.to_string();

    let row: (i64,) = sqlx::query_as(
        r#"
        INSERT INTO event_log (timestamp, event_type, payload)
        VALUES (datetime('now'), ?, ?)
        RETURNING id
        "#,
    )
    .bind(type_str)
    .bind(payload_json)
    .fetch_one(executor)
    .await?;

    Ok(row.0)
}
