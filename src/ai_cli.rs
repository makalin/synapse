// use gpui::*; // Commented out for CLI version
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use tokio::process::Command as AsyncCommand;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AICLITool {
    pub id: String,
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub env_vars: HashMap<String, String>,
    pub description: String,
    pub version: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AICLIType {
    Aider,
    GPTPilot,
    CursorCLI,
    Continue,
    AiderPro,
    Custom(String),
}

pub struct AICLIManager {
    tools: Vec<AICLITool>,
    active_connections: Arc<Mutex<HashMap<String, tokio::process::Child>>>,
}

impl AICLIManager {
    pub fn new() -> Self {
        let mut manager = Self {
            tools: Vec::new(),
            active_connections: Arc::new(Mutex::new(HashMap::new())),
        };
        
        // Initialize known AI CLI tools
        manager.initialize_default_tools();
        manager
    }

    fn initialize_default_tools(&mut self) {
        // Aider
        self.tools.push(AICLITool {
            id: "aider".to_string(),
            name: "Aider".to_string(),
            command: "aider".to_string(),
            args: vec![],
            env_vars: HashMap::new(),
            description: "AI pair programming tool that lets you write code collaboratively".to_string(),
            version: "0.0.0".to_string(),
            enabled: true,
        });

        // GPT-Pilot
        self.tools.push(AICLITool {
            id: "gpt-pilot".to_string(),
            name: "GPT-Pilot".to_string(),
            command: "pilot".to_string(),
            args: vec![],
            env_vars: HashMap::new(),
            description: "AI software development platform".to_string(),
            version: "0.0.0".to_string(),
            enabled: true,
        });

        // Cursor CLI
        self.tools.push(AICLITool {
            id: "cursor-cli".to_string(),
            name: "Cursor CLI".to_string(),
            command: "cursor".to_string(),
            args: vec![],
            env_vars: HashMap::new(),
            description: "Cursor editor CLI interface".to_string(),
            version: "0.0.0".to_string(),
            enabled: true,
        });

        // Continue
        self.tools.push(AICLITool {
            id: "continue".to_string(),
            name: "Continue".to_string(),
            command: "continue".to_string(),
            args: vec![],
            env_vars: HashMap::new(),
            description: "Open-source autopilot for VS Code".to_string(),
            version: "0.0.0".to_string(),
            enabled: true,
        });
    }

    pub fn detect_installed_tools(&mut self) -> Vec<String> {
        let mut detected = Vec::new();
        
        for tool in &self.tools {
            if self.check_tool_installed(&tool.command) {
                detected.push(tool.id.clone());
            }
        }
        
        detected
    }

    fn check_tool_installed(&self, command: &str) -> bool {
        Command::new("which")
            .arg(command)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    pub async fn execute_tool(
        &mut self,
        tool_id: &str,
        input: &str,
        context: &str,
    ) -> anyhow::Result<String> {
        let tool = self.tools.iter()
            .find(|t| t.id == tool_id)
            .ok_or_else(|| anyhow::anyhow!("Tool not found"))?;

        if !tool.enabled {
            return Err(anyhow::anyhow!("Tool is disabled"));
        }

        let mut cmd = AsyncCommand::new(&tool.command);
        cmd.args(&tool.args);
        cmd.envs(&tool.env_vars);
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut child = cmd.spawn()?;
        
        // Send input
        if let Some(mut stdin) = child.stdin.take() {
            use tokio::io::AsyncWriteExt;
            stdin.write_all(input.as_bytes()).await?;
            stdin.write_all(b"\n").await?;
            stdin.write_all(context.as_bytes()).await?;
        }

        let output = child.wait_with_output().await?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(anyhow::anyhow!(
                "Tool execution failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    pub fn add_custom_tool(&mut self, tool: AICLITool) {
        self.tools.push(tool);
    }

    pub fn get_tools(&self) -> &[AICLITool] {
        &self.tools
    }

    pub fn enable_tool(&mut self, tool_id: &str, enabled: bool) {
        if let Some(tool) = self.tools.iter_mut().find(|t| t.id == tool_id) {
            tool.enabled = enabled;
        }
    }
}

impl Default for AICLIManager {
    fn default() -> Self {
        Self::new()
    }
}

// UI Panel commented out for CLI version
// All UI rendering code has been removed for CLI compilation
