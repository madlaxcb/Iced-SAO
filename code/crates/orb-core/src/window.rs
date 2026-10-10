use iced::{window, Task};

/// TitleBar 可以请求应用执行的窗口动作。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowCommand {
    /// 最小化当前窗口。
    Minimize,
    /// 在最大化和普通状态之间切换。
    ToggleMaximize,
    /// 关闭当前窗口。
    Close,
    /// 开始拖动当前窗口。
    Drag,
}

/// 将窗口命令映射为 iced 原生窗口任务。
pub fn task<T: 'static>(command: WindowCommand, id: Option<window::Id>) -> Task<T> {
    match (command, id) {
        (WindowCommand::Minimize, Some(id)) => window::minimize(id, true),
        (WindowCommand::ToggleMaximize, Some(id)) => window::toggle_maximize(id),
        (WindowCommand::Close, Some(id)) => window::close(id),
        (WindowCommand::Drag, Some(id)) => window::drag(id),
        (_, None) => Task::none(),
    }
}

#[cfg(test)]
mod tests {
    use super::WindowCommand;

    #[test]
    fn window_commands_cover_title_bar_actions() {
        assert_eq!(WindowCommand::Minimize, WindowCommand::Minimize);
        assert_eq!(WindowCommand::ToggleMaximize, WindowCommand::ToggleMaximize);
        assert_eq!(WindowCommand::Close, WindowCommand::Close);
        assert_eq!(WindowCommand::Drag, WindowCommand::Drag);
    }
}
