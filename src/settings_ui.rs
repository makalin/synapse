use crate::settings::{AppSettings, SettingsManager};
use gpui::*;

pub struct SettingsWindow {
    settings_manager: SettingsManager,
    temp_settings: AppSettings,
}

impl SettingsWindow {
    pub fn new(cx: &mut ViewContext<Self>) -> View<Self> {
        let settings_manager = SettingsManager::new();
        let temp_settings = settings_manager.get().clone();

        cx.new_view(|_cx| Self {
            settings_manager,
            temp_settings,
        })
    }

    pub fn save_settings(&mut self, cx: &mut ViewContext<Self>) -> anyhow::Result<()> {
        self.settings_manager.update(|s| {
            *s = self.temp_settings.clone();
        })?;
        cx.notify();
        Ok(())
    }

    pub fn reset_settings(&mut self, cx: &mut ViewContext<Self>) -> anyhow::Result<()> {
        self.temp_settings = AppSettings::default();
        cx.notify();
        Ok(())
    }
}

impl Render for SettingsWindow {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(rgb(0x0a0a0a))
            .child(
                // Header
                div()
                    .flex()
                    .w_full()
                    .p(px(16.0))
                    .bg(rgb(0x1a1a1a))
                    .border_b()
                    .border_color(rgb(0x333333))
                    .justify_between()
                    .items_center()
                    .child(
                        span()
                            .text_color(rgb(0xffffff))
                            .font_weight(FontWeight::BOLD)
                            .font_size(px(18.0))
                            .text("Settings"),
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(8.0))
                            .child(
                                button()
                                    .text_color(rgb(0xffffff))
                                    .text("Reset")
                                    .on_click(|_, cx| {
                                        if let Err(e) = self.reset_settings(cx) {
                                            eprintln!("Failed to reset settings: {}", e);
                                        }
                                    }),
                            )
                            .child(
                                button()
                                    .text_color(rgb(0x00ff00))
                                    .text("Save")
                                    .on_click(|_, cx| {
                                        if let Err(e) = self.save_settings(cx) {
                                            eprintln!("Failed to save settings: {}", e);
                                        }
                                    }),
                            ),
                    ),
            )
            .child(
                // Settings content
                div()
                    .flex()
                    .flex_row()
                    .w_full()
                    .flex_1()
                    .overflow_y_scroll()
                    .child(
                        // Sidebar navigation
                        div()
                            .flex()
                            .flex_col()
                            .w(DefiniteLength::Fraction(0.2))
                            .h_full()
                            .bg(rgb(0x0f0f0f))
                            .border_r()
                            .border_color(rgb(0x333333))
                            .p(px(12.0))
                            .child(
                                span()
                                    .text_color(rgb(0x888888))
                                    .font_size(px(12.0))
                                    .text("Categories"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .mt(px(8.0))
                                    .gap(px(4.0))
                                    .child(
                                        div()
                                            .p(px(8.0))
                                            .bg(rgb(0x1a1a1a))
                                            .rounded_md()
                                            .child(
                                                span()
                                                    .text_color(rgb(0xffffff))
                                                    .text("Theme"),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .p(px(8.0))
                                            .bg(rgb(0x1a1a1a))
                                            .rounded_md()
                                            .child(
                                                span()
                                                    .text_color(rgb(0xffffff))
                                                    .text("Terminal"),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .p(px(8.0))
                                            .bg(rgb(0x1a1a1a))
                                            .rounded_md()
                                            .child(
                                                span()
                                                    .text_color(rgb(0xffffff))
                                                    .text("Editor"),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .p(px(8.0))
                                            .bg(rgb(0x1a1a1a))
                                            .rounded_md()
                                            .child(
                                                span()
                                                    .text_color(rgb(0xffffff))
                                                    .text("Agents"),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .p(px(8.0))
                                            .bg(rgb(0x1a1a1a))
                                            .rounded_md()
                                            .child(
                                                span()
                                                    .text_color(rgb(0xffffff))
                                                    .text("UI"),
                                            ),
                                    ),
                            ),
                    )
                    .child(
                        // Settings form
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .p(px(24.0))
                            .gap(px(24.0))
                            .child(
                                // Theme settings
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(16.0))
                                    .child(
                                        span()
                                            .text_color(rgb(0xffffff))
                                            .font_weight(FontWeight::BOLD)
                                            .font_size(px(16.0))
                                            .text("Theme"),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                label()
                                                    .text_color(rgb(0xcccccc))
                                                    .text("Background Color"),
                                            )
                                            .child(
                                                input()
                                                    .w(px(200.0))
                                                    .bg(rgb(0x1a1a1a))
                                                    .text_color(rgb(0xffffff))
                                                    .value(self.temp_settings.theme.background.clone())
                                                    .on_input(|value, cx| {
                                                        self.temp_settings.theme.background = value;
                                                        cx.notify();
                                                    }),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                label()
                                                    .text_color(rgb(0xcccccc))
                                                    .text("Accent Color"),
                                            )
                                            .child(
                                                input()
                                                    .w(px(200.0))
                                                    .bg(rgb(0x1a1a1a))
                                                    .text_color(rgb(0xffffff))
                                                    .value(self.temp_settings.theme.accent.clone())
                                                    .on_input(|value, cx| {
                                                        self.temp_settings.theme.accent = value;
                                                        cx.notify();
                                                    }),
                                            ),
                                    ),
                            )
                            .child(
                                // Terminal settings
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(16.0))
                                    .child(
                                        span()
                                            .text_color(rgb(0xffffff))
                                            .font_weight(FontWeight::BOLD)
                                            .font_size(px(16.0))
                                            .text("Terminal"),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                label()
                                                    .text_color(rgb(0xcccccc))
                                                    .text("Font Size"),
                                            )
                                            .child(
                                                input()
                                                    .w(px(200.0))
                                                    .bg(rgb(0x1a1a1a))
                                                    .text_color(rgb(0xffffff))
                                                    .value(self.temp_settings.terminal.font_size.to_string())
                                                    .on_input(|value, cx| {
                                                        if let Ok(size) = value.parse::<f32>() {
                                                            self.temp_settings.terminal.font_size = size;
                                                            cx.notify();
                                                        }
                                                    }),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                label()
                                                    .text_color(rgb(0xcccccc))
                                                    .text("Default Shell"),
                                            )
                                            .child(
                                                input()
                                                    .w(px(200.0))
                                                    .bg(rgb(0x1a1a1a))
                                                    .text_color(rgb(0xffffff))
                                                    .value(self.temp_settings.terminal.default_shell.clone())
                                                    .on_input(|value, cx| {
                                                        self.temp_settings.terminal.default_shell = value;
                                                        cx.notify();
                                                    }),
                                            ),
                                    ),
                            )
                            .child(
                                // Agent settings
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(16.0))
                                    .child(
                                        span()
                                            .text_color(rgb(0xffffff))
                                            .font_weight(FontWeight::BOLD)
                                            .font_size(px(16.0))
                                            .text("Agents"),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(8.0))
                                            .child(
                                                label()
                                                    .flex()
                                                    .items_center()
                                                    .gap(px(4.0))
                                                    .child(
                                                        input()
                                                            .r#type("checkbox")
                                                            .checked(self.temp_settings.agents.auto_start)
                                                            .on_change(|checked, cx| {
                                                                self.temp_settings.agents.auto_start = checked;
                                                                cx.notify();
                                                            }),
                                                    )
                                                    .child(
                                                        span()
                                                            .text_color(rgb(0xcccccc))
                                                            .text("Auto-start agents"),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(4.0))
                                                    .child(
                                                        label()
                                                            .text_color(rgb(0xcccccc))
                                                            .text("Max Concurrent Agents"),
                                                    )
                                                    .child(
                                                        input()
                                                            .w(px(200.0))
                                                            .bg(rgb(0x1a1a1a))
                                                            .text_color(rgb(0xffffff))
                                                            .value(self.temp_settings.agents.max_concurrent.to_string())
                                                            .on_input(|value, cx| {
                                                                if let Ok(max) = value.parse::<usize>() {
                                                                    self.temp_settings.agents.max_concurrent = max;
                                                                    cx.notify();
                                                                }
                                                            }),
                                                    ),
                                            ),
                                    ),
                            ),
                    ),
            )
    }
}

