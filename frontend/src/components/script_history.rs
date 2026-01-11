use dioxus::prelude::*;
use crate::models::history::ScriptHistory;
use crate::services::history::{fetch_history, rename_history_entry, delete_history_entry};
use crate::components::audio_player::AudioPlayer;
use chrono::Local;

#[component]
pub fn ScriptHistoryPanel(on_load_script: EventHandler<String>) -> Element {
    let mut history = use_signal(|| Vec::<ScriptHistory>::new());
    let mut is_loading = use_signal(|| false);
    let mut error_msg = use_signal(|| None::<String>);
    let mut editing_entry = use_signal(|| None::<String>);
    let mut new_name = use_signal(|| String::new());
    let mut playing_entry = use_signal(|| None::<String>);
    let mut is_expanded = use_signal(|| false);

    // Load history on mount
    use_effect(move || {
        spawn(async move {
            load_history(history, is_loading, error_msg).await;
        });
    });

    rsx! {
        div {
            style: "border: 1px solid #ccc; padding: 15px; margin-top: 15px; border-radius: 4px; background-color: #f9f9f9;",
            
            // Header with expand/collapse
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px; cursor: pointer;",
                onclick: move |_| {
                    is_expanded.set(!is_expanded());
                },
                h3 {
                    style: "margin: 0; font-size: 18px; color: #333;",
                    if is_expanded() { "▼ " } else { "▶ " }
                    "Script History ({history().len()})"
                }
                button {
                    style: "padding: 5px 10px; background-color: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 0.9em;",
                    onclick: move |e| {
                        e.stop_propagation();
                        spawn(async move {
                            load_history(history, is_loading, error_msg).await;
                        });
                    },
                    "↻ Refresh"
                }
            }

            if let Some(msg) = error_msg() {
                div {
                    style: "color: #d32f2f; background-color: #ffebee; padding: 8px; margin-bottom: 10px; font-size: 0.9em; border-radius: 4px;",
                    "{msg}"
                }
            }

            if is_expanded() {
                if is_loading() {
                    div { style: "text-align: center; padding: 20px;", "Loading history..." }
                } else if history().is_empty() {
                    div {
                        style: "text-align: center; padding: 20px; color: #666;",
                        "No script history yet. Synthesize a script and save it to history."
                    }
                } else {
                    div {
                        style: "display: flex; flex-direction: column; gap: 10px; max-height: 400px; overflow-y: auto;",
                        for entry in history() {
                            div {
                                key: "{entry.id}",
                                style: "border: 1px solid #ddd; padding: 12px; border-radius: 4px; background-color: white;",
                                
                                if editing_entry() == Some(entry.id.clone()) {
                                    // Edit mode
                                    div {
                                        style: "display: flex; gap: 5px; align-items: center;",
                                        input {
                                            value: "{new_name}",
                                            oninput: move |e: FormEvent| new_name.set(e.value()),
                                            style: "flex-grow: 1; padding: 5px; border: 1px solid #ccc; border-radius: 4px;"
                                        }
                                        button {
                                            style: "padding: 5px 10px; background-color: #28a745; color: white; border: none; border-radius: 4px; cursor: pointer;",
                                            onclick: {
                                                let entry_id = entry.id.clone();
                                                let name = new_name();
                                                move |_| {
                                                    let id = entry_id.clone();
                                                    let nn = name.clone();
                                                    spawn(async move {
                                                        rename_entry(id, nn, history, is_loading, error_msg, editing_entry).await;
                                                    });
                                                }
                                            },
                                            "Save"
                                        }
                                        button {
                                            style: "padding: 5px 10px; background-color: #6c757d; color: white; border: none; border-radius: 4px; cursor: pointer;",
                                            onclick: move |_| {
                                                editing_entry.set(None);
                                            },
                                            "Cancel"
                                        }
                                    }
                                } else {
                                    // View mode
                                    div {
                                        style: "display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 8px;",
                                        div {
                                            style: "flex-grow: 1;",
                                            div { style: "font-weight: bold; font-size: 16px;", "{entry.name}" }
                                            div {
                                                style: "font-size: 0.85em; color: #666; margin-top: 2px;",
                                                {format_timestamp(&entry.created_at)}
                                            }
                                            div {
                                                style: "font-size: 0.9em; color: #555; margin-top: 5px; font-style: italic; max-height: 60px; overflow: hidden;",
                                                {truncate_text(&entry.script_text, 150)}
                                            }
                                        }
                                        div {
                                            style: "display: flex; gap: 5px; flex-wrap: wrap;",
                                            button {
                                                style: "padding: 5px 10px; background-color: #17a2b8; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 0.85em;",
                                                onclick: {
                                                    let script = entry.script_text.clone();
                                                    move |_| {
                                                        on_load_script.call(script.clone());
                                                    }
                                                },
                                                "Load"
                                            }
                                            button {
                                                style: "padding: 5px 10px; background-color: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 0.85em;",
                                                onclick: {
                                                    let entry_id = entry.id.clone();
                                                    move |_| {
                                                        if playing_entry() == Some(entry_id.clone()) {
                                                            playing_entry.set(None);
                                                        } else {
                                                            playing_entry.set(Some(entry_id.clone()));
                                                        }
                                                    }
                                                },
                                                if playing_entry() == Some(entry.id.clone()) { "⏸ Stop" } else { "▶ Play" }
                                            }
                                            button {
                                                style: "padding: 5px 10px; background-color: #ffc107; color: black; border: none; border-radius: 4px; cursor: pointer; font-size: 0.85em;",
                                                onclick: {
                                                    let name = entry.name.clone();
                                                    let id = entry.id.clone();
                                                    move |_| {
                                                        new_name.set(name.clone());
                                                        editing_entry.set(Some(id.clone()));
                                                    }
                                                },
                                                "Rename"
                                            }
                                            button {
                                                style: "padding: 5px 10px; background-color: #dc3545; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 0.85em;",
                                                onclick: {
                                                    let entry_id = entry.id.clone();
                                                    move |_| {
                                                        if confirm("Are you sure you want to delete this history entry?") {
                                                            let id = entry_id.clone();
                                                            spawn(async move {
                                                                delete_entry(id, history, is_loading, error_msg).await;
                                                            });
                                                        }
                                                    }
                                                },
                                                "Delete"
                                            }
                                        }
                                    }
                                    
                                    if playing_entry() == Some(entry.id.clone()) {
                                        div {
                                            style: "margin-top: 8px;",
                                            AudioPlayer {
                                                audio_url: format!("http://localhost:8000/files/audio/{}", urlencoding::encode(&entry.audio_path))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn load_history(
    mut history: Signal<Vec<ScriptHistory>>,
    mut is_loading: Signal<bool>,
    mut error_msg: Signal<Option<String>>
) {
    is_loading.set(true);
    error_msg.set(None);
    
    match fetch_history().await {
        Ok(data) => {
            history.set(data);
        }
        Err(e) => {
            error_msg.set(Some(e));
        }
    }
    
    is_loading.set(false);
}

async fn rename_entry(
    id: String,
    new_name: String,
    mut history: Signal<Vec<ScriptHistory>>,
    mut is_loading: Signal<bool>,
    mut error_msg: Signal<Option<String>>,
    mut editing_entry: Signal<Option<String>>
) {
    match rename_history_entry(id, new_name).await {
        Ok(_) => {
            editing_entry.set(None);
            load_history(history, is_loading, error_msg).await;
        }
        Err(e) => {
            error_msg.set(Some(e));
        }
    }
}

async fn delete_entry(
    id: String,
    mut history: Signal<Vec<ScriptHistory>>,
    mut is_loading: Signal<bool>,
    mut error_msg: Signal<Option<String>>
) {
    match delete_history_entry(id).await {
        Ok(_) => {
            load_history(history, is_loading, error_msg).await;
        }
        Err(e) => {
            error_msg.set(Some(e));
        }
    }
}

fn format_timestamp(dt: &chrono::DateTime<chrono::Utc>) -> String {
    let local = dt.with_timezone(&Local);
    local.format("%Y-%m-%d %I:%M %p").to_string()
}

fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len])
    }
}

fn confirm(_message: &str) -> bool {
    // In a real app, you'd use a modal dialog
    // For now, we'll just return true (always confirm)
    // TODO: Implement proper confirmation dialog
    true
}
