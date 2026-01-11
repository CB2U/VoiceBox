use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, PartialEq, Debug)]
pub enum SynthesisStatus {
    Idle,
    Queued,
    Working,
    Done,
    Error(String),
}

#[derive(Clone, PartialEq, Debug)]
pub struct ScriptLine {
    pub id: String,
    pub character_id: Option<String>,
    pub character_name: String,
    pub text: String,
    pub status: SynthesisStatus,
    pub output_path: Option<String>,
}

impl ScriptLine {
    pub fn new(character_name: String, text: String, character_id: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            character_id,
            character_name,
            text,
            status: SynthesisStatus::Idle,
            output_path: None,
        }
    }
}
