// use gpui::*; // Commented out for CLI version
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guide {
    pub id: String,
    pub title: String,
    pub category: GuideCategory,
    pub content: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GuideCategory {
    GettingStarted,
    Features,
    AI,
    Scripting,
    Customization,
    Troubleshooting,
    Advanced,
}

pub struct GuideManager {
    guides: Vec<Guide>,
}

impl GuideManager {
    pub fn new() -> Self {
        let mut manager = Self {
            guides: Vec::new(),
        };
        
        manager.load_default_guides();
        manager
    }

    fn load_default_guides(&mut self) {
        // Getting Started
        self.guides.push(Guide {
            id: "getting-started".to_string(),
            title: "Getting Started with SYNAPSE".to_string(),
            category: GuideCategory::GettingStarted,
            content: r#"
# Getting Started with SYNAPSE

SYNAPSE is a runtime-first AI orchestration console designed for modern development workflows.

## First Steps

1. **Create Your First Terminal**
   - Press `Cmd+N` or use the File menu
   - A new terminal will appear in the grid

2. **Explore the Interface**
   - **Grid**: Center area for terminals and processes
   - **Roster**: Sidebar showing file tree and active agents
   - **Lens**: Overlay code editor (toggle with View menu)

3. **Connect AI Tools**
   - Go to AI CLI Tools panel
   - SYNAPSE will auto-detect installed tools
   - Configure API keys in Settings

## Basic Workflow

1. Start terminals for different tasks
2. Run AI agents in parallel
3. Monitor outputs in the Logs panel
4. Review results in Reports
5. Use the Lens for quick code edits

## Next Steps

- Read the "AI Integration" guide
- Check out "Scripting" for automation
- Explore themes in Settings
"#.to_string(),
            tags: vec!["beginner".to_string(), "setup".to_string()],
        });

        // AI Integration
        self.guides.push(Guide {
            id: "ai-integration".to_string(),
            title: "AI CLI Integration".to_string(),
            category: GuideCategory::AI,
            content: r#"
# AI CLI Integration

SYNAPSE supports multiple AI CLI tools for code generation and assistance.

## Supported Tools

### Aider
- Install: `pip install aider-chat`
- Usage: Select Aider in AI CLI panel
- Features: Pair programming, code editing

### GPT-Pilot
- Install: `npm install -g gpt-pilot`
- Usage: Run pilot commands in terminal
- Features: Full project generation

### Cursor CLI
- Install: Follow Cursor documentation
- Usage: Use cursor command in terminals
- Features: AI-powered editing

## Configuration

1. Open Settings → AI Tools
2. Enter API keys for each service
3. Enable/disable specific tools
4. Set default tool for new sessions

## Best Practices

- Use different tools for different tasks
- Monitor token usage in Reports
- Set up rate limiting for API calls
"#.to_string(),
            tags: vec!["ai".to_string(), "integration".to_string()],
        });

        // Scripting
        self.guides.push(Guide {
            id: "scripting".to_string(),
            title: "Scripting in SYNAPSE".to_string(),
            category: GuideCategory::Scripting,
            content: r#"
# Scripting Support

SYNAPSE includes a powerful scripting engine for automation.

## Supported Languages

- **Lua**: Lightweight, fast scripting
- **JavaScript**: Web-standard scripting
- **Python**: Full Python support
- **Shell**: Bash/Zsh scripts

## Creating Scripts

1. Open Script Editor
2. Select language
3. Write your script
4. Save and run

## Example Scripts

### Format Code
\`\`\`bash
#!/bin/bash
# Format all Rust files
find . -name "*.rs" -exec rustfmt {} \;
\`\`\`

### Auto-deploy
\`\`\`python
import subprocess
subprocess.run(["cargo", "build", "--release"])
subprocess.run(["git", "add", "."])
subprocess.run(["git", "commit", "-m", "Auto-commit"])
\`\`\`

## Script Context

Scripts have access to:
- Terminal IDs
- Agent IDs
- Environment variables
- Current project path
"#.to_string(),
            tags: vec!["scripting".to_string(), "automation".to_string()],
        });

        // Themes
        self.guides.push(Guide {
            id: "themes".to_string(),
            title: "Customizing Themes".to_string(),
            category: GuideCategory::Customization,
            content: r#"
# Theme Customization

SYNAPSE comes with multiple built-in themes and supports custom themes.

## Built-in Themes

- **Dark**: Default dark theme
- **Light**: Light theme for daytime
- **Synthwave**: Retro aesthetic
- **Dracula**: Popular color scheme
- **Nord**: Arctic color palette

## Creating Custom Themes

1. Open Settings → Themes
2. Click "Create New Theme"
3. Customize colors and fonts
4. Save and apply

## Theme Components

- Background colors
- Foreground colors
- Accent colors
- Terminal colors
- Editor colors
- Font settings

## Export/Import

Themes can be exported as JSON and shared with others.
"#.to_string(),
            tags: vec!["themes".to_string(), "customization".to_string()],
        });

        // Troubleshooting
        self.guides.push(Guide {
            id: "troubleshooting".to_string(),
            title: "Troubleshooting".to_string(),
            category: GuideCategory::Troubleshooting,
            content: r#"
# Troubleshooting

Common issues and solutions.

## Terminal Not Starting

- Check if shell is installed
- Verify PATH environment variable
- Check terminal permissions

## AI Tools Not Detected

- Ensure tools are in PATH
- Check installation
- Restart SYNAPSE
- Manually add in Settings

## Performance Issues

- Reduce number of terminals
- Disable unused features
- Check system resources
- Update graphics drivers

## Crashes

- Check logs in Logs panel
- Report issues with log files
- Try resetting settings
"#.to_string(),
            tags: vec!["troubleshooting".to_string(), "help".to_string()],
        });
    }

    pub fn get_guides(&self) -> &[Guide] {
        &self.guides
    }

    pub fn get_guide(&self, id: &str) -> Option<&Guide> {
        self.guides.iter().find(|g| g.id == id)
    }

    pub fn get_guides_by_category(&self, category: &GuideCategory) -> Vec<&Guide> {
        self.guides.iter()
            .filter(|g| &g.category == category)
            .collect()
    }
}

impl Default for GuideManager {
    fn default() -> Self {
        Self::new()
    }
}

// UI Viewer commented out for CLI version
