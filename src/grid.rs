use crate::terminal::Terminal;
use gpui::*;

pub struct Grid {
    terminals: Vec<View<Terminal>>,
    layout: GridLayout,
}

#[derive(Clone, Copy)]
enum GridLayout {
    Single,
    HorizontalSplit,
    VerticalSplit,
    Quad,
}

impl Grid {
    pub fn new(cx: &mut ViewContext<Grid>) -> View<Self> {
        cx.new_view(|_cx| Self {
            terminals: Vec::new(),
            layout: GridLayout::Single,
        })
    }

    pub fn add_terminal(&mut self, cx: &mut ViewContext<Self>) {
        let terminal = Terminal::new(cx);
        self.terminals.push(terminal);
        self.update_layout();
        cx.notify();
    }

    fn update_layout(&mut self) {
        let count = self.terminals.len();
        self.layout = match count {
            1 => GridLayout::Single,
            2 => GridLayout::HorizontalSplit,
            3..=4 => GridLayout::VerticalSplit,
            _ => GridLayout::Quad,
        };
    }
}

impl Render for Grid {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        if self.terminals.is_empty() {
            return div()
                .flex()
                .w_full()
                .h_full()
                .items_center()
                .justify_center()
                .text_color(rgb(0x666666))
                .child("Press Cmd+N to create a new terminal");
        }

        match self.layout {
            GridLayout::Single => {
                div()
                    .flex()
                    .w_full()
                    .h_full()
                    .child(self.terminals[0].clone())
            }
            GridLayout::HorizontalSplit => {
                div()
                    .flex()
                    .w_full()
                    .h_full()
                    .flex_row()
                    .child(
                        div()
                            .flex()
                            .flex_1()
                            .h_full()
                            .border_r()
                            .border_color(rgb(0x333333))
                            .child(self.terminals[0].clone()),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_1()
                            .h_full()
                            .child(self.terminals.get(1).cloned()),
                    )
            }
            GridLayout::VerticalSplit => {
                div()
                    .flex()
                    .w_full()
                    .h_full()
                    .flex_col()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .w_full()
                            .flex_1()
                            .child(
                                div()
                                    .flex()
                                    .flex_1()
                                    .h_full()
                                    .border_r()
                                    .border_color(rgb(0x333333))
                                    .child(self.terminals.get(0).cloned()),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_1()
                                    .h_full()
                                    .child(self.terminals.get(1).cloned()),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .w_full()
                            .flex_1()
                            .border_t()
                            .border_color(rgb(0x333333))
                            .child(
                                div()
                                    .flex()
                                    .flex_1()
                                    .h_full()
                                    .border_r()
                                    .border_color(rgb(0x333333))
                                    .child(self.terminals.get(2).cloned()),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_1()
                                    .h_full()
                                    .child(self.terminals.get(3).cloned()),
                            ),
                    )
            }
            GridLayout::Quad => {
                // For more than 4 terminals, use a simple grid
                div()
                    .flex()
                    .w_full()
                    .h_full()
                    .flex_col()
                    .children(
                        self.terminals
                            .chunks(2)
                            .map(|chunk| {
                                div()
                                    .flex()
                                    .flex_row()
                                    .w_full()
                                    .flex_1()
                                    .children(chunk.iter().map(|term| {
                                        div()
                                            .flex()
                                            .flex_1()
                                            .h_full()
                                            .border_r()
                                            .border_color(rgb(0x333333))
                                            .child(term.clone())
                                    }))
                            })
                            .collect::<Vec<_>>(),
                    )
            }
        }
    }
}

