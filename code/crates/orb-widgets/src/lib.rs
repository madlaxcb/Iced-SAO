//! orb-widgets：组件（button/panel/menu/hud/dialog/toast 等）。

use iced::widget::{
    button, canvas, checkbox, container, radio, scrollable, slider, text, text_input, toggler,
};
use iced::{Color, Element, Length, Point, Rectangle, Renderer};
use orb_theme::OrbTheme;
use orb_tokens::Rgba;
use std::f32::consts::TAU;

/// Button 的视觉变体。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    /// 橙色主操作。
    Primary,
    /// 普通次要操作。
    Secondary,
    /// 无填充幽灵按钮。
    Ghost,
    /// 危险操作。
    Danger,
}

/// 返回 Button 变体的稳定标识。
pub fn button_variant_label(variant: ButtonVariant) -> &'static str {
    match variant {
        ButtonVariant::Primary => "primary",
        ButtonVariant::Secondary => "secondary",
        ButtonVariant::Ghost => "ghost",
        ButtonVariant::Danger => "danger",
    }
}

/// M4.2 控件种类。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlKind {
    /// 单行文本输入。
    TextInput,
    /// 复选框。
    Checkbox,
    /// 单选框。
    Radio,
    /// 开关。
    Switch,
    /// 滑块。
    Slider,
    /// 可滚动区域。
    ScrollArea,
}

/// 返回控件种类的稳定标识。
pub fn control_label(kind: ControlKind) -> &'static str {
    match kind {
        ControlKind::TextInput => "text-input",
        ControlKind::Checkbox => "checkbox",
        ControlKind::Radio => "radio",
        ControlKind::Switch => "switch",
        ControlKind::Slider => "slider",
        ControlKind::ScrollArea => "scroll-area",
    }
}

/// Overlay 的基础种类。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverlayKind {
    /// 浮层容器。
    Overlay,
    /// 提示气泡。
    Tooltip,
    /// 上下文菜单。
    ContextMenu,
    /// 模态对话框。
    Modal,
    /// 轻量通知。
    Toast,
}

/// Toast 的语义级别。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastLevel {
    /// 普通信息。
    Info,
    /// 成功信息。
    Success,
    /// 警告信息。
    Warning,
    /// 错误信息。
    Error,
}

/// 返回 Overlay 种类的稳定标识。
pub fn overlay_label(kind: OverlayKind) -> &'static str {
    match kind {
        OverlayKind::Overlay => "overlay",
        OverlayKind::Tooltip => "tooltip",
        OverlayKind::ContextMenu => "context-menu",
        OverlayKind::Modal => "modal",
        OverlayKind::Toast => "toast",
    }
}

/// 返回 Toast 级别的稳定标识。
pub fn toast_level_label(level: ToastLevel) -> &'static str {
    match level {
        ToastLevel::Info => "info",
        ToastLevel::Success => "success",
        ToastLevel::Warning => "warning",
        ToastLevel::Error => "error",
    }
}

/// 将 Ring 进度限制在闭区间 `[0, 1]`。
pub fn ring_progress(value: f32) -> f32 {
    value.clamp(0.0, 1.0)
}

/// HpBar 的分档结果。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HpBand {
    /// 良好（>= 0.5）。
    Good,
    /// 警告（>= 0.2）。
    Warn,
    /// 危险（< 0.2）。
    Bad,
}

/// 按 HP 比例返回确定性分档。
pub fn hp_bar_band(value: f32) -> HpBand {
    let value = value.clamp(0.0, 1.0);
    if value >= 0.5 {
        HpBand::Good
    } else if value >= 0.2 {
        HpBand::Warn
    } else {
        HpBand::Bad
    }
}

/// 循环滚动：前进一格（自动回绕）。
pub fn loop_scroll_next(index: usize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    (index + 1) % len
}

/// 循环滚动：后退一格（自动回绕）。
pub fn loop_scroll_prev(index: usize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    (index + len - 1) % len
}

