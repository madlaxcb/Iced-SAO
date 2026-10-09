//! 色板：浅色（默认）与深色预设（计划书附录 B）。
//!
//! 所有颜色均为 [`Rgba`](crate::Rgba)；文字对比度底线 ≥ 4.5:1、橙底文字用深色（附录 B 备注）。

use serde::{Deserialize, Serialize};

use crate::Rgba;

/// 语义色板。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Palette {
    /// 玻璃面板底色（半透明）。
    pub glass_fill: Rgba,
    /// 玻璃面板悬停底色（更不透明）。
    pub glass_fill_hover: Rgba,
    /// 玻璃面板 1px 内侧高光描边。
    pub glass_stroke: Rgba,
    /// 外侧轻描边 / 分隔。
    pub glass_edge: Rgba,
    /// 文字底板（不透明——文字永远落在足够不透明的底板上，计划书 1.4.1）。
    pub plate: Rgba,
    /// 窗口 / 应用背景。
    pub background: Rgba,
    /// 主文字。
    pub text_primary: Rgba,
    /// 次要文字。
    pub text_secondary: Rgba,
    /// 禁用文字。
    pub text_disabled: Rgba,
    /// 橙色强调（选中 / 主操作，占屏 < 10%，计划书 1.4.3）。
    pub accent: Rgba,
    /// 橙色填充上的文字（深色，约 5:1）。
    pub on_accent: Rgba,
    /// 状态条分色：良好（绿）。
    pub hp_good: Rgba,
    /// 状态条分色：警告（黄）。
    pub hp_warn: Rgba,
    /// 状态条分色：危险（红）。
    pub hp_bad: Rgba,
}

impl Palette {
    /// 浅色（默认，附录 B）。
    pub fn light() -> Self {
        Self {
            glass_fill: [255, 255, 255, 209],       // #FFFFFF @0.82
            glass_fill_hover: [255, 255, 255, 235], // @0.92
            glass_stroke: [255, 255, 255, 230],     // @0.90
            glass_edge: [0, 0, 0, 20],              // #000000 @0.08
            plate: [0xF4, 0xF4, 0xF4, 255],         // #F4F4F4
            background: [0xEA, 0xEC, 0xEF, 255],
            text_primary: [0x3C, 0x3C, 0x3C, 255],   // #3C3C3C
            text_secondary: [0x7A, 0x7A, 0x7A, 255], // #7A7A7A
            text_disabled: [0xB0, 0xB0, 0xB0, 255],  // #B0B0B0
            accent: [0xF5, 0xA6, 0x23, 255],         // #F5A623
            on_accent: [0x3C, 0x3C, 0x3C, 255],      // 深色文字（约 5:1）
            hp_good: [0x7C, 0xC2, 0x43, 255],        // #7CC243
            hp_warn: [0xF5, 0xD3, 0x3D, 255],        // #F5D33D
            hp_bad: [0xE5, 0x48, 0x3E, 255],         // #E5483E
        }
    }

    /// 深色（glass.fill #1E2228 @0.78，强调色保持橙色并复核对比度）。
    pub fn dark() -> Self {
        Self {
            glass_fill: [0x1E, 0x22, 0x28, 199], // @0.78
            glass_fill_hover: [0x1E, 0x22, 0x28, 224],
            glass_stroke: [255, 255, 255, 36],
            glass_edge: [0, 0, 0, 60],
            plate: [0x26, 0x2B, 0x32, 255],
            background: [0x15, 0x18, 0x1C, 255],
            text_primary: [0xE8, 0xEA, 0xED, 255], // #E8EAED
            text_secondary: [0x9A, 0xA0, 0xA6, 255],
            text_disabled: [0x5F, 0x66, 0x6D, 255],
            accent: [0xF5, 0xA6, 0x23, 255],
            on_accent: [0x20, 0x1A, 0x08, 255], // 深底字（深色主题复核值，M2 校准）
            hp_good: [0x7C, 0xC2, 0x43, 255],
            hp_warn: [0xF5, 0xD3, 0x3D, 255],
            hp_bad: [0xE5, 0x48, 0x3E, 255],
        }
    }
}
