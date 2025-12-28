use gpui::*;

pub struct Lens {
    content: String,
    visible: bool,
}

impl Lens {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            visible: false,
        }
    }
}

impl Render for Lens {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .absolute()
            .w_full()
            .h_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgba(0x000000, 0.9))
            .z_index(1000)
            .child(
                div()
                    .w(px(800.0))
                    .h(px(600.0))
                    .bg(rgb(0x1a1a1a))
                    .border()
                    .border_color(rgb(0x444444))
                    .rounded_md()
                    .p(px(16.0))
                    .shadow_lg()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .w_full()
                            .h_full()
                            .child(
                                div()
                                    .flex()
                                    .w_full()
                                    .mb(px(8.0))
                                    .justify_between()
                                    .items_center()
                                    .child(
                                        div()
                                            .child("The Lens - Code Editor")
                                            .font_weight(FontWeight::BOLD),
                                    )
                                    .child(
                                        div()
                                            .child("Close (Esc)")
                                            .on_click(cx.listener(|this, _, _| {
                                                this.visible = false;
                                            })),
                                    ),
                            )
                            .child(
                                div()
                                    .w_full()
                                    .flex_1()
                                    .bg(rgb(0x0a0a0a))
                                    .child(self.content.clone()),
                            ),
                    ),
            )
    }
}

