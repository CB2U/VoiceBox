use dioxus::prelude::*;
use dioxus::document::eval; // Use eval from document module for 0.6
use crate::models::character::Character;
use crate::models::script::{ScriptLine, SynthesisStatus};
use crate::services::script_parser::parse_script;
use crate::services::api::synthesize_audio;
use crate::utils::audio::combine_wavs;
use crate::components::audio_player::AudioPlayer;
use crate::components::progress_bar::ProgressBar;
use std::path::PathBuf;
use rfd::FileDialog;

#[component]
pub fn ScriptEditor(characters: Signal<Vec<Character>>) -> Element {
    let mut script_text = use_signal(|| String::new());
    let mut parsed_lines = use_signal(|| Vec::<ScriptLine>::new());
    let mut is_synthesizing = use_signal(|| false);
    let mut synthesis_error = use_signal(|| None::<String>);
    let mut current_line_index = use_signal(|| 0usize);
    let mut export_status = use_signal(|| None::<String>);
    
    // Parse script whenever text changes
    let on_script_change = move |e: FormEvent| {
        let text = e.value();
        script_text.set(text.clone());
        
        let chars = characters.read();
        let lines = parse_script(&text, &chars);
        parsed_lines.set(lines);
    };
    
    // Synthesize button handler
    let on_synthesize = move |_| {
        println!("🎬 Synthesize button clicked");
        
        // Reset state
        synthesis_error.set(None);
        current_line_index.set(0);
        
        // Validate: check for unknown characters
        let lines = parsed_lines.read();
        println!("📝 Parsed lines count: {}", lines.len());
        
        let unknown_chars: Vec<String> = lines
            .iter()
            .filter(|line| line.character_id.is_none())
            .map(|line| line.character_name.clone())
            .collect();
        
        if !unknown_chars.is_empty() {
            let error_msg = format!(
                "Unknown character(s): {}. Please add them in Character Management first.",
                unknown_chars.join(", ")
            );
            println!("❌ {}", error_msg);
            synthesis_error.set(Some(error_msg));
            return;
        }
        
        if lines.is_empty() {
            let error_msg = "No dialogue lines to synthesize.".to_string();
            println!("❌ {}", error_msg);
            synthesis_error.set(Some(error_msg));
            return;
        }
        
        println!("✅ Validation passed. Starting synthesis...");
        
        // Start synthesis
        is_synthesizing.set(true);
        
        // Clone data for async task
        let chars = characters.read().clone();
        let lines_to_process: Vec<ScriptLine> = lines.clone();
        
        println!("👥 Characters available: {}", chars.len());
        for (i, line) in lines_to_process.iter().enumerate() {
            println!("  Line {}: [{}] \"{}\"", i, line.character_name, line.text);
        }
        
        spawn(async move {
            println!("🚀 Async synthesis task started");
            
            // Load settings to get output directory
            let output_dir = match reqwest::Client::new()
                .get("http://localhost:8000/settings")
                .send()
                .await
            {
                Ok(resp) => {
                    if resp.status().is_success() {
                        match resp.json::<crate::models::settings::Settings>().await {
                            Ok(settings) => {
                                println!("📁 Using output directory from settings: {}", settings.output_directory);
                                std::path::PathBuf::from(settings.output_directory)
                            }
                            Err(e) => {
                                let error_msg = format!("Failed to parse settings: {}", e);
                                println!("❌ {}", error_msg);
                                synthesis_error.set(Some(error_msg));
                                is_synthesizing.set(false);
                                return;
                            }
                        }
                    } else {
                        let error_msg = format!("Failed to load settings: {}", resp.status());
                        println!("❌ {}", error_msg);
                        synthesis_error.set(Some(error_msg));
                        is_synthesizing.set(false);
                        return;
                    }
                }
                Err(e) => {
                    let error_msg = format!("Failed to connect to settings API: {}", e);
                    println!("❌ {}", error_msg);
                    synthesis_error.set(Some(error_msg));
                    is_synthesizing.set(false);
                    return;
                }
            };
            
            println!("📁 Creating output directory: {}", output_dir.display());
            
            if let Err(e) = std::fs::create_dir_all(&output_dir) {
                let error_msg = format!("Failed to create output directory: {}", e);
                println!("❌ {}", error_msg);
                synthesis_error.set(Some(error_msg));
                is_synthesizing.set(false);
                return;
            }
            
            println!("✅ Output directory ready");
            
            // Process each line sequentially
            for (index, line) in lines_to_process.iter().enumerate() {
                println!("\n🎯 Processing line {}/{}", index + 1, lines_to_process.len());
                println!("   Character: {}", line.character_name);
                println!("   Text: {}", line.text);
                
                current_line_index.set(index);
                
                // Update status to Working
                let mut updated_lines = parsed_lines.write();
                if let Some(l) = updated_lines.get_mut(index) {
                    l.status = SynthesisStatus::Working;
                }
                drop(updated_lines);
                
                // Find the character to get reference audio
                let character = chars.iter().find(|c| Some(&c.id) == line.character_id.as_ref());
                
                if let Some(char) = character {
                    println!("   Found character: {}", char.name);
                    
                    if let Some(ref voice_path) = char.voice_path {
                        println!("   Voice path: {}", voice_path);
                        
                        // Generate output filename
                        let output_filename = format!("line_{:03}.wav", index);
                        let output_path = output_dir.join(&output_filename);
                        let output_path_str = output_path.to_string_lossy().to_string();
                        
                        println!("   Output will be: {}", output_path_str);
                        println!("   Calling synthesis API...");
                        
                        // Call synthesis API
                        match synthesize_audio(
                            line.text.clone(),
                            voice_path.clone(),
                            output_path_str.clone(),
                        ).await {
                            Ok(path) => {
                                println!("   ✅ Synthesis successful: {}", path);
                                
                                // Update status to Done
                                let mut updated_lines = parsed_lines.write();
                                if let Some(l) = updated_lines.get_mut(index) {
                                    l.status = SynthesisStatus::Done;
                                    l.output_path = Some(path);
                                }
                            }
                            Err(e) => {
                                let error_msg = format!("Failed to synthesize line {}: {}", index + 1, e);
                                println!("   ❌ {}", error_msg);
                                
                                // Update status to Error
                                let mut updated_lines = parsed_lines.write();
                                if let Some(l) = updated_lines.get_mut(index) {
                                    l.status = SynthesisStatus::Error(e.clone());
                                }
                                synthesis_error.set(Some(error_msg));
                                is_synthesizing.set(false);
                                return;
                            }
                        }
                    } else {
                        // No voice reference
                        let error_msg = format!("Character '{}' has no voice reference audio.", char.name);
                        println!("   ❌ {}", error_msg);
                        
                        let mut updated_lines = parsed_lines.write();
                        if let Some(l) = updated_lines.get_mut(index) {
                            l.status = SynthesisStatus::Error("No voice reference".to_string());
                        }
                        synthesis_error.set(Some(error_msg));
                        is_synthesizing.set(false);
                        return;
                    }
                } else {
                    // This shouldn't happen since we validated earlier
                    let error_msg = format!("Character '{}' not found in character list", line.character_name);
                    println!("   ❌ {}", error_msg);
                    
                    let mut updated_lines = parsed_lines.write();
                    if let Some(l) = updated_lines.get_mut(index) {
                        l.status = SynthesisStatus::Error("Character not found".to_string());
                    }
                    synthesis_error.set(Some(error_msg));
                    is_synthesizing.set(false);
                    return;
                }
            }
            
            // All done!
            println!("\n🎉 All lines synthesized successfully!");
            is_synthesizing.set(false);
        });
    };
    
    // Export button handler
    let on_export = move |_| {
        export_status.set(None);
        
        let lines = parsed_lines.read();
        
        // Check if all lines are synthesized
        let all_done = lines.iter().all(|line| matches!(line.status, SynthesisStatus::Done));
        if !all_done {
            export_status.set(Some("Please synthesize the script first.".to_string()));
            return;
        }
        
        // Collect output paths
        let output_paths: Vec<PathBuf> = lines
            .iter()
            .filter_map(|line| line.output_path.as_ref().map(|p| PathBuf::from(p)))
            .collect();
        
        if output_paths.is_empty() {
            export_status.set(Some("No audio files to export.".to_string()));
            return;
        }
        
        // Open file dialog to select save location
        let file_dialog = FileDialog::new()
            .set_file_name("conversation.wav")
            .add_filter("WAV Audio", &["wav"]);
        
        if let Some(output_path) = file_dialog.save_file() {
            // Perform the export
            match combine_wavs(output_paths, output_path.clone()) {
                Ok(_) => {
                    export_status.set(Some(format!("Successfully exported to: {}", output_path.display())));
                }
                Err(e) => {
                    export_status.set(Some(format!("Export failed: {}", e)));
                }
            }
        }
    };
    let lines = parsed_lines.read();
    
    rsx! {
        div {
            style: "flex-grow: 1; padding: 20px; display: flex; flex-direction: column; gap: 15px; overflow-y: auto; background-color: #f5f5f5;",
            
            // Header
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",
                h2 {
                    style: "margin: 0; font-size: 24px; color: #333;",
                    "Script Editor"
                }
                div {
                    style: "display: flex; gap: 10px;",
                    button {
                        style: "background-color: #007bff; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; font-size: 14px;",
                        disabled: is_synthesizing(),
                        onclick: on_synthesize,
                        if is_synthesizing() {
                            "Synthesizing..."
                        } else {
                            "Synthesize"
                        }
                    }
                    button {
                        style: "background-color: #28a745; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; font-size: 14px;",
                        disabled: is_synthesizing(),
                        onclick: on_export,
                        "Export WAV"
                    }
                }
            }
            
            // Instructions
            div {
                style: "background-color: #e3f2fd; padding: 12px; border-radius: 4px; border-left: 4px solid #2196f3;",
                p {
                    style: "margin: 0; font-size: 14px; color: #1976d2;",
                    strong { "Format: " }
                    "[CharacterName]: Dialogue text"
                }
                p {
                    style: "margin: 5px 0 0 0; font-size: 12px; color: #1976d2;",
                    "Example: [Gandalf]: You cannot pass!"
                }
            }
            
            // Paralinguistic Tags Guide
            div {
                style: "background-color: #f3e5f5; padding: 12px; border-radius: 4px; border-left: 4px solid #9c27b0;",
                div {
                    style: "display: flex; flex-direction: column; gap: 5px;",
                    p {
                        style: "margin: 0; font-size: 14px; color: #7b1fa2; font-weight: bold;",
                        "Paralinguistic Tags (Click to insert):"
                    }
                    div {
                        style: "display: flex; flex-wrap: wrap; gap: 8px; margin-top: 5px;",
                        for tag in ["[clear throat]", "[sigh]", "[shush]", "[cough]", "[groan]", "[sniff]", "[gasp]", "[chuckle]", "[laugh]"] {
                            button {
                                style: "background-color: #e1bee7; color: #4a148c; padding: 2px 10px; border-radius: 12px; font-size: 11px; font-family: monospace; border: 1px solid #ce93d8; cursor: pointer; transition: background-color 0.2s;",
                                onclick: {
                                    let t = tag;
                                    move |_| {
                                        let mut eval_js = eval(
                                            r#"
                                            (async () => {
                                                let textarea = document.getElementById('script-textarea');
                                                let start = textarea.selectionStart;
                                                let end = textarea.selectionEnd;
                                                let text = textarea.value;
                                                let insertion = await dioxus.recv();
                                                textarea.value = text.substring(0, start) + insertion + text.substring(end);
                                                textarea.selectionStart = textarea.selectionEnd = start + insertion.length;
                                                textarea.focus();
                                                // Trigger Dioxus oninput
                                                textarea.dispatchEvent(new Event('input', { bubbles: true }));
                                            })();
                                            "#
                                        );
                                        eval_js.send(serde_json::Value::String(t.to_string())).unwrap();
                                    }
                                },
                                "{tag}"
                            }
                        }
                    }
                }
            }

            // Character Names Shortcuts
            div {
                style: "background-color: #e8f5e9; padding: 12px; border-radius: 4px; border-left: 4px solid #4caf50;",
                div {
                    style: "display: flex; flex-direction: column; gap: 5px;",
                    p {
                        style: "margin: 0; font-size: 14px; color: #2e7d32; font-weight: bold;",
                        "Character Shortcuts (Click to insert):"
                    }
                    div {
                        style: "display: flex; flex-wrap: wrap; gap: 8px; margin-top: 5px;",
                        for char in characters.read().iter() {
                            button {
                                style: "background-color: #c8e6c9; color: #1b5e20; padding: 2px 10px; border-radius: 12px; font-size: 11px; font-family: bold; border: 1px solid #a5d6a7; cursor: pointer;",
                                onclick: {
                                    let name = char.name.clone();
                                    move |_| {
                                        let insertion = format!("[{}]: ", name);
                                        let mut eval_js = eval(
                                            r#"
                                            (async () => {
                                                let textarea = document.getElementById('script-textarea');
                                                let start = textarea.selectionStart;
                                                let end = textarea.selectionEnd;
                                                let text = textarea.value;
                                                let insertion = await dioxus.recv();
                                                textarea.value = text.substring(0, start) + insertion + text.substring(end);
                                                textarea.selectionStart = textarea.selectionEnd = start + insertion.length;
                                                textarea.focus();
                                                textarea.dispatchEvent(new Event('input', { bubbles: true }));
                                            })();
                                            "#
                                        );
                                        eval_js.send(serde_json::Value::String(insertion)).unwrap();
                                    }
                                },
                                "[{char.name}]"
                            }
                        }
                    }
                }
            }
            
            // Error Display
            if let Some(error) = synthesis_error() {
                div {
                    style: "background-color: #f8d7da; padding: 12px; border-radius: 4px; border-left: 4px solid #dc3545; color: #721c24;",
                    strong { "Error: " }
                    "{error}"
                }
            }
            
            // Export Status Display
            if let Some(status) = export_status() {
                div {
                    style: if status.contains("Success") { "background-color: #d4edda; padding: 12px; border-radius: 4px; border-left: 4px solid #28a745; color: #155724;" } else { "background-color: #f8d7da; padding: 12px; border-radius: 4px; border-left: 4px solid #dc3545; color: #721c24;" },
                    "{status}"
                }
            }
            
            // Progress Indicator
            if is_synthesizing() {
                {
                    let total = parsed_lines.read().len();
                    // Fix: If total is 0, avoid division by zero
                    if total > 0 {
                        // Current line is 0-indexed, so current working line is index + 0.5?
                        // Or just show progress as completed lines / total
                        let completed = parsed_lines.read().iter().filter(|l| matches!(l.status, SynthesisStatus::Done)).count();
                        let current_idx = current_line_index();
                        
                        // We show progress based on:
                        // 1. Completed lines (solid steps)
                        // 2. The fact that one is currently processing
                        let prog = (completed as f64 / total as f64) * 100.0;
                        
                        let label = if completed == total { 
                            "All lines synthesized! Combining...".to_string() 
                        } else { 
                            format!("Synthesizing line {} of {}...", current_idx + 1, total) 
                        };
                        
                        rsx! {
                            ProgressBar { progress: prog, label: label }
                        }
                    } else {
                        rsx! { div { "Validating lines..." } }
                    }
                }
            }
            
            // Script Input Area
            div {
                style: "display: flex; flex-direction: column; gap: 8px; flex-grow: 1;",
                label {
                    style: "font-weight: bold; font-size: 14px; color: #333;",
                    "Script"
                }
                textarea {
                    id: "script-textarea",
                    style: "flex-grow: 1; min-height: 200px; padding: 12px; border: 1px solid #ccc; border-radius: 4px; font-family: 'Courier New', monospace; font-size: 14px; resize: vertical;",
                    placeholder: "Enter your script here...\n\n[Gandalf]: You cannot pass!\n[Frodo]: I wish the ring had never come to me.",
                    value: "{script_text}",
                    oninput: on_script_change,
                }
            }
            
            // Live Preview Section
            if !lines.is_empty() {
                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",
                    label {
                        style: "font-weight: bold; font-size: 14px; color: #333;",
                        "Parsed Lines ({lines.len()})"
                    }
                    div {
                        style: "background-color: white; border: 1px solid #ccc; border-radius: 4px; padding: 12px; max-height: 300px; overflow-y: auto;",
                        for line in lines.iter() {
                            {
                                let status_color = get_status_color(&line.status);
                                let char_color = if line.character_id.is_some() { "#28a745" } else { "#dc3545" };
                                let is_unknown = line.character_id.is_none();
                                rsx! {
                                    div {
                                        key: "{line.id}",
                                        style: "padding: 8px; margin-bottom: 8px; border-left: 3px solid {status_color}; background-color: #f9f9f9; border-radius: 2px;",
                                        div {
                                            style: "display: flex; justify-content: space-between; align-items: center;",
                                            div {
                                                span {
                                                    style: "font-weight: bold; color: {char_color};",
                                                    "[{line.character_name}]"
                                                }
                                                span {
                                                    style: "margin-left: 8px; color: #333;",
                                                    "{line.text}"
                                                }
                                            }
                                             if is_unknown {
                                                 span {
                                                     style: "font-size: 12px; color: #dc3545; font-style: italic;",
                                                     "⚠ Unknown character"
                                                 }
                                             } else if matches!(line.status, SynthesisStatus::Done) {
                                                 span {
                                                     style: "font-size: 12px; color: #28a745;",
                                                     "{get_status_icon(&line.status)} Done"
                                                 }
                                             }
                                         }
                                         // Show audio player for completed lines
                                         if let SynthesisStatus::Done = line.status {
                                             if let Some(ref output_path) = line.output_path {
                                                 div {
                                                     style: "margin-top: 8px;",
                                                     AudioPlayer {
                                                         audio_url: format!("http://localhost:8000/files/audio/{}", urlencoding::encode(output_path))
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
        }
    }
}

fn get_status_color(status: &SynthesisStatus) -> &'static str {
    match status {
        SynthesisStatus::Idle => "#6c757d",
        SynthesisStatus::Queued => "#ffc107",
        SynthesisStatus::Working => "#007bff",
        SynthesisStatus::Done => "#28a745",
        SynthesisStatus::Error(_) => "#dc3545",
    }
}

fn get_status_icon(status: &SynthesisStatus) -> &'static str {
    match status {
        SynthesisStatus::Idle => "⚪",
        SynthesisStatus::Queued => "🟡",
        SynthesisStatus::Working => "🔵",
        SynthesisStatus::Done => "✅",
        SynthesisStatus::Error(_) => "❌",
    }
}
