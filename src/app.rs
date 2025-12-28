use crate::actions::*;
use crate::agent::AgentManager;
use crate::grid::Grid;
use crate::lens::Lens;
use crate::logs::LogPanel;
use crate::report::ReportWindow;
use crate::roster::Roster;
use crate::settings::SettingsManager;
use crate::settings_ui::SettingsWindow;
use crate::status_bar::StatusBar;
use gpui::*;

#[derive(Clone, PartialEq, IntoElement)]
struct Tab {
    title: SharedString,
    active: bool,
    id: ElementId,
}

impl RenderOnce for Tab {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .id(self.id)
            .px_4()
            .py_2()
            .bg(if self.active {
                rgb(0x2a2a2a)
            } else {
                rgb(0x1a1a1a)
            })
            .child(self.title.clone())
    }
}

#[derive(IntoElement)]
struct IconButton {
    icon: SharedString,
    id: ElementId,
}

impl RenderOnce for IconButton {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .id(self.id)
            .px_4()
            .py_2()
            .bg(rgb(0x1a1a1a))
            .child(self.icon.clone())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ActiveTab {
    Grid,
    Logs,
    Reports,
}

pub struct SynapseApp {
    grid: View<Grid>,
    roster: View<Roster>,
    lens: View<Lens>,
    log_panel: View<LogPanel>,
    report_window: View<ReportWindow>,
    status_bar: View<StatusBar>,
    settings_manager: SettingsManager,
    _agent_manager: AgentManager,
    show_roster: bool,
    show_lens: bool,
    show_logs: bool,

    show_reports: bool,
    show_settings: bool,
    active_tab: ActiveTab,
}


impl SynapseApp {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        let settings_manager = SettingsManager::new();
        let settings = settings_manager.get();

        let grid = cx.new_view(|cx| Grid::new(cx));
        let roster = cx.new_view(|_cx| Roster::new());
        let lens = cx.new_view(|_cx| Lens::new());
        let log_panel = cx.new_view(|_cx| LogPanel::new());
        let report_window = cx.new_view(|_cx| ReportWindow::new());
        let status_bar = cx.new_view(|cx| StatusBar::new(cx));
        let agent_manager = AgentManager::new();

        cx.subscribe_global_action(move |this: &mut Self, action: &NewTerminal, cx: &mut WindowContext| {
            this.grid.update(cx, |grid, cx| {
                grid.add_terminal(cx);
            });
        });

        cx.new_view(|_cx| Self {
            grid,
            roster,
            lens,
            log_panel,
            report_window,
            status_bar,
            settings_manager,
            _agent_manager: agent_manager,
            show_roster: settings.ui.show_roster,
            show_lens: false,
            show_logs: false,
            show_reports: false,
            show_settings: false,
            active_tab: ActiveTab::Grid,
        })
    }
}

impl Render for SynapseApp {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let settings = self.settings_manager.get();
        let roster_width = if self.show_roster {
            settings.ui.roster_width
        } else {
            0.0
        };

        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(rgb(0x0a0a0a))
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
                            .child(match self.active_tab {
                                ActiveTab::Grid => self.grid.clone().into_any_element(),
                                ActiveTab::Logs => self.log_panel.clone().into_any_element(),
                                ActiveTab::Reports => self.report_window.clone().into_any_element(),
                            }),
                    )
                    .children(if self.show_roster {
                        vec![div()
                            .flex()
                            .w(px(roster_width as f32))
                            .h_full()
                            .bg(rgb(0x1a1a1a))
                            .border_l(px(1.0))
                            .border_color(rgb(0x333333))
                            .child(self.roster.clone())
                            .into_any_element()]
                    } else {
                        vec![]
                    }),
            )
            .child(
                div()
                    .flex()
                    .w_full()
                    .h(px(32.0))
                    .bg(rgb(0x1a1a1a))
                    .border_t(px(1.0))
                    .border_color(rgb(0x333333))
                    .items_center()
                    .px(px(8.0))
                    .gap(px(4.0))
                    .child(
                        Tab {
                            title: "Terminals".into(),
                            active: self.active_tab == ActiveTab::Grid,
                            id: cx.element_id().into(),
                        }
                    )
                    .child(
                        Tab {
                            title: "Logs".into(),
                            active: self.active_tab == ActiveTab::Logs,
                            id: cx.element_id().into(),
                        }
                    )
                    .child(
                        Tab {
                            title: "Reports".into(),
                            active: self.active_tab == ActiveTab::Reports,
                            id: cx.element_id().into(),
                        }
                    )
                    .child(div().flex_1())
                    .child(
                        IconButton {
                            icon: "Settings".into(),
                            id: cx.element_id().into(),
                        }
                    ),
            )
            .child(
                if settings.ui.show_status_bar {
                    self.status_bar.clone().into_any_element()
                } else {
                    div().into_any_element()
                },
            )
            .children(if self.show_lens {
                vec![self.lens.clone().into_any_element()]
            } else {
                vec![]
            })
            .children(if self.show_settings {
                vec![div()
                    .absolute()
                    .w_full()
                    .h_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .bg(Rgba { r: 0.0, g: 0.0, b: 0.0, a: 0.8 })
                    .z_index(2000)
                    .child(
                        div()
                            .w(px(900.0))
                            .h(px(700.0))
                            .bg(rgb(0x1a1a1a))
                            .border(px(1.0))
                            .border_color(rgb(0x444444))
                            .rounded(px(6.0))
                            .shadow_lg()
                            .child(cx.new_view(|cx| SettingsWindow::new(cx))),
                    )
                    .into_any_element()]
            } else {
                vec![]
            })
    }
}