/// 玻璃面板配置。
#[derive(Debug, Clone, Copy)]
pub struct GlassPanelStyle {
    /// 面板填充色。
    pub fill: Rgba,
    /// 面板描边色。
    pub stroke: Rgba,
    /// 面板圆角。
    pub radius: f32,
    /// 阴影透明度。
    pub shadow_alpha: f32,
}

/// 创建一个使用给定材质配置的玻璃面板。
pub fn glass_panel<'a, Message>(
    content: impl Into<Element<'a, Message, OrbTheme>>,
    style: GlassPanelStyle,
) -> iced::widget::Container<'a, Message, OrbTheme> {
    container(content)
        .padding(12)
        .style(move |_| iced::widget::container::Style {
            background: Some(
                Color::from_rgba8(
                    style.fill[0],
                    style.fill[1],
                    style.fill[2],
                    style.fill[3] as f32 / 255.0,
                )
                .into(),
            ),
            border: iced::Border {
                color: Color::from_rgba8(
                    style.stroke[0],
                    style.stroke[1],
                    style.stroke[2],
                    style.stroke[3] as f32 / 255.0,
                ),
                width: 1.0,
                radius: style.radius.into(),
            },
            shadow: iced::Shadow {
                color: Color::from_rgba8(0, 0, 0, style.shadow_alpha),
                offset: iced::Vector::new(0.0, 4.0),
                blur_radius: 12.0,
            },
            ..iced::widget::container::Style::default()
        })
}

/// 创建一个覆盖在内容之上的浮层容器。
pub fn overlay<'a, Message>(
    content: impl Into<Element<'a, Message, OrbTheme>>,
) -> iced::widget::Container<'a, Message, OrbTheme> {
    container(content).padding(12)
}

/// 创建主题化 Tooltip。
pub fn tooltip<'a, Message>(
    content: impl Into<Element<'a, Message, OrbTheme>>,
    hint: impl Into<Element<'a, Message, OrbTheme>>,
) -> iced::widget::Tooltip<'a, Message, OrbTheme> {
    iced::widget::tooltip(content, hint, iced::widget::tooltip::Position::Top)
        .style(orb_theme::style_container)
}

/// 创建主题化 ContextMenu。
pub fn context_menu<'a, Message>(
    items: impl Into<Element<'a, Message, OrbTheme>>,
) -> iced::widget::Container<'a, Message, OrbTheme> {
    container(items)
        .padding(8)
        .style(orb_theme::style_container)
}

/// 创建主题化 Modal 对话框。
pub fn modal<'a, Message>(
    content: impl Into<Element<'a, Message, OrbTheme>>,
) -> iced::widget::Container<'a, Message, OrbTheme> {
    container(content)
        .padding(24)
        .style(orb_theme::style_container)
}

/// 创建主题化 Toast。
pub fn toast<'a, Message>(
    message: impl Into<Element<'a, Message, OrbTheme>>,
    level: ToastLevel,
) -> iced::widget::Container<'a, Message, OrbTheme> {
    let _ = level;
    container(message)
        .padding(12)
        .style(orb_theme::style_container)
}

/// 创建主题化 Button。
pub fn themed_button<'a, Message>(
    label: impl Into<Element<'a, Message, OrbTheme>>,
    variant: ButtonVariant,
    on_press: Option<Message>,
) -> button::Button<'a, Message, OrbTheme> {
    let mut result = button(label).padding(10);
    if let Some(message) = on_press {
        result = result.on_press(message);
    }
    let class: button::StyleFn<'_, OrbTheme> = Box::new(move |theme, status| {
        let mut style = orb_theme::style_button(theme, status);
        match variant {
            ButtonVariant::Primary => {}
            ButtonVariant::Secondary => {
                let fill = theme.glass_fill();
                style.background = Some(
                    iced::Color::from_rgba8(fill[0], fill[1], fill[2], fill[3] as f32 / 255.0)
                        .into(),
                );
            }
            ButtonVariant::Ghost => {
                style.background = None;
                style.border.width = 0.0;
            }
            ButtonVariant::Danger => {
                let danger = theme.tokens.palette.hp_bad;
                style.background = Some(
                    iced::Color::from_rgba8(
                        danger[0],
                        danger[1],
                        danger[2],
                        danger[3] as f32 / 255.0,
                    )
                    .into(),
                );
                style.text_color = iced::Color::from_rgba8(
                    theme.tokens.palette.on_accent[0],
                    theme.tokens.palette.on_accent[1],
                    theme.tokens.palette.on_accent[2],
                    theme.tokens.palette.on_accent[3] as f32 / 255.0,
                );
            }
        }
        style
    });
    result.class(class)
}

