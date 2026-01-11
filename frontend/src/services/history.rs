use crate::models::history::ScriptHistory;
use std::collections::HashMap;

pub async fn fetch_history() -> Result<Vec<ScriptHistory>, String> {
    match reqwest::Client::new()
        .get("http://localhost:8000/history")
        .send()
        .await
    {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<Vec<ScriptHistory>>().await {
                    Ok(history) => Ok(history),
                    Err(e) => Err(format!("Failed to parse history: {}", e)),
                }
            } else {
                Err(format!("Server error: {}", resp.status()))
            }
        }
        Err(e) => Err(format!("Failed to fetch history: {}", e)),
    }
}

pub async fn save_to_history(
    name: String,
    script_text: String,
    audio_path: String,
    character_mappings: HashMap<String, String>,
) -> Result<ScriptHistory, String> {
    #[derive(serde::Serialize)]
    struct CreateHistoryRequest {
        name: String,
        script_text: String,
        audio_path: String,
        character_mappings: HashMap<String, String>,
    }

    let payload = CreateHistoryRequest {
        name,
        script_text,
        audio_path,
        character_mappings,
    };

    match reqwest::Client::new()
        .post("http://localhost:8000/history")
        .json(&payload)
        .send()
        .await
    {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<ScriptHistory>().await {
                    Ok(entry) => Ok(entry),
                    Err(e) => Err(format!("Failed to parse response: {}", e)),
                }
            } else {
                match resp.text().await {
                    Ok(text) => Err(format!("Failed to save history: {}", text)),
                    Err(_) => Err("Failed to save history".to_string()),
                }
            }
        }
        Err(e) => Err(format!("Failed to save history: {}", e)),
    }
}

pub async fn rename_history_entry(id: String, new_name: String) -> Result<ScriptHistory, String> {
    let mut updates = HashMap::new();
    updates.insert("name".to_string(), new_name);

    match reqwest::Client::new()
        .patch(&format!("http://localhost:8000/history/{}", id))
        .json(&updates)
        .send()
        .await
    {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<ScriptHistory>().await {
                    Ok(entry) => Ok(entry),
                    Err(e) => Err(format!("Failed to parse response: {}", e)),
                }
            } else {
                match resp.text().await {
                    Ok(text) => Err(format!("Failed to rename: {}", text)),
                    Err(_) => Err("Failed to rename".to_string()),
                }
            }
        }
        Err(e) => Err(format!("Failed to rename: {}", e)),
    }
}

pub async fn delete_history_entry(id: String) -> Result<(), String> {
    match reqwest::Client::new()
        .delete(&format!("http://localhost:8000/history/{}", id))
        .send()
        .await
    {
        Ok(resp) => {
            if resp.status().is_success() {
                Ok(())
            } else {
                match resp.text().await {
                    Ok(text) => Err(format!("Failed to delete: {}", text)),
                    Err(_) => Err("Failed to delete".to_string()),
                }
            }
        }
        Err(e) => Err(format!("Failed to delete: {}", e)),
    }
}
