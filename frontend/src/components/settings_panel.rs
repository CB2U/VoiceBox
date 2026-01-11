use dioxus::prelude::*;
use crate::models::settings::Settings;

#[component]
pub fn SettingsPanel() -> Element {
    let mut settings = use_signal(|| Settings::default());
    let mut is_loading = use_signal(|| false);
    let mut save_status = use_signal(|| None::<String>);
    let mut error_msg = use_signal(|| None::<String>);
    
    // Load settings on mount
    use_effect(move || {
        spawn(async move {
            load_settings(settings, is_loading, error_msg).await;
        });
    });
    
    rsx! {
        div {
            style: "flex-grow: 1; padding: 20px; display: flex; flex-direction: column; gap: 20px; overflow-y: auto; background-color: #f5f5f5;",
            
            // Header
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",
                h2 {
                    style: "margin: 0; font-size: 24px; color: #333;",
                    "Settings"
                }
                button {
                    style: "background-color: #28a745; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; font-size: 14px;",
                    disabled: is_loading(),
                    onclick: move |_| {
                        let s = settings();
                        spawn(async move {
                            save_settings(s, save_status, error_msg).await;
                        });
                    },
                    "Save Settings"
                }
            }
            
            // Status messages
            if let Some(msg) = save_status() {
                div {
                    style: "background-color: #d4edda; padding: 12px; border-radius: 4px; border-left: 4px solid #28a745; color: #155724;",
                    "{msg}"
                }
            }
            
            if let Some(msg) = error_msg() {
                div {
                    style: "background-color: #f8d7da; padding: 12px; border-radius: 4px; border-left: 4px solid #dc3545; color: #721c24;",
                    "{msg}"
                }
            }
            
            // Directory Settings
            div {
                style: "background-color: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                h3 {
                    style: "margin-top: 0; color: #333;",
                    "Directory Settings"
                }
                
                // Output Directory
                div {
                    style: "margin-bottom: 20px;",
                    label {
                        style: "display: block; font-weight: bold; margin-bottom: 8px; color: #555;",
                        "Synthesized Audio Output Directory"
                    }
                    div {
                        style: "display: flex; gap: 10px;",
                        input {
                            value: "{settings().output_directory}",
                            oninput: move |e: FormEvent| {
                                let mut s = settings();
                                s.output_directory = e.value();
                                settings.set(s);
                                save_status.set(None);
                            },
                            style: "flex-grow: 1; padding: 8px; border: 1px solid #ccc; border-radius: 4px;",
                            placeholder: "/path/to/output"
                        }
                        button {
                            style: "padding: 8px 15px; background-color: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;",
                            onclick: move |_| {
                                let mut s = settings();
                                spawn(async move {
                                    if let Some(path) = rfd::AsyncFileDialog::new()
                                        .set_title("Select Output Directory")
                                        .pick_folder()
                                        .await
                                    {
                                        s.output_directory = path.path().to_string_lossy().to_string();
                                        settings.set(s);
                                        save_status.set(None);
                                    }
                                });
                            },
                            "Browse..."
                        }
                    }
                    p {
                        style: "margin: 5px 0 0 0; font-size: 12px; color: #666;",
                        "Where synthesized audio files will be saved"
                    }
                }
                
                // Voice Files Directory
                div {
                    style: "margin-bottom: 20px;",
                    label {
                        style: "display: block; font-weight: bold; margin-bottom: 8px; color: #555;",
                        "Voice Reference Files Directory"
                    }
                    div {
                        style: "display: flex; gap: 10px;",
                        input {
                            value: "{settings().voice_files_directory}",
                            oninput: move |e: FormEvent| {
                                let mut s = settings();
                                s.voice_files_directory = e.value();
                                settings.set(s);
                                save_status.set(None);
                            },
                            style: "flex-grow: 1; padding: 8px; border: 1px solid #ccc; border-radius: 4px;",
                            placeholder: "/path/to/voice/files"
                        }
                        button {
                            style: "padding: 8px 15px; background-color: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;",
                            onclick: move |_| {
                                let mut s = settings();
                                spawn(async move {
                                    if let Some(path) = rfd::AsyncFileDialog::new()
                                        .set_title("Select Voice Files Directory")
                                        .pick_folder()
                                        .await
                                    {
                                        s.voice_files_directory = path.path().to_string_lossy().to_string();
                                        settings.set(s);
                                        save_status.set(None);
                                    }
                                });
                            },
                            "Browse..."
                        }
                    }
                    p {
                        style: "margin: 5px 0 0 0; font-size: 12px; color: #666;",
                        "Where voice reference audio files are stored"
                    }
                }
            }
            
            // Projects Directory
            div {
                style: "background-color: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                h3 {
                    style: "margin-top: 0; color: #333;",
                    "Project Save Location"
                }
                
                div {
                    style: "margin-bottom: 20px;",
                    label {
                        style: "display: block; font-weight: bold; margin-bottom: 8px; color: #555;",
                        "Projects Directory"
                    }
                    div {
                        style: "display: flex; gap: 10px;",
                        input {
                            value: "{settings().projects_directory}",
                            oninput: move |e: FormEvent| {
                                let mut s = settings();
                                s.projects_directory = e.value();
                                settings.set(s);
                                save_status.set(None);
                            },
                            style: "flex-grow: 1; padding: 8px; border: 1px solid #ccc; border-radius: 4px;",
                            placeholder: "/path/to/projects"
                        }
                        button {
                            style: "padding: 8px 15px; background-color: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;",
                            onclick: move |_| {
                                let mut s = settings();
                                spawn(async move {
                                    if let Some(path) = rfd::AsyncFileDialog::new()
                                        .set_title("Select Projects Directory")
                                        .pick_folder()
                                        .await
                                    {
                                        s.projects_directory = path.path().to_string_lossy().to_string();
                                        settings.set(s);
                                        save_status.set(None);
                                    }
                                });
                            },
                            "Browse..."
                        }
                    }
                    p {
                        style: "margin: 5px 0 0 0; font-size: 12px; color: #666;",
                        "Where project data (characters, scripts, etc.) will be saved. Each project will be stored in a subfolder."
                    }
                }
            }
            
            // File Naming Settings (placeholder for future implementation)
            div {
                style: "background-color: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                h3 {
                    style: "margin-top: 0; color: #333;",
                    "File Naming (Coming Soon)"
                }
                p {
                    style: "color: #666;",
                    "Options for custom file naming and auto-naming will be available in a future update."
                }
            }
        }
    }
}

