use dioxus::prelude::*;

mod models;
mod services;
mod components;

use models::character::Character;
use services::persistence::{load_characters, save_characters};
use components::{sidebar::Sidebar, editor::Editor};

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    // State
    let mut characters = use_signal(|| load_characters());
    let mut selected_id = use_signal(|| None::<String>);

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

    rsx! {
        div {
            style: "display: flex; height: 100vh; width: 100vw; overflow: hidden; font-family: sans-serif;",
            
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
    }
}
