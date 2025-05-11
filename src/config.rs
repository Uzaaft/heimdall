use std::{
    fmt,
    path::{Path, PathBuf},
    str::FromStr,
};

use global_hotkey::hotkey::Code;
use serde::Deserialize;

use crate::error::{AppError, AppResult};

#[derive(Deserialize, Debug)]
pub(crate) enum Arrow {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Arrow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Up => write!(f, "Up"),
            Self::Down => write!(f, "Down"),
            Self::Left => write!(f, "Left"),
            Self::Right => write!(f, "Right"),
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
impl fmt::Display for Binding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let modifiers = self.modifiers.join("+").to_lowercase();
        match &self.key {
            Some(key) => {
                if key.parse::<u32>().is_ok() {
                    write!(f, "Digit{key}")
                } else if key.as_str() == "Enter" {
                    write!(f, "{modifiers}+{}", Code::Enter)
                } else if key.as_str() == "=" {
                    write!(f, "Equal")
                } else if modifiers.is_empty() {
                    write!(f, "Key{}", key.to_uppercase())
                } else {
                    write!(f, "{modifiers}+Key{}", key.to_uppercase())
                }
            }
            None => match &self.arrow {
                Some(arrow) => {
                    if modifiers.is_empty() {
                        write!(f, "Arrow{}", arrow)
                    } else {
                        write!(f, "{}+Arrow{}", modifiers, arrow)
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
    pub fn from_str(s: &str) -> AppResult<Self, toml::de::Error> {
        toml::from_str(s)
    }

    pub fn config_path() -> AppResult<PathBuf> {
        let s = std::env::var("XDG_CONFIG_HOME")
            .unwrap_or_else(|_| format!("{}/.config", std::env::var("HOME").unwrap()))
            + "/heimdall/config.toml";
        PathBuf::from_str(&s).map_err(|e| AppError::Config(e.to_string()))
    }

    /// Read config file from XDG_CONFIG_HOME. Fallback to ~/.config/heimdall/config.toml
    pub fn read_config() -> AppResult<Self> {
        let config_path = Self::config_path()?;
        if !Path::is_file(&config_path) {
            return Err(AppError::Config("Config file not found".to_string()));
        }
        let config_file = std::fs::read_to_string(config_path)?;
        Ok(Self::from_str(&config_file)?)
    }
}