/// 创建主题化单行文本输入框。
pub fn text_input_control<'a, Message: Clone>(
    placeholder: &str,
    value: &str,
    on_input: impl Fn(String) -> Message + 'a,
) -> text_input::TextInput<'a, Message, OrbTheme> {
    text_input(placeholder, value)
        .on_input(on_input)
        .style(orb_theme::style_text_input)
}

/// 创建主题化复选框。
pub fn checkbox_control<'a, Message>(
    label: impl Into<String>,
    checked: bool,
    on_toggle: impl Fn(bool) -> Message + 'a,
) -> checkbox::Checkbox<'a, Message, OrbTheme> {
    checkbox(checked).label(label.into()).on_toggle(on_toggle)
}

/// 创建主题化单选框。
pub fn radio_control<'a, Message, V>(
    label: impl Into<String>,
    value: V,
    selected: Option<V>,
    on_click: impl FnOnce(V) -> Message,
) -> radio::Radio<'a, Message, OrbTheme>
where
    Message: Clone,
    V: Copy + Eq + 'a,
{
    radio(label, value, selected, on_click)
}

/// 创建主题化开关。
pub fn switch_control<'a, Message>(
    label: impl Into<String>,
    checked: bool,
    on_toggle: impl Fn(bool) -> Message + 'a,
) -> toggler::Toggler<'a, Message, OrbTheme> {
    toggler(checked).label(label.into()).on_toggle(on_toggle)
}

/// 创建主题化滑块。
pub fn slider_control<'a, Message: Clone>(
    range: std::ops::RangeInclusive<f32>,
    value: f32,
    on_change: impl Fn(f32) -> Message + 'a,
) -> slider::Slider<'a, f32, Message, OrbTheme> {
    slider(range, value, on_change).style(orb_theme::style_slider)
}

/// 创建主题化可滚动区域。
pub fn scroll_area<'a, Message>(
    content: impl Into<Element<'a, Message, OrbTheme>>,
) -> scrollable::Scrollable<'a, Message, OrbTheme> {
    scrollable(content).style(orb_theme::style_scrollable)
}

/// 创建一个圆形按钮。
pub fn circle_button<'a, Message>(
    content: impl Into<Element<'a, Message, OrbTheme>>,
    size: f32,
    on_press: Option<Message>,
) -> button::Button<'a, Message, OrbTheme> {
    let mut result = button(content)
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .padding(0);
    if let Some(message) = on_press {
        result = result.on_press(message);
    }
    result
}

/// 创建主题化 Panel。
pub fn panel<'a, Message>(
    content: impl Into<Element<'a, Message, OrbTheme>>,
) -> iced::widget::Container<'a, Message, OrbTheme> {
    container(content).padding(16)
}

/// 创建主题化 Card。
pub fn card<'a, Message>(
    content: impl Into<Element<'a, Message, OrbTheme>>,
) -> iced::widget::Container<'a, Message, OrbTheme> {
    container(content)
        .padding(12)
        .style(orb_theme::style_container)
}

/// 创建主题化分隔线。
pub fn divider<'a, Message>() -> iced::widget::Rule<'a, OrbTheme> {
    iced::widget::rule::horizontal(1).style(orb_theme::style_rule)
}

/// 创建主题化区块标题。
pub fn section_header<'a, Message>(
    title: impl Into<Element<'a, Message, OrbTheme>>,
) -> iced::widget::Container<'a, Message, OrbTheme> {
    container(title).padding([8, 0])
}