async fn load_settings(
    mut settings: Signal<Settings>,
    mut is_loading: Signal<bool>,
    mut error_msg: Signal<Option<String>>
) {
    is_loading.set(true);
    error_msg.set(None);
    
    match reqwest::Client::new()
        .get("http://localhost:8000/settings")
        .send()
        .await
    {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<Settings>().await {
                    Ok(data) => {
                        settings.set(data);
                    }
                    Err(e) => {
                        error_msg.set(Some(format!("Failed to parse settings: {}", e)));
                    }
                }
            } else {
                error_msg.set(Some(format!("Server error: {}", resp.status())));
            }
        }
        Err(e) => {
            error_msg.set(Some(format!("Failed to load settings: {}", e)));
        }
    }
    
    is_loading.set(false);
}

async fn save_settings(
    settings: Settings,
    mut save_status: Signal<Option<String>>,
    mut error_msg: Signal<Option<String>>
) {
    error_msg.set(None);
    save_status.set(None);
    
    match reqwest::Client::new()
        .post("http://localhost:8000/settings")
        .json(&settings)
        .send()
        .await
    {
        Ok(resp) => {
            if resp.status().is_success() {
                save_status.set(Some("Settings saved successfully!".to_string()));
            } else {
                match resp.text().await {
                    Ok(text) => error_msg.set(Some(format!("Failed to save: {}", text))),
                    Err(_) => error_msg.set(Some("Failed to save settings".to_string())),
                }
            }
        }
        Err(e) => {
            error_msg.set(Some(format!("Failed to save settings: {}", e)));
        }
    }
}
