//! 几何与动效 Token：圆角档位、间距栅格、阴影档位、字号阶梯、动效时长（计划书附录 B）。

use serde::{Deserialize, Serialize};

/// 圆角档位（计划书 1.4.2：全库同一套档位，不允许临时数值）。
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Radius {
    /// 4px：小控件（checkbox、badge 内圆角）。
    pub xs: f32,
    /// 8px：输入框、小按钮。
    pub sm: f32,
    /// 12px：卡片、面板内块。
    pub md: f32,
    /// 16px：面板。
    pub lg: f32,
    /// 24px：大面板、窗口级容器。
    pub xl: f32,
    /// 全圆（胶囊 / 圆形）：实现时取 min(边长/2, 此值)。
    pub full: f32,
}

impl Default for Radius {
    fn default() -> Self {
        Self {
            xs: 4.0,
            sm: 8.0,
            md: 12.0,
            lg: 16.0,
            xl: 24.0,
            full: 999.0,
        }
    }
}

/// 间距栅格：4px 基数（4/8/12/16/24/32）。
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Spacing {
    /// 4px：图标与文字间隙。
    pub xs: f32,
    /// 8px：相关元素间距。
    pub sm: f32,
    /// 12px：组内块间距。
    pub md: f32,
    /// 16px：容器内边距基准。
    pub lg: f32,
    /// 24px：分组间距。
    pub xl: f32,
    /// 32px：页面区块间距。
    pub xxl: f32,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            xs: 4.0,
            sm: 8.0,
            md: 12.0,
            lg: 16.0,
            xl: 24.0,
            xxl: 32.0,
        }
    }
}

/// 阴影档位（偏移 / 模糊 / 不透明度，计划书附录 B：sm/md/lg 三档，仅纵向偏移）。
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Shadow {
    /// sm：纵向偏移。
    pub sm_offset_y: f32,
    /// sm：模糊半径。
    pub sm_blur: f32,
    /// sm：不透明度。
    pub sm_alpha: f32,
    /// md：纵向偏移。
    pub md_offset_y: f32,
    /// md：模糊半径。
    pub md_blur: f32,
    /// md：不透明度。
    pub md_alpha: f32,
    /// lg：纵向偏移。
    pub lg_offset_y: f32,
    /// lg：模糊半径。
    pub lg_blur: f32,
    /// lg：不透明度。
    pub lg_alpha: f32,
}

impl Default for Shadow {
    fn default() -> Self {
        Self {
            sm_offset_y: 1.0,
            sm_blur: 3.0,
            sm_alpha: 0.12,
            md_offset_y: 4.0,
            md_blur: 12.0,
            md_alpha: 0.16,
            lg_offset_y: 8.0,
            lg_blur: 24.0,
            lg_alpha: 0.20,
        }
    }
}

/// 字号阶梯（最小 12px，计划书附录 B；浅色主题字重校准在 Windows 上做，计划书 3.4）。
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Typography {
    /// 12px：辅助 / 标注（最小可读字号）。
    pub xs: f32,
    /// 14px：正文。
    pub sm: f32,
    /// 16px：正文强调 / 输入框。
    pub md: f32,
    /// 20px：小标题。
    pub lg: f32,
    /// 28px：页标题。
    pub xl: f32,
    /// 40px：展示数字 / 启动序列。
    pub xxl: f32,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            xs: 12.0,
            sm: 14.0,
            md: 16.0,
            lg: 20.0,
            xl: 28.0,
            xxl: 40.0,
        }
    }
}

/// 动效 Token（计划书 2.5 / 附录 B）。
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Motion {
    /// 100ms：点击反馈。
    pub duration_feedback_ms: u64,
    /// 180ms：切换。
    pub duration_toggle_ms: u64,
    /// 260ms：面板展开 / 滑入。
    pub duration_panel_ms: u64,
    /// 380ms：启动序列单步。
    pub duration_startup_ms: u64,
    /// 错峰间隔 40ms 起。
    pub stagger_interval_ms: u64,
    /// 最多 8 项后合并（附录 B）。
    pub stagger_max_items: u32,
    /// 缓动：面板 / 位移。
    pub easing_panel: EasingKind,
    /// 缓动：反馈 / 淡入。
    pub easing_feedback: EasingKind,
}

/// 缓动类型（计划书附录 B：ease-out 默认、ease-in-out 位移、过冲 ≤ 4% 仅位移/缩放）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EasingKind {
    /// 线性。
    Linear,
    /// ease-out（默认反馈）。
    EaseOut,
    /// ease-in-out（位移）。
    EaseInOut,
    /// 轻微过冲（≤ 4%，仅作用于位移 / 缩放，不作用于布局尺寸——计划书 5.2）。
    EaseOutBack,
}

impl Default for Motion {
    fn default() -> Self {
        Self {
            duration_feedback_ms: 100,
            duration_toggle_ms: 180,
            duration_panel_ms: 260,
            duration_startup_ms: 380,
            stagger_interval_ms: 40,
            stagger_max_items: 8,
            easing_panel: EasingKind::EaseInOut,
            easing_feedback: EasingKind::EaseOut,
        }
    }
}
