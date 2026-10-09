//! 组件陈列馆（Gallery）：组件 × 状态的展示与走查载体。
//!
//! 布局：左侧导航 + 内容区；窗口标题与 About 页显示 Git 提交号（W 关卡版本一致性，
//! 计划书 3.6）；debug 构建支持主题文件热更新（计划书 2.4）。

pub mod components;
pub mod overlays;
pub mod primitives;
pub mod theme_file;

use iced::widget::{button, column, container, row, text, Space};
use iced::{Element, Task};
use orb_theme::{OrbTheme, Variant};
use orb_tokens::{rgba_f32, Rgba};

/// 注入的 Git 短提交号（build.rs）。
pub const GIT_SHA: &str = env!("GIT_SHA");

type GalleryElement<'a> = Element<'a, Message, OrbTheme>;

/// 主题变体（浅色为默认）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeVariant {
    /// 浅色主题。
    Light,
    /// 深色主题。
    Dark,
}

impl ThemeVariant {
    fn from(v: Variant) -> Self {
        match v {
            Variant::Light => Self::Light,
            Variant::Dark => Self::Dark,
        }
    }
}

fn switch_variant(v: Variant) -> Variant {
    match v {
        Variant::Light => Variant::Dark,
        Variant::Dark => Variant::Light,
    }
}

/// 页面（逐步填充：M2 Token 页已实现；M4 组件页、M5 浮层页）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    /// Token 展示页。
    Tokens,
    /// 组件展示页。
    Components,
    /// 浮层展示页。
    Overlays,
    /// 项目说明页。
    About,
    /// 视觉原语展示页。
    Primitives,
}

/// Gallery 的 UI 消息。
#[derive(Debug, Clone)]
pub enum Message {
    /// 处理 Components 页消息。
    Component(ComponentMessage),
    /// 处理 M5 浮层页面消息。
    Overlay(overlays::OverlayMessage),
    /// 切换当前页面。
    Select(Page),
    /// 切换浅色和深色主题。
    ToggleVariant,
    /// 切换不透明降级模式。
    ToggleOpaque,
    /// 主题文件发生变化。
    ThemeFileChanged,
}

/// Components 页交互消息（演示控件为真实可交互，非静态展示）。
#[derive(Debug, Clone)]
pub enum ComponentMessage {
    /// 文本输入变化。
    Input(String),
    /// 复选框切换。
    Check(bool),
    /// 单选切换。
    Radio(u8),
    /// 开关切换。
    Switch(bool),
    /// 滑块变化。
    Slide(f32),
    /// Tabs 切换。
    Tab(usize),
    /// 列表项选中。
    ListSelect(usize),
    /// 动画帧推进（Loading 相位）。
    Tick,
}

/// Components 页演示状态。
#[derive(Debug, Clone)]
pub struct ComponentDemo {
    /// 文本输入值。
    pub input: String,
    /// 复选框选中态。
    pub checked: bool,
    /// 单选项。
    pub radio: u8,
    /// 开关状态。
    pub switch_on: bool,
    /// 滑块值。
    pub slider: f32,
    /// Tabs 选中项。
    pub tab: usize,
    /// 列表选中项。
    pub list_selected: usize,
    /// Loading 相位（0..1，按 Tick 推进）。
    pub loading_phase: f32,
}

impl Default for ComponentDemo {
    fn default() -> Self {
        Self {
            input: String::new(),
            checked: true,
            radio: 0,
            switch_on: true,
            slider: 60.0,
            tab: 0,
            list_selected: 0,
            loading_phase: 0.0,
        }
    }
}

/// Gallery 应用状态。
pub struct Gallery {
    theme: OrbTheme,
    page: Page,
    component_demo: ComponentDemo,
    overlay_demo: overlays::OverlayDemo,
}

