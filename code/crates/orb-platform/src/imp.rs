//! Windows 实现：按标题查找窗口并应用亚克力材质。
//!
//! M0 预研用 FindWindowW 按唯一标题定位窗口（避免 iced 公开 API 未暴露 HWND 的问题）；
//! 正式通道在 M1 换成 raw-window-handle 直取句柄。
//!
//! 2026-10-08 用户决策：点击穿透（WS_EX_TRANSPARENT）已从产品范围移除，相关代码删除；
//! 历史验证结论（穿透可行）保留在 doc/adr/adr-003。

use crate::PlatformError;
use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::FindWindowW;

fn find_hwnd(title: &str) -> Option<isize> {
    let mut wide: Vec<u16> = title.encode_utf16().collect();
    wide.push(0);
    // SAFETY: wide 是以 NUL 结尾的合法 UTF-16 缓冲；类名传 null 表示任意类
    let hwnd = unsafe { FindWindowW(PCWSTR::null(), PCWSTR::from_raw(wide.as_ptr())) };
    match hwnd {
        Ok(h) if !h.0.is_null() => Some(h.0 as isize),
        _ => None,
    }
}

/// 对标题对应的窗口应用整窗亚克力材质（G2 路线，计划书 2.6）。
pub fn apply_acrylic_by_title(title: &str, color: [u8; 4]) -> Result<(), PlatformError> {
    let hwnd = find_hwnd(title).ok_or(PlatformError::WindowNotFound)?;
    let [r, g, b, a] = color;
    window_vibrancy::apply_acrylic(HwndHandle(HWND(hwnd as _)), Some((r, g, b, a)))
        .map_err(|err| PlatformError::CallFailed(err.to_string()))
}

/// 把裸 HWND 包装成 raw-window-handle 能接受的窗口句柄。
struct HwndHandle(HWND);

impl raw_window_handle::HasWindowHandle for HwndHandle {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        let mut handle = raw_window_handle::Win32WindowHandle::new(unsafe {
            // SAFETY: HWND 非 0（find_hwnd 已过滤）
            core::num::NonZeroIsize::new_unchecked(self.0 .0 as isize)
        });
        handle.hinstance = None;
        // SAFETY: handle 含有效的非零 HWND，生命周期由 &self 保证
        Ok(unsafe {
            raw_window_handle::WindowHandle::borrow_raw(raw_window_handle::RawWindowHandle::Win32(
                handle,
            ))
        })
    }
}
