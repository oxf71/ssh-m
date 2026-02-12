use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    #[serde(default = "default_terminal")]
    pub default_terminal: String,
    #[serde(default = "default_ssh_config_path")]
    pub ssh_config_path: String,
}

fn default_terminal() -> String {
    "terminal".to_string()
}

fn default_ssh_config_path() -> String {
    "~/.ssh/config".to_string()
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            default_terminal: default_terminal(),
            ssh_config_path: default_ssh_config_path(),
        }
    }
}

fn settings_path() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    home.join(".ssh-m").join("settings.json")
}

pub fn load_settings() -> AppSettings {
    let path = settings_path();
    if let Ok(content) = fs::read_to_string(&path) {
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        AppSettings::default()
    }
}

pub fn save_settings_to_file(settings: &AppSettings) -> Result<(), String> {
    let path = settings_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create settings dir: {}", e))?;
    }
    let content = serde_json::to_string_pretty(settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;
    fs::write(&path, content).map_err(|e| format!("Failed to write settings: {}", e))?;
    Ok(())
}
