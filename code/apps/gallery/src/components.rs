//! M4.1 通用组件展示页（真实可交互：状态由 [`ComponentDemo`] 持有）。

use crate::{ComponentDemo, ComponentMessage, GalleryElement, Message};
use iced::widget::{column, row, text};
use orb_widgets::{
    button_variant_label, card, checkbox_control, divider, panel, radio_control, scroll_area,
    section_header, slider_control, switch_control, text_input_control, themed_button,
    ButtonVariant,
};

/// 构建 M4.1 通用组件展示页。
pub fn view(demo: &ComponentDemo) -> GalleryElement<'_> {
    let buttons = row![
        themed_button(
            text(button_variant_label(ButtonVariant::Primary)),
            ButtonVariant::Primary,
            None
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
            text_input_control("Placeholder", &demo.input, |value| {
                Message::Component(ComponentMessage::Input(value))
            }),
            checkbox_control("Checkbox", demo.checked, |value| {
                Message::Component(ComponentMessage::Check(value))
            }),
            radio_control("Radio A", 0_u8, Some(demo.radio), |value| {
                Message::Component(ComponentMessage::Radio(value))
            }),
            radio_control("Radio B", 1_u8, Some(demo.radio), |value| {
                Message::Component(ComponentMessage::Radio(value))
            }),
            switch_control("Switch", demo.switch_on, |value| {
                Message::Component(ComponentMessage::Switch(value))
            }),
            slider_control(0.0..=100.0, demo.slider, |value| {
                Message::Component(ComponentMessage::Slide(value))
            }),
            text(format!("Slider value: {:.0}", demo.slider)),
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
