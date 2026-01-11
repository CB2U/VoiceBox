use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct YouTubeRequest {
    url: String,
    start_time: String,
    end_time: String,
    character_id: String,
}

#[derive(Deserialize)]
struct AudioFileResponse {
    file_path: String,
    filename: String,
}

#[component]
pub fn YouTubeImport(
    character_id: String,
    on_success: EventHandler<String> // Returns path to the downloaded file
) -> Element {
    let mut url = use_signal(|| "".to_string());
    let mut start_time = use_signal(|| "".to_string());
    let mut end_time = use_signal(|| "".to_string());
    let mut is_loading = use_signal(|| false);
    let mut error_msg = use_signal(|| None::<String>);

    rsx! {
        div {
            style: "border: 1px solid #ccc; padding: 10px; margin-top: 10px; border-radius: 4px; background-color: #f9f9f9;",
            h4 { 
                style: "margin-top: 0;",
                "Import from YouTube" 
            }
            
            div {
                style: "display: flex; flex-direction: column; gap: 5px; margin-bottom: 10px;",
                label { style: "font-weight: bold; font-size: 0.9em;", "Video URL" }
                input {
                    value: "{url}",
                    oninput: move |e: FormEvent| url.set(e.value()),
                    placeholder: "https://youtube.com/watch?v=...",
                    style: "padding: 5px;"
                }
            }
            
            div {
                style: "display: flex; gap: 10px; margin-bottom: 10px;",
                div {
                    style: "flex-grow: 1;",
                    label { style: "display: block; font-weight: bold; font-size: 0.9em;", "Start Time" }
                    input {
                        value: "{start_time}",
                        oninput: move |e: FormEvent| start_time.set(e.value()),
                        placeholder: "00:00:10",
                        style: "width: 100%; padding: 5px;"
                    }
                }
                div {
                    style: "flex-grow: 1;",
                    label { style: "display: block; font-weight: bold; font-size: 0.9em;", "End Time" }
                    input {
                        value: "{end_time}",
                        oninput: move |e: FormEvent| end_time.set(e.value()),
                        placeholder: "00:00:15",
                        style: "width: 100%; padding: 5px;"
                    }
                }
            }
            
            if let Some(msg) = error_msg() {
                div { 
                    style: "color: red; margin-bottom: 10px; font-size: 0.9em;",
                    "{msg}" 
                }
            }
            
            button {
                disabled: "{is_loading}",
                style: "background-color: #007bff; color: white; padding: 8px 15px; border: none; border-radius: 4px; cursor: pointer; width: 100%;",
                onclick: move |_| {
                    if url().is_empty() || start_time().is_empty() || end_time().is_empty() {
                        error_msg.set(Some("Please fill all fields".to_string()));
                        return;
                    }
                    
                    let c_id = character_id.clone();
                    let req_url = url();
                    let req_start = start_time();
                    let req_end = end_time();

                    spawn(async move {
                        is_loading.set(true);
                        error_msg.set(None);
                        
                        let client = reqwest::Client::new();
                        let payload = YouTubeRequest {
                            url: req_url,
                            start_time: req_start,
                            end_time: req_end,
                            character_id: c_id,
                        };
                        
                        match client.post("http://localhost:8000/extract-from-youtube")
                            .json(&payload)
                            .send()
                            .await {
                                Ok(resp) => {
                                    if resp.status().is_success() {
                                        match resp.json::<AudioFileResponse>().await {
                                            Ok(data) => {
                                                is_loading.set(false);
                                                // Clear inputs on success? Maybe not, allow tweaking.
                                                on_success.call(data.file_path);
                                            },
                                            Err(_) => {
                                                error_msg.set(Some("Failed to parse response".to_string()));
                                                is_loading.set(false);
                                            }
                                        }
                                    } else {
                                        error_msg.set(Some(format!("Server error: {}", resp.status())));
                                        is_loading.set(false);
                                    }
                                }
                                Err(e) => {
                                    error_msg.set(Some(format!("Request failed: {}", e)));
                                    is_loading.set(false);
                                }
                            }
                    });
                },
                if is_loading() { "Extracting..." } else { "Extract Audio" }
            }
        }
    }
}
