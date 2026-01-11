use crate::models::project::Project;
use crate::models::character::Character;
use crate::models::settings::Settings;

const API_BASE_URL: &str = "http://localhost:8000";

// ... (Synthesis structs)

/// Fetch all projects from the backend
pub async fn fetch_projects() -> Result<Vec<Project>, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/projects", API_BASE_URL))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("Server error: {}", response.status()));
    }
    
    response
        .json::<Vec<Project>>()
        .await
        .map_err(|e| format!("Failed to parse projects: {}", e))
}

/// Create a new project
pub async fn create_project(name: String) -> Result<Project, String> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/projects?name={}", API_BASE_URL, name))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("Server error: {}", response.status()));
    }
    
    response
        .json::<Project>()
        .await
        .map_err(|e| format!("Failed to parse project: {}", e))
}

/// Fetch settings (which includes active_project_id)
pub async fn fetch_settings() -> Result<Settings, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/settings", API_BASE_URL))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("Server error: {}", response.status()));
    }
    
    response
        .json::<Settings>()
        .await
        .map_err(|e| format!("Failed to parse settings: {}", e))
}

/// Update settings (e.g. to switch project)
pub async fn update_settings(settings: Settings) -> Result<Settings, String> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/settings", API_BASE_URL))
        .json(&settings)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("Server error: {}", response.status()));
    }
    
    response
        .json::<Settings>()
        .await
        .map_err(|e| format!("Failed to parse settings: {}", e))
}

/// Fetch characters for the current active project
pub async fn fetch_characters() -> Result<Vec<Character>, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/characters", API_BASE_URL))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("Server error: {}", response.status()));
    }
    
    response
        .json::<Vec<Character>>()
        .await
        .map_err(|e| format!("Failed to parse characters: {}", e))
}

/// Save characters for the current active project
pub async fn backend_save_characters(characters: Vec<Character>) -> Result<(), String> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/characters", API_BASE_URL))
        .json(&characters)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("Server error: {}", response.status()));
    }
    
    Ok(())
}
#[derive(serde::Serialize)]
pub struct SynthesisRequest {
    pub text: String,
    pub reference_audio_path: String,
    pub cfg_weight: f32,
    pub exaggeration: f32,
}

/// Call the backend synthesis API and save the result to a file
pub async fn synthesize_audio(
    text: String,
    reference_audio_path: String,
    output_path: String,
    cfg_weight: f32,
    exaggeration: f32,
) -> Result<String, String> {
    let client = reqwest::Client::new();
    let payload = SynthesisRequest {
        text,
        reference_audio_path,
        cfg_weight,
        exaggeration,
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
    
    let audio_bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;
    
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

/// Helper to get YouTube progress URL
pub fn get_youtube_progress_url(task_id: &str) -> String {
    format!("{}/youtube/progress/{}", API_BASE_URL, task_id)
}
