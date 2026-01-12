use dioxus::prelude::*;
use crate::models::audio_processing::{
    AudioFormat, AudioPreviewRequest, AudioProcessRequest, QualitySettings,
};
use crate::models::history::ScriptHistory;
use crate::services::{audio_api, history::fetch_history};

#[component]
pub fn AudioPostProcessing() -> Element {
    // State
    let mut history_entries = use_signal(|| Vec::<ScriptHistory>::new());
    let mut selected_history_id = use_signal(|| None::<String>);
    let mut pitch_shift = use_signal(|| 0.0f32);
    let mut speed_factor = use_signal(|| 1.0f32);
    let mut output_format = use_signal(|| AudioFormat::Wav);
    let mut mp3_bitrate = use_signal(|| "192k".to_string());
    let mut ogg_quality = use_signal(|| 5i32);
    let mut preview_url = use_signal(|| None::<String>);
    let mut processing = use_signal(|| false);
    let mut status_message = use_signal(|| None::<String>);
    let mut error_message = use_signal(|| None::<String>);

    // Fetch history on mount
    use_effect(move || {
        spawn(async move {
            if let Ok(entries) = fetch_history().await {
                history_entries.set(entries);
            }
        });
    });

    // Get selected history entry
    let selected_entry = {
        let entries = history_entries.read();
        let sel_id = selected_history_id.read();
        sel_id
            .as_ref()
            .and_then(|id| entries.iter().find(|e| e.id == *id).cloned())
    };

    // Handlers
    let mut on_select_history = move |id: String| {
        selected_history_id.set(Some(id));
        preview_url.set(None);
        error_message.set(None);
        status_message.set(None);
    };

    let on_pitch_change = move |evt: Event<FormData>| {
        if let Ok(value) = evt.value().parse::<f32>() {
            pitch_shift.set(value.clamp(-12.0, 12.0));
        }
    };

    let on_speed_change = move |evt: Event<FormData>| {
        if let Ok(value) = evt.value().parse::<f32>() {
            speed_factor.set(value.clamp(0.5, 2.0));
        }
    };

    let on_format_change = move |evt: Event<FormData>| {
        let format = match evt.value().as_str() {
            "mp3" => AudioFormat::Mp3,
            "ogg" => AudioFormat::Ogg,
            "flac" => AudioFormat::Flac,
            _ => AudioFormat::Wav,
        };
        output_format.set(format);
    };

    let on_reset = move |_| {
        pitch_shift.set(0.0);
        speed_factor.set(1.0);
        output_format.set(AudioFormat::Wav);
        preview_url.set(None);
        error_message.set(None);
        status_message.set(None);
    };

    let on_preview = move |_| {
        let history_id = match selected_history_id.read().clone() {
            Some(id) => id,
            None => return,
        };

        processing.set(true);
        error_message.set(None);
        status_message.set(Some("Generating preview...".to_string()));

        spawn(async move {
            let request = AudioPreviewRequest {
                history_id,
                pitch_shift: pitch_shift(),
                speed_factor: speed_factor(),
                preview_duration: 10.0,
            };

            match audio_api::generate_preview(request).await {
                Ok(response) => {
                    let url = format!("http://localhost:8000{}", response.preview_url);
                    preview_url.set(Some(url));
                    status_message.set(Some("Preview ready!".to_string()));
                    error_message.set(None);
                }
                Err(e) => {
                    error_message.set(Some(format!("Preview failed: {}", e)));
                    status_message.set(None);
                }
            }
            processing.set(false);
        });
    };

    let on_process = move |_| {
        let history_id = match selected_history_id.read().clone() {
            Some(id) => id,
            None => return,
        };

        processing.set(true);
        error_message.set(None);
        status_message.set(Some("Processing audio...".to_string()));

        spawn(async move {
            let quality_settings = match output_format() {
                AudioFormat::Mp3 => QualitySettings {
                    bitrate: Some(mp3_bitrate()),
                    quality: None,
                    compression: None,
                },
                AudioFormat::Ogg => QualitySettings {
                    bitrate: None,
                    quality: Some(ogg_quality()),
                    compression: None,
                },
                _ => QualitySettings::default(),
            };

            let request = AudioProcessRequest {
                history_id,
                pitch_shift: pitch_shift(),
                speed_factor: speed_factor(),
                output_format: output_format(),
                quality_settings,
            };

            match audio_api::process_audio(request).await {
                Ok(_response) => {
                    status_message.set(Some("Processing complete! File added to history.".to_string()));
                    error_message.set(None);
                    
                    // Refresh history
                    if let Ok(entries) = fetch_history().await {
                        history_entries.set(entries);
                    }
                }
                Err(e) => {
                    error_message.set(Some(format!("Processing failed: {}", e)));
                    status_message.set(None);
                }
            }
            processing.set(false);
        });
    };

    // Styles
    let container_style = "display: flex; height: 100%; width: 100%; background-color: #f5f5f5;";
    let sidebar_style = "width: 300px; background-color: #fff; border-right: 1px solid #ddd; overflow-y: auto; padding: 20px;";
    let main_panel_style = "flex: 1; padding: 30px; overflow-y: auto;";
    let history_item_style = "padding: 12px; margin-bottom: 8px; background-color: #f9f9f9; border-radius: 4px; cursor: pointer; border: 2px solid transparent;";
    let selected_item_style = "padding: 12px; margin-bottom: 8px; background-color: #e3f2fd; border-radius: 4px; cursor: pointer; border: 2px solid #2196f3;";
    let control_group_style = "margin-bottom: 25px;";
    let label_style = "display: block; font-weight: bold; margin-bottom: 8px; color: #333;";
    let slider_style = "width: 100%; margin-bottom: 8px;";
    let button_style = "padding: 12px 24px; margin-right: 10px; background-color: #2196f3; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px; font-weight: bold;";
    let button_secondary_style = "padding: 12px 24px; margin-right: 10px; background-color: #757575; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px; font-weight: bold;";
    let disabled_button_style = "padding: 12px 24px; margin-right: 10px; background-color: #ccc; color: #666; border: none; border-radius: 4px; cursor: not-allowed; font-size: 14px; font-weight: bold;";

    rsx! {
        div {
            style: "{container_style}",
            
            // Left Sidebar - History List
            div {
                style: "{sidebar_style}",
                h2 { style: "margin-top: 0; color: #333;", "History Files" }
                
                if history_entries().is_empty() {
                    p { style: "color: #999; font-style: italic;", "No history files available" }
                } else {
                    for entry in history_entries() {
                        div {
                            key: "{entry.id}",
                            style: "{history_item_style}",
                            onclick: {
                                let id = entry.id.clone();
                                move |_| on_select_history(id.clone())
                            },
                            div {
                                style: "font-weight: bold; margin-bottom: 4px;",
                                "{entry.name}"
                            }
                            div {
                                style: "font-size: 12px; color: #666;",
                                "Created: {entry.created_at}"
                            }
                        }
                    }
                }
            }
            
            // Main Panel - Controls
            div {
                style: "{main_panel_style}",
                h1 { style: "margin-top: 0; color: #333;", "Audio Post-Processing" }
                
                if selected_entry.is_none() {
                    div {
                        style: "text-align: center; padding: 60px; color: #999; font-size: 18px;",
                        "Select a history file to begin"
                    }
                } else {
                    // Pitch Control
                    div {
                        style: "{control_group_style}",
                        label { style: "{label_style}", "Pitch Shift (semitones)" }
                        input {
                            r#type: "range",
                            min: "-12",
                            max: "12",
                            step: "0.5",
                            value: "{pitch_shift()}",
                            style: "{slider_style}",
                            oninput: on_pitch_change,
                        }
                        div {
                            style: "display: flex; justify-content: space-between; align-items: center;",
                            span { style: "font-size: 14px; color: #666;", "-12" }
                            span {
                                style: "font-size: 18px; font-weight: bold; color: #2196f3;",
                                "{pitch_shift():.1} semitones"
                            }
                            span { style: "font-size: 14px; color: #666;", "+12" }
                        }
                    }
                    
                    // Speed Control
                    div {
                        style: "{control_group_style}",
                        label { style: "{label_style}", "Speed" }
                        input {
                            r#type: "range",
                            min: "0.5",
                            max: "2.0",
                            step: "0.1",
                            value: "{speed_factor()}",
                            style: "{slider_style}",
                            oninput: on_speed_change,
                        }
                        div {
                            style: "display: flex; justify-content: space-between; align-items: center;",
                            span { style: "font-size: 14px; color: #666;", "0.5x" }
                            span {
                                style: "font-size: 18px; font-weight: bold; color: #2196f3;",
                                "{speed_factor():.1}x ({(speed_factor() * 100.0):.0}%)"
                            }
                            span { style: "font-size: 14px; color: #666;", "2.0x" }
                        }
                    }
                    
                    // Format Selection
                    div {
                        style: "{control_group_style}",
                        label { style: "{label_style}", "Output Format" }
                        select {
                            style: "width: 100%; padding: 10px; font-size: 14px; border: 1px solid #ddd; border-radius: 4px;",
                            value: "{output_format().as_str()}",
                            onchange: on_format_change,
                            option { value: "wav", "WAV (Lossless)" }
                            option { value: "mp3", "MP3 (Lossy)" }
                            option { value: "ogg", "OGG (Lossy)" }
                            option { value: "flac", "FLAC (Lossless)" }
                        }
                        
                        // Quality settings for MP3
                        if output_format() == AudioFormat::Mp3 {
                            div {
                                style: "margin-top: 10px;",
                                label { style: "font-size: 13px; color: #666; margin-bottom: 5px; display: block;", "Bitrate" }
                                select {
                                    style: "width: 100%; padding: 8px; font-size: 13px; border: 1px solid #ddd; border-radius: 4px;",
                                    value: "{mp3_bitrate()}",
                                    onchange: move |evt: Event<FormData>| mp3_bitrate.set(evt.value()),
                                    option { value: "128k", "128 kbps" }
                                    option { value: "192k", selected: true, "192 kbps (Recommended)" }
                                    option { value: "256k", "256 kbps" }
                                    option { value: "320k", "320 kbps" }
                                }
                            }
                        }
                        
                        // Quality settings for OGG
                        if output_format() == AudioFormat::Ogg {
                            div {
                                style: "margin-top: 10px;",
                                label { style: "font-size: 13px; color: #666; margin-bottom: 5px; display: block;", "Quality (0-10)" }
                                input {
                                    r#type: "number",
                                    min: "0",
                                    max: "10",
                                    value: "{ogg_quality()}",
                                    style: "width: 100%; padding: 8px; font-size: 13px; border: 1px solid #ddd; border-radius: 4px;",
                                    oninput: move |evt: Event<FormData>| {
                                        if let Ok(val) = evt.value().parse::<i32>() {
                                            ogg_quality.set(val.clamp(0, 10));
                                        }
                                    },
                                }
                            }
                        }
                    }
                    
                    // Preview Player
                    if let Some(url) = preview_url.read().as_ref() {
                        div {
                            style: "{control_group_style}",
                            label { style: "{label_style}", "Preview (10 seconds)" }
                            audio {
                                controls: true,
                                style: "width: 100%;",
                                src: "{url}",
                                autoplay: true,
                            }
                        }
                    }
                    
                    // Action Buttons
                    div {
                        style: "margin-top: 30px;",
                        if processing() {
                            button {
                                style: "{disabled_button_style}",
                                disabled: true,
                                "Processing..."
                            }
                        } else {
                            button {
                                style: "{button_style}",
                                onclick: on_preview,
                                "🎧 Preview (10s)"
                            }
                            button {
                                style: "{button_style}",
                                onclick: on_process,
                                "💾 Apply & Save"
                            }
                            button {
                                style: "{button_secondary_style}",
                                onclick: on_reset,
                                "↺ Reset All"
                            }
                        }
                    }
                    
                    // Status Messages
                    if let Some(msg) = status_message.read().as_ref() {
                        div {
                            style: "margin-top: 20px; padding: 15px; background-color: #d4edda; color: #155724; border-radius: 4px; border: 1px solid #c3e6cb;",
                            "{msg}"
                        }
                    }
                    
                    if let Some(err) = error_message.read().as_ref() {
                        div {
                            style: "margin-top: 20px; padding: 15px; background-color: #f8d7da; color: #721c24; border-radius: 4px; border: 1px solid #f5c6cb;",
                            "{err}"
                        }
                    }
                }
            }
        }
    }
}
