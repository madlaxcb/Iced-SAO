//! 各 widget 样式函数（纯函数，公开供 testkit 快照与单测使用——ADR-002）。
//!
//! SAO 风格视觉语言（计划书 1.3）：浅色半透明玻璃、圆角档位、胶囊 / 圆形、单一橙色强调。
//! 所有颜色 / 圆角 / 阴影一律来自 [`orb_tokens::Tokens`]，禁止字面值（计划书 6.2）。

use crate::OrbTheme;
use orb_tokens::{rgba_f32, Rgba};

fn color(c: Rgba) -> iced::Color {
    let f = rgba_f32(c);
    iced::Color {
        r: f[0],
        g: f[1],
        b: f[2],
        a: f[3],
    }
}

fn border(c: Rgba, width: f32) -> iced::Border {
    iced::Border {
        color: color(c),
        width,
        radius: 0.0.into(),
    }
}

fn capsule(theme: &OrbTheme) -> iced::border::Radius {
    theme.tokens.radius.full.into()
}

fn shadow(color: iced::Color, offset_y: f32, blur: f32) -> iced::Shadow {
    iced::Shadow {
        color,
        offset: iced::Vector::new(0.0, offset_y),
        blur_radius: blur,
    }
}

// ---------------------------------------------------------------------------
// 各 widget 样式函数
// ---------------------------------------------------------------------------

/// container：plate 文字底板 + 圆角 md + 外侧轻描边。
pub fn style_container(theme: &OrbTheme) -> iced::widget::container::Style {
    let p = &theme.tokens.palette;
    iced::widget::container::Style {
        text_color: Some(color(p.text_primary)),
        background: Some(color(theme.opaque(p.plate)).into()),
        border: border(p.glass_edge, 0.0),
        shadow: iced::Shadow::default(),
        snap: false,
    }
}

/// button：胶囊形玻璃按钮（hover 更实 + accent 边，active 橙底深字）。
pub fn style_button(
    theme: &OrbTheme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    let p = &theme.tokens.palette;
    let (fill, stroke, label, shadow) = match status {
        iced::widget::button::Status::Active => (
            theme.glass_fill(),
            p.glass_stroke,
            p.text_primary,
            iced::Shadow::default(),
        ),
        iced::widget::button::Status::Hovered => (
            theme.glass_fill_hover(),
            p.accent,
            p.text_primary,
            shadow(
                iced::Color::from_rgba8(0, 0, 0, theme.tokens.shadow.md_alpha),
                theme.tokens.shadow.md_offset_y,
                theme.tokens.shadow.md_blur,
            ),
        ),
        iced::widget::button::Status::Pressed => {
            (p.accent, p.accent, p.on_accent, iced::Shadow::default())
        }
        iced::widget::button::Status::Disabled => (
            theme.glass_fill(),
            p.text_disabled,
            p.text_disabled,
            iced::Shadow::default(),
        ),
    };
    iced::widget::button::Style {
        background: Some(color(fill).into()),
        text_color: color(label),
        border: iced::Border {
            color: color(stroke),
            width: 1.0,
            radius: capsule(theme),
        },
        shadow,
        snap: false,
    }
}

/// text_input：圆角 sm 底板，聚焦 accent 边。
pub fn style_text_input(
    theme: &OrbTheme,
    status: iced::widget::text_input::Status,
) -> iced::widget::text_input::Style {
    let p = &theme.tokens.palette;
    let line = match status {
        iced::widget::text_input::Status::Focused { .. } => p.accent,
        iced::widget::text_input::Status::Hovered => p.glass_stroke,
        iced::widget::text_input::Status::Active | iced::widget::text_input::Status::Disabled => {
            p.glass_edge
        }
    };
    let value = match status {
        iced::widget::text_input::Status::Disabled => p.text_disabled,
        _ => p.text_primary,
    };
    iced::widget::text_input::Style {
        background: color(theme.opaque(p.plate)).into(),
        border: iced::Border {
            color: color(line),
            width: 1.0,
            radius: theme.tokens.radius.sm.into(),
        },
        icon: color(p.text_secondary),
        placeholder: color(p.text_disabled),
        value: color(value),
        selection: iced::Color::from_rgba8(0xF5, 0xA6, 0x23, 0.30),
    }
}

