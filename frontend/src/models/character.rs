use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub description: String,
    pub voice_path: Option<String>,
}

impl Character {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description: String::new(),
            voice_path: None,
        }
    }
}
