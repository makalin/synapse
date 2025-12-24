use gpui::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct StatusBar {
    message: String,
    message_type: MessageType,
    terminal_count: usize,
    active_agents: usize,
    cpu_usage: f32,
    memory_usage: f32,
}

#[derive(Debug, Clone, PartialEq)]
enum MessageType {
    Info,
    Success,
    Warning,
    Error,
}

impl StatusBar {
    pub fn new(cx: &mut WindowContext) -> impl View {
        // Simplified - return the view directly
        div()
            .flex()
            .w_full()
            .h(px(24.0))
            .bg(rgb(0x1a1a1a))
            .text("Status Bar")
    }

    pub fn set_message(&mut self, message: String, message_type: MessageType) {
        self.message = message;
        self.message_type = message_type;
    }

    pub fn update_stats(&mut self, terminal_count: usize, active_agents: usize) {
        self.terminal_count = terminal_count;
        self.active_agents = active_agents;
        self.cpu_usage = 12.5;
        self.memory_usage = 2.1;
    }
}
