//! 像素对齐工具：让线条和边界在常见 DPI 缩放下落到稳定的物理像素。

use iced::Rectangle;

/// 将逻辑尺寸换算为物理像素。
pub fn pixel_scale(value: f32, scale: f32) -> f32 {
    value * scale
}

/// 将逻辑坐标吸附到当前缩放对应的物理像素网格。
pub fn snap(value: f32, scale: f32) -> f32 {
    (value * scale).round() / scale
}

/// 对矩形的位置和尺寸分别进行像素吸附。
pub fn snap_rect(rect: Rectangle, scale: f32) -> Rectangle {
    Rectangle {
        x: snap(rect.x, scale),
        y: snap(rect.y, scale),
        width: snap(rect.width, scale),
        height: snap(rect.height, scale),
    }
}
