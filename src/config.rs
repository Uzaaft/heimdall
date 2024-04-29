use anyhow::Result;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) enum Arrow {
    Up,
    Down,
    Left,
    Right,
}

impl ToString for Arrow {
    fn to_string(&self) -> String {
        match self {
            Self::Up => "Up".to_string(),
            Self::Down => "Down".to_string(),
            Self::Left => "Left".to_string(),
            Self::Right => "Right".to_string(),
        }
    }
}

impl From<Arrow> for String {
    fn from(arrow: Arrow) -> Self {
        arrow.to_string()
    }
}
impl From<String> for Arrow {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Up" => Self::Up,
            "Down" => Self::Down,
            "Left" => Self::Left,
            "Right" => Self::Right,
            _ => panic!("Invalid arrow"),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Binding {
    pub key: Option<String>,
    pub arrow: Option<Arrow>,
    // TODO: Make this optional,
    pub modifiers: Vec<String>,
    pub command: String,
}

// To string
impl ToString for Binding {
    fn to_string(&self) -> String {
        let modifiers = self.modifiers.join("+");
        match &self.key {
            Some(key) => {
                if &modifiers == "=" {
                    "Equal".to_string()
                } else if modifiers.is_empty() {
                    format!("Key{}", key.to_uppercase())
                } else {
                    format!("{}+Key{}", modifiers, key.to_uppercase())
                }
            }
            None => match &self.arrow {
                Some(arrow) => {
                    if modifiers.is_empty() {
                        format!("Arrow{}", arrow.to_string())
                    } else {
                        format!("{}+Arrow{}", modifiers, arrow.to_string())
                    }
                }
                None => panic!("Invalid binding. Must have either key or arrow"),
            },
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub bindings: Vec<Binding>,
}

impl Config {
    pub fn from_str(s: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(s)
    }
    // Read config file from XDG_CONFIG_HOME. Fallback to ~/.config/heimdall/config.toml
    // TODO: Add better error handling
    pub fn read_config() -> Result<Self> {
        let config_path = std::env::var("XDG_CONFIG_HOME")
            .unwrap_or_else(|_| format!("{}/.config", std::env::var("HOME").unwrap()))
            + "/heimdall/config.toml";
        let config_file = std::fs::read_to_string(config_path)?;
        Ok(Self::from_str(&config_file)?)
    }
}

#[cfg(test)]
mod test {
    const CONFIG: &str = r#"
        [[bindings]]
        key = "C"
        modifiers = ["Ctrl", "Shift"]
        command = "echo hello"
        [[bindings]]
        key = "D"
        modifiers = ["Ctrl"]
        command = "echo hello"
        "#;
    use super::*;
    #[test]
    fn test_parsing_config() {
        let config = Config::from_str(CONFIG).unwrap();
        assert_eq!(config.bindings.first().unwrap().key, "C");
        assert_eq!(
            config.bindings.first().unwrap().modifiers,
            vec!["Ctrl", "Shift"]
        );
        assert_eq!(config.bindings.first().unwrap().command, "echo hello");
        // Second binding
        assert_eq!(config.bindings.last().unwrap().key, "D");
        assert_eq!(config.bindings.last().unwrap().modifiers, vec!["Ctrl"]);
        assert_eq!(config.bindings.last().unwrap().command, "echo hello");
    }

    #[test]
    fn test_binding_to_string() {
        let binding = Binding {
            key: "C".to_string(),
            modifiers: vec!["Ctrl".to_string(), "Shift".to_string()],
            command: "echo hello".to_string(),
        };
        assert_eq!(binding.to_string(), "Ctrl+Shift+KeyC");
    }
}
