//! orb-theme：主题（Theme 类型、各 widget 样式、字体注册、浅色/深色/不透明降级预设）。

pub mod styles;

use orb_tokens::Tokens;
use serde::{Deserialize, Serialize};

pub use styles::{
    style_button, style_checkbox, style_container, style_pick_list, style_pick_list_menu,
    style_progress_bar, style_radio, style_rule, style_scrollable, style_slider, style_text,
    style_text_input, style_toggler,
};

/// 主题变体（浅色为默认，计划书 2.4）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Variant {
    /// 浅色（默认）。
    Light,
    /// 深色。
    Dark,
}

/// orb 主题（方案 B：自定义 Theme 类型 + 各 widget Catalog，ADR-002）。
///
/// - `tokens` 整体可替换（TOML 热加载的数据入口在 [`OrbTheme::from_toml_str`]）；
/// - `opaque_fallback` = true 时玻璃色提为不透明（透明 / 模糊不可用时的降级，
///   计划书 2.4：每个主题必须自带「不透明降级版」）。
#[derive(Debug, Clone)]
pub struct OrbTheme {
    /// 全量设计 Token（整体可替换以支持热更新）。
    pub tokens: Tokens,
    /// 主题变体（浅 / 深）。
    pub variant: Variant,
    /// 不透明降级开关（透明 / 模糊不可用时置 true）。
    pub opaque_fallback: bool,
}

impl OrbTheme {
    /// 浅色（默认）。
    pub fn light() -> Self {
        Self {
            tokens: Tokens::light(),
            variant: Variant::Light,
            opaque_fallback: false,
        }
    }

    /// 深色。
    pub fn dark() -> Self {
        Self {
            tokens: Tokens::dark(),
            variant: Variant::Dark,
            opaque_fallback: false,
        }
    }

    /// 不透明降级版（布局与功能不变，仅材质切换，计划书 1.4.6）。
    pub fn opaque_fallback(mut self) -> Self {
        self.opaque_fallback = true;
        self
    }

    /// 从 TOML 字符串加载 Token（变体由调用方决定）。
    pub fn from_toml_str(variant: Variant, s: &str) -> Result<Self, toml::de::Error> {
        Ok(Self {
            tokens: Tokens::from_toml_str(s)?,
            variant,
            opaque_fallback: false,
        })
    }

    /// 玻璃色降级解析：`opaque_fallback` 时把半透明色提为不透明。
    pub fn opaque(&self, c: orb_tokens::Rgba) -> orb_tokens::Rgba {
        if self.opaque_fallback {
            [c[0], c[1], c[2], 255]
        } else {
            c
        }
    }

    /// 面板底色（降级感知）。
    pub fn glass_fill(&self) -> orb_tokens::Rgba {
        self.opaque(self.tokens.palette.glass_fill)
    }

    /// 面板悬停底色（降级感知）。
    pub fn glass_fill_hover(&self) -> orb_tokens::Rgba {
        self.opaque(self.tokens.palette.glass_fill_hover)
    }
}

impl Default for OrbTheme {
    fn default() -> Self {
        Self::light()
    }
}

impl iced::theme::Base for OrbTheme {
    fn default(_preference: iced::theme::Mode) -> Self {
        Self::light()
    }

    fn mode(&self) -> iced::theme::Mode {
        match self.variant {
            Variant::Light => iced::theme::Mode::Light,
            Variant::Dark => iced::theme::Mode::Dark,
        }
    }

    fn base(&self) -> iced::theme::Style {
        iced::theme::Style {
            background_color: orb_tokens::rgba_f32(self.tokens.palette.background).into(),
            text_color: orb_tokens::rgba_f32(self.tokens.palette.text_primary).into(),
        }
    }

    fn palette(&self) -> Option<iced::theme::Palette> {
        let p = &self.tokens.palette;
        Some(iced::theme::Palette {
            background: orb_tokens::rgba_f32(p.background).into(),
            text: orb_tokens::rgba_f32(p.text_primary).into(),
            primary: orb_tokens::rgba_f32(p.accent).into(),
            success: orb_tokens::rgba_f32(p.hp_good).into(),
            warning: orb_tokens::rgba_f32(p.hp_warn).into(),
            danger: orb_tokens::rgba_f32(p.hp_bad).into(),
        })
    }

    fn name(&self) -> &str {
        match (self.variant, self.opaque_fallback) {
            (Variant::Light, false) => "orb-light",
            (Variant::Light, true) => "orb-light-opaque",
            (Variant::Dark, false) => "orb-dark",
            (Variant::Dark, true) => "orb-dark-opaque",
        }
    }
}

// ---------------------------------------------------------------------------
// Catalog 实现（模式照抄上一项目已验证写法：StyleFn + 纯函数样式）
// ---------------------------------------------------------------------------