/// text 默认继承父级前景色。
pub fn style_text(_theme: &OrbTheme) -> iced::widget::text::Style {
    iced::widget::text::Style { color: None }
}

/// checkbox：圆角 sm，选中 accent 填充。
pub fn style_checkbox(
    theme: &OrbTheme,
    status: iced::widget::checkbox::Status,
) -> iced::widget::checkbox::Style {
    let p = &theme.tokens.palette;
    let (checked, hovered) = match status {
        iced::widget::checkbox::Status::Active { is_checked } => (is_checked, false),
        iced::widget::checkbox::Status::Hovered { is_checked } => (is_checked, true),
        iced::widget::checkbox::Status::Disabled { .. } => (false, false),
    };
    let disabled = matches!(status, iced::widget::checkbox::Status::Disabled { .. });
    let line = if hovered || checked {
        p.accent
    } else {
        p.glass_stroke
    };
    let icon: Rgba = if disabled {
        p.text_disabled
    } else if checked {
        p.on_accent
    } else {
        [0, 0, 0, 0]
    };
    let bg = if disabled {
        theme.glass_fill()
    } else if checked {
        p.accent
    } else {
        theme.glass_fill()
    };
    iced::widget::checkbox::Style {
        background: color(bg).into(),
        icon_color: color(icon),
        border: iced::Border {
            color: color(line),
            width: 1.0,
            radius: theme.tokens.radius.sm.into(),
        },
        text_color: Some(color(if disabled {
            p.text_disabled
        } else {
            p.text_primary
        })),
    }
}

/// toggler：胶囊滑块（radius full），开态 accent。
pub fn style_toggler(
    theme: &OrbTheme,
    status: iced::widget::toggler::Status,
) -> iced::widget::toggler::Style {
    let p = &theme.tokens.palette;
    let on = matches!(
        status,
        iced::widget::toggler::Status::Active { is_toggled: true }
            | iced::widget::toggler::Status::Hovered { is_toggled: true }
    );
    let disabled = matches!(status, iced::widget::toggler::Status::Disabled { .. });
    let line = match status {
        iced::widget::toggler::Status::Active { .. } => p.glass_stroke,
        iced::widget::toggler::Status::Hovered { .. } => p.accent,
        iced::widget::toggler::Status::Disabled { .. } => p.text_disabled,
    };
    iced::widget::toggler::Style {
        background: color(if disabled {
            theme.glass_fill()
        } else if on {
            p.accent
        } else {
            theme.glass_fill_hover()
        })
        .into(),
        background_border_width: 1.0,
        background_border_color: color(line),
        foreground: color(if disabled {
            p.text_disabled
        } else if on {
            p.on_accent
        } else {
            p.text_secondary
        })
        .into(),
        foreground_border_width: 0.0,
        foreground_border_color: color(p.plate),
        text_color: Some(color(if disabled {
            p.text_disabled
        } else {
            p.text_primary
        })),
        border_radius: Some(capsule(theme)),
        padding_ratio: 0.25,
    }
}

/// radio：正圆（radius full），选中 accent 圆点。
pub fn style_radio(
    theme: &OrbTheme,
    status: iced::widget::radio::Status,
) -> iced::widget::radio::Style {
    let p = &theme.tokens.palette;
    let (selected, hovered) = match status {
        iced::widget::radio::Status::Active { is_selected } => (is_selected, false),
        iced::widget::radio::Status::Hovered { is_selected } => (is_selected, true),
    };
    iced::widget::radio::Style {
        background: color(theme.glass_fill()).into(),
        dot_color: color(if selected { p.accent } else { p.text_secondary }),
        border_width: 1.0,
        border_color: color(if hovered { p.accent } else { p.glass_stroke }),
        text_color: Some(color(p.text_primary)),
    }
}

/// slider：细线轨道 + 圆形滑块（计划书 1.5：圆形滑块）。
pub fn style_slider(
    theme: &OrbTheme,
    status: iced::widget::slider::Status,
) -> iced::widget::slider::Style {
    let p = &theme.tokens.palette;
    let rail_active = matches!(
        status,
        iced::widget::slider::Status::Hovered | iced::widget::slider::Status::Dragged
    );
    iced::widget::slider::Style {
        rail: iced::widget::slider::Rail {
            backgrounds: (
                color(p.glass_edge).into(),
                color(if rail_active {
                    p.accent
                } else {
                    p.glass_stroke
                })
                .into(),
            ),
            width: 4.0,
            border: border(p.glass_edge, 0.0),
        },
        handle: iced::widget::slider::Handle {
            shape: iced::widget::slider::HandleShape::Circle { radius: 8.0 },
            background: color(theme.glass_fill_hover()).into(),
            border_width: 1.0,
            border_color: color(if status == iced::widget::slider::Status::Dragged {
                p.accent
            } else {
                p.glass_stroke
            }),
        },
    }
}

