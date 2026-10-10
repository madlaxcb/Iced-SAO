//! Overlay 生命周期状态模型。

use std::time::Duration;

/// 通用控件交互状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InteractionState {
    hovered: bool,
    pressed: bool,
    focused: bool,
    disabled: bool,
}

impl InteractionState {
    /// 创建普通可交互状态。
    pub const fn new() -> Self {
        Self {
            hovered: false,
            pressed: false,
            focused: false,
            disabled: false,
        }
    }

    /// 应用一次交互事件。
    pub fn apply(&mut self, event: InteractionEvent) {
        match event {
            InteractionEvent::Hover(value) => self.hovered = value,
            InteractionEvent::Press(value) => self.pressed = value,
            InteractionEvent::Focus(value) => self.focused = value,
            InteractionEvent::Disable(value) => {
                self.disabled = value;
                if value {
                    self.hovered = false;
                    self.pressed = false;
                    self.focused = false;
                }
            }
        }
    }

    /// 返回当前状态是否可交互。
    pub const fn is_interactive(self) -> bool {
        !self.disabled
    }

    /// 返回当前状态是否处于强调态。
    pub const fn is_emphasized(self) -> bool {
        self.hovered || self.pressed || self.focused
    }
}

impl Default for InteractionState {
    fn default() -> Self {
        Self::new()
    }
}

/// 通用交互事件。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionEvent {
    /// 鼠标或指针进入/离开。
    Hover(bool),
    /// 按下/释放。
    Press(bool),
    /// 获取/失去焦点。
    Focus(bool),
    /// 启用/禁用。
    Disable(bool),
}

/// 启动序列的阶段。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartupPhase {
    /// 初始进入。
    Enter,
    /// 展示品牌标识。
    Brand,
    /// 展示内容。
    Content,
    /// 启动完成。
    Complete,
}

/// 启动序列状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StartupSequence {
    phase: StartupPhase,
    elapsed: Duration,
}

impl StartupSequence {
    /// 创建从 Enter 开始的启动序列。
    pub const fn new() -> Self {
        Self {
            phase: StartupPhase::Enter,
            elapsed: Duration::ZERO,
        }
    }

    /// 返回当前阶段。
    pub const fn phase(self) -> StartupPhase {
        self.phase
    }

    /// 推进启动序列，并在阶段完成时切换到下一阶段。
    pub fn advance(&mut self, delta: Duration) {
        self.elapsed += delta;
        while self.phase != StartupPhase::Complete {
            let threshold = match self.phase {
                StartupPhase::Enter => Duration::from_millis(100),
                StartupPhase::Brand => Duration::from_millis(180),
                StartupPhase::Content => Duration::from_millis(260),
                StartupPhase::Complete => Duration::ZERO,
            };
            if self.elapsed < threshold {
                break;
            }
            self.elapsed -= threshold;
            self.phase = match self.phase {
                StartupPhase::Enter => StartupPhase::Brand,
                StartupPhase::Brand => StartupPhase::Content,
                StartupPhase::Content => StartupPhase::Complete,
                StartupPhase::Complete => StartupPhase::Complete,
            };
        }
    }
}

impl Default for StartupSequence {
    fn default() -> Self {
        Self::new()
    }
}

/// Toast 生命周期状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToastState {
    visible: bool,
    elapsed: Duration,
    duration: Duration,
}

impl ToastState {
    /// 创建一个指定显示时长的可见 Toast。
    pub const fn visible_for(duration: Duration) -> Self {
        Self {
            visible: true,
            elapsed: Duration::ZERO,
            duration,
        }
    }

    /// 返回 Toast 是否仍然可见。
    pub const fn is_visible(self) -> bool {
        self.visible
    }

    /// 推进 Toast 生命周期。
    pub fn advance(&mut self, delta: Duration) {
        if self.visible {
            self.elapsed += delta;
            if self.elapsed >= self.duration {
                self.visible = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{InteractionEvent, InteractionState, StartupPhase, StartupSequence, ToastState};
    use std::time::Duration;

    #[test]
    fn startup_sequence_reaches_complete_in_token_order() {
        let mut startup = StartupSequence::new();
        assert_eq!(startup.phase(), StartupPhase::Enter);
        startup.advance(Duration::from_millis(100));
        assert_eq!(startup.phase(), StartupPhase::Brand);
        startup.advance(Duration::from_millis(180));
        assert_eq!(startup.phase(), StartupPhase::Content);
        startup.advance(Duration::from_millis(260));
        assert_eq!(startup.phase(), StartupPhase::Complete);
    }

    #[test]
    fn startup_sequence_consumes_large_delta() {
        let mut startup = StartupSequence::new();
        startup.advance(Duration::from_secs(1));
        assert_eq!(startup.phase(), StartupPhase::Complete);
    }

    #[test]
    fn interaction_state_clears_focus_when_disabled() {
        let mut state = InteractionState::new();
        state.apply(InteractionEvent::Hover(true));
        state.apply(InteractionEvent::Press(true));
        state.apply(InteractionEvent::Focus(true));
        assert!(state.is_interactive());
        assert!(state.is_emphasized());
        state.apply(InteractionEvent::Disable(true));
        assert!(!state.is_interactive());
        assert!(!state.is_emphasized());
    }

    #[test]
    fn toast_auto_dismisses_after_duration() {
        let mut toast = ToastState::visible_for(Duration::from_millis(380));
        toast.advance(Duration::from_millis(379));
        assert!(toast.is_visible());
        toast.advance(Duration::from_millis(1));
        assert!(!toast.is_visible());
    }
}
