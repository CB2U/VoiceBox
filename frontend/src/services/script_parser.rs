use crate::models::character::Character;
use crate::models::script::ScriptLine;
use regex::Regex;

/// Parses a script string into a vector of ScriptLine items.
/// 
/// The parser looks for lines matching the pattern: `[CharacterName]: dialogue text`
/// - Lines that don't match this pattern are ignored
/// - Empty lines are ignored
/// - Character names are matched case-insensitively against the provided character list
/// 
/// # Arguments
/// * `text` - The raw script text to parse
/// * `characters` - A slice of available characters to validate against
/// 
/// # Returns
/// A vector of ScriptLine items, one for each valid dialogue line found
pub fn parse_script(text: &str, characters: &[Character]) -> Vec<ScriptLine> {
    // Regex pattern: [Name]: Text
    // Captures: 1 = character name, 2 = dialogue text
    let line_pattern = Regex::new(r"^\s*\[([^\]]+)\]\s*:\s*(.+)\s*$").unwrap();
    
    let mut script_lines = Vec::new();
    
    for line in text.lines() {
        // Skip empty lines
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        // Try to match the dialogue pattern
        if let Some(captures) = line_pattern.captures(trimmed) {
            let character_name = captures.get(1).unwrap().as_str().trim().to_string();
            let dialogue_text = captures.get(2).unwrap().as_str().trim().to_string();
            
            // Find matching character (case-insensitive)
            let character_id = characters
                .iter()
                .find(|c| c.name.eq_ignore_ascii_case(&character_name))
                .map(|c| c.id.clone());
            
            script_lines.push(ScriptLine::new(
                character_name,
                dialogue_text,
                character_id,
            ));
        }
        // Lines that don't match are silently ignored (as per spec)
    }
    
    script_lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::character::Character;

    #[test]
    fn test_parse_simple_script() {
        let characters = vec![
            Character::new("Gandalf".to_string()),
            Character::new("Frodo".to_string()),
        ];
        
        let script = "[Gandalf]: You cannot pass!\n[Frodo]: I wish the ring had never come to me.";
        let lines = parse_script(script, &characters);
        
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].character_name, "Gandalf");
        assert_eq!(lines[0].text, "You cannot pass!");
        assert!(lines[0].character_id.is_some());
        
        assert_eq!(lines[1].character_name, "Frodo");
        assert_eq!(lines[1].text, "I wish the ring had never come to me.");
        assert!(lines[1].character_id.is_some());
    }

    #[test]
    fn test_parse_with_empty_lines() {
        let characters = vec![Character::new("Gandalf".to_string())];
        
        let script = "\n[Gandalf]: Hello\n\n\n[Gandalf]: World\n";
        let lines = parse_script(script, &characters);
        
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].text, "Hello");
        assert_eq!(lines[1].text, "World");
    }

    #[test]
    fn test_parse_unknown_character() {
        let characters = vec![Character::new("Gandalf".to_string())];
        
        let script = "[Unknown]: Hello there";
        let lines = parse_script(script, &characters);
        
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].character_name, "Unknown");
        assert!(lines[0].character_id.is_none());
    }

    #[test]
    fn test_parse_case_insensitive() {
        let characters = vec![Character::new("Gandalf".to_string())];
        
        let script = "[gandalf]: lowercase\n[GANDALF]: uppercase\n[GaNdAlF]: mixed";
        let lines = parse_script(script, &characters);
        
        assert_eq!(lines.len(), 3);
        assert!(lines[0].character_id.is_some());
        assert!(lines[1].character_id.is_some());
        assert!(lines[2].character_id.is_some());
    }

    #[test]
    fn test_parse_ignores_non_dialogue_lines() {
        let characters = vec![Character::new("Gandalf".to_string())];
        
        let script = "This is a stage direction\n[Gandalf]: Actual dialogue\nAnother note";
        let lines = parse_script(script, &characters);
        
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].text, "Actual dialogue");
    }

    #[test]
    fn test_parse_with_extra_whitespace() {
        let characters = vec![Character::new("Gandalf".to_string())];
        
        let script = "  [ Gandalf ]  :   Hello world   ";
        let lines = parse_script(script, &characters);
        
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].character_name, "Gandalf");
        assert_eq!(lines[0].text, "Hello world");
    }

    #[test]
    fn test_parse_empty_script() {
        let characters = vec![Character::new("Gandalf".to_string())];
        
        let script = "";
        let lines = parse_script(script, &characters);
        
        assert_eq!(lines.len(), 0);
    }
}