impl Gallery {
    fn new() -> Self {
        #[cfg(debug_assertions)]
        theme_file::ensure_samples();
        Self {
            theme: theme_file::load_or_default(ThemeVariant::Light),
            page: Page::Tokens,
            component_demo: ComponentDemo::default(),
            overlay_demo: overlays::OverlayDemo::new(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Component(message) => {
                let demo = &mut self.component_demo;
                match message {
                    ComponentMessage::Input(value) => demo.input = value,
                    ComponentMessage::Check(value) => demo.checked = value,
                    ComponentMessage::Radio(value) => demo.radio = value,
                    ComponentMessage::Switch(value) => demo.switch_on = value,
                    ComponentMessage::Slide(value) => demo.slider = value,
                    ComponentMessage::Tab(value) => demo.tab = value,
                    ComponentMessage::ListSelect(value) => demo.list_selected = value,
                    // 40ms 一帧：2s 旋转一圈
                    ComponentMessage::Tick => {
                        demo.loading_phase =
                            orb_widgets::loading_normalize(demo.loading_phase + 0.04);
                    }
                }
            }
            Message::Overlay(message) => {
                return self.overlay_demo.update(message).map(Message::Overlay)
            }
            Message::Select(page) => self.page = page,
            Message::ToggleVariant => {
                self.theme = theme_file::load_or_default(ThemeVariant::from(switch_variant(
                    self.theme.variant,
                )));
            }
            Message::ToggleOpaque => self.theme.opaque_fallback = !self.theme.opaque_fallback,
            Message::ThemeFileChanged => {
                let variant = ThemeVariant::from(self.theme.variant);
                let opaque_fallback = self.theme.opaque_fallback;
                self.theme = theme_file::load_or_default(variant);
                self.theme.opaque_fallback = opaque_fallback;
            }
        }
        Task::none()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        let motion = iced::time::every(std::time::Duration::from_millis(40))
            .map(|_| Message::Overlay(overlays::OverlayMessage::Tick));
        // Loading 相位只在 Components 页活动时推进
        let component_tick = if self.page == Page::Components {
            iced::time::every(std::time::Duration::from_millis(40))
                .map(|_| Message::Component(ComponentMessage::Tick))
        } else {
            iced::Subscription::none()
        };
        #[cfg(debug_assertions)]
        {
            iced::Subscription::batch([
                iced::Subscription::run(theme_file::theme_stream),
                motion,
                component_tick,
            ])
        }
        #[cfg(not(debug_assertions))]
        {
            iced::Subscription::batch([motion, component_tick])
        }
    }

    fn view(&self) -> GalleryElement<'_> {
        let controls = iced::widget::Column::new()
            .push(
                button(text("switch light/dark").size(12))
                    .on_press(Message::ToggleVariant)
                    .width(iced::Fill),
            )
            .push(
                button(
                    text(if self.theme.opaque_fallback {
                        "opaque fallback: ON"
                    } else {
                        "opaque fallback: OFF"
                    })
                    .size(12),
                )
                .on_press(Message::ToggleOpaque)
                .width(iced::Fill),
            )
            .spacing(4);

        let nav: iced::widget::Column<'_, Message, OrbTheme> = iced::widget::Column::new()
            .push(container(text("orb gallery").size(16)).padding(12))
            .push(Self::nav_button(Page::Tokens, "Tokens", self.page))
            .push(Self::nav_button(Page::Components, "Components", self.page))
            .push(Self::nav_button(Page::Overlays, "Overlays", self.page))
            .push(Self::nav_button(Page::About, "About", self.page))
            .push(Self::nav_button(Page::Primitives, "Primitives", self.page))
            .push(Space::new().height(iced::Length::Fill))
            .push(container(controls).padding(12))
            .push(container(text(format!("commit {GIT_SHA}")).size(11)).padding(12))
            .spacing(4);

        let body = match self.page {
            Page::Tokens => view_tokens(&self.theme),
            Page::Components => components::view(&self.component_demo),
            Page::Overlays => self.overlay_demo.view(&self.theme),
            Page::About => view_about(),
            Page::Primitives => primitives::view(&self.theme),
        };

        let content: iced::widget::Column<'_, Message, OrbTheme> = column![
            container(row![
                text(format!(
                    "{:?}{}",
                    ThemeVariant::from(self.theme.variant),
                    if self.theme.opaque_fallback {
                        "  (opaque fallback)"
                    } else {
                        ""
                    }
                ))
                .size(20),
                Space::new().width(iced::Length::Fill),
            ])
            .width(iced::Fill)
            .padding(12),
            container(body).width(iced::Fill),
        ];