/// 创建一个带独立缓存的环形进度画布。
pub fn ring<'a, Message: 'a>(
    progress: f32,
    color: Color,
    size: f32,
) -> Element<'a, Message, OrbTheme> {
    canvas(RingProgram {
        progress: ring_progress(progress),
        color,
        cache: canvas::Cache::new(),
    })
    .width(size)
    .height(size)
    .into()
}

/// 创建一个带独立缓存的胶囊 HP 条画布，颜色按分档取主题 Token。
pub fn hp_bar<'a, Message: 'a>(
    value: f32,
    width: f32,
    height: f32,
) -> Element<'a, Message, OrbTheme> {
    canvas(HpBarProgram {
        value: value.clamp(0.0, 1.0),
        cache: canvas::Cache::new(),
    })
    .width(Length::Fixed(width))
    .height(Length::Fixed(height))
    .into()
}

#[derive(Debug)]
struct HpBarProgram {
    value: f32,
    cache: canvas::Cache,
}

impl<Message> canvas::Program<Message, OrbTheme> for HpBarProgram {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &OrbTheme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let token = match hp_bar_band(self.value) {
            HpBand::Good => theme.tokens.palette.hp_good,
            HpBand::Warn => theme.tokens.palette.hp_warn,
            HpBand::Bad => theme.tokens.palette.hp_bad,
        };
        let color = Color::from_rgba8(token[0], token[1], token[2], token[3] as f32 / 255.0);
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            let radius = frame.height() / 2.0;
            let track = canvas::Path::rounded_rectangle(Point::ORIGIN, frame.size(), radius.into());
            frame.fill(&track, Color::from_rgba8(255, 255, 255, 0.18));
            let fill_width = frame.width() * self.value;
            if fill_width > 0.0 {
                let fill = canvas::Path::rounded_rectangle(
                    Point::ORIGIN,
                    iced::Size::new(fill_width.max(frame.height()), frame.height()),
                    radius.into(),
                );
                frame.fill(&fill, color);
            }
        });
        vec![geometry]
    }
}

/// 创建 SAO 风格的圆形图标导航栏（MenuRail），选中项带主题强调色描边。
pub fn menu_rail<'a, Message: Clone + 'a>(
    icons: Vec<Element<'a, Message, OrbTheme>>,
    selected: Option<usize>,
    on_select: impl Fn(usize) -> Message + 'a,
) -> Element<'a, Message, OrbTheme> {
    let mut column = iced::widget::Column::new().spacing(8);
    for (index, icon) in icons.into_iter().enumerate() {
        let is_selected = selected == Some(index);
        let press = on_select(index);
        let button = circle_button(icon, 48.0, Some(press));
        let wrapped = container(button).style(move |_: &OrbTheme| {
            if is_selected {
                iced::widget::container::Style {
                    border: iced::Border {
                        color: Color::from_rgba8(0xF5, 0xA6, 0x23, 1.0),
                        width: 2.0,
                        radius: 26.0.into(),
                    },
                    ..iced::widget::container::Style::default()
                }
            } else {
                iced::widget::container::Style::default()
            }
        });
        column = column.push(wrapped);
    }
    column.into()
}

/// 创建与 MenuRail 配套的展开面板（MenuPanel）。
pub fn menu_panel<'a, Message: 'a>(
    title: impl Into<Element<'a, Message, OrbTheme>>,
    content: impl Into<Element<'a, Message, OrbTheme>>,
) -> iced::widget::Container<'a, Message, OrbTheme> {
    let column = iced::widget::Column::new()
        .push(container(title).padding([8, 0]))
        .push(iced::widget::rule::horizontal(1).style(orb_theme::style_rule))
        .push(content)
        .spacing(12);
    container(column)
        .padding(16)
        .style(orb_theme::style_container)
}

