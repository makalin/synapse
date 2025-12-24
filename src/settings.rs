// use gpui::*; // Commented out for CLI version
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: ThemeSettings,
    pub terminal: TerminalSettings,
    pub editor: EditorSettings,
    pub agents: AgentSettings,
    pub ui: UISettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeSettings {
    pub background: String,
    pub foreground: String,
    pub accent: String,
    pub border: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalSettings {
    pub font_size: f32,
    pub font_family: String,
    pub default_shell: String,
    pub scrollback_lines: usize,
    pub cursor_style: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSettings {
    pub font_size: f32,
    pub font_family: String,
    pub tab_size: usize,
    pub word_wrap: bool,
    pub line_numbers: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSettings {
    pub auto_start: bool,
    pub max_concurrent: usize,
    pub timeout_seconds: u64,
    pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UISettings {
    pub show_status_bar: bool,
    pub show_roster: bool,
    pub roster_width: f32,
    pub animation_enabled: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: ThemeSettings {
                background: "#0a0a0a".to_string(),
                foreground: "#ffffff".to_string(),
                accent: "#00ff00".to_string(),
                border: "#333333".to_string(),
            },
            terminal: TerminalSettings {
                font_size: 12.0,
                font_family: "Monaco".to_string(),
                default_shell: "sh".to_string(),
                scrollback_lines: 1000,
                cursor_style: "block".to_string(),
            },
            editor: EditorSettings {
                font_size: 14.0,
                font_family: "Monaco".to_string(),
                tab_size: 4,
                word_wrap: false,
                line_numbers: true,
            },
            agents: AgentSettings {
                auto_start: false,
                max_concurrent: 5,
                timeout_seconds: 300,
                log_level: "info".to_string(),
            },
            ui: UISettings {
                show_status_bar: true,
                show_roster: true,
                roster_width: 0.15,
                animation_enabled: true,
            },
        }
    }
}

pub struct SettingsManager {
    settings: AppSettings,
    config_path: PathBuf,
}

impl SettingsManager {
    pub fn new() -> Self {
        let config_path = Self::get_config_path();
        let settings = Self::load_settings(&config_path);
        
        Self {
            settings,
            config_path,
        }
    }

    fn get_config_path() -> PathBuf {
        if let Some(home) = dirs::home_dir() {
            home.join(".synapse").join("config.json")
        } else {
            PathBuf::from("./synapse_config.json")
        }
    }

    fn load_settings(path: &PathBuf) -> AppSettings {
        if path.exists() {
            if let Ok(contents) = fs::read_to_string(path) {
                if let Ok(settings) = serde_json::from_str::<AppSettings>(&contents) {
                    return settings;
                }
            }
        }
        AppSettings::default()
    }

    pub fn get(&self) -> &AppSettings {
        &self.settings
    }

    pub fn update<F>(&mut self, f: F) -> anyhow::Result<()>
    where
        F: FnOnce(&mut AppSettings),
    {
        f(&mut self.settings);
        self.save()
    }

    pub fn save(&self) -> anyhow::Result<()> {
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(&self.settings)?;
        fs::write(&self.config_path, json)?;
        Ok(())
    }

    pub fn reset_to_defaults(&mut self) -> anyhow::Result<()> {
        self.settings = AppSettings::default();
        self.save()
    }
}

impl Default for SettingsManager {
    fn default() -> Self {
        Self::new()
    }
}