impl iced::widget::container::Catalog for OrbTheme {
    type Class<'a> = iced::widget::container::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(styles::style_container)
    }

    fn style(&self, class: &Self::Class<'_>) -> iced::widget::container::Style {
        class(self)
    }
}

impl iced::widget::button::Catalog for OrbTheme {
    type Class<'a> = iced::widget::button::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(styles::style_button)
    }

    fn style(
        &self,
        class: &Self::Class<'_>,
        status: iced::widget::button::Status,
    ) -> iced::widget::button::Style {
        class(self, status)
    }
}

impl iced::widget::text_input::Catalog for OrbTheme {
    type Class<'a> = iced::widget::text_input::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(styles::style_text_input)
    }

    fn style(
        &self,
        class: &Self::Class<'_>,
        status: iced::widget::text_input::Status,
    ) -> iced::widget::text_input::Style {
        class(self, status)
    }
}

impl iced::widget::text::Catalog for OrbTheme {
    type Class<'a> = iced::widget::text::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(styles::style_text)
    }

    fn style(&self, class: &Self::Class<'_>) -> iced::widget::text::Style {
        class(self)
    }
}

impl iced::widget::checkbox::Catalog for OrbTheme {
    type Class<'a> = iced::widget::checkbox::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(styles::style_checkbox)
    }

    fn style(
        &self,
        class: &Self::Class<'_>,
        status: iced::widget::checkbox::Status,
    ) -> iced::widget::checkbox::Style {
        class(self, status)
    }
}

impl iced::widget::toggler::Catalog for OrbTheme {
    type Class<'a> = iced::widget::toggler::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(styles::style_toggler)
    }

    fn style(
        &self,
        class: &Self::Class<'_>,
        status: iced::widget::toggler::Status,
    ) -> iced::widget::toggler::Style {
        class(self, status)
    }
}

impl iced::widget::radio::Catalog for OrbTheme {
    type Class<'a> = iced::widget::radio::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(styles::style_radio)
    }

    fn style(
        &self,
        class: &Self::Class<'_>,
        status: iced::widget::radio::Status,
    ) -> iced::widget::radio::Style {
        class(self, status)
    }
}

impl iced::widget::slider::Catalog for OrbTheme {
    type Class<'a> = iced::widget::slider::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(styles::style_slider)
    }

    fn style(
        &self,
        class: &Self::Class<'_>,
        status: iced::widget::slider::Status,
    ) -> iced::widget::slider::Style {
        class(self, status)
    }
}

impl iced::widget::progress_bar::Catalog for OrbTheme {
    type Class<'a> = iced::widget::progress_bar::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(styles::style_progress_bar)
    }

    fn style(&self, class: &Self::Class<'_>) -> iced::widget::progress_bar::Style {
        class(self)
    }
}

impl iced::widget::scrollable::Catalog for OrbTheme {
    type Class<'a> = iced::widget::scrollable::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(styles::style_scrollable)
    }

    fn style(
        &self,
        class: &Self::Class<'_>,
        status: iced::widget::scrollable::Status,
    ) -> iced::widget::scrollable::Style {
        class(self, status)
    }
}

impl iced::widget::rule::Catalog for OrbTheme {
    type Class<'a> = iced::widget::rule::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(styles::style_rule)
    }

    fn style(&self, class: &Self::Class<'_>) -> iced::widget::rule::Style {
        class(self)
    }
}

impl iced::widget::pick_list::Catalog for OrbTheme {
    type Class<'a> = iced::widget::pick_list::StyleFn<'a, Self>;

    // pick_list::Catalog: menu::Catalog: scrollable::Catalog 继承链导致 Self::Class 歧义，
    // 必须 fully qualified（上一项目同款写法）
    fn default<'a>() -> <Self as iced::widget::pick_list::Catalog>::Class<'a> {
        Box::new(styles::style_pick_list)
    }

    fn style(
        &self,
        class: &<Self as iced::widget::pick_list::Catalog>::Class<'_>,
        status: iced::widget::pick_list::Status,
    ) -> iced::widget::pick_list::Style {
        class(self, status)
    }
}

impl iced::widget::overlay::menu::Catalog for OrbTheme {
    type Class<'a> = iced::widget::overlay::menu::StyleFn<'a, Self>;

    fn default<'a>() -> <Self as iced::widget::overlay::menu::Catalog>::Class<'a> {
        Box::new(styles::style_pick_list_menu)
    }

    fn style(
        &self,
        class: &<Self as iced::widget::overlay::menu::Catalog>::Class<'_>,
    ) -> iced::widget::overlay::menu::Style {
        class(self)
    }
}
