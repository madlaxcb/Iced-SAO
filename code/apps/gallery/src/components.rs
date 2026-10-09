//! M4.1 通用组件展示页。

use crate::{GalleryElement, Message};
use iced::widget::{column, row, text};
use orb_theme::OrbTheme;
use orb_widgets::{
    button_variant_label, card, checkbox_control, divider, panel, radio_control, scroll_area,
    section_header, slider_control, switch_control, text_input_control, themed_button,
    ButtonVariant,
};

/// 构建 M4.1 通用组件展示页。
pub fn view(_theme: &OrbTheme) -> GalleryElement<'static> {
    let buttons = row![
        themed_button(
            text(button_variant_label(ButtonVariant::Primary)),
            ButtonVariant::Primary,
            Some(Message::Select(crate::Page::Components))
        ),
        themed_button(
            text(button_variant_label(ButtonVariant::Secondary)),
            ButtonVariant::Secondary,
            None
        ),
        themed_button(
            text(button_variant_label(ButtonVariant::Ghost)),
            ButtonVariant::Ghost,
            None
        ),
        themed_button(
            text(button_variant_label(ButtonVariant::Danger)),
            ButtonVariant::Danger,
            None
        ),
    ]
    .spacing(8);

    panel(
        column![
            section_header(text("M4.1 Common Components").size(24)),
            text("Button variants: primary / secondary / ghost / danger"),
            buttons,
            divider::<Message>(),
            text("TextInput / Checkbox / Radio / Switch / Slider / ScrollArea").size(18),
            text_input_control("Placeholder", "", |_| Message::Select(
                crate::Page::Components
            )),
            checkbox_control("Checkbox", true, |_| Message::Select(
                crate::Page::Components
            )),
            radio_control("Radio A", 0_u8, Some(0_u8), |_| Message::Select(
                crate::Page::Components
            )),
            switch_control("Switch", true, |_| Message::Select(crate::Page::Components)),
            slider_control(0.0..=100.0, 60.0, |_| Message::Select(
                crate::Page::Components
            )),
            scroll_area(
                column![
                    text("ScrollArea content"),
                    text("Scrollable content preview")
                ]
                .spacing(6)
            )
            .height(80),
            divider::<Message>(),
            card(
                column![
                    text("Card").size(18),
                    text("主题化容器，沿用 Token 的圆角、底色、描边与阴影。"),
                ]
                .spacing(6)
            ),
            divider::<Message>(),
            text("Panel / Card / Divider / SectionHeader 均已接入 orb-theme Catalog。"),
        ]
        .spacing(12),
    )
    .padding(20)
    .into()
}
