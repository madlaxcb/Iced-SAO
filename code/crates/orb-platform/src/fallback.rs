//! Linux 开发宿主降级实现：所有 Windows 专有操作返回 [`PlatformError::Unsupported`]（计划书 3.7）。

use crate::PlatformError;

/// Linux 上不可用：亚克力是 Windows DWM 能力。
pub fn apply_acrylic_by_title(_title: &str, _color: [u8; 4]) -> Result<(), PlatformError> {
    Err(PlatformError::Unsupported)
}
