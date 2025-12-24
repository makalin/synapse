// use gpui::*; // Commented out for CLI version
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub id: String,
    pub name: String,
    pub colors: ThemeColors,
    pub fonts: ThemeFonts,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    pub background: String,
    pub foreground: String,
    pub accent: String,
    pub border: String,
    pub success: String,
    pub error: String,
    pub warning: String,
    pub info: String,
    pub terminal_bg: String,
    pub terminal_fg: String,
    pub editor_bg: String,
    pub editor_fg: String,
    pub sidebar_bg: String,
    pub sidebar_fg: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeFonts {
    pub ui_font: String,
    pub code_font: String,
    pub ui_size: f32,
    pub code_size: f32,
}

pub struct ThemeManager {
    themes: HashMap<String, Theme>,
    current_theme_id: String,
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut manager = Self {
            themes: HashMap::new(),
            current_theme_id: "dark".to_string(),
        };
        
        manager.load_default_themes();
        manager
    }

    fn load_default_themes(&mut self) {
        // Dark Theme
        self.themes.insert("dark".to_string(), Theme {
            id: "dark".to_string(),
            name: "Dark".to_string(),
            colors: ThemeColors {
                background: "#0a0a0a".to_string(),
                foreground: "#ffffff".to_string(),
                accent: "#00ff00".to_string(),
                border: "#333333".to_string(),
                success: "#00ff00".to_string(),
                error: "#ff0000".to_string(),
                warning: "#ffff00".to_string(),
                info: "#0088ff".to_string(),
                terminal_bg: "#000000".to_string(),
                terminal_fg: "#00ff00".to_string(),
                editor_bg: "#1a1a1a".to_string(),
                editor_fg: "#ffffff".to_string(),
                sidebar_bg: "#0f0f0f".to_string(),
                sidebar_fg: "#cccccc".to_string(),
            },
            fonts: ThemeFonts {
                ui_font: "System".to_string(),
                code_font: "Monaco".to_string(),
                ui_size: 12.0,
                code_size: 14.0,
            },
            description: "Default dark theme".to_string(),
        });

        // Light Theme
        self.themes.insert("light".to_string(), Theme {
            id: "light".to_string(),
            name: "Light".to_string(),
            colors: ThemeColors {
                background: "#ffffff".to_string(),
                foreground: "#000000".to_string(),
                accent: "#0066cc".to_string(),
                border: "#cccccc".to_string(),
                success: "#00aa00".to_string(),
                error: "#cc0000".to_string(),
                warning: "#ffaa00".to_string(),
                info: "#0066cc".to_string(),
                terminal_bg: "#f5f5f5".to_string(),
                terminal_fg: "#000000".to_string(),
                editor_bg: "#ffffff".to_string(),
                editor_fg: "#000000".to_string(),
                sidebar_bg: "#f0f0f0".to_string(),
                sidebar_fg: "#333333".to_string(),
            },
            fonts: ThemeFonts {
                ui_font: "System".to_string(),
                code_font: "Monaco".to_string(),
                ui_size: 12.0,
                code_size: 14.0,
            },
            description: "Light theme for daytime use".to_string(),
        });

        // Synthwave Theme
        self.themes.insert("synthwave".to_string(), Theme {
            id: "synthwave".to_string(),
            name: "Synthwave".to_string(),
            colors: ThemeColors {
                background: "#1a0033".to_string(),
                foreground: "#ff00ff".to_string(),
                accent: "#00ffff".to_string(),
                border: "#ff00ff".to_string(),
                success: "#00ff00".to_string(),
                error: "#ff0066".to_string(),
                warning: "#ffaa00".to_string(),
                info: "#00ffff".to_string(),
                terminal_bg: "#0d0015".to_string(),
                terminal_fg: "#00ffff".to_string(),
                editor_bg: "#1a0033".to_string(),
                editor_fg: "#ff00ff".to_string(),
                sidebar_bg: "#0d0015".to_string(),
                sidebar_fg: "#ff00ff".to_string(),
            },
            fonts: ThemeFonts {
                ui_font: "System".to_string(),
                code_font: "Monaco".to_string(),
                ui_size: 12.0,
                code_size: 14.0,
            },
            description: "Retro synthwave aesthetic".to_string(),
        });

        // Dracula Theme
        self.themes.insert("dracula".to_string(), Theme {
            id: "dracula".to_string(),
            name: "Dracula".to_string(),
            colors: ThemeColors {
                background: "#282a36".to_string(),
                foreground: "#f8f8f2".to_string(),
                accent: "#bd93f9".to_string(),
                border: "#44475a".to_string(),
                success: "#50fa7b".to_string(),
                error: "#ff5555".to_string(),
                warning: "#f1fa8c".to_string(),
                info: "#8be9fd".to_string(),
                terminal_bg: "#1e1f29".to_string(),
                terminal_fg: "#f8f8f2".to_string(),
                editor_bg: "#282a36".to_string(),
                editor_fg: "#f8f8f2".to_string(),
                sidebar_bg: "#21222c".to_string(),
                sidebar_fg: "#f8f8f2".to_string(),
            },
            fonts: ThemeFonts {
                ui_font: "System".to_string(),
                code_font: "Monaco".to_string(),
                ui_size: 12.0,
                code_size: 14.0,
            },
            description: "Dracula color scheme".to_string(),
        });

        // Nord Theme
        self.themes.insert("nord".to_string(), Theme {
            id: "nord".to_string(),
            name: "Nord".to_string(),
            colors: ThemeColors {
                background: "#2e3440".to_string(),
                foreground: "#d8dee9".to_string(),
                accent: "#88c0d0".to_string(),
                border: "#3b4252".to_string(),
                success: "#a3be8c".to_string(),
                error: "#bf616a".to_string(),
                warning: "#ebcb8b".to_string(),
                info: "#5e81ac".to_string(),
                terminal_bg: "#1e1e1e".to_string(),
                terminal_fg: "#d8dee9".to_string(),
                editor_bg: "#2e3440".to_string(),
                editor_fg: "#d8dee9".to_string(),
                sidebar_bg: "#1e1e1e".to_string(),
                sidebar_fg: "#d8dee9".to_string(),
            },
            fonts: ThemeFonts {
                ui_font: "System".to_string(),
                code_font: "Monaco".to_string(),
                ui_size: 12.0,
                code_size: 14.0,
            },
            description: "Nord color palette".to_string(),
        });
    }

    pub fn get_theme(&self, id: &str) -> Option<&Theme> {
        self.themes.get(id)
    }

    pub fn get_current_theme(&self) -> Option<&Theme> {
        self.themes.get(&self.current_theme_id)
    }

    pub fn set_theme(&mut self, id: &str) -> bool {
        if self.themes.contains_key(id) {
            self.current_theme_id = id.to_string();
            true
        } else {
            false
        }
    }

    pub fn add_theme(&mut self, theme: Theme) {
        self.themes.insert(theme.id.clone(), theme);
    }

    pub fn get_all_themes(&self) -> Vec<&Theme> {
        self.themes.values().collect()
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

