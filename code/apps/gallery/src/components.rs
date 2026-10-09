//! M4.1 + M4.4 通用组件展示页（真实可交互：状态由 [`ComponentDemo`] 持有）。

use crate::{ComponentDemo, ComponentMessage, GalleryElement, Message};
use iced::widget::{column, row, text};
use orb_widgets::{
    avatar, badge, button_variant_label, card, checkbox_control, divider, glow, list_item, loading,
    panel, radial_menu, radio_control, readout, scroll_area, section_header, select,
    slider_control, split_pane, switch_control, table, tabs, text_area, text_input_control,
    themed_button, title_bar, BadgeLevel, ButtonVariant,
};

/// 列表演示数据。
const LIST_ITEMS: [(&str, Option<&str>); 3] = [
    ("Weapons", Some("12 equipped")),
    ("Items", Some("4 potions")),
    ("Skills", None),
];

const TABLE_HEADERS: [&str; 3] = ["Name", "Class", "Status"];

fn table_rows() -> Vec<Vec<String>> {
    vec![
        vec!["Kirito".into(), "Swordsman".into(), "Online".into()],
        vec!["Asuna".into(), "Rapier".into(), "Ready".into()],
        vec!["Sinon".into(), "Archer".into(), "Standby".into()],
    ]
}

const RADIAL_ITEMS: [&str; 4] = ["Weapon", "Item", "Skill", "System"];
const SELECT_OPTIONS: [&str; 3] = ["Status", "Equipment", "Skills"];

/// 构建 M4 通用组件展示页。
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
            text("List / Tabs / Readout / Loading / Badge").size(18),
            column![
                list_item(
                    LIST_ITEMS[0].0,
                    LIST_ITEMS[0].1,
                    demo.list_selected == 0,
                    Message::Component(ComponentMessage::ListSelect(0))
                ),
                list_item(
                    LIST_ITEMS[1].0,
                    LIST_ITEMS[1].1,
                    demo.list_selected == 1,
                    Message::Component(ComponentMessage::ListSelect(1))
                ),
                list_item(
                    LIST_ITEMS[2].0,
                    LIST_ITEMS[2].1,
                    demo.list_selected == 2,
                    Message::Component(ComponentMessage::ListSelect(2))
                ),
            ]
            .spacing(4),
            tabs(&["Status", "Equipment", "Skills"], demo.tab, |index| {
                Message::Component(ComponentMessage::Tab(index))
            }),
            row![
                readout("72 / 100", "HP", None),
                readout("Lv.87", "Player", None),
                readout("1,250", "Gold", None),
            ]
            .spacing(32),
            row![
                loading(demo.loading_phase, 32.0),
                text("Loading spins at 2s / revolution (40ms tick)")
            ]
            .spacing(12),
            row![
                badge("NEW", BadgeLevel::Accent),
                badge("online", BadgeLevel::Good),
                badge("low hp", BadgeLevel::Warn),
                badge("defeated", BadgeLevel::Bad),
                badge("draft", BadgeLevel::Neutral),
            ]
            .spacing(8),
            divider::<Message>(),
            card(
                column![
                    text("Card").size(18),
                    text("主题化容器，沿用 Token 的圆角、底色、描边与阴影。"),
                ]
                .spacing(6)
            ),
            divider::<Message>(),
            text("P2: Table / RadialMenu / Avatar / Glow / TitleBar").size(18),
            table(
                &TABLE_HEADERS,
                &table_rows(),
                demo.table_selected,
                |index| Message::Component(ComponentMessage::TableSelect(index)),
            ),
            text(format!(
                "Table selected row: {}",
                demo.table_selected
                    .map(|index| (index + 1).to_string())
                    .unwrap_or_else(|| "none".into())
            )),
            row![
                radial_menu(
                    &RADIAL_ITEMS,
                    demo.radial_selected,
                    |index| Message::Component(ComponentMessage::RadialSelect(index)),
                    180.0,
                ),
                column![
                    text(format!(
                        "RadialMenu selected: {}",
                        demo.radial_selected
                            .and_then(|index| RADIAL_ITEMS.get(index).copied())
                            .unwrap_or("none")
                    )),
                    row![avatar("Kirito", 48.0), avatar("桐人", 48.0)].spacing(8),
                    glow(card(text("Glow / accent shadow")), 0.8),
                ]
                .spacing(10),
            ]
            .spacing(20),
            title_bar(
                "Gallery demo title bar",
                Message::Component(ComponentMessage::TitleBarMinimize),
                Message::Component(ComponentMessage::TitleBarClose),
            ),
            text(
                "TitleBar buttons are no-op demo actions; real window APIs stay in the app layer."
            ),
            divider::<Message>(),
            text("P3: Select / TextArea / SplitPane / BackgroundLayer").size(18),
            select(
                &SELECT_OPTIONS,
                SELECT_OPTIONS
                    .iter()
                    .find(|value| Some(**value) == demo.select_option.as_deref()),
                |value: &str| Message::Component(ComponentMessage::SelectOption(value.to_string())),
            ),
            text(format!(
                "Select: {}",
                demo.select_option.as_deref().unwrap_or("none")
            )),
            text_area(&demo.text_area, |action| {
                Message::Component(ComponentMessage::TextArea(action))
            }),
            text(format!(
                "TextArea: {}",
                demo.text_area.text().replace('\n', " / ")
            )),
            split_pane(
                card(text("SplitPane left")),
                card(text("SplitPane right")),
                demo.split_ratio,
            ),
            text(format!("SplitPane ratio: {:.0}%", demo.split_ratio * 100.0)),
            orb_widgets::background_layer(text("BackgroundLayer with opaque fallback"), 0.75,)
                .height(48),
            divider::<Message>(),
            text("Panel / Card / Divider / SectionHeader 均已接入 orb-theme Catalog。"),
        ]
        .spacing(12),
    )
    .padding(20)
    .into()
}
