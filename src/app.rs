use crate::agent::AgentManager;
use crate::grid::Grid;
use crate::lens::Lens;
use crate::logs::{LogLevel, LogPanel};
use crate::report::ReportWindow;
use crate::roster::Roster;
use crate::settings::SettingsManager;
use crate::settings_ui::SettingsWindow;
use crate::status_bar::StatusBar;
use gpui::*;

pub struct SynapseApp {
    // Views stored as their struct types - GPUI 0.2 pattern
    _grid: (),
    _roster: (),
    _lens: (),
    _log_panel: (),
    _report_window: (),
    _status_bar: (),
    settings_manager: SettingsManager,
    agent_manager: AgentManager,
    show_roster: bool,
    show_lens: bool,
    show_logs: bool,
    show_reports: bool,
    show_settings: bool,
    active_tab: ActiveTab,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ActiveTab {
    Grid,
    Logs,
    Reports,
}

impl SynapseApp {
    pub fn new(cx: &mut WindowContext) -> Handle<Self> {
        let settings_manager = SettingsManager::new();
        let settings = settings_manager.get();
        
        let grid = Grid::new(cx);
        let roster = Roster::new(cx);
        let lens = Lens::new(cx);
        let log_panel = LogPanel::new(cx);
        let report_window = ReportWindow::new(cx);
        let status_bar = StatusBar::new(cx);
        let agent_manager = AgentManager::new();

        // Log initial message
        log_panel.update(cx, |panel, cx| {
            panel.log(LogLevel::Info, "SYNAPSE", "Application started".to_string(), cx);
        });

        cx.new_view(|_cx| Self {
            grid,
            roster,
            lens,
            log_panel,
            report_window,
            status_bar,
            settings_manager,
            agent_manager,
            show_roster: settings.ui.show_roster,
            show_lens: false,
            show_logs: false,
            show_reports: false,
            show_settings: false,
            active_tab: ActiveTab::Grid,
        })
    }

    pub fn new_terminal(&mut self, _: &MenuAction, cx: &mut ViewContext<Self>) {
        self.grid.update(cx, |grid, cx| {
            grid.add_terminal(cx);
        });
        self.update_status(cx);
        self.log_panel.update(cx, |panel, cx| {
            panel.log(LogLevel::Info, "Terminal", "New terminal created".to_string(), cx);
        });
    }

    pub fn toggle_roster(&mut self, _: &MenuAction, cx: &mut ViewContext<Self>) {
        self.show_roster = !self.show_roster;
        cx.notify();
    }

    pub fn toggle_lens(&mut self, _: &MenuAction, cx: &mut ViewContext<Self>) {
        self.show_lens = !self.show_lens;
        cx.notify();
    }

    pub fn toggle_logs(&mut self, _: &MenuAction, cx: &mut ViewContext<Self>) {
        self.show_logs = !self.show_logs;
        if self.show_logs {
            self.active_tab = ActiveTab::Logs;
        }
        cx.notify();
    }

    pub fn toggle_reports(&mut self, _: &MenuAction, cx: &mut ViewContext<Self>) {
        self.show_reports = !self.show_reports;
        if self.show_reports {
            self.active_tab = ActiveTab::Reports;
        }
        cx.notify();
    }

    pub fn open_settings(&mut self, _: &MenuAction, cx: &mut ViewContext<Self>) {
        self.show_settings = true;
        cx.notify();
    }

    pub fn switch_to_grid(&mut self, cx: &mut ViewContext<Self>) {
        self.active_tab = ActiveTab::Grid;
        self.show_logs = false;
        self.show_reports = false;
        cx.notify();
    }

    pub fn switch_to_logs(&mut self, cx: &mut ViewContext<Self>) {
        self.active_tab = ActiveTab::Logs;
        self.show_logs = true;
        self.show_reports = false;
        cx.notify();
    }

    pub fn switch_to_reports(&mut self, cx: &mut ViewContext<Self>) {
        self.active_tab = ActiveTab::Reports;
        self.show_reports = true;
        self.show_logs = false;
        cx.notify();
    }

