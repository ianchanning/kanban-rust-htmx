use axum::response::{Html, IntoResponse};
use axum::extract::{State, Path};
use sqlx::SqlitePool;
use crate::models::{Note, WipGroup, Sprite, UpdateSpriteStatus, EventType};
use axum::http::StatusCode;


// A simple HTML escaping function. For production, consider a dedicated crate like `html_escape`.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&#x27;")
}

pub fn render_note_card(note: &Note) -> Html<String> {
    let button_class = "text-red-400 hover:text-red-600 text-lg font-bold";
    let hx_target_note_id = format!("#note-{}", note.id);

    Html(format!(
        r#"
        <div id="note-{0}" class="bg-gray-700 p-3 rounded-md mb-2 border border-gray-600 flex justify-between items-center" style="background-color: {1};">
            <p class="text-gray-200">{2}</p>
            <button hx-delete="/api/notes/{0}" hx-confirm="Are you sure you want to delete this note?" hx-swap="outerHTML swap:.2s" hx-target="{3}" class="{4}">
                &times;
            </button>
        </div>
        "#,
        note.id, note.color, html_escape(&note.title), hx_target_note_id, button_class
    ))
}

pub fn render_wip_group_card(wip_group: &WipGroup) -> Html<String> {
    Html(format!(
        r#"
        <div id="wip-group-{}" class="flex-1 bg-gray-800 p-4 rounded-lg shadow-lg border border-gray-700">
            <h2 class="text-2xl font-semibold mb-4 text-white">{}</h2>
            <!-- Placeholder for notes and sprites, these will be loaded dynamically or via OOB updates -->
            <div id="notes-for-wip-group-{}"></div>
            <div id="sprites-for-wip-group-{}"></div>
        </div>
        "#,
        wip_group.id, html_escape(&wip_group.name), wip_group.id, wip_group.id
    ))
}

pub async fn get_kanban_board_html(
    State(pool): State<SqlitePool>,
) -> impl IntoResponse {
    let mut html = String::new();
    match WipGroup::find_all(&pool).await {
        Ok(wip_groups) => {
            html.push_str(r#"<div class="flex space-x-4">"#); // Start flex container for columns
            for wip_group in wip_groups {
                html.push_str(&format!(
                    r#"
                    <div id="wip-group-{0}" class="flex-1 bg-gray-800 p-4 rounded-lg shadow-lg border border-gray-700">
                        <h2 class="text-2xl font-semibold mb-4 text-white">{1}</h2>
                        <div id="notes-for-wip-group-{0}">
                    "#,
                    wip_group.id, html_escape(&wip_group.name)
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
                                        <span class="font-mono text-lg">{0}</span>
                                        <span class="{1} w-2 h-2 rounded-full"></span>
                                    </div>
                                    "#,
                                    html_escape(&sprite.sigil), status_color
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
                            html.push_str(&render_note_card(&note).0);
                        }
                    },
                    Err(e) => eprintln!("Error fetching notes for WIP group {}: {:?}", wip_group.id, e),
                }
                html.push_str(r#"</div>"#); // End notes-for-wip-group div
                
                // Add Note Form
                let input_class = "w-full p-2 mb-2 bg-gray-600 border border-gray-500 rounded text-white placeholder-gray-400";
                let button_class = "w-full bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded";
                                                let hx_target_notes_for_wip_group = format!("#notes-for-wip-group-{}", wip_group.id);
                                                html.push_str(&format!(
                                                    r#"
                                                    <div class="mt-4 p-3 bg-gray-700 rounded-lg">
                                                        <h3 class="text-lg font-semibold mb-2 text-white">Add New Note</h3>
                                                        <form hx-post="/api/notes" hx-target="{0}" hx-swap="beforeend" hx-on::after-request="this.reset()">
                                                            <input type="hidden" name="wip_group_id" value="{1}">
                                                            <input type="text" name="title" placeholder="Note Title"
                                                                   class="{2}" required>
                                                            <input type="text" name="color" placeholder="Color (e.g., #RRGGBB)"
                                                                   class="{2}">
                                                            <button type="submit"
                                                                    class="{3}">
                                                                Add Note
                                                            </button>
                                                        </form>
                                                    </div>
                                                    "#,
                                                    hx_target_notes_for_wip_group, wip_group.id, input_class, button_class
                                                ));                html.push_str(r#"</div>"#); // End wip-group div
            }
            html.push_str(r#"</div>"#); // End flex container
            Html(html).into_response()
        },
        Err(e) => {
            eprintln!("Error fetching WIP groups: {:?}", e);
            Html("<div>Error loading Kanban board</div>".to_string()).into_response()
        }
    }
}

pub async fn get_sprite_statuses(
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
                    sprite.id, html_escape(&sprite.sigil), status_color, html_escape(&sprite.status)
                ));
            }
            Html(html).into_response()
        },
        Err(_) => Html("<div>Error loading sprites</div>".to_string()).into_response(),
    }
}

pub async fn update_sprite_status(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    axum::Json(payload): axum::Json<UpdateSpriteStatus>, // Use axum::Json to avoid conflict
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
                sprite.id, html_escape(&sprite.sigil), status_color, html_escape(&sprite.status)
            );
            Html(html).into_response()
        },
        Ok(None) => (StatusCode::NOT_FOUND, "Sprite not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get_sprite_fragment(
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
                sprite.id, html_escape(&sprite.sigil), status_color, html_escape(&sprite.status)
            );
            Html(html).into_response()
        },
        Ok(None) => (StatusCode::NOT_FOUND, "Sprite not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}