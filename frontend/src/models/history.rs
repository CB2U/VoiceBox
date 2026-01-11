use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScriptHistory {
    pub id: String,
    pub name: String,
    pub script_text: String,
    pub audio_path: String,
    pub created_at: DateTime<Utc>,
    pub character_mappings: HashMap<String, String>,
}
