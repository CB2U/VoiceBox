use dioxus::prelude::*;
use crate::models::character::Character;
use crate::models::script::{ScriptLine, SynthesisStatus};
use crate::services::script_parser::parse_script;
use crate::services::api::synthesize_audio;
use crate::utils::audio::combine_wavs;
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
            
            // Create output directory
            let output_dir = PathBuf::from("frontend/data/synthesis");
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
                div {
                    style: "background-color: #fff3cd; padding: 12px; border-radius: 4px; border-left: 4px solid #ffc107; color: #856404;",
                    "Synthesizing line {current_line_index() + 1} of {parsed_lines.read().len()}..."
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
