//! M3 视觉原语展示页。

use crate::{color_of, GalleryElement, Message};
use iced::widget::{column, container, row, text, Space};
use iced::Element;
use orb_core::{pixel_scale, snap};
use orb_theme::OrbTheme;
use orb_widgets::{circle_button, glass_panel, ring, GlassPanelStyle};

/// 构建 M3 视觉原语展示页。
pub fn view(theme: &OrbTheme) -> GalleryElement<'static> {
    let p = &theme.tokens.palette;
    let panel_style = GlassPanelStyle {
        fill: p.glass_fill,
        stroke: p.glass_stroke,
        radius: theme.tokens.radius.lg,
        shadow_alpha: theme.tokens.shadow.md_alpha,
    };

    let panel = glass_panel(
        column![
            text("GlassPanel").size(theme.tokens.typography.lg),
            text("G1 伪毛玻璃：半透明填充 + 描边 + 阴影"),
            text(format!(
                "radius={}px · shadow alpha={}",
                panel_style.radius, panel_style.shadow_alpha
            ))
            .size(12),
        ]
        .spacing(6),
        panel_style,
    )
    .width(360);

    let circle = circle_button(
        container(text("+").size(24)).center(56),
        56.0,
        Some(Message::Select(crate::Page::Primitives)),
    );

    let rings: Element<'static, Message, OrbTheme> = row![
        column![ring(0.25, color_of(p.accent), 96.0), text("25%").size(12)].spacing(4),
        column![ring(0.65, color_of(p.accent), 96.0), text("65%").size(12)].spacing(4),
        column![ring(1.0, color_of(p.accent), 96.0), text("100%").size(12)].spacing(4),
    ]
    .spacing(20)
    .into();

    column![
        text("M3 Visual Primitives").size(theme.tokens.typography.xl),
        text("GlassPanel · CircleButton · Ring · independent canvas cache").size(13),
        row![panel, column![text("CircleButton"), circle].spacing(8)].spacing(24),
        text("Ring").size(theme.tokens.typography.lg),
        rings,
        glass_panel(
            column![
                text("Animation primitives").size(theme.tokens.typography.lg),
                text("Tween / Sequence / Stagger / ManualClock"),
                text("feedback 100ms · toggle 180ms · panel 260ms · stagger 40ms").size(12),
            ]
            .spacing(6),
            panel_style,
        ),
        text("Pixel alignment").size(theme.tokens.typography.lg),
        row![
            text(format!("100%: {}px", snap(10.24, 1.0))),
            text(format!("125%: {}px", snap(10.24, 1.25))),
            text(format!("150%: {}px", snap(10.24, 1.5))),
            text(format!("200%: {}px", snap(10.24, 2.0))),
        ]
        .spacing(18),
        text(format!(
            "100px logical = {}px physical @125%",
            pixel_scale(100.0, 1.25)
        ))
        .size(12),
        Space::new().height(8),
    ]
    .spacing(14)
    .padding(16)
    .into()
}
