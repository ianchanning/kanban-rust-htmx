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
use tokio::time; // Added for heartbeat watchdog
use tower_http::services::ServeDir;
use tracing::info;
use std::time::Duration; // Added for heartbeat watchdog

mod ledger;
mod models;
mod rewind;

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
        // Admin Endpoints
        .route("/api/admin/rewind", post(admin_rewind)) // New Admin endpoint for rewind logic
        .route("/api/admin/emergency-blow", post(admin_emergency_blow)) // New Admin endpoint for emergency blow
        // HTMX Endpoints
        .route("/htmx/sprites", get(get_sprite_statuses))
        .route("/htmx/sprites/:id/status", put(update_sprite_status))
        .route("/htmx/sprites/:id", get(get_sprite_fragment))
        .route("/htmx/kanban-board", get(get_kanban_board_html)) // New HTMX route for the full board
        .with_state(pool)
        .fallback_service(ServeDir::new("public"));

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    info!("Listening on {}", listener.local_addr()?);

    // Spawn the heartbeat watchdog task
    let watchdog_pool = pool.clone();
    tokio::spawn(async move {
        heartbeat_watchdog(watchdog_pool).await;
    });

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
            Html(html).into_response()
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
            Html(html).into_response()
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
    let current_note = match Note::find_by_id(&pool, id).await {
        Ok(Some(note)) => note,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let old_wip_group_id = current_note.wip_group_id;

    match Note::update(&pool, id, payload.clone(), EventType::NoteUpdated).await {
        Ok(Some(note)) => {
            // Check if wip_group_id changed
            if let Some(new_wip_group_id) = payload.wip_group_id {
                if new_wip_group_id != old_wip_group_id {
                    tokio::spawn(async {
                        run_git_clean_room().await;
                    });
                }
            }
            (StatusCode::OK, Json(note)).into_response()
        },
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

async fn admin_rewind(
    State(pool): State<SqlitePool>,
) -> impl IntoResponse {
    match rewind::rewind_state(&pool).await {
        Ok(_) => (StatusCode::OK, "Rewind successful").into_response(),
        Err(e) => {
            eprintln!("Error during rewind: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Rewind failed: {}", e)).into_response()
        }
    }
}

async fn admin_emergency_blow(
    State(pool): State<SqlitePool>,
) -> impl IntoResponse {
    match rewind::truncate_tables(&pool).await {
        Ok(_) => {
            info!("Emergency Blow successful: All application data cleared.");

            // Optional Integration: Connect the Red Handle to lsprite.sh logic
            if let Ok(lsprite_path) = std::env::var("LSPRITE_SH_PATH") {
                info!("LSPRITE_SH_PATH is set. Attempting to execute {} reset-cave", lsprite_path);
                let output = tokio::process::Command::new(lsprite_path)
                    .arg("reset-cave") // Assuming lsprite.sh has a 'reset-cave' command
                    .output()
                    .await;

                match output {
                    Ok(output) => {
                        if !output.status.success() {
                            eprintln!("lsprite.sh reset-cave failed: {:?}", output);
                            eprintln!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
                            eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
                            return (StatusCode::INTERNAL_SERVER_ERROR, format!("Emergency Blow successful, but lsprite.sh reset-cave failed: {}", String::from_utf8_lossy(&output.stderr))).into_response();
                        } else {
                            info!("lsprite.sh reset-cave successful.");
                            info!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
                            info!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
                            return (StatusCode::OK, "Emergency Blow successful: All application data cleared and lsprite.sh reset-cave executed.").into_response();
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to execute lsprite.sh reset-cave: {:?}", e);
                        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Emergency Blow successful, but failed to execute lsprite.sh reset-cave: {}", e)).into_response();
                    }
                }
            }
            (StatusCode::OK, "Emergency Blow successful: All application data cleared.").into_response()
        },
        Err(e) => {
            eprintln!("Error during Emergency Blow: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Emergency Blow failed: {}", e)).into_response()
        }
    }
}

async fn get_kanban_board_html(
    State(pool): State<SqlitePool>,
) -> impl IntoResponse {
    let mut html = String::new();
    match WipGroup::find_all(&pool).await {
        Ok(wip_groups) => {
            html.push_str(r#"<div class="flex space-x-4">"#); // Start flex container for columns
            for wip_group in wip_groups {
                html.push_str(&format!(
                    r#"
                    <div id="wip-group-{}" class="flex-1 bg-gray-800 p-4 rounded-lg shadow-lg border border-gray-700">
                        <h2 class="text-2xl font-semibold mb-4 text-white">{}</h2>
                    "#,
                    wip_group.id, wip_group.name
                ));

                // Add sprites for this WIP group
                match Sprite::find_by_wip_group_id(&pool, wip_group.id).await {
                    Ok(sprites) => {
                        if !sprites.is_empty() {
                            html.push_str(r#"<div class="mb-4 flex flex-wrap gap-2">"#);
                            for sprite in sprites {
                                let status_color = match sprite.status.as_str() {
                                    "Idle" => "bg-gray-500",
                                    "Busy" => "bg-yellow-500",
                                    "Done" => "bg-green-500",
                                    "Failed" => "bg-red-500",
                                    _ => "bg-gray-600",
                                };
                                html.push_str(&format!(
                                    r#"
                                    <div class="flex items-center space-x-1 text-sm bg-gray-700 p-1 rounded-full pr-2">
                                        <span class="font-mono text-lg">{}</span>
                                        <span class="{} w-2 h-2 rounded-full"></span>
                                    </div>
                                    "#,
                                    sprite.sigil, status_color
                                ));
                            }
                            html.push_str(r#"</div>"#);
                        }
                    },
                    Err(e) => eprintln!("Error fetching sprites for WIP group {}: {:?}", wip_group.id, e),
                }

                // Add notes for this WIP group
                match Note::find_by_wip_group_id(&pool, wip_group.id).await {
                    Ok(notes) => {
                        for note in notes {
                            html.push_str(&format!(
                                r#"
                                <div class="bg-gray-700 p-3 rounded-md mb-2 border border-gray-600">
                                    <p class="text-gray-200">{}</p>
                                </div>
                                "#,
                                note.title
                            ));
                        }
                    },
                    Err(e) => eprintln!("Error fetching notes for WIP group {}: {:?}", wip_group.id, e),
                }
                html.push_str(r#"</div>"#); // End wip-group div
            }
            html.push_str(r#"</div>"#); // End flex container
            Html(html)
        },
        Err(e) => {
            eprintln!("Error fetching WIP groups: {:?}", e);
            Html("<div>Error loading Kanban board</div>".to_string())
        }
    }
}


// Heartbeat Watchdog Task
async fn heartbeat_watchdog(pool: SqlitePool) {
    let mut interval = time::interval(Duration::from_secs(30)); // Check every 30 seconds
    let expiration_threshold_minutes = 5; // Sprites expire after 5 minutes of no activity

    loop {
        interval.tick().await;
        info!("Heartbeat watchdog: Checking for expired sprites...");

        let minutes_ago = -expiration_threshold_minutes; // Use as i32
        let result = sqlx::query!(
            r#"
            DELETE FROM sprites
            WHERE last_seen < datetime('now', ? || ' minutes ago')
            "#,
            minutes_ago.to_string()
        )
        .execute(&pool)
        .await;

        match result {
            Ok(query_result) => {
                let rows_affected = query_result.rows_affected();
                if rows_affected > 0 {
                    info!("Heartbeat watchdog: Removed {} expired sprites.", rows_affected);
                } else {
                    info!("Heartbeat watchdog: No expired sprites found.");
                }
            }
            Err(e) => {
                eprintln!("Heartbeat watchdog: Error deleting expired sprites: {:?}", e);
            }
        }
    }
}

async fn run_git_clean_room() {
    info!("Executing Sprite Clean-Room Protocol: git reset --hard && git clean -fd");
    let output_reset = tokio::process::Command::new("git")
        .arg("reset")
        .arg("--hard")
        .output()
        .await;

    match output_reset {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("git reset --hard failed: {:?}", output);
                eprintln!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
                eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
            } else {
                info!("git reset --hard successful.");
                info!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
                info!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
        },
        Err(e) => {
            eprintln!("Failed to execute git reset --hard: {:?}", e);
        }
    }

    let output_clean = tokio::process::Command::new("git")
        .arg("clean")
        .arg("-fd")
        .output()
        .await;

    match output_clean {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("git clean -fd failed: {:?}", output);
                eprintln!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
                eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
            } else {
                info!("git clean -fd successful.");
                info!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
                info!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
        },
        Err(e) => {
            eprintln!("Failed to execute git clean -fd: {:?}", e);
        }
    }
}