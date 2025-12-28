// use gpui::*; // Commented out for CLI version
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangelogEntry {
    pub version: String,
    pub date: String,
    pub changes: Vec<Change>,
    pub category: ChangeCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChangeCategory {
    Added,
    Changed,
    Fixed,
    Removed,
    Security,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    pub category: ChangeCategory,
    pub description: String,
}

pub struct ChangelogManager {
    entries: Vec<ChangelogEntry>,
}

impl ChangelogManager {
    pub fn new() -> Self {
        let mut manager = Self {
            entries: Vec::new(),
        };
        
        manager.load_changelog();
        manager
    }

    fn load_changelog(&mut self) {
        // Version 0.2.0
        self.entries.push(ChangelogEntry {
            version: "0.2.0".to_string(),
            date: "2025-01-XX".to_string(),
            changes: vec![
                Change {
                    category: ChangeCategory::Added,
                    description: "AI CLI integration system with support for Aider, GPT-Pilot, Cursor CLI, and Continue".to_string(),
                },
                Change {
                    category: ChangeCategory::Added,
                    description: "Scripting engine with Lua, JavaScript, Python, and Shell support".to_string(),
                },
                Change {
                    category: ChangeCategory::Added,
                    description: "Enhanced theme system with 5 built-in themes and custom theme support".to_string(),
                },
                Change {
                    category: ChangeCategory::Added,
                    description: "Text-to-Speech (TTS) and speech recognition for accessibility".to_string(),
                },
                Change {
                    category: ChangeCategory::Added,
                    description: "Built-in how-to guides system".to_string(),
                },
                Change {
                    category: ChangeCategory::Added,
                    description: "Changelog and update notes viewer".to_string(),
                },
                Change {
                    category: ChangeCategory::Changed,
                    description: "Improved UI with tab-based navigation".to_string(),
                },
                Change {
                    category: ChangeCategory::Changed,
                    description: "Enhanced settings with more customization options".to_string(),
                },
            ],
            category: ChangeCategory::Added,
        });

        // Version 0.1.0
        self.entries.push(ChangelogEntry {
            version: "0.1.0".to_string(),
            date: "2025-01-XX".to_string(),
            changes: vec![
                Change {
                    category: ChangeCategory::Added,
                    description: "Initial release with terminal grid, roster, and lens editor".to_string(),
                },
                Change {
                    category: ChangeCategory::Added,
                    description: "PTY-based terminal integration".to_string(),
                },
                Change {
                    category: ChangeCategory::Added,
                    description: "Report window for agent analytics".to_string(),
                },
                Change {
                    category: ChangeCategory::Added,
                    description: "Log tracking panel with filtering".to_string(),
                },
                Change {
                    category: ChangeCategory::Added,
                    description: "Status bar with system information".to_string(),
                },
                Change {
                    category: ChangeCategory::Added,
                    description: "Agent management system".to_string(),
                },
                Change {
                    category: ChangeCategory::Added,
                    description: "Settings system with JSON persistence".to_string(),
                },
            ],
            category: ChangeCategory::Added,
        });
    }

    pub fn get_entries(&self) -> &[ChangelogEntry] {
        &self.entries
    }

    pub fn get_latest_version(&self) -> Option<&ChangelogEntry> {
        self.entries.first()
    }
}

impl Default for ChangelogManager {
    fn default() -> Self {
        Self::new()
    }
}

// UI Viewer commented out for CLI version
