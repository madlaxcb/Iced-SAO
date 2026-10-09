//! orb-tokens：设计 Token（纯数据，serde）。
//!
//! 颜色（含 alpha）、渐变色标、阴影档位、圆角档位、间距栅格、字号阶梯、动效时长与缓动。
//! 主题由数据文件驱动（TOML），改色板/圆角/动效无需改代码（计划书 1.4.8）。
//!
//! 颜色统一用 [`Rgba`] = `[r, g, b, a]`（各分量 0..=255）；转 f32 的辅助见 [`rgba_f32`]。
//! 组件目录禁止字面颜色 / 尺寸 / 时长（计划书 6.2），一律引用本 crate。

use serde::{Deserialize, Serialize};

pub mod metrics;
pub mod palette;

pub use metrics::{Motion, Radius, Shadow, Spacing, Typography};
pub use palette::Palette;

/// RGBA 颜色：`[r, g, b, a]`，各分量 0..=255。
pub type Rgba = [u8; 4];

/// RGBA → iced/palette 友好的 0..1 f32 数组。
pub fn rgba_f32(c: Rgba) -> [f32; 4] {
    [
        f32::from(c[0]) / 255.0,
        f32::from(c[1]) / 255.0,
        f32::from(c[2]) / 255.0,
        f32::from(c[3]) / 255.0,
    ]
}

/// 渐变色标：offset 0.0..1.0 + 颜色。
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GradientStop {
    /// 色标位置（0.0 = 起点，1.0 = 终点）。
    pub offset: f32,
    /// 色标颜色。
    pub color: Rgba,
}

/// 线性渐变（多色标，计划书 2.4）。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gradient {
    /// 角度（度，0 = 向右，90 = 向上，与 CSS 惯例一致由 orb-core 解释）。
    pub angle_deg: f32,
    /// 色标序列（按 offset 升序）。
    pub stops: Vec<GradientStop>,
}

impl Gradient {
    /// 双色标便捷构造。
    pub fn two_stops(angle_deg: f32, from: Rgba, to: Rgba) -> Self {
        Self {
            angle_deg,
            stops: vec![
                GradientStop {
                    offset: 0.0,
                    color: from,
                },
                GradientStop {
                    offset: 1.0,
                    color: to,
                },
            ],
        }
    }
}

/// 全量设计 Token（整体可替换以支持热更新，ADR-002）。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tokens {
    /// 语义色板。
    pub palette: Palette,
    /// 圆角档位。
    pub radius: Radius,
    /// 间距栅格。
    pub spacing: Spacing,
    /// 阴影档位。
    pub shadow: Shadow,
    /// 字号阶梯。
    pub typography: Typography,
    /// 动效时长与缓动。
    pub motion: Motion,
}

impl Default for Tokens {
    fn default() -> Self {
        Self::light()
    }
}

impl Tokens {
    /// 浅色主题（默认，计划书附录 B 起始值，M2 视觉校准后可修订）。
    pub fn light() -> Self {
        Self {
            palette: Palette::light(),
            radius: Radius::default(),
            spacing: Spacing::default(),
            shadow: Shadow::default(),
            typography: Typography::default(),
            motion: Motion::default(),
        }
    }

    /// 深色主题（glass.fill #1E2228 @0.78，附录 B）。
    pub fn dark() -> Self {
        Self {
            palette: Palette::dark(),
            radius: Radius::default(),
            spacing: Spacing::default(),
            shadow: Shadow::default(),
            typography: Typography::default(),
            motion: Motion::default(),
        }
    }

    /// 从 TOML 字符串加载（主题文件热加载的数据入口）。
    pub fn from_toml_str(s: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(s)
    }

    /// 序列化为 TOML（保存 / 调试）。
    pub fn to_toml_string(&self) -> String {
        toml::to_string_pretty(self).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_dark_roundtrip() {
        for tokens in [Tokens::light(), Tokens::dark()] {
            let s = tokens.to_toml_string();
            let parsed = Tokens::from_toml_str(&s).expect("roundtrip parse");
            assert_eq!(tokens, parsed);
        }
    }

    #[test]
    fn appendix_b_initial_values() {
        let light = Tokens::light();
        // 附录 B：glass.fill #FFFFFF @0.82 → alpha 209
        assert_eq!(light.palette.glass_fill, [255, 255, 255, 209]);
        // accent #F5A623
        assert_eq!(light.palette.accent, [0xF5, 0xA6, 0x23, 255]);
        // 橙底文字用深色（约 5:1，附录 B 备注）
        assert_eq!(light.palette.on_accent, [0x3C, 0x3C, 0x3C, 255]);

        let dark = Tokens::dark();
        // 深色 glass.fill #1E2228 @0.78 → 0.78*255 ≈ 199
        assert_eq!(dark.palette.glass_fill[0], 0x1E);
        assert_eq!(dark.palette.glass_fill[1], 0x22);
        assert_eq!(dark.palette.glass_fill[2], 0x28);
    }

    #[test]
    fn radius_scale_is_fixed() {
        // 圆润一致（计划书 1.4.2）：4/8/12/16/24/全圆
        let r = Radius::default();
        assert_eq!(
            (r.xs, r.sm, r.md, r.lg, r.xl, r.full),
            (4.0, 8.0, 12.0, 16.0, 24.0, 999.0)
        );
    }

    #[test]
    fn motion_durations_match_plan() {
        // 计划书附录 B：100/180/260/380ms
        let m = Motion::default();
        assert_eq!(m.duration_feedback_ms, 100);
        assert_eq!(m.duration_toggle_ms, 180);
        assert_eq!(m.duration_panel_ms, 260);
        assert_eq!(m.duration_startup_ms, 380);
        // 错峰间隔 40ms 起，最多 8 项
        assert_eq!(m.stagger_interval_ms, 40);
        assert_eq!(m.stagger_max_items, 8);
    }

    #[test]
    fn rgba_f32_conversion() {
        let f = rgba_f32([255, 128, 0, 209]);
        assert!((f[0] - 1.0).abs() < 1e-6);
        assert!((f[1] - 128.0 / 255.0).abs() < 1e-6);
        assert!((f[3] - 209.0 / 255.0).abs() < 1e-6);
    }
}
