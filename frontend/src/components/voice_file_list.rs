use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::components::audio_player::AudioPlayer;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct VoiceFile {
    filename: String,
    path: String,
    size: i64,
    created_at: String,
}

#[component]
pub fn VoiceFileList(character_id: String, on_file_selected: EventHandler<String>) -> Element {
    let mut files = use_signal(|| Vec::<VoiceFile>::new());
    let mut is_loading = use_signal(|| false);
    let mut error_msg = use_signal(|| None::<String>);
    let mut editing_file = use_signal(|| None::<String>);
    let mut new_name = use_signal(|| String::new());
    let mut playing_file = use_signal(|| None::<String>);

    // Load files on mount
    use_effect(move || {
        spawn(async move {
            load_files(files, is_loading, error_msg).await;
        });
    });

    rsx! {
        div {
            style: "border: 1px solid #ccc; padding: 10px; margin-top: 10px; border-radius: 4px; background-color: #f9f9f9;",
            
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px;",
                h4 { style: "margin: 0;", "Voice Reference Files" }
                button {
                    style: "padding: 5px 10px; background-color: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 0.9em;",
                    onclick: move |_| {
                        spawn(async move {
                            load_files(files, is_loading, error_msg).await;
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

            if is_loading() {
                div { style: "text-align: center; padding: 20px;", "Loading files..." }
            } else if files().is_empty() {
                div { style: "text-align: center; padding: 20px; color: #666;", "No voice files yet. Upload a file or extract from YouTube." }
            } else {
                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",
                    for file in files() {
                        div {
                            key: "{file.path}",
                            style: "border: 1px solid #ddd; padding: 10px; border-radius: 4px; background-color: white;",
                            
                            if editing_file() == Some(file.path.clone()) {
                                // Edit mode
                                div {
                                    style: "display: flex; gap: 5px; align-items: center;",
                                    input {
                                        value: "{new_name}",
                                        oninput: move |e: FormEvent| new_name.set(e.value()),
                                        style: "flex-grow: 1; padding: 5px;"
                                    }
                                    button {
                                        style: "padding: 5px 10px; background-color: #28a745; color: white; border: none; border-radius: 4px; cursor: pointer;",
                                        onclick: {
                                            let file_path = file.path.clone();
                                            let name = new_name();
                                            move |_| {
                                                let fp = file_path.clone();
                                                let nn = name.clone();
                                                spawn(async move {
                                                    rename_file(fp, nn, files, is_loading, error_msg, editing_file).await;
                                                });
                                            }
                                        },
                                        "Save"
                                    }
                                    button {
                                        style: "padding: 5px 10px; background-color: #6c757d; color: white; border: none; border-radius: 4px; cursor: pointer;",
                                        onclick: move |_| {
                                            editing_file.set(None);
                                        },
                                        "Cancel"
                                    }
                                }
                            } else {
                                // View mode
                                div {
                                    style: "display: flex; justify-content: space-between; align-items: center;",
                                    div {
                                        style: "flex-grow: 1;",
                                        div { style: "font-weight: bold;", "{file.filename}" }
                                        div { style: "font-size: 0.85em; color: #666;", "{format_size(file.size)}" }
                                    }
                                    div {
                                        style: "display: flex; gap: 5px;",
                                        button {
                                            style: "padding: 5px 10px; background-color: #17a2b8; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 0.85em;",
                                            onclick: {
                                                let path = file.path.clone();
                                                move |_| {
                                                    on_file_selected.call(path.clone());
                                                }
                                            },
                                            "Use"
                                        }
                                        button {
                                            style: "padding: 5px 10px; background-color: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 0.85em;",
                                            onclick: {
                                                let path = file.path.clone();
                                                move |_| {
                                                    if playing_file() == Some(path.clone()) {
                                                        playing_file.set(None);
                                                    } else {
                                                        playing_file.set(Some(path.clone()));
                                                    }
                                                }
                                            },
                                            if playing_file() == Some(file.path.clone()) { "⏸ Stop" } else { "▶ Play" }
                                        }
                                        button {
                                            style: "padding: 5px 10px; background-color: #ffc107; color: black; border: none; border-radius: 4px; cursor: pointer; font-size: 0.85em;",
                                            onclick: {
                                                let fname = file.filename.clone();
                                                let fpath = file.path.clone();
                                                move |_| {
                                                    new_name.set(fname.clone());
                                                    editing_file.set(Some(fpath.clone()));
                                                }
                                            },
                                            "Rename"
                                        }
                                        button {
                                            style: "padding: 5px 10px; background-color: #dc3545; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 0.85em;",
                                            onclick: {
                                                let path = file.path.clone();
                                                move |_| {
                                                    if confirm("Are you sure you want to delete this file?") {
                                                        let fp = path.clone();
                                                        spawn(async move {
                                                            delete_file(fp, files, is_loading, error_msg).await;
                                                        });
                                                    }
                                                }
                                            },
                                            "Delete"
                                        }
                                    }
                                }
                                
                                if playing_file() == Some(file.path.clone()) {
                                    div {
                                        style: "margin-top: 8px;",
                                        AudioPlayer {
                                            audio_url: format!("http://localhost:8000/files/audio/{}", urlencoding::encode(&file.path))
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

async fn load_files(
    mut files: Signal<Vec<VoiceFile>>,
    mut is_loading: Signal<bool>,
    mut error_msg: Signal<Option<String>>
) {
    is_loading.set(true);
    error_msg.set(None);
    
    match reqwest::Client::new()
        .get("http://localhost:8000/files/voice-references")
        .send()
        .await
    {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<Vec<VoiceFile>>().await {
                    Ok(data) => {
                        files.set(data);
                    }
                    Err(e) => {
                        error_msg.set(Some(format!("Failed to parse response: {}", e)));
                    }
                }
            } else {
                error_msg.set(Some(format!("Server error: {}", resp.status())));
            }
        }
        Err(e) => {
            error_msg.set(Some(format!("Failed to load files: {}", e)));
        }
    }
    
    is_loading.set(false);
}

async fn rename_file(
    old_path: String,
    new_name: String,
    mut files: Signal<Vec<VoiceFile>>,
    mut is_loading: Signal<bool>,
    mut error_msg: Signal<Option<String>>,
    mut editing_file: Signal<Option<String>>
) {
    #[derive(Serialize)]
    struct RenameRequest {
        old_path: String,
        new_name: String,
    }
    
    let payload = RenameRequest { old_path, new_name };
    
    match reqwest::Client::new()
        .post("http://localhost:8000/files/rename")
        .json(&payload)
        .send()
        .await
    {
        Ok(resp) => {
            if resp.status().is_success() {
                editing_file.set(None);
                load_files(files, is_loading, error_msg).await;
            } else {
                match resp.text().await {
                    Ok(text) => error_msg.set(Some(format!("Rename failed: {}", text))),
                    Err(_) => error_msg.set(Some("Rename failed".to_string())),
                }
            }
        }
        Err(e) => {
            error_msg.set(Some(format!("Rename failed: {}", e)));
        }
    }
}

async fn delete_file(
    path: String,
    mut files: Signal<Vec<VoiceFile>>,
    mut is_loading: Signal<bool>,
    mut error_msg: Signal<Option<String>>
) {
    match reqwest::Client::new()
        .delete(format!("http://localhost:8000/files/voice-reference?path={}", urlencoding::encode(&path)))
        .send()
        .await
    {
        Ok(resp) => {
            if resp.status().is_success() {
                load_files(files, is_loading, error_msg).await;
            } else {
                error_msg.set(Some("Delete failed".to_string()));
            }
        }
        Err(e) => {
            error_msg.set(Some(format!("Delete failed: {}", e)));
        }
    }
}

fn format_size(bytes: i64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

fn confirm(message: &str) -> bool {
    // In a real app, you'd use a modal dialog
    // For now, we'll just return true (always confirm)
    // TODO: Implement proper confirmation dialog
    true
}
