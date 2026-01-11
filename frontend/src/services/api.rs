use serde::{Deserialize, Serialize};
use std::io::Cursor;

const API_BASE_URL: &str = "http://localhost:8000";

#[derive(Serialize)]
pub struct SynthesisRequest {
    pub text: String,
    pub reference_audio_path: String,
}

#[derive(Deserialize)]
pub struct SynthesisResponse {
    // The backend returns a WAV audio stream
    // We'll handle it as bytes
}

/// Call the backend synthesis API and save the result to a file
pub async fn synthesize_audio(
    text: String,
    reference_audio_path: String,
    output_path: String,
) -> Result<String, String> {
    let client = reqwest::Client::new();
    let payload = SynthesisRequest {
        text,
        reference_audio_path,
    };
    
    let response = client
        .post(format!("{}/synthesize", API_BASE_URL))
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("Server error: {}", response.status()));
    }
    
    // Get the audio bytes
    let audio_bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;
    
    // Save to file
    std::fs::write(&output_path, audio_bytes)
        .map_err(|e| format!("Failed to write audio file: {}", e))?;
    
    Ok(output_path)
}

/// Check if the backend is running and responsive
pub async fn check_backend_health() -> bool {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(2))
        .build()
        .unwrap();
    
    match client
        .get(format!("{}/health", API_BASE_URL))
        .send()
        .await
    {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}
