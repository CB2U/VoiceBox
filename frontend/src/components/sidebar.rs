use dioxus::prelude::*;
use crate::models::character::Character;

#[component]
pub fn Sidebar(
    characters: Signal<Vec<Character>>,
    selected_id: Signal<Option<String>>,
    on_add: EventHandler<()>,
) -> Element {
    let chars = characters.read();
    rsx! {
        div {
            class: "sidebar",
            style: "width: 250px; background-color: #f0f0f0; padding: 10px; border-right: 1px solid #ccc; height: 100vh; display: flex; flex-direction: column;",
            
            button {
                style: "margin-bottom: 10px; padding: 10px; width: 100%;",
                onclick: move |_| on_add.call(()),
                "+ Add Character"
            }

            div {
                style: "flex-grow: 1; overflow-y: auto;",
                for char in chars.iter() {
                    {
                        let id = char.id.clone();
                        rsx! {
                            div {
                                key: "{char.id}",
                                style: if selected_id.read().as_deref() == Some(char.id.as_str()) {
                                    "padding: 8px; margin-bottom: 4px; background-color: #007bff; color: white; cursor: pointer; border-radius: 4px;"
                                } else {
                                    "padding: 8px; margin-bottom: 4px; background-color: white; cursor: pointer; border-radius: 4px;"
                                },
                                onclick: move |_| selected_id.set(Some(id.clone())),
                                "{char.name}"
                            }
                        }
                    }
                }
            }
        }
    }
}
