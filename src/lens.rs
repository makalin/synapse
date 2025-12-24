use gpui::*;

pub struct Lens {
    content: String,
    visible: bool,
}

impl Lens {
    pub fn new(cx: &mut ViewContext<Lens>) -> View<Self> {
        cx.new_view(|_cx| Self {
            content: String::new(),
            visible: false,
        })
    }
}

impl Render for Lens {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .absolute()
            .inset_0()
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
                                        span()
                                            .text_color(rgb(0xffffff))
                                            .text("The Lens - Code Editor")
                                            .font_weight(FontWeight::BOLD),
                                    )
                                    .child(
                                        button()
                                            .text_color(rgb(0xffffff))
                                            .text("Close (Esc)")
                                            .on_click(|_, cx| {
                                                // Close lens - would need to communicate with parent
                                            }),
                                    ),
                            )
                            .child(
                                textarea()
                                    .w_full()
                                    .flex_1()
                                    .bg(rgb(0x0a0a0a))
                                    .text_color(rgb(0xffffff))
                                    .font_family("Monaco")
                                    .font_size(px(14.0))
                                    .placeholder("Start typing your code...")
                                    .value(self.content.clone())
                                    .on_input(|_value, _cx| {
                                        // Handle input
                                    }),
                            ),
                    ),
            )
    }
}

