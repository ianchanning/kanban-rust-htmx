use axum::{
    routing::{get, post, put, delete},
    extract::{State, Path, Json},
    response::{IntoResponse, Html},
    http::StatusCode,
    Router,
};
use sqlx::sqlite::{SqlitePool};
use sqlx::migrate::Migrator;
use std::path::Path as FilePath; // Alias to avoid conflict with axum::extract::Path
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::info;

mod ledger;
mod models;

use models::{Note, CreateNote, UpdateNote, ReorderNote, WipGroup, CreateWipGroup, UpdateWipGroup, EventType, Sprite, UpdateSpriteStatus};

static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    if !database_url.starts_with("sqlite:") {
        panic!("Only sqlite is supported");
    }
    
    let db_path_str = database_url.trim_start_matches("sqlite:");
    if db_path_str != ":memory:" {
         let path = FilePath::new(db_path_str);
         if !path.exists() {
             info!("Database file not found. Creating...");
             std::fs::File::create(path)?;
         }
    }

    let pool = SqlitePool::connect(&database_url).await?;

    info!("Running migrations...");
    MIGRATOR.run(&pool).await?;
    info!("Migrations passed.");
    
    // Verify table existence
    let row: (i64,) = sqlx::query_as("SELECT count(*) FROM sqlite_master WHERE type='table' AND name='event_log'")
        .fetch_one(&pool)
        .await?;

    if row.0 == 1 {
        info!("VERIFICATION: event_log table exists.");
    } else {
        panic!("VERIFICATION FAILED: event_log table missing.");
    }

    let app = Router::new()
        .route("/api/notes", post(create_note).get(list_notes))
        .route("/api/notes/:id", get(get_note).put(update_note).delete(delete_note))
        .route("/api/notes/:id/reorder", put(reorder_note))
        .route("/api/wip_groups", post(create_wip_group).get(list_wip_groups))
        .route("/api/wip_groups/:id", get(get_wip_group).put(update_wip_group).delete(delete_wip_group))
        // HTMX Endpoints
        .route("/htmx/sprites", get(get_sprite_statuses))
        .route("/htmx/sprites/:id/status", put(update_sprite_status))
        .route("/htmx/sprites/:id", get(get_sprite_fragment))
        .with_state(pool)
        .fallback_service(ServeDir::new("public"));

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

// HTMX Handlers
async fn get_sprite_statuses(
    State(pool): State<SqlitePool>,
) -> impl IntoResponse {
    match Sprite::find_all(&pool).await {
        Ok(sprites) => {
            let mut html = String::new();
            for sprite in sprites {
                let status_color = match sprite.status.as_str() {
                    "Idle" => "bg-gray-500",
                    "Busy" => "bg-yellow-500",
                    "Done" => "bg-green-500",
                    "Failed" => "bg-red-500",
                    _ => "bg-gray-600",
                };
                html.push_str(&format!(
                    r#"<div id="sprite-{}" class="flex items-center space-x-2 p-1 border-b border-gray-600">
                        <span class="font-mono text-lg">{}</span>
                        <span class="{} w-3 h-3 rounded-full"></span>
                        <span class="text-sm text-gray-400">{}</span>
                    </div>"#,
                    sprite.id, sprite.sigil, status_color, sprite.status
                ));
            }
            Html(html)
        },
        Err(_) => Html("<div>Error loading sprites</div>".to_string()),
    }
}

async fn update_sprite_status(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateSpriteStatus>,
) -> impl IntoResponse {
    match Sprite::update_status(&pool, &id, payload.status, EventType::SpriteUpdated).await {
        Ok(Some(sprite)) => {
            let status_color = match sprite.status.as_str() {
                "Idle" => "bg-gray-500",
                "Busy" => "bg-yellow-500",
                "Done" => "bg-green-500",
                "Failed" => "bg-red-500",
                _ => "bg-gray-600",
            };
            let html = format!(
                r#"<div id="sprite-{}" class="flex items-center space-x-2 p-1 border-b border-gray-600" hx-swap-oob="true">
                    <span class="font-mono text-lg">{}</span>
                    <span class="{} w-3 h-3 rounded-full"></span>
                    <span class="text-sm text-gray-400">{}</span>
                </div>"#,
                sprite.id, sprite.sigil, status_color, sprite.status
            );
            Html(html)
        },
        Ok(None) => (StatusCode::NOT_FOUND, "Sprite not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn get_sprite_fragment(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match Sprite::find_by_id(&pool, &id).await {
        Ok(Some(sprite)) => {
            let status_color = match sprite.status.as_str() {
                "Idle" => "bg-gray-500",
                "Busy" => "bg-yellow-500",
                "Done" => "bg-green-500",
                "Failed" => "bg-red-500",
                _ => "bg-gray-600",
            };
            let html = format!(
                r#"<div id="sprite-{}" class="flex items-center space-x-2 p-1 border-b border-gray-600" hx-swap-oob="true">
                    <span class="font-mono text-lg">{}</span>
                    <span class="{} w-3 h-3 rounded-full"></span>
                    <span class="text-sm text-gray-400">{}</span>
                </div>"#,
                sprite.id, sprite.sigil, status_color, sprite.status
            );
            Html(html)
        },
        Ok(None) => (StatusCode::NOT_FOUND, "Sprite not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

// Note Handlers
async fn create_note(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateNote>,
) -> impl IntoResponse {
    match Note::create(&pool, payload, EventType::NoteCreated).await {
        Ok(note) => (StatusCode::CREATED, Json(note)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn list_notes(
    State(pool): State<SqlitePool>,
) -> impl IntoResponse {
    match Note::find_all(&pool).await {
        Ok(notes) => (StatusCode::OK, Json(notes)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn get_note(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match Note::find_by_id(&pool, id).await {
        Ok(Some(note)) => (StatusCode::OK, Json(note)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn update_note(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateNote>,
) -> impl IntoResponse {
    match Note::update(&pool, id, payload, EventType::NoteUpdated).await {
        Ok(Some(note)) => (StatusCode::OK, Json(note)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn delete_note(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match Note::delete(&pool, id, EventType::NoteDeleted).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn reorder_note(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<ReorderNote>,
) -> impl IntoResponse {
    match Note::reorder(&pool, id, payload.new_position, EventType::NoteUpdated).await {
        Ok(Some(note)) => (StatusCode::OK, Json(note)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

// WipGroup Handlers
async fn create_wip_group(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateWipGroup>,
) -> impl IntoResponse {
    match WipGroup::create(&pool, payload, EventType::WipGroupCreated).await {
        Ok(wip_group) => (StatusCode::CREATED, Json(wip_group)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn list_wip_groups(
    State(pool): State<SqlitePool>,
) -> impl IntoResponse {
    match WipGroup::find_all(&pool).await {
        Ok(wip_groups) => (StatusCode::OK, Json(wip_groups)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn get_wip_group(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match WipGroup::find_by_id(&pool, id).await {
        Ok(Some(wip_group)) => (StatusCode::OK, Json(wip_group)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn update_wip_group(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateWipGroup>,
) -> impl IntoResponse {
    match WipGroup::update(&pool, id, payload, EventType::WipGroupUpdated).await {
        Ok(Some(wip_group)) => (StatusCode::OK, Json(wip_group)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn delete_wip_group(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match WipGroup::delete(&pool, id, EventType::WipGroupDeleted).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}