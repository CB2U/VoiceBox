use dioxus::prelude::*;
use crate::models::project::Project;
use crate::services::api::{fetch_projects, create_project, update_settings, fetch_settings};

#[component]
pub fn ProjectSelector(on_project_changed: EventHandler<()>) -> Element {
    let mut projects = use_signal(|| Vec::<Project>::new());
    let mut active_project_id = use_signal(|| None::<String>);
    let mut show_create_modal = use_signal(|| false);
    let mut new_project_name = use_signal(|| String::new());

    // Load projects and current settings on mount
    use_effect(move || {
        spawn(async move {
            if let Ok(p) = fetch_projects().await {
                projects.set(p);
            }
            if let Ok(settings) = fetch_settings().await {
                active_project_id.set(settings.active_project_id);
            }
        });
    });

    let select_project = move |id: String| {
        let id_clone = id.clone();
        spawn(async move {
            if let Ok(mut settings) = fetch_settings().await {
                settings.active_project_id = Some(id_clone);
                if let Ok(_) = update_settings(settings).await {
                    active_project_id.set(Some(id.clone()));
                    on_project_changed.call(());
                }
            }
        });
    };

    let handle_create = move |_| {
        let name = new_project_name.read().clone();
        if name.is_empty() { return; }
        
        spawn(async move {
            match create_project(name).await {
                Ok(project) => {
                    println!("✨ Project created: {}", project.name);
                    projects.write().push(project.clone());
                    select_project(project.id);
                    show_create_modal.set(false);
                    new_project_name.set(String::new());
                }
                Err(e) => {
                    eprintln!("❌ Failed to create project: {}", e);
                }
            }
        });
    };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 10px; padding: 10px; background-color: #34495e; color: white;",
            
            span { style: "font-weight: bold;", "Campaign:" }
            
            select {
                style: "padding: 5px; border-radius: 4px; background-color: #2c3e50; color: white; border: 1px solid #7f8c8d;",
                value: active_project_id().unwrap_or_default(),
                onchange: move |evt| {
                    let val = evt.value();
                    if val == "NEW" {
                        show_create_modal.set(true);
                    } else if !val.is_empty() {
                        select_project(val);
                    }
                },
                
                option { value: "", disabled: true, "Select a Campaign" }
                for p in projects.read().iter() {
                    option { value: "{p.id}", selected: Some(p.id.clone()) == active_project_id(), "{p.name}" }
                }
                option { value: "NEW", "+ Create New Campaign..." }
            }

            if show_create_modal() {
                div {
                    style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 1000;",
                    div {
                        style: "background: #fff; padding: 20px; border-radius: 8px; color: #333; width: 300px;",
                        h3 { "New Campaign" }
                        input {
                            style: "width: 100%; padding: 8px; margin: 10px 0; border: 1px solid #ccc; border-radius: 4px;",
                            placeholder: "Campaign Name...",
                            value: "{new_project_name}",
                            oninput: move |evt| new_project_name.set(evt.value())
                        }
                        div {
                            style: "display: flex; justify-content: flex-end; gap: 10px;",
                            button {
                                style: "padding: 8px 16px; border: none; border-radius: 4px; background: #95a5a6; color: white; cursor: pointer;",
                                onclick: move |_| show_create_modal.set(false),
                                "Cancel"
                            }
                            button {
                                style: "padding: 8px 16px; border: none; border-radius: 4px; background: #3498db; color: white; cursor: pointer;",
                                onclick: handle_create,
                                "Create"
                            }
                        }
                    }
                }
            }
        }
    }
}