/// 创建水平循环滚动条带（LoopScroll）。
pub fn loop_scroll<'a, Message>(
    content: impl Into<Element<'a, Message, OrbTheme>>,
) -> scrollable::Scrollable<'a, Message, OrbTheme> {
    scrollable(content)
        .direction(scrollable::Direction::Horizontal(
            scrollable::Scrollbar::default(),
        ))
        .style(orb_theme::style_scrollable)
}

// ---------------------------------------------------------------------------
// M4.4 数据展示与反馈：List / Readout / Loading / Tabs / Badge
// ---------------------------------------------------------------------------

/// Rgba Token → iced Color（组件内部辅助）。
fn color_of_token(c: Rgba) -> Color {
    Color::from_rgba8(c[0], c[1], c[2], c[3] as f32 / 255.0)
}

/// 列表项的单行展示文本（title + 可选副标题）。
pub fn list_item_line(title: &str, subtitle: Option<&str>) -> String {
    match subtitle {
        Some(subtitle) => format!("{title} — {subtitle}"),
        None => title.to_string(),
    }
}

/// 创建单行列表项（选中态用强调色描边，DoD：状态齐全）。
pub fn list_item<'a, Message: Clone + 'a>(
    title: &str,
    subtitle: Option<&str>,
    selected: bool,
    on_press: Message,
) -> Element<'a, Message, OrbTheme> {
    let line = list_item_line(title, subtitle);
    button(
        container(text(line).size(orb_tokens::Typography::default().sm))
            .width(Length::Fill)
            .padding(10),
    )
    .on_press(on_press)
    .width(Length::Fill)
    .style(move |theme: &OrbTheme, _status| {
        let p = &theme.tokens.palette;
        let radius = orb_tokens::Radius::default().sm;
        iced::widget::button::Style {
            background: Some(color_of_token(theme.glass_fill()).into()),
            text_color: color_of_token(p.text_primary),
            border: iced::Border {
                color: color_of_token(if selected { p.accent } else { p.glass_edge }),
                width: if selected { 2.0 } else { 1.0 },
                radius: radius.into(),
            },
            ..iced::widget::button::Style::default()
        }
    })
    .into()
}

/// Readout 的展示文本（value + 可选单位）。
pub fn readout_text(value: &str, unit: Option<&str>) -> String {
    match unit {
        Some(unit) => format!("{value} {unit}"),
        None => value.to_string(),
    }
}

/// 创建数值读出（大号数值 + 小号标签）。
pub fn readout<'a, Message: 'a>(
    value: &str,
    label: &str,
    unit: Option<&str>,
) -> Element<'a, Message, OrbTheme> {
    let typography = orb_tokens::Typography::default();
    iced::widget::Column::new()
        .push(text(readout_text(value, unit)).size(typography.lg))
        .push(text(label.to_string()).size(typography.xs))
        .spacing(2)
        .into()
}

/// 将任意相位归一化到 0.0..1.0（Loading 旋转相位）。
pub fn loading_normalize(phase: f32) -> f32 {
    let p = phase % 1.0;
    if p >= 0.0 {
        p
    } else {
        p + 1.0
    }
}

/// 创建环形 Loading（相位由调用方按 Tick 推进，保持确定性动画架构）。
pub fn loading<'a, Message: 'a>(phase: f32, size: f32) -> Element<'a, Message, OrbTheme> {
    canvas(LoadingProgram {
        phase: loading_normalize(phase),
        cache: canvas::Cache::new(),
    })
    .width(Length::Fixed(size))
    .height(Length::Fixed(size))
    .into()
}

#[derive(Debug)]
struct LoadingProgram {
    phase: f32,
    cache: canvas::Cache,
}

