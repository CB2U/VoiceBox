use dioxus::prelude::*;

mod models;
mod services;
mod components;
mod utils;

use models::character::Character;
use services::persistence::{load_characters, save_characters};
use services::api::check_backend_health;
use components::{sidebar::Sidebar, editor::Editor, script_editor::ScriptEditor};

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Characters,
    ScriptEditor,
}

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    // State
    let mut characters = use_signal(|| load_characters());
    let mut selected_id = use_signal(|| None::<String>);
    let mut active_tab = use_signal(|| Tab::Characters);
    let mut backend_status = use_signal(|| None::<bool>); // None = checking, Some(true) = online, Some(false) = offline

    // Check backend health on startup and periodically
    use_effect(move || {
        spawn(async move {
            loop {
                let is_healthy = check_backend_health().await;
                backend_status.set(Some(is_healthy));
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        });
    });

    // Autosave effect
    use_effect(move || {
        let chars = characters.read();
        if let Err(e) = save_characters(&chars) {
            println!("Error saving characters: {}", e);
        }
    });

    // Handlers
    let on_add = move |_| {
        let new_char = Character::new("New Character".to_string());
        let new_id = new_char.id.clone();
        characters.write().push(new_char);
        selected_id.set(Some(new_id));
    };

    let on_update = move |updated_char: Character| {
        let mut chars = characters.write();
        if let Some(idx) = chars.iter().position(|c| c.id == updated_char.id) {
            chars[idx] = updated_char;
        }
    };

    let on_delete = move |id: String| {
        let mut chars = characters.write();
        chars.retain(|c| c.id != id);
        if selected_id.read().as_deref() == Some(&id) {
            selected_id.set(None);
        }
    };
    
    // Derived state for Editor
    let selected_char = {
        let chars = characters.read();
        let sel = selected_id.read();
        sel.as_ref().and_then(|id| chars.iter().find(|c| c.id == *id).cloned())
    };
    
    let current_tab = active_tab();
    let is_characters_tab = current_tab == Tab::Characters;
    let is_script_tab = current_tab == Tab::ScriptEditor;
    
    let char_tab_bg = if is_characters_tab { "#34495e" } else { "#2c3e50" };
    let char_tab_border = if is_characters_tab { "3px solid #3498db" } else { "none" };
    let script_tab_bg = if is_script_tab { "#34495e" } else { "#2c3e50" };
    let script_tab_border = if is_script_tab { "3px solid #3498db" } else { "none" };

    // Backend status display
    let (status_text, status_color, status_bg) = match backend_status() {
        None => ("Checking...", "#856404", "#fff3cd"),
        Some(true) => ("Backend Online", "#155724", "#d4edda"),
        Some(false) => ("Backend Offline", "#721c24", "#f8d7da"),
    };
    
    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100vh; width: 100vw; overflow: hidden; font-family: sans-serif;",
            
            // Tab Navigation with Backend Status
            div {
                style: "display: flex; background-color: #2c3e50; color: white; padding: 0; align-items: center;",
                button {
                    style: "flex: 1; padding: 15px; border: none; background-color: {char_tab_bg}; color: white; cursor: pointer; font-size: 16px; border-bottom: {char_tab_border};",
                    onclick: move |_| active_tab.set(Tab::Characters),
                    "Character Management"
                }
                button {
                    style: "flex: 1; padding: 15px; border: none; background-color: {script_tab_bg}; color: white; cursor: pointer; font-size: 16px; border-bottom: {script_tab_border};",
                    onclick: move |_| active_tab.set(Tab::ScriptEditor),
                    "Script Editor"
                }
                div {
                    style: "padding: 8px 16px; margin: 8px 16px; background-color: {status_bg}; color: {status_color}; border-radius: 4px; font-size: 12px; font-weight: bold; white-space: nowrap;",
                    "{status_text}"
                }
            }
            
            // Tab Content
            div {
                style: "display: flex; flex-grow: 1; overflow: hidden;",
                
                if is_characters_tab {
                    Sidebar {
                        characters: characters,
                        selected_id: selected_id,
                        on_add: on_add,
                    }
                    
                    if let Some(char) = selected_char {
                        Editor {
                            selected_char: char,
                            on_update: on_update,
                            on_delete: on_delete,
                        }
                    } else {
                        div {
                            style: "flex-grow: 1; display: flex; align-items: center; justify-content: center; background-color: #fff;",
                            "Select a character to start editing"
                        }
                    }
                }
                
                if is_script_tab {
                    ScriptEditor {
                        characters: characters,
                    }
                }
            }
        }
    }
}
