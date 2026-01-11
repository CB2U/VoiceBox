use dioxus::prelude::*;
use crate::models::character::Character;
use crate::components::youtube_import::YouTubeImport;
use crate::components::voice_file_list::VoiceFileList;

#[component]
pub fn Editor(
    selected_char: Character,
    on_update: EventHandler<Character>,
    on_delete: EventHandler<String>, // Character ID
) -> Element {
    let char = selected_char;
    let char_id = char.id.clone();
    
    rsx! {
        div {
            key: "{char.id}",
            style: "flex-grow: 1; padding: 20px; display: flex; flex-direction: column; gap: 15px; overflow-y: auto;",
            
            div {
                label { "Name" }
                input {
                    value: "{char.name}",
                    oninput: {
                        let char = char.clone();
                        move |e: FormEvent| {
                            let mut c = char.clone();
                            c.name = e.value();
                            on_update.call(c);
                        }
                    }
                }
            }

            div {
                label { "Description" }
                textarea {
                    value: "{char.description}",
                    oninput: {
                        let char = char.clone();
                        move |e: FormEvent| {
                            let mut c = char.clone();
                            c.description = e.value();
                            on_update.call(c);
                        }
                    }
                }
            }

            div {
                label { "Voice Reference (File)" }
                div {
                    style: "display: flex; gap: 10px; align-items: center;",
                    input {
                        readonly: true,
                        value: "{char.voice_path.clone().unwrap_or_default()}",
                        style: "flex-grow: 1;"
                    }
                    button {
                        onclick: {
                            let char = char.clone();
                            move |_| {
                                let mut c = char.clone();
                                let on_update = on_update; 
                                spawn(async move {
                                        let path_opt = rfd::AsyncFileDialog::new()
                                        .add_filter("Audio", &["wav", "mp3"])
                                        .pick_file()
                                        .await;
                                        
                                        if let Some(handle) = path_opt {
                                            let path = handle.path().to_string_lossy().to_string();
                                            c.voice_path = Some(path);
                                            on_update.call(c);
                                        }
                                });
                            }
                        },
                        "Select File"
                    }
                }
            }
            
            // YouTube Import component
            YouTubeImport {
                character_id: char_id.clone(),
                on_success: {
                    let char = char.clone();
                    move |path: String| {
                        let mut c = char.clone();
                        c.voice_path = Some(path);
                        on_update.call(c);
                    }
                }
            }
            
            // Voice File List component
            VoiceFileList {
                character_id: char_id.clone(),
                on_file_selected: {
                    let char = char.clone();
                    move |path: String| {
                        let mut c = char.clone();
                        c.voice_path = Some(path);
                        on_update.call(c);
                    }
                }
            }

            div {
                    // Spacer
                    style: "flex-grow: 1;"
            }

            div {
                style: "display: flex; gap: 10px;",
                button {
                    style: "flex-grow: 1; background-color: #28a745; color: white; padding: 10px; border: none; border-radius: 4px; cursor: pointer;",
                    onclick: {
                        let char = char.clone();
                        move |_| on_update.call(char.clone())
                    },
                    "Save Character"
                }
                button {
                    style: "flex-grow: 1; background-color: #ff4d4d; color: white; padding: 10px; border: none; border-radius: 4px; cursor: pointer;",
                    onclick: move |_| on_delete.call(char_id.clone()),
                    "Delete Character"
                }
            }
        }
    }
}
