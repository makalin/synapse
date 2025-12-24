// use gpui::*; // Commented out for CLI version
use serde::{Deserialize, Serialize};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub status: AgentStatus,
    pub created_at: u64,
    pub pid: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgentStatus {
    Stopped,
    Starting,
    Running,
    Stopping,
    Error(String),
}

pub struct AgentManager {
    agents: Vec<Agent>,
    processes: Arc<Mutex<Vec<(String, Child)>>>,
}

impl AgentManager {
    pub fn new() -> Self {
        Self {
            agents: Vec::new(),
            processes: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_agent(&mut self, name: String, command: String, args: Vec<String>) -> Agent {
        let id = format!("agent_{}", SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos());
        
        let agent = Agent {
            id: id.clone(),
            name,
            command,
            args,
            status: AgentStatus::Stopped,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            pid: None,
        };

        self.agents.push(agent.clone());
        agent
    }

    pub fn start_agent(&mut self, id: &str) -> anyhow::Result<()> {
        if let Some(agent) = self.agents.iter_mut().find(|a| a.id == id) {
            agent.status = AgentStatus::Starting;

            let mut cmd = Command::new(&agent.command);
            cmd.args(&agent.args);
            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::piped());

            match cmd.spawn() {
                Ok(child) => {
                    let pid = child.id();
                    agent.pid = Some(pid);
                    agent.status = AgentStatus::Running;
                    
                    let mut processes = self.processes.lock().unwrap();
                    processes.push((id.to_string(), child));
                    Ok(())
                }
                Err(e) => {
                    agent.status = AgentStatus::Error(e.to_string());
                    Err(anyhow::anyhow!("Failed to start agent: {}", e))
                }
            }
        } else {
            Err(anyhow::anyhow!("Agent not found"))
        }
    }

    pub fn stop_agent(&mut self, id: &str) -> anyhow::Result<()> {
        if let Some(agent) = self.agents.iter_mut().find(|a| a.id == id) {
            agent.status = AgentStatus::Stopping;

            let mut processes = self.processes.lock().unwrap();
            if let Some(pos) = processes.iter().position(|(aid, _)| aid == id) {
                let (_, mut child) = processes.remove(pos);
                if let Err(e) = child.kill() {
                    agent.status = AgentStatus::Error(e.to_string());
                    return Err(anyhow::anyhow!("Failed to kill agent: {}", e));
                }
            }

            agent.status = AgentStatus::Stopped;
            agent.pid = None;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Agent not found"))
        }
    }

    pub fn remove_agent(&mut self, id: &str) -> anyhow::Result<()> {
        if let Some(agent) = self.agents.iter().find(|a| a.id == id) {
            if agent.status == AgentStatus::Running {
                self.stop_agent(id)?;
            }
        }

        self.agents.retain(|a| a.id != id);
        Ok(())
    }

    pub fn get_agents(&self) -> &[Agent] {
        &self.agents
    }

    pub fn get_agent(&self, id: &str) -> Option<&Agent> {
        self.agents.iter().find(|a| a.id == id)
    }

    pub fn update_agent_status(&mut self) {
        let mut processes = self.processes.lock().unwrap();
        let mut finished_ids = Vec::new();
        
        for (id, child) in processes.iter_mut() {
            match child.try_wait() {
                Ok(Some(_)) => {
                    // Process finished
                    finished_ids.push(id.clone());
                }
                Ok(None) => {
                    // Still running
                }
                Err(_) => {
                    // Error checking status
                    finished_ids.push(id.clone());
                }
            }
        }
        
        // Remove finished processes
        processes.retain(|(id, _)| !finished_ids.contains(id));
        
        // Update agent statuses
        for id in finished_ids {
            if let Some(agent) = self.agents.iter_mut().find(|a| a.id == id) {
                agent.status = AgentStatus::Stopped;
                agent.pid = None;
            }
        }
    }
}

impl Default for AgentManager {
    fn default() -> Self {
        Self::new()
    }
}