/// progress_bar：胶囊轨道 + accent 进度。
pub fn style_progress_bar(theme: &OrbTheme) -> iced::widget::progress_bar::Style {
    let p = &theme.tokens.palette;
    iced::widget::progress_bar::Style {
        background: color(theme.glass_fill()).into(),
        bar: color(p.accent).into(),
        border: iced::Border {
            color: color(p.glass_stroke),
            width: 1.0,
            radius: capsule(theme),
        },
    }
}

/// scrollable：主题化细圆滚动条。
pub fn style_scrollable(
    theme: &OrbTheme,
    _status: iced::widget::scrollable::Status,
) -> iced::widget::scrollable::Style {
    let p = &theme.tokens.palette;
    let rail = |bg: Rgba, scroller: Rgba| iced::widget::scrollable::Rail {
        background: Some(color(theme.opaque(bg)).into()),
        border: border(p.glass_edge, 0.0),
        scroller: iced::widget::scrollable::Scroller {
            background: color(scroller).into(),
            border: border(p.glass_stroke, 0.0),
        },
    };
    iced::widget::scrollable::Style {
        container: iced::widget::container::Style::default(),
        vertical_rail: rail(p.plate, p.glass_stroke),
        horizontal_rail: rail(p.plate, p.glass_stroke),
        gap: None,
        auto_scroll: iced::widget::scrollable::AutoScroll {
            background: color(theme.glass_fill()).into(),
            border: border(p.glass_stroke, 1.0),
            shadow: shadow(
                iced::Color::from_rgba8(0, 0, 0, theme.tokens.shadow.sm_alpha),
                theme.tokens.shadow.sm_offset_y,
                theme.tokens.shadow.sm_blur,
            ),
            icon: color(p.text_secondary),
        },
    }
}

/// rule：淡色分隔线（glass_edge）。
pub fn style_rule(theme: &OrbTheme) -> iced::widget::rule::Style {
    iced::widget::rule::Style {
        color: color(theme.tokens.palette.glass_edge),
        radius: 0.0.into(),
        fill_mode: iced::widget::rule::FillMode::Full,
        snap: true,
    }
}

/// pick_list：圆角 sm + 玻璃底。
pub fn style_pick_list(
    theme: &OrbTheme,
    status: iced::widget::pick_list::Status,
) -> iced::widget::pick_list::Style {
    let p = &theme.tokens.palette;
    let line = match status {
        iced::widget::pick_list::Status::Active => p.glass_stroke,
        iced::widget::pick_list::Status::Hovered
        | iced::widget::pick_list::Status::Opened { .. } => p.accent,
    };
    iced::widget::pick_list::Style {
        text_color: color(p.text_primary),
        placeholder_color: color(p.text_disabled),
        handle_color: color(p.text_secondary),
        background: color(theme.glass_fill()).into(),
        border: iced::Border {
            color: color(line),
            width: 1.0,
            radius: theme.tokens.radius.sm.into(),
        },
    }
}

/// pick_list 下拉菜单：plate 底 + accent 选中。
pub fn style_pick_list_menu(theme: &OrbTheme) -> iced::widget::overlay::menu::Style {
    let p = &theme.tokens.palette;
    iced::widget::overlay::menu::Style {
        background: color(theme.opaque(p.plate)).into(),
        border: iced::Border {
            color: color(p.glass_stroke),
            width: 1.0,
            radius: theme.tokens.radius.sm.into(),
        },
        text_color: color(p.text_primary),
        selected_text_color: color(p.on_accent),
        selected_background: color(p.accent).into(),
        shadow: shadow(
            iced::Color::from_rgba8(0, 0, 0, theme.tokens.shadow.md_alpha),
            theme.tokens.shadow.md_offset_y,
            theme.tokens.shadow.md_blur,
        ),
    }
}
