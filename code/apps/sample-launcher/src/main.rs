#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

//! 示例应用：启动器主界面、状态面板、设置与对话通知流程。

use iced::widget::{button, checkbox, column, container, mouse_area, row, text, Space};
use iced::{Element, Length, Task};
use orb_core::{InteractionState, StartupPhase, StartupSequence};
use orb_theme::OrbTheme;
use orb_widgets::{
    card, divider, hp_bar, hp_bar_band, loop_scroll, loop_scroll_next, loop_scroll_prev,
    menu_panel, menu_rail, menu_rail_next, menu_rail_prev, modal, ring, section_header, title_bar,
    toast, ToastLevel,
};

/// LoopScroll 条带占位数量。
const MENU_ITEMS: usize = 3;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .font(orb_theme::fonts::REGULAR)
        .font(orb_theme::fonts::BOLD)
        .default_font(orb_theme::fonts::default())
        .title(|app: &App| format!("Sample Launcher · {}", app.page.label()))
        .theme(|app: &App| app.theme.clone())
        .subscription(App::subscription)
        .window_size((1100.0, 720.0))
        .run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Page {
    Launcher,
    Status,
    Settings,
    About,
}

impl Page {
    fn label(self) -> &'static str {
        match self {
            Self::Launcher => "Launcher",
            Self::Status => "Status",
            Self::Settings => "Settings",
            Self::About => "About",
        }
    }

    fn label_zh(self) -> &'static str {
        match self {
            Self::Launcher => "启动器",
            Self::Status => "状态",
            Self::Settings => "设置",
            Self::About => "关于",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Language {
    Chinese,
    English,
}

impl Language {
    fn toggle(self) -> Self {
        match self {
            Self::Chinese => Self::English,
            Self::English => Self::Chinese,
        }
    }

    fn button_label(self) -> &'static str {
        match self {
            Self::Chinese => "English",
            Self::English => "中文",
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Select(Page),
    ToggleTheme,
    ToggleReducedMotion(bool),
    OpenDialog,
    CloseDialog,
    ShowToast,
    Tick,
    MenuSelect(usize),
    MenuNext,
    MenuPrev,
    LoopNext,
    LoopPrev,
    WindowCommand(orb_core::WindowCommand),
    ToggleLanguage,
}

struct App {
    page: Page,
    theme: OrbTheme,
    reduced_motion: bool,
    dialog_open: bool,
    toast_visible: bool,
    menu_selected: Option<usize>,
    loop_index: usize,
    interaction: InteractionState,
    language: Language,
    startup: StartupSequence,
}

impl App {
    fn new() -> Self {
        Self {
            page: Page::Launcher,
            theme: OrbTheme::light(),
            reduced_motion: false,
            dialog_open: false,
            toast_visible: false,
            menu_selected: Some(0),
            loop_index: 0,
            interaction: InteractionState::new(),
            language: Language::Chinese,
            startup: StartupSequence::new(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Select(page) => self.page = page,
            Message::ToggleTheme => {
                self.theme = match self.theme.variant {
                    orb_theme::Variant::Light => OrbTheme::dark(),
                    orb_theme::Variant::Dark => OrbTheme::light(),
                }
            }
            Message::ToggleLanguage => self.language = self.language.toggle(),
            Message::ToggleReducedMotion(value) => self.reduced_motion = value,
            Message::OpenDialog => self.dialog_open = true,
            Message::CloseDialog => self.dialog_open = false,
            Message::ShowToast => self.toast_visible = true,
            Message::Tick => {
                self.toast_visible = false;
                if self.reduced_motion {
                    for _ in 0..3 {
                        self.startup.advance(std::time::Duration::from_secs(1));
                    }
                } else {
                    self.startup.advance(std::time::Duration::from_millis(40));
                }
            }
            Message::MenuSelect(index) => self.menu_selected = Some(index),
            Message::MenuNext => {
                self.menu_selected = menu_rail_next(self.menu_selected, MENU_ITEMS);
            }
            Message::MenuPrev => {
                self.menu_selected = menu_rail_prev(self.menu_selected, MENU_ITEMS);
            }
            Message::LoopNext => self.loop_index = loop_scroll_next(self.loop_index, MENU_ITEMS),
            Message::LoopPrev => self.loop_index = loop_scroll_prev(self.loop_index, MENU_ITEMS),
            Message::WindowCommand(command) => {
                return iced::window::latest().then(move |id| orb_core::window::task(command, id));
            }
        }
        Task::none()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        // MenuRail 键盘导航（M5 DoD：键盘与鼠标都可用）
        let keyboard = iced::keyboard::listen().filter_map(|event| match event {
            iced::keyboard::Event::KeyPressed {
                key: iced::keyboard::Key::Named(named),
                ..
            } => match named {
                iced::keyboard::key::Named::ArrowDown | iced::keyboard::key::Named::ArrowRight => {
                    Some(Message::MenuNext)
                }
                iced::keyboard::key::Named::ArrowUp | iced::keyboard::key::Named::ArrowLeft => {
                    Some(Message::MenuPrev)
                }
                _ => None,
            },
            _ => None,
        });
        iced::Subscription::batch([
            iced::time::every(std::time::Duration::from_millis(380)).map(|_| Message::Tick),
            keyboard,
        ])
    }

    fn view(&self) -> Element<'_, Message, OrbTheme> {
        let navigation = column![
            text("ORB LAUNCHER").size(18),
            nav_button(Page::Launcher, self.page, self.language),
            nav_button(Page::Status, self.page, self.language),
            nav_button(Page::Settings, self.page, self.language),
            nav_button(Page::About, self.page, self.language),
            Space::new().height(Length::Fill),
            button("Light / Dark").on_press(Message::ToggleTheme),
            button(self.language.button_label()).on_press(Message::ToggleLanguage),
        ]
        .spacing(8)
        .padding(16)
        .width(220);

        let body = match self.page {
            Page::Launcher => self.launcher_view(),
            Page::Status => self.status_view(),
            Page::Settings => self.settings_view(),
            Page::About => self.about_view(),
        };

        let title = mouse_area(title_bar(
            "Sample Launcher",
            Message::WindowCommand(orb_core::WindowCommand::Minimize),
            Message::WindowCommand(orb_core::WindowCommand::Close),
        ))
        .on_press(Message::WindowCommand(orb_core::WindowCommand::Drag))
        .on_double_click(Message::WindowCommand(
            orb_core::WindowCommand::ToggleMaximize,
        ));

        row![
            container(navigation),
            container(column![title, body].spacing(8)).width(Length::Fill),
        ]
        .height(Length::Fill)
        .into()
    }

    fn launcher_view(&self) -> Element<'_, Message, OrbTheme> {
        if self.startup.phase() != StartupPhase::Complete {
            let phase = match self.startup.phase() {
                StartupPhase::Enter => "Enter / 进入",
                StartupPhase::Brand => "Brand / 品牌",
                StartupPhase::Content => "Content / 内容",
                StartupPhase::Complete => "Complete / 完成",
            };
            return card(
                column![
                    section_header(text("ORB / SAO").size(32)),
                    text(phase).size(22),
                    text("Startup sequence / 启动序列"),
                ]
                .spacing(16),
            )
            .padding(48)
            .into();
        }
        let icons: Vec<Element<'_, Message, OrbTheme>> = ["W", "I", "S"]
            .iter()
            .map(|label| text(*label).size(18).into())
            .collect();
        let titles = ["Weapons", "Items", "System"];
        let selected = self.menu_selected.unwrap_or(0).min(titles.len() - 1);
        let loop_items = (0..MENU_ITEMS).fold(iced::widget::Row::new().spacing(8), |row, index| {
            let marker = if index == self.loop_index {
                "◉"
            } else {
                "○"
            };
            row.push(container(text(format!("{marker} slot {index}")).size(14)).padding(8))
        });

        card(
            column![
                section_header(text("Launcher").size(28)),
                row![
                    menu_rail(icons, self.menu_selected, Message::MenuSelect),
                    menu_panel(
                        text(titles[selected]).size(20),
                        text("Circular rail expands this panel to the right."),
                    ),
                ]
                .spacing(16),
                divider::<Message>(),
                row![
                    button("◀").on_press(Message::LoopPrev),
                    loop_scroll(container(loop_items).width(Length::Fill)).width(Length::Fill),
                    button("▶").on_press(Message::LoopNext),
                ]
                .spacing(8),
                text(format!("Loop index: {} / {MENU_ITEMS}", self.loop_index)),
                row![
                    button("Open menu").on_press(Message::ShowToast),
                    button("Open dialog").on_press(Message::OpenDialog),
                ]
                .spacing(10),
                text(format!(
                    "Interaction emphasized: {}",
                    self.interaction.is_emphasized()
                )),
                button("Hover / press target").on_press(Message::ShowToast),
            ]
            .spacing(14),
        )
        .padding(24)
        .into()
    }

    fn status_view(&self) -> Element<'_, Message, OrbTheme> {
        card(
            column![
                section_header(text("Status Panel").size(28)),
                row![
                    ring(0.72, iced::Color::from_rgb(0.96, 0.65, 0.14), 128.0),
                    column![text("HP").size(18), text("72 / 100").size(28), text("GOOD")]
                        .spacing(8),
                ]
                .spacing(24),
                row![
                    column![text("72 / 100"), hp_bar(0.72, 180.0, 16.0)],
                    column![text("18 / 100"), hp_bar(0.18, 180.0, 16.0)],
                    column![text("5 / 100"), hp_bar(0.05, 180.0, 16.0)],
                ]
                .spacing(12),
                hp_bar(0.72, 320.0, 16.0),
                text(format!("HpBar band: {:?}", hp_bar_band(0.72))),
                divider::<Message>(),
                text("Ring · Readout · HpBar token composition"),
            ]
            .spacing(16),
        )
        .padding(24)
        .into()
    }

    fn about_view(&self) -> Element<'_, Message, OrbTheme> {
        card(
            column![
                section_header(text("About").size(28)),
                text("Sample Launcher · orb UI component showcase"),
                text("M7 Windows demo build"),
                divider::<Message>(),
                text("Rust + iced 0.14 · G1 pseudo-frosted glass"),
            ]
            .spacing(14),
        )
        .padding(24)
        .into()
    }

    fn settings_view(&self) -> Element<'_, Message, OrbTheme> {
        card(
            column![
                section_header(text("Settings").size(28)),
                row![
                    checkbox(self.reduced_motion).on_toggle(Message::ToggleReducedMotion),
                    text("Reduced motion"),
                ]
                .spacing(8),
                text(if self.reduced_motion {
                    "Animations jump to final state."
                } else {
                    "Animations use token durations."
                }),
                divider::<Message>(),
                button("Show notification").on_press(Message::ShowToast),
                if self.dialog_open {
                    modal(column![
                        text("Dialog").size(20),
                        text("This demonstrates the in-window modal surface."),
                        button("Close").on_press(Message::CloseDialog),
                    ])
                } else {
                    modal(text("Dialog is closed"))
                },
                if self.toast_visible {
                    toast(text("Saved successfully"), ToastLevel::Success)
                } else {
                    toast(text("No active notification"), ToastLevel::Info)
                },
            ]
            .spacing(14),
        )
        .padding(24)
        .into()
    }
}