    fn update_status(&mut self, cx: &mut ViewContext<Self>) {
        let terminal_count = self.grid.read(cx).terminals.len();
        let active_agents = self.agent_manager.get_agents().iter()
            .filter(|a| a.status == crate::agent::AgentStatus::Running)
            .count();
        
        self.status_bar.update(cx, |bar, cx| {
            bar.update_stats(terminal_count, active_agents, cx);
        });
    }
}

impl Render for SynapseApp {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let settings = self.settings_manager.get();
        let roster_width = if self.show_roster { settings.ui.roster_width } else { 0.0 };
        
        // Update status periodically
        self.update_status(cx);

        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(rgb(0x0a0a0a))
            .child(
                // Main content area
                div()
                    .flex()
                    .flex_row()
                    .w_full()
                    .flex_1()
                    .child(
                        // Zone A: The Grid / Logs / Reports
                        div()
                            .flex()
                            .flex_1()
                            .h_full()
                            .child(
                                match self.active_tab {
                                    ActiveTab::Grid => self.grid.clone(),
                                    ActiveTab::Logs => self.log_panel.clone(),
                                    ActiveTab::Reports => self.report_window.clone(),
                                }
                            ),
                    )
                    .when(self.show_roster, |div| {
                        div.child(
                            // Zone B: The Roster (Sidebar)
                            div()
                                .flex()
                                .w(DefiniteLength::Fraction(roster_width))
                                .h_full()
                                .bg(rgb(0x1a1a1a))
                                .border_l()
                                .border_color(rgb(0x333333))
                                .child(self.roster.clone()),
                        )
                    })
            )
            .child(
                // Tab bar
                div()
                    .flex()
                    .w_full()
                    .h(px(32.0))
                    .bg(rgb(0x1a1a1a))
                    .border_t()
                    .border_color(rgb(0x333333))
                    .items_center()
                    .px(px(8.0))
                    .gap(px(4.0))
                    .child(
                        button()
                            .px(px(12.0))
                            .py(px(6.0))
                            .bg(if self.active_tab == ActiveTab::Grid { rgb(0x2a2a2a) } else { rgb(0x1a1a1a) })
                            .text_color(rgb(0xffffff))
                            .text("Terminals")
                            .on_click(|_, cx| {
                                // Switch to grid tab
                            }),
                    )
                    .child(
                        button()
                            .px(px(12.0))
                            .py(px(6.0))
                            .bg(if self.active_tab == ActiveTab::Logs { rgb(0x2a2a2a) } else { rgb(0x1a1a1a) })
                            .text_color(rgb(0xffffff))
                            .text("Logs")
                            .on_click(|_, cx| {
                                // Switch to logs tab
                            }),
                    )
                    .child(
                        button()
                            .px(px(12.0))
                            .py(px(6.0))
                            .bg(if self.active_tab == ActiveTab::Reports { rgb(0x2a2a2a) } else { rgb(0x1a1a1a) })
                            .text_color(rgb(0xffffff))
                            .text("Reports")
                            .on_click(|_, cx| {
                                // Switch to reports tab
                            }),
                    )
                    .child(
                        div()
                            .flex_1(),
                    )
                    .child(
                        button()
                            .px(px(12.0))
                            .py(px(6.0))
                            .bg(rgb(0x1a1a1a))
                            .text_color(rgb(0xffffff))
                            .text("Settings")
                            .on_click(|_, cx| {
                                // Open settings
                            }),
                    ),
            )
            .child(
                // Status bar
                if settings.ui.show_status_bar {
                    Some(self.status_bar.clone())
                } else {
                    None
                }
            )
            .when(self.show_lens, |div| {
                // Zone C: The Lens (Overlay)
                div.child(self.lens.clone())
            })
            .when(self.show_settings, |div| {
                // Settings overlay
                div.child(
                    div()
                        .absolute()
                        .inset_0()
                        .flex()
                        .items_center()
                        .justify_center()
                        .bg(rgba(0x000000, 0.8))
                        .z_index(2000)
                        .child(
                            div()
                                .w(px(900.0))
                                .h(px(700.0))
                                .bg(rgb(0x1a1a1a))
                                .border()
                                .border_color(rgb(0x444444))
                                .rounded_md()
                                .shadow_lg()
                                .child(SettingsWindow::new(cx)),
                        ),
                )
            })
    }
}
