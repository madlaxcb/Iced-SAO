# ADR-007：优先使用 iced 原生窗口动作

## 状态

暂定，等待 Windows W1 验证关卡确认。

## 决策

TitleBar 的最小化、关闭、最大化切换和拖动先由应用层直接调用 iced 0.14 的跨平台窗口任务。`orb-widgets::title_bar` 只负责渲染并发出消息，不持有窗口 ID，也不依赖 `orb-platform`。

Gallery 目前通过 `window::latest()` 获取当前窗口 ID，再调用 `window::minimize`、`window::close`、`window::toggle_maximize` 和 `window::drag`。

## 升级条件

只有 Windows W1 实测出现以下任一问题，才升级到 `orb-platform` 方案：

- iced 原生动作无法完成标题栏拖动、双击最大化、最小化或关闭；
- 无边框窗口下出现必须使用平台命中测试才能修复的标题栏区域问题；
- 四边缩放或 Snap 行为需要 Windows 专有补偿；
- 不同 Windows 版本、渲染后端或窗口模式出现 iced API 无法屏蔽的平台差异。

Linux 行为不能作为 Windows 结论。四边缩放和 Snap 在 W1 前不实现补偿逻辑。

## 后续验证

W1 在 Windows 上记录：拖动、标题栏双击最大化、最小化、关闭、四边缩放、贴边 Snap，以及窗口恢复后的尺寸和位置。若全部通过，则继续保持方案 1，不新增平台抽象。
