use directories::UserDirs;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use egui::Pos2;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    pub window_pos: Pos2,
    pub excel_path: PathBuf,
    pub sound_enabled: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            window_pos: Pos2::new(1840.0, 0.0),  // Standardposition
            excel_path: get_default_excel_path(),
            sound_enabled: true,
        }
    }
}

impl Settings {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(config_dir) = dirs::config_dir() {
            let config_file = config_dir.join("ticket-tracker/config.toml");
            if config_file.exists() {
                let content = fs::read_to_string(config_file)?;
                return Ok(toml::from_str(&content)?);
            }
        }
        Ok(Self::default())
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(config_dir) = dirs::config_dir() {
            let config_dir = config_dir.join("ticket-tracker");
            fs::create_dir_all(&config_dir)?;
            let config_file = config_dir.join("config.toml");
            let content = toml::to_string_pretty(self)?;
            fs::write(config_file, content)?;
        }
        Ok(())
    }
}

fn get_default_excel_path() -> PathBuf {
    if let Some(user_dirs) = UserDirs::new() {
        if let Some(doc_dir) = user_dirs.document_dir() {
            return doc_dir.join("tickets.xlsx");
        }
    }
    PathBuf::from("tickets.xlsx")
}
