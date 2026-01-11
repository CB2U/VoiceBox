use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    pub output_directory: String,
    pub voice_files_directory: String,
    pub projects_directory: String,
    pub active_project_id: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            output_directory: String::from("frontend/data/output"),
            voice_files_directory: String::from("frontend/data/voices"),
            projects_directory: String::from("frontend/data/projects"),
            active_project_id: None,
        }
    }
}
