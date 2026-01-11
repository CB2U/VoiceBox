use std::path::PathBuf;
use std::fs;
use crate::models::character::Character;

const DATA_FILE: &str = "data/characters.json";

fn get_data_file_path() -> PathBuf {
    // For now, use current directory. In production this should be Config dir.
    let mut path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    path.push(DATA_FILE);
    path
}

pub fn load_characters() -> Vec<Character> {
    load_from_path(get_data_file_path())
}

fn load_from_path(path: PathBuf) -> Vec<Character> {
    if !path.exists() {
        return Vec::new();
    }
    
    match fs::read_to_string(&path) {
        Ok(content) => {
            serde_json::from_str(&content).unwrap_or_else(|e| {
                println!("Failed to parse characters: {}", e);
                Vec::new()
            })
        }
        Err(e) => {
            println!("Failed to read characters file: {}", e);
            Vec::new()
        }
    }
}

pub fn save_characters(characters: &[Character]) -> Result<(), std::io::Error> {
    save_to_path(characters, get_data_file_path())
}

fn save_to_path(characters: &[Character], path: PathBuf) -> Result<(), std::io::Error> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(characters)?;
    fs::write(path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_save_and_load() {
        let mut path = std::env::temp_dir();
        path.push(format!("test_chars_{}.json", Uuid::new_v4()));

        let chars = vec![
            Character::new("Alice".to_string()),
            Character::new("Bob".to_string()),
        ];

        // Save
        save_to_path(&chars, path.clone()).expect("Failed to save");

        // Load
        let loaded = load_from_path(path.clone());

        assert_eq!(chars, loaded);
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].name, "Alice");

        // Cleanup
        let _ = fs::remove_file(path);
    }
    
    #[test]
    fn test_load_non_existent() {
        let mut path = std::env::temp_dir();
        path.push(format!("test_nonexistent_{}.json", Uuid::new_v4()));
        let loaded = load_from_path(path);
        assert!(loaded.is_empty());
    }
}