        row![
            container(nav)
                .width(200)
                .height(iced::Length::Fill)
                .style(|t: &OrbTheme| {
                    let p = &t.tokens.palette;
                    container::Style {
                        background: Some(color_of(t.opaque(p.plate)).into()),
                        ..container::Style::default()
                    }
                }),
            container(content)
                .width(iced::Fill)
                .height(iced::Fill)
                .padding(8),
        ]
        .into()
    }

    fn nav_button(page: Page, label: &str, current: Page) -> button::Button<'_, Message, OrbTheme> {
        let label = if page == current {
            format!("▶ {label}")
        } else {
            label.to_string()
        };
        button(text(label).size(14))
            .on_press(Message::Select(page))
            .width(iced::Fill)
    }
}

/// 启动入口（main.rs 调用）。
pub fn run() -> iced::Result {
    iced::application(Gallery::new, Gallery::update, Gallery::view)
        .font(orb_theme::fonts::REGULAR)
        .font(orb_theme::fonts::BOLD)
        .default_font(orb_theme::fonts::default())
        .title(|g: &Gallery| {
            format!(
                "orb gallery @ {GIT_SHA} ({:?}{})",
                ThemeVariant::from(g.theme.variant),
                if g.theme.opaque_fallback {
                    " / opaque"
                } else {
                    ""
                }
            )
        })
        .theme(|g: &Gallery| g.theme.clone())
        .window_size((1100.0, 720.0))
        .subscription(Gallery::subscription)
        .run()
}

fn view_about() -> GalleryElement<'static> {
    column![
        text(format!("commit: {GIT_SHA}")).size(14),
        text(format!("version: {}", env!("CARGO_PKG_VERSION"))).size(14),
        text("theme: orb light/dark + opaque fallback (ADR-002)"),
        text("renderer: wgpu (DX12/Vulkan)；快照走 tiny-skia（ADR-005）"),
    ]
    .spacing(6)
    .padding(12)
    .into()
}

// ---------------------------------------------------------------------------
// Token 页（M2）：色板 / 圆角 / 阴影 / 字号 / 动效 预览
// ---------------------------------------------------------------------------

fn section(title: &str) -> GalleryElement<'static> {
    text(title.to_string())
        .size(orb_tokens::Typography::default().lg)
        .into()
}

fn swatch(label: &str, c: Rgba) -> GalleryElement<'static> {
    column![
        container(Space::new().width(64).height(32))
            .width(64)
            .style(move |_| iced::widget::container::Style {
                background: Some(iced::Background::Color(color_of(c))),
                border: iced::Border {
                    color: iced::Color::from_rgba8(0, 0, 0, 0.25),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..iced::widget::container::Style::default()
            }),
        text(format!(
            "{label}\n#{:02X}{:02X}{:02X}@{:02X}",
            c[0], c[1], c[2], c[3]
        ))
        .size(10),
    ]
    .spacing(4)
    .into()
}

pub(crate) fn color_of(c: Rgba) -> iced::Color {
    let f = rgba_f32(c);
    iced::Color {
        r: f[0],
        g: f[1],
        b: f[2],
        a: f[3],
    }
}

