use gpui::*;

pub struct Roster {
    file_tree: Vec<String>,
    active_agents: Vec<String>,
}

impl Roster {
    pub fn new(cx: &mut ViewContext<Roster>) -> View<Self> {
        cx.new_view(|_cx| Self {
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
        })
    }
}

impl Render for Roster {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .p(px(12.0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .mb(px(24.0))
                    .child(
                        div()
                            .mb(px(8.0))
                            .text_color(rgb(0x888888))
                            .font_weight(FontWeight::BOLD)
                            .text("File Tree"),
                    )
                    .children(self.file_tree.iter().map(|item| {
                        div()
                            .py(px(2.0))
                            .text_color(rgb(0xcccccc))
                            .font_family("Monaco")
                            .font_size(px(11.0))
                            .text(item)
                    })),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .mb(px(24.0))
                    .child(
                        div()
                            .mb(px(8.0))
                            .text_color(rgb(0x888888))
                            .font_weight(FontWeight::BOLD)
                            .text("Active Agents"),
                    )
                    .children(self.active_agents.iter().map(|agent| {
                        div()
                            .py(px(2.0))
                            .text_color(rgb(0x00ff00))
                            .font_family("Monaco")
                            .font_size(px(11.0))
                            .text(agent)
                    })),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .mb(px(8.0))
                            .text_color(rgb(0x888888))
                            .font_weight(FontWeight::BOLD)
                            .text("System Telemetry"),
                    )
                    .child(
                        div()
                            .py(px(2.0))
                            .text_color(rgb(0xcccccc))
                            .font_family("Monaco")
                            .font_size(px(11.0))
                            .text("CPU: 12%"),
                    )
                    .child(
                        div()
                            .py(px(2.0))
                            .text_color(rgb(0xcccccc))
                            .font_family("Monaco")
                            .font_size(px(11.0))
                            .text("RAM: 2.1 GB"),
                    ),
            )
    }
}

