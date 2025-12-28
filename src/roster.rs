use gpui::*;

pub struct Roster {
    file_tree: Vec<String>,
    active_agents: Vec<String>,
}

impl Roster {
    pub fn new() -> Self {
        Self {
            file_tree: vec![
                "src/".to_string(),
                "  main.rs".to_string(),
                "  app.rs".to_string(),
                "  grid.rs".to_string(),
                "  terminal.rs".to_string(),
                "  lens.rs".to_string(),
                "  roster.rs".to_string(),
                "Cargo.toml".to_string(),
                "README.md".to_string(),
            ],
            active_agents: vec![
                "Terminal #1".to_string(),
                "Terminal #2".to_string(),
            ],
        }
    }
}

impl Render for Roster {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .p_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .mb_6()
                    .child(
                        div()
                            .mb_2()
                            .text_color(rgb(0x888888))
                            .font_weight(FontWeight::BOLD)
                            .child("File Tree"),
                    )
                    .children(self.file_tree.iter().map(|item| {
                        div()
                            .py_1()
                            .text_color(rgb(0xcccccc))
                            .child(item.clone())
                    })),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .mb_6()
                    .child(
                        div()
                            .mb_2()
                            .text_color(rgb(0x888888))
                            .font_weight(FontWeight::BOLD)
                            .child("Active Agents"),
                    )
                    .children(self.active_agents.iter().map(|agent| {
                        div()
                            .py_1()
                            .text_color(rgb(0x00ff00))
                            .child(agent.clone())
                    })),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .mb_2()
                            .text_color(rgb(0x888888))
                            .font_weight(FontWeight::BOLD)
                            .child("System Telemetry"),
                    )
                    .child(
                        div()
                            .py_1()
                            .text_color(rgb(0xcccccc))
                            .child("CPU: 12%"),
                    )
                    .child(
                        div()
                            .py_1()
                            .text_color(rgb(0xcccccc))
                            .child("RAM: 2.1 GB"),
                    ),
            )
    }
}