impl<Message> canvas::Program<Message, OrbTheme> for LoadingProgram {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &OrbTheme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let accent = color_of_token(theme.tokens.palette.accent);
        let phase = self.phase;
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            let center = frame.center();
            let radius = center.x.min(center.y) * 0.78;
            let track = canvas::Path::circle(center, radius);
            frame.stroke(
                &track,
                canvas::Stroke {
                    width: 3.0,
                    style: canvas::Style::Solid(Color::from_rgba8(255, 255, 255, 0.18)),
                    ..canvas::Stroke::default()
                },
            );
            let start = iced::Radians(phase * TAU);
            let arc = canvas::Path::new(|path| {
                path.arc(canvas::path::Arc {
                    center,
                    radius,
                    start_angle: start,
                    end_angle: iced::Radians(start.0 + TAU / 4.0),
                });
            });
            frame.stroke(
                &arc,
                canvas::Stroke {
                    width: 3.0,
                    style: canvas::Style::Solid(accent),
                    ..canvas::Stroke::default()
                },
            );
        });
        vec![geometry]
    }
}

/// 将选中索引钳制到标签数量范围内（len = 0 时返回 0）。
pub fn tabs_clamp_select(selected: usize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    selected.min(len - 1)
}

/// 创建胶囊标签页（Tabs），选中项使用强调色底。
pub fn tabs<'a, Message: Clone + 'a>(
    labels: &[&str],
    selected: usize,
    on_select: impl Fn(usize) -> Message + 'a,
) -> Element<'a, Message, OrbTheme> {
    let selected = tabs_clamp_select(selected, labels.len());
    let mut row = iced::widget::Row::new().spacing(4);
    for (index, label) in labels.iter().enumerate() {
        let is_selected = index == selected;
        let press = on_select(index);
        let tab = button(text((*label).to_string()).size(orb_tokens::Typography::default().xs))
            .on_press(press)
            .padding([6, 14])
            .style(move |theme: &OrbTheme, _status| {
                let p = &theme.tokens.palette;
                let radius = orb_tokens::Radius::default().full;
                let (bg, fg, edge) = if is_selected {
                    (p.accent, p.on_accent, p.accent)
                } else {
                    (theme.glass_fill(), p.text_primary, p.glass_edge)
                };
                iced::widget::button::Style {
                    background: Some(color_of_token(bg).into()),
                    text_color: color_of_token(fg),
                    border: iced::Border {
                        color: color_of_token(edge),
                        width: 1.0,
                        radius: radius.into(),
                    },
                    ..iced::widget::button::Style::default()
                }
            });
        row = row.push(tab);
    }
    row.into()
}

/// Badge / Tag 的语义级别。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeLevel {
    /// 中性。
    Neutral,
    /// 强调。
    Accent,
    /// 良好（绿）。
    Good,
    /// 警告（黄）。
    Warn,
    /// 危险（红）。
    Bad,
}

/// 返回 Badge 级别的稳定标识。
pub fn badge_level_label(level: BadgeLevel) -> &'static str {
    match level {
        BadgeLevel::Neutral => "neutral",
        BadgeLevel::Accent => "accent",
        BadgeLevel::Good => "good",
        BadgeLevel::Warn => "warn",
        BadgeLevel::Bad => "bad",
    }
}

/// 创建胶囊徽标（Badge / Tag），颜色按级别取主题 Token。
pub fn badge<'a, Message: 'a>(label: &str, level: BadgeLevel) -> Element<'a, Message, OrbTheme> {
    container(text(label.to_string()).size(orb_tokens::Typography::default().xs))
        .padding([3, 10])
        .style(move |theme: &OrbTheme| {
            let p = &theme.tokens.palette;
            let (bg, fg) = match level {
                BadgeLevel::Neutral => (p.glass_stroke, p.text_primary),
                BadgeLevel::Accent => (p.accent, p.on_accent),
                BadgeLevel::Good => (p.hp_good, p.on_accent),
                BadgeLevel::Warn => (p.hp_warn, p.on_accent),
                BadgeLevel::Bad => (p.hp_bad, p.on_accent),
            };
            iced::widget::container::Style {
                background: Some(color_of_token(bg).into()),
                text_color: Some(color_of_token(fg)),
                border: iced::Border {
                    color: color_of_token(bg),
                    width: 0.0,
                    radius: orb_tokens::Radius::default().full.into(),
                },
                ..iced::widget::container::Style::default()
            }
        })
        .into()
}

