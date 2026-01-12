use crate::models::audio_processing::{
    AudioPreviewRequest, AudioPreviewResponse, AudioProcessRequest, AudioProcessResponse,
    AudioFormatInfo,
};

const API_BASE: &str = "http://localhost:8000";

pub async fn generate_preview(
    request: AudioPreviewRequest,
) -> Result<AudioPreviewResponse, String> {
    let url = format!("{}/audio/preview", API_BASE);

    let response = reqwest::Client::new()
        .post(&url)
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Failed to send preview request: {}", e))?;

    if !response.status().is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Preview generation failed: {}", error_text));
    }

    response
        .json::<AudioPreviewResponse>()
        .await
        .map_err(|e| format!("Failed to parse preview response: {}", e))
}

pub async fn process_audio(
    request: AudioProcessRequest,
) -> Result<AudioProcessResponse, String> {
    let url = format!("{}/audio/process", API_BASE);

    let response = reqwest::Client::new()
        .post(&url)
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Failed to send process request: {}", e))?;

    if !response.status().is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Audio processing failed: {}", error_text));
    }

    response
        .json::<AudioProcessResponse>()
        .await
        .map_err(|e| format!("Failed to parse process response: {}", e))
}

pub async fn get_supported_formats() -> Result<Vec<AudioFormatInfo>, String> {
    let url = format!("{}/audio/formats", API_BASE);

    let response = reqwest::Client::new()
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch formats: {}", e))?;

    if !response.status().is_success() {
        return Err("Failed to fetch supported formats".to_string());
    }

    response
        .json::<Vec<AudioFormatInfo>>()
        .await
        .map_err(|e| format!("Failed to parse formats: {}", e))
}
