//! M5 动效与浮层状态展示页。

use crate::{GalleryElement, Message};
use iced::widget::{button, column, mouse_area, row, text};
use iced::Task;
use orb_core::{InteractionEvent, InteractionState, StartupSequence, ToastState};
use orb_theme::OrbTheme;
use orb_widgets::{context_menu, modal, overlay, toast, tooltip, ToastLevel};
use std::time::Duration;

/// M5 页面状态。
pub struct OverlayDemo {
    startup: StartupSequence,
    toast: Option<ToastState>,
    interaction: InteractionState,
    modal_open: bool,
}

impl OverlayDemo {
    /// 创建初始状态。
    pub fn new() -> Self {
        Self {
            startup: StartupSequence::new(),
            toast: None,
            interaction: InteractionState::new(),
            modal_open: false,
        }
    }

    /// 处理 M5 页面消息。
    pub fn update(&mut self, message: OverlayMessage) -> Task<OverlayMessage> {
        match message {
            OverlayMessage::AdvanceStartup => self.startup.advance(Duration::from_millis(100)),
            OverlayMessage::ShowToast => {
                self.toast = Some(ToastState::visible_for(Duration::from_millis(380)))
            }
            OverlayMessage::OpenModal => self.modal_open = true,
            OverlayMessage::Escape => self.modal_open = false,
            OverlayMessage::Hover(value) => self.interaction.apply(InteractionEvent::Hover(value)),
            OverlayMessage::Press(value) => self.interaction.apply(InteractionEvent::Press(value)),
            OverlayMessage::Focus(value) => self.interaction.apply(InteractionEvent::Focus(value)),
            OverlayMessage::Disable(value) => {
                self.interaction.apply(InteractionEvent::Disable(value))
            }
            OverlayMessage::Tick => {
                if let Some(toast) = &mut self.toast {
                    toast.advance(Duration::from_millis(40));
                    if !toast.is_visible() {
                        self.toast = None;
                    }
                }
            }
        }
        Task::none()
    }

    /// 构建 M5 页面。
    pub fn view(&self, _theme: &OrbTheme) -> GalleryElement<'static> {
        let tooltip_preview = tooltip(
            text("Hover target"),
            text("Tooltip: concise contextual help"),
        );
        let context_preview = context_menu(column![
            text("ContextMenu"),
            text("Rename"),
            text("Duplicate"),
            text("Delete"),
        ]);
        let modal_preview = if self.modal_open {
            modal(column![
                text("Modal").size(18),
                text("Focused dialog surface with explicit dismissal."),
                button("Close").on_press(Message::Overlay(OverlayMessage::Escape)),
            ])
        } else {
            modal(button("Open modal").on_press(Message::Overlay(OverlayMessage::OpenModal)))
        };
        let toast_preview = toast(
            row![text("Toast: saved successfully"), text("  ×")],
            ToastLevel::Success,
        );
        let toast_state = if self.toast.is_some() {
            "visible (380ms)"
        } else {
            "hidden"
        };

        overlay(
            column![
                text("M5 Overlays & Motion").size(24),
                text("Overlay · Tooltip · ContextMenu · Modal · Toast"),
                row![
                    text(format!("Startup phase: {:?}", self.startup.phase())),
                    button("advance 100ms")
                        .on_press(Message::Overlay(OverlayMessage::AdvanceStartup)),
                ]
                .spacing(12),
                text(format!("Toast lifecycle: {toast_state}")),
                text(format!(
                    "Interaction: emphasized={} interactive={}",
                    self.interaction.is_emphasized(),
                    self.interaction.is_interactive()
                )),
                mouse_area(text("Hover / press / focus target").size(16))
                    .on_enter(Message::Overlay(OverlayMessage::Hover(true)))
                    .on_exit(Message::Overlay(OverlayMessage::Hover(false)))
                    .on_press(Message::Overlay(OverlayMessage::Press(true)))
                    .on_release(Message::Overlay(OverlayMessage::Press(false))),
                button("show toast").on_press(Message::Overlay(OverlayMessage::ShowToast)),
                tooltip_preview,
                context_preview,
                modal_preview,
                toast_preview,
                text("Toast duration uses the motion token and is driven by deterministic ticks."),
            ]
            .spacing(14),
        )
        .padding(20)
        .into()
    }
}

impl Default for OverlayDemo {
    fn default() -> Self {
        Self::new()
    }
}

/// M5 页面内部消息。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverlayMessage {
    /// 推进启动序列。
    AdvanceStartup,
    /// 显示 Toast。
    ShowToast,
    /// 打开 Modal。
    OpenModal,
    /// 关闭当前浮层。
    Escape,
    /// 设置 hover 状态。
    Hover(bool),
    /// 设置 press 状态。
    Press(bool),
    /// 设置 focus 状态。
    Focus(bool),
    /// 设置 disabled 状态。
    Disable(bool),
    /// 推进 Toast 生命周期。
    Tick,
}

/// 构建无状态的 M4.3/M5 展示页。
#[allow(dead_code)]
pub fn view(theme: &OrbTheme) -> GalleryElement<'static> {
    OverlayDemo::new().view(theme)
}

#[allow(dead_code)]
fn _message(_: Message) {}