/// Token 页：调板色块、圆角档位、阴影档位、字号阶梯、动效参数。
pub fn view_tokens(theme: &OrbTheme) -> GalleryElement<'_> {
    let p = &theme.tokens.palette;
    let t = &theme.tokens;

    let colors: [(&str, Rgba); 14] = [
        ("glass_fill", p.glass_fill),
        ("glass_hover", p.glass_fill_hover),
        ("glass_stroke", p.glass_stroke),
        ("glass_edge", p.glass_edge),
        ("plate", p.plate),
        ("background", p.background),
        ("text_primary", p.text_primary),
        ("text_secondary", p.text_secondary),
        ("text_disabled", p.text_disabled),
        ("accent", p.accent),
        ("on_accent", p.on_accent),
        ("hp_good", p.hp_good),
        ("hp_warn", p.hp_warn),
        ("hp_bad", p.hp_bad),
    ];
    let swatches: Vec<GalleryElement<'_>> =
        colors.iter().map(|(label, c)| swatch(label, *c)).collect();
    let swatch_rows = swatches
        .into_iter()
        .fold(row![].spacing(12), |row, swatch| row.push(swatch));

    let radius_row = row![
        preview_radius(theme.tokens.radius.xs, "4"),
        preview_radius(theme.tokens.radius.sm, "8"),
        preview_radius(theme.tokens.radius.md, "12"),
        preview_radius(theme.tokens.radius.lg, "16"),
        preview_radius(theme.tokens.radius.xl, "24"),
    ]
    .spacing(12);

    let shadow_row = row![
        preview_shadow(
            theme.tokens.shadow.sm_offset_y,
            theme.tokens.shadow.sm_blur,
            theme.tokens.shadow.sm_alpha,
            "sm"
        ),
        preview_shadow(
            theme.tokens.shadow.md_offset_y,
            theme.tokens.shadow.md_blur,
            theme.tokens.shadow.md_alpha,
            "md"
        ),
        preview_shadow(
            theme.tokens.shadow.lg_offset_y,
            theme.tokens.shadow.lg_blur,
            theme.tokens.shadow.lg_alpha,
            "lg"
        ),
    ]
    .spacing(24);

    let type_col = column![
        text("12px 辅助标注").size(theme.tokens.typography.xs),
        text("14px 正文文本").size(theme.tokens.typography.sm),
        text("16px 正文强调").size(theme.tokens.typography.md),
        text("20px 小标题").size(theme.tokens.typography.lg),
        text("28px 页面标题").size(theme.tokens.typography.xl),
        text("40px 展示数字").size(theme.tokens.typography.xxl),
    ]
    .spacing(4);

    let motion_text = format!(
        "feedback {}ms · toggle {}ms · panel {}ms · startup {}ms · stagger {}ms ×{}",
        t.motion.duration_feedback_ms,
        t.motion.duration_toggle_ms,
        t.motion.duration_panel_ms,
        t.motion.duration_startup_ms,
        t.motion.stagger_interval_ms,
        t.motion.stagger_max_items,
    );

    column![
        section("Palette"),
        swatch_rows,
        section("Radius"),
        radius_row,
        section("Shadow"),
        shadow_row,
        section("Typography"),
        type_col,
        section("Motion"),
        text(motion_text).size(12),
    ]
    .spacing(10)
    .padding(12)
    .into()
}

fn preview_radius(r: f32, label: &str) -> GalleryElement<'static> {
    column![
        container(Space::new().width(48).height(48))
            .width(48)
            .height(48)
            .style(move |_| iced::widget::container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgba8(
                    0xF5, 0xA6, 0x23, 1.0
                ))),
                border: iced::Border {
                    radius: r.into(),
                    width: 0.0,
                    color: iced::Color::TRANSPARENT,
                },
                ..iced::widget::container::Style::default()
            }),
        text(format!("{label}px")).size(10),
    ]
    .spacing(4)
    .into()
}

fn preview_shadow(offset_y: f32, blur: f32, alpha: f32, label: &str) -> GalleryElement<'static> {
    column![
        container(Space::new().width(72).height(40))
            .width(72)
            .height(40)
            .style(move |_| iced::widget::container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb8(
                    255, 255, 255,
                ))),
                border: iced::Border {
                    radius: 8.0.into(),
                    width: 1.0,
                    color: iced::Color::from_rgba8(0, 0, 0, 0.12),
                },
                shadow: iced::Shadow {
                    color: iced::Color::from_rgba8(0, 0, 0, alpha),
                    offset: iced::Vector::new(0.0, offset_y),
                    blur_radius: blur,
                },
                ..iced::widget::container::Style::default()
            }),
        text(format!("{label}: y{offset_y} b{blur} a{alpha}")).size(10),
    ]
    .spacing(4)
    .into()
}
