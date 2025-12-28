use crate::settings::{AppSettings, SettingsManager};
use gpui::*;
// use log;

pub struct SettingsWindow {
    settings_manager: SettingsManager,
    temp_settings: AppSettings,
}

impl SettingsWindow {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let settings_manager = SettingsManager::new();
        let temp_settings = settings_manager.get().clone();

        Self {
            settings_manager,
            temp_settings,
        }
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
                    .p_4()
                    .bg(rgb(0x1a1a1a))
                    .border_b_width(px(1.0))
                    .border_color(rgb(0x333333))
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .child("Settings"),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .child("Reset")
                                    .on_click(cx.listener(|this, _, cx| {
                                        if let Err(e) = this.reset_settings(cx) {
                                            log::error!("Failed to reset settings: {}", e);
                                        }
                                    })),
                            )
                            .child(
                                div()
                                    .child("Save")
                                    .on_click(cx.listener(|this, _, cx| {
                                        if let Err(e) = this.save_settings(cx) {
                                            log::error!("Failed to save settings: {}", e);
                                        }
                                    })),
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
                            .w_1_5()
                            .h_full()
                            .bg(rgb(0x0f0f0f))
                            .border_r_width(px(1.0))
                            .border_color(rgb(0x333333))
                            .p_3()
                            .child(div().text_xs().text_color(rgb(0x888888)).child("Categories"))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .mt_2()
                                    .gap_1()
                                    .child(
                                        div()
                                            .p_2()
                                            .bg(rgb(0x1a1a1a))
                                            .rounded_md()
                                            .child(div().child("Theme")),
                                    )
                                    .child(
                                        div()
                                            .p_2()
                                            .bg(rgb(0x1a1a1a))
                                            .rounded_md()
                                            .child(div().child("Terminal")),
                                    )
                                    .child(
                                        div()
                                            .p_2()
                                            .bg(rgb(0x1a1a1a))
                                            .rounded_md()
                                            .child(div().child("Editor")),
                                    )
                                    .child(
                                        div()
                                            .p_2()
                                            .bg(rgb(0x1a1a1a))
                                            .rounded_md()
                                            .child(div().child("Agents")),
                                    )
                                    .child(
                                        div()
                                            .p_2()
                                            .bg(rgb(0x1a1a1a))
                                            .rounded_md()
                                            .child(div().child("UI")),
                                    ),
                            ),
                    )
                    .child(
                        // Settings form
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .p_6()
                            .gap_6()
                            .child(
                                // Theme settings
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_4()
                                    .child(
                                        div()
                                            .text_lg()
                                            .font_weight(FontWeight::BOLD)
                                            .child("Theme"),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .child(div().child("Background Color"))
                                            .child(
                                                div()
                                                    .w_1_2()
                                                    .bg(rgb(0x1a1a1a))
                                                    .child(self.temp_settings.theme.background.clone()),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .child(div().child("Accent Color"))
                                            .child(
                                                div()
                                                    .w_1_2()
                                                    .bg(rgb(0x1a1a1a))
                                                    .child(self.temp_settings.theme.accent.clone()),
                                            ),
                                    ),
                            )
                            .child(
                                // Terminal settings
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_4()
                                    .child(
                                        div()
                                            .text_lg()
                                            .font_weight(FontWeight::BOLD)
                                            .child("Terminal"),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .child(div().child("Font Size"))
                                            .child(
                                                div()
                                                    .w_1_2()
                                                    .bg(rgb(0x1a1a1a))
                                                    .child(self.temp_settings.terminal.font_size.to_string()),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .child(div().child("Default Shell"))
                                            .child(
                                                div()
                                                    .w_1_2()
                                                    .bg(rgb(0x1a1a1a))
                                                    .child(self.temp_settings.terminal.default_shell.clone()),
                                            ),
                                    ),
                            )
                            .child(
                                // Agent settings
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_4()
                                    .child(
                                        div()
                                            .text_lg()
                                            .font_weight(FontWeight::BOLD)
                                            .child("Agents"),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap_1()
                                                    .child(
                                                        div()
                                                            .child("Auto-start agents"),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap_1()
                                                    .child(
                                                        div().child("Max Concurrent Agents"),
                                                    )
                                                    .child(
                                                        div()
                                                            .w_1_2()
                                                            .bg(rgb(0x1a1a1a))
                                                            .child(self.temp_settings.agents.max_concurrent.to_string()),
                                                    ),
                                            ),
                                    ),
                            ),
                    ),
            )
    }
}
