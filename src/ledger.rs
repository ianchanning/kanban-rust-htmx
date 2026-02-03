use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqliteExecutor, Result};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    MOVE,
    CREATE,
    ASSIGN,
    FAIL,
    REWIND,
    // Fallback for unknown types if schema evolves
    UNKNOWN(String),
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventType::MOVE => write!(f, "MOVE"),
            EventType::CREATE => write!(f, "CREATE"),
            EventType::ASSIGN => write!(f, "ASSIGN"),
            EventType::FAIL => write!(f, "FAIL"),
            EventType::REWIND => write!(f, "REWIND"),
            EventType::UNKNOWN(s) => write!(f, "{}", s),
        }
    }
}

impl From<String> for EventType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "MOVE" => EventType::MOVE,
            "CREATE" => EventType::CREATE,
            "ASSIGN" => EventType::ASSIGN,
            "FAIL" => EventType::FAIL,
            "REWIND" => EventType::REWIND,
            _ => EventType::UNKNOWN(s),
        }
    }
}

impl From<&str> for EventType {
    fn from(s: &str) -> Self {
        match s {
            "MOVE" => EventType::MOVE,
            "CREATE" => EventType::CREATE,
            "ASSIGN" => EventType::ASSIGN,
            "FAIL" => EventType::FAIL,
            "REWIND" => EventType::REWIND,
            _ => EventType::UNKNOWN(s.to_string()),
        }
    }
}

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
    E: SqliteExecutor<'e>,
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