fn nav_button(
    page: Page,
    current: Page,
    language: Language,
) -> iced::widget::Button<'static, Message, OrbTheme> {
    let page_label = match language {
        Language::Chinese => page.label_zh(),
        Language::English => page.label(),
    };
    let label = if page == current {
        format!("▶ {page_label}")
    } else {
        page_label.into()
    };
    button(text(label))
        .on_press(Message::Select(page))
        .width(Length::Fill)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_labels_are_stable() {
        assert_eq!(Page::Launcher.label(), "Launcher");
        assert_eq!(Page::Status.label(), "Status");
        assert_eq!(Page::Settings.label(), "Settings");
        assert_eq!(Page::About.label(), "About");
    }

    #[test]
    fn app_routes_page_and_theme_changes() {
        let mut app = App::new();
        let _ = app.update(Message::Select(Page::Status));
        assert_eq!(app.page, Page::Status);

        assert_eq!(app.theme.variant, orb_theme::Variant::Light);
        let _ = app.update(Message::ToggleTheme);
        assert_eq!(app.theme.variant, orb_theme::Variant::Dark);
    }

    #[test]
    fn notification_state_is_toggled_by_messages() {
        let mut app = App::new();
        let _ = app.update(Message::ShowToast);
        assert!(app.toast_visible);
        let _ = app.update(Message::Tick);
        assert!(!app.toast_visible);
    }

    #[test]
    fn menu_selection_and_loop_wrap_are_deterministic() {
        let mut app = App::new();
        assert_eq!(app.menu_selected, Some(0));
        let _ = app.update(Message::MenuSelect(2));
        assert_eq!(app.menu_selected, Some(2));

        // 键盘步进：环形 + None 兜底（menu_rail_next/prev）
        let _ = app.update(Message::MenuNext);
        assert_eq!(app.menu_selected, Some(0));
        let _ = app.update(Message::MenuPrev);
        assert_eq!(app.menu_selected, Some(2));
        app.menu_selected = None;
        let _ = app.update(Message::MenuPrev);
        assert_eq!(app.menu_selected, Some(MENU_ITEMS - 1));

        assert_eq!(app.loop_index, 0);
        let _ = app.update(Message::LoopPrev);
        assert_eq!(app.loop_index, MENU_ITEMS - 1);
        let _ = app.update(Message::LoopNext);
        assert_eq!(app.loop_index, 0);
        let _ = app.update(Message::LoopNext);
        assert_eq!(app.loop_index, 1);
    }
}
