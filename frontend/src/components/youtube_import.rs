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

#[derive(Deserialize)]
struct ErrorResponse {
    detail: String,
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
                    oninput: move |e: FormEvent| {
                        url.set(e.value());
                        error_msg.set(None); // Clear error when user types
                    },
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
                        oninput: move |e: FormEvent| {
                            start_time.set(e.value());
                            error_msg.set(None);
                        },
                        placeholder: "00:00:10",
                        style: "width: 100%; padding: 5px;"
                    }
                }
                div {
                    style: "flex-grow: 1;",
                    label { style: "display: block; font-weight: bold; font-size: 0.9em;", "End Time" }
                    input {
                        value: "{end_time}",
                        oninput: move |e: FormEvent| {
                            end_time.set(e.value());
                            error_msg.set(None);
                        },
                        placeholder: "00:00:15",
                        style: "width: 100%; padding: 5px;"
                    }
                }
            }
            
            if let Some(msg) = error_msg() {
                div { 
                    style: "color: #d32f2f; background-color: #ffebee; padding: 8px; margin-bottom: 10px; font-size: 0.9em; border-radius: 4px; border-left: 4px solid #d32f2f;",
                    "{msg}" 
                }
            }
            
            button {
                disabled: is_loading(),
                style: if is_loading() {
                    "background-color: #999; color: white; padding: 8px 15px; border: none; border-radius: 4px; cursor: not-allowed; width: 100%;"
                } else {
                    "background-color: #007bff; color: white; padding: 8px 15px; border: none; border-radius: 4px; cursor: pointer; width: 100%;"
                },
                onclick: move |_| {
                    // Client-side validation
                    let url_val = url().trim().to_string();
                    let start_val = start_time().trim().to_string();
                    let end_val = end_time().trim().to_string();
                    
                    if url_val.is_empty() || start_val.is_empty() || end_val.is_empty() {
                        error_msg.set(Some("Please fill in all fields".to_string()));
                        return;
                    }
                    
                    // Basic URL validation
                    if !url_val.starts_with("http://") && !url_val.starts_with("https://") {
                        error_msg.set(Some("Please enter a valid URL starting with http:// or https://".to_string()));
                        return;
                    }
                    
                    if !url_val.contains("youtube.com") && !url_val.contains("youtu.be") {
                        error_msg.set(Some("Please enter a valid YouTube URL".to_string()));
                        return;
                    }
                    
                    let c_id = character_id.clone();

                    spawn(async move {
                        is_loading.set(true);
                        error_msg.set(None);
                        
                        let client = reqwest::Client::new();
                        let payload = YouTubeRequest {
                            url: url_val,
                            start_time: start_val,
                            end_time: end_val,
                            character_id: c_id,
                        };
                        
                        match client.post("http://localhost:8000/extract-from-youtube")
                            .json(&payload)
                            .send()
                            .await {
                                Ok(resp) => {
                                    let status = resp.status();
                                    
                                    if status.is_success() {
                                        // Success - parse the response
                                        match resp.json::<AudioFileResponse>().await {
                                            Ok(data) => {
                                                is_loading.set(false);
                                                on_success.call(data.file_path);
                                            },
                                            Err(e) => {
                                                error_msg.set(Some(format!("Failed to parse response: {}", e)));
                                                is_loading.set(false);
                                            }
                                        }
                                    } else {
                                        // Error - try to extract error message from response body
                                        match resp.json::<ErrorResponse>().await {
                                            Ok(err_data) => {
                                                error_msg.set(Some(err_data.detail));
                                            },
                                            Err(_) => {
                                                // Fallback if we can't parse the error response
                                                error_msg.set(Some(format!("Server error ({})", status)));
                                            }
                                        }
                                        is_loading.set(false);
                                    }
                                }
                                Err(e) => {
                                    // Network error or connection refused
                                    let err_message = if e.is_connect() {
                                        "Cannot connect to server. Please make sure the backend is running.".to_string()
                                    } else if e.is_timeout() {
                                        "Request timed out. Please try again.".to_string()
                                    } else {
                                        format!("Network error: {}", e)
                                    };
                                    
                                    error_msg.set(Some(err_message));
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
