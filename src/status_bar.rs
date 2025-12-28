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

#[derive(Debug, Clone, PartialEq, Default)]
enum MessageType {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

impl StatusBar {
    pub fn new(cx: &mut WindowContext) -> Self {
        Self {
            message: "Welcome to Synapse!".to_string(),
            message_type: MessageType::Info,
            terminal_count: 0,
            active_agents: 0,
            cpu_usage: 0.0,
            memory_usage: 0.0,
        }
    }

    pub fn set_message(&mut self, message: String, message_type: MessageType, cx: &mut ViewContext<Self>) {
        self.message = message;
        self.message_type = message_type;
        cx.notify();
    }

    pub fn update_stats(&mut self, terminal_count: usize, active_agents: usize, cx: &mut ViewContext<Self>) {
        self.terminal_count = terminal_count;
        self.active_agents = active_agents;
        self.cpu_usage = 12.5; // Replace with actual CPU usage
        self.memory_usage = 2.1; // Replace with actual memory usage
        cx.notify();
    }
}

impl Render for StatusBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .w_full()
            .h(px(24.0))
            .bg(rgb(0x1a1a1a))
            .child(
                div()
                    .flex()
                    .items_center()
                    .px_2()
                    .text_color(match self.message_type {
                        MessageType::Info => rgb(0xcccccc),
                        MessageType::Success => rgb(0x00ff00),
                        MessageType::Warning => rgb(0xffaa00),
                        MessageType::Error => rgb(0xff0000),
                    })
                    .child(self.message.clone()),
            )
            .child(div().flex_1())
            .child(
                div()
                    .flex()
                    .items_center()
                    .px_2()
                    .text_color(rgb(0xcccccc))
                    .child(format!("Terminals: {}", self.terminal_count)),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .px_2()
                    .text_color(rgb(0xcccccc))
                    .child(format!("Agents: {}", self.active_agents)),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .px_2()
                    .text_color(rgb(0xcccccc))
                    .child(format!("CPU: {:.1}%", self.cpu_usage)),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .px_2()
                    .text_color(rgb(0xcccccc))
                    .child(format!("MEM: {:.1} GB", self.memory_usage)),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .px_2()
                    .text_color(rgb(0xcccccc))
                    .child(
                        SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .map(|d| format!("{}", d.as_secs()))
                            .unwrap_or_default(),
                    ),
            )
    }
}
