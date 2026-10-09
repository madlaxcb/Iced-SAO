//! orb-platform：平台抽象（窗口特效、无边框与缩放、DPI、系统动画设置）。
//!
//! Windows 实现使用 windows crate；Linux 开发宿主返回 [`PlatformError::Unsupported`]，
//! 不追求一致（计划书 3.7）。
//!
//! 2026-10-08 用户决策：不做透明叠加窗口形态与点击穿透；
//! 毛玻璃（亚克力）以普通窗口形态提供。历史验证结论见 doc/adr/adr-003。

/// 平台操作错误。
#[derive(Debug, Clone)]
pub enum PlatformError {
    /// 按标题找不到目标窗口。
    WindowNotFound,
    /// 平台调用失败。
    CallFailed(String),
    /// 当前平台不支持该操作（Linux 开发宿主）。
    Unsupported,
}

impl core::fmt::Display for PlatformError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::WindowNotFound => write!(f, "window not found by title"),
            Self::CallFailed(msg) => write!(f, "platform call failed: {msg}"),
            Self::Unsupported => write!(f, "operation unsupported on this platform"),
        }
    }
}

impl std::error::Error for PlatformError {}

#[cfg(windows)]
mod imp;

#[cfg(windows)]
pub use imp::apply_acrylic_by_title;

#[cfg(not(windows))]
mod fallback;

#[cfg(not(windows))]
pub use fallback::apply_acrylic_by_title;