/// MenuRail 键盘导航：向下 / 右方向（None 选中时落到第一项；空列表返回 None）。
pub fn menu_rail_next(selected: Option<usize>, len: usize) -> Option<usize> {
    if len == 0 {
        return None;
    }
    Some(match selected {
        None => 0,
        Some(index) => (index + 1) % len,
    })
}

/// MenuRail 键盘导航：向上 / 左方向（None 选中时落到最后一项；空列表返回 None）。
pub fn menu_rail_prev(selected: Option<usize>, len: usize) -> Option<usize> {
    if len == 0 {
        return None;
    }
    Some(match selected {
        None => len - 1,
        Some(index) => (index + len - 1) % len,
    })
}

#[derive(Debug)]
struct RingProgram {
    progress: f32,
    color: Color,
    cache: canvas::Cache,
}

impl<Message> canvas::Program<Message, OrbTheme> for RingProgram {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &OrbTheme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let progress = self.progress;
        let color = self.color;
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            let center = frame.center();
            let radius = center.x.min(center.y) * 0.78;
            let background = canvas::Path::circle(center, radius);
            frame.stroke(
                &background,
                canvas::Stroke {
                    width: 6.0,
                    style: canvas::Style::Solid(Color::from_rgba8(255, 255, 255, 0.18)),
                    ..canvas::Stroke::default()
                },
            );
            if progress > 0.0 {
                let arc = canvas::Path::new(|path| {
                    path.arc(canvas::path::Arc {
                        center,
                        radius,
                        start_angle: iced::Radians(-std::f32::consts::FRAC_PI_2),
                        end_angle: iced::Radians(-std::f32::consts::FRAC_PI_2 + TAU * progress),
                    });
                });
                frame.stroke(
                    &arc,
                    canvas::Stroke {
                        width: 6.0,
                        style: canvas::Style::Solid(color),
                        ..canvas::Stroke::default()
                    },
                );
            }
        });
        vec![geometry]
    }
}

#[cfg(test)]
mod tests {
    use super::{
        button_variant_label, control_label, overlay_label, ring_progress, toast_level_label,
        ButtonVariant, ControlKind, OverlayKind, ToastLevel,
    };

    #[test]
    fn overlay_kinds_have_stable_labels() {
        assert_eq!(overlay_label(OverlayKind::Overlay), "overlay");
        assert_eq!(overlay_label(OverlayKind::Tooltip), "tooltip");
        assert_eq!(overlay_label(OverlayKind::ContextMenu), "context-menu");
        assert_eq!(overlay_label(OverlayKind::Modal), "modal");
        assert_eq!(overlay_label(OverlayKind::Toast), "toast");
        assert_eq!(toast_level_label(ToastLevel::Info), "info");
        assert_eq!(toast_level_label(ToastLevel::Success), "success");
        assert_eq!(toast_level_label(ToastLevel::Warning), "warning");
        assert_eq!(toast_level_label(ToastLevel::Error), "error");
    }

    #[test]
    fn control_kinds_have_stable_labels() {
        assert_eq!(control_label(ControlKind::TextInput), "text-input");
        assert_eq!(control_label(ControlKind::Checkbox), "checkbox");
        assert_eq!(control_label(ControlKind::Radio), "radio");
        assert_eq!(control_label(ControlKind::Switch), "switch");
        assert_eq!(control_label(ControlKind::Slider), "slider");
        assert_eq!(control_label(ControlKind::ScrollArea), "scroll-area");
    }

    #[test]
    fn button_variants_have_stable_labels() {
        assert_eq!(button_variant_label(ButtonVariant::Primary), "primary");
        assert_eq!(button_variant_label(ButtonVariant::Secondary), "secondary");
        assert_eq!(button_variant_label(ButtonVariant::Ghost), "ghost");
        assert_eq!(button_variant_label(ButtonVariant::Danger), "danger");
    }

    #[test]
    fn ring_progress_clamps_to_unit_interval() {
        assert_eq!(ring_progress(-0.2), 0.0);
        assert_eq!(ring_progress(0.35), 0.35);
        assert_eq!(ring_progress(1.2), 1.0);
    }

    #[test]
    fn hp_bar_band_thresholds_are_deterministic() {
        use super::{hp_bar_band, HpBand};
        assert_eq!(hp_bar_band(1.0), HpBand::Good);
        assert_eq!(hp_bar_band(0.72), HpBand::Good);
        assert_eq!(hp_bar_band(0.5), HpBand::Good);
        assert_eq!(hp_bar_band(0.49), HpBand::Warn);
        assert_eq!(hp_bar_band(0.2), HpBand::Warn);
        assert_eq!(hp_bar_band(0.19), HpBand::Bad);
        assert_eq!(hp_bar_band(-1.0), HpBand::Bad);
    }

    #[test]
    fn loop_scroll_index_wraps_around() {
        use super::{loop_scroll_next, loop_scroll_prev};
        assert_eq!(loop_scroll_next(0, 5), 1);
        assert_eq!(loop_scroll_next(4, 5), 0);
        assert_eq!(loop_scroll_prev(0, 5), 4);
        assert_eq!(loop_scroll_prev(2, 5), 1);
        assert_eq!(loop_scroll_next(0, 1), 0);
        assert_eq!(loop_scroll_prev(0, 1), 0);
        assert_eq!(loop_scroll_next(3, 0), 0);
    }

    #[test]
    fn list_item_line_combines_title_and_subtitle() {
        use super::list_item_line;
        assert_eq!(list_item_line("Weapons", None), "Weapons");
        assert_eq!(
            list_item_line("Swords", Some("12 equipped")),
            "Swords — 12 equipped"
        );
    }

    #[test]
    fn readout_text_appends_unit_when_present() {
        use super::readout_text;
        assert_eq!(readout_text("72 / 100", Some("HP")), "72 / 100 HP");
        assert_eq!(readout_text("72 / 100", None), "72 / 100");
    }

    #[test]
    fn loading_phase_normalizes_into_unit_interval() {
        use super::loading_normalize;
        assert!((loading_normalize(0.5) - 0.5).abs() < 1e-6);
        assert!((loading_normalize(1.25) - 0.25).abs() < 1e-6);
        assert!((loading_normalize(-0.25) - 0.75).abs() < 1e-6);
        assert!((loading_normalize(-1.0) - 0.0).abs() < 1e-6);
    }

    #[test]
    fn tabs_selection_clamps_into_bounds() {
        use super::tabs_clamp_select;
        assert_eq!(tabs_clamp_select(0, 3), 0);
        assert_eq!(tabs_clamp_select(2, 3), 2);
        assert_eq!(tabs_clamp_select(9, 3), 2);
        assert_eq!(tabs_clamp_select(0, 0), 0);
    }

    #[test]
    fn badge_levels_have_stable_labels() {
        use super::{badge_level_label, BadgeLevel};
        assert_eq!(badge_level_label(BadgeLevel::Neutral), "neutral");
        assert_eq!(badge_level_label(BadgeLevel::Accent), "accent");
        assert_eq!(badge_level_label(BadgeLevel::Good), "good");
        assert_eq!(badge_level_label(BadgeLevel::Warn), "warn");
        assert_eq!(badge_level_label(BadgeLevel::Bad), "bad");
    }

    #[test]
    fn menu_rail_keyboard_steps_are_cyclic_and_none_safe() {
        use super::{menu_rail_next, menu_rail_prev};
        assert_eq!(menu_rail_next(None, 3), Some(0));
        assert_eq!(menu_rail_next(Some(1), 3), Some(2));
        assert_eq!(menu_rail_next(Some(2), 3), Some(0));
        assert_eq!(menu_rail_prev(None, 3), Some(2));
        assert_eq!(menu_rail_prev(Some(0), 3), Some(2));
        assert_eq!(menu_rail_prev(Some(1), 3), Some(0));
        assert_eq!(menu_rail_next(None, 0), None);
        assert_eq!(menu_rail_prev(Some(0), 0), None);
    }
}
