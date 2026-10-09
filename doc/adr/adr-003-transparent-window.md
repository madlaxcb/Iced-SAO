# ADR-003：透明 / 叠加窗口方案

- 状态：**已验证可行；产品不采用（用户决策 2026-10-08）**
- 关联：计划书 2.6、M0-P2、R2、W0 验证记录
- 2026-10-08 更新：用户决定产品不做透明叠加形态与点击穿透；毛玻璃改为「普通窗口 + 亚克力」。本 ADR 保留验证结论作为技术储备，`orb-platform` 中的穿透实现已删除。

## 背景

SAO 风格的启动器形态最初设想依赖透明、无边框、置顶、不抢焦点、点击穿透。风险：透明窗口在 iced + wgpu + Windows 下可能有黑块/闪烁/不透明背景（计划书 5.2，参考软件更新记录亦出现过）。

## 决策

1. **技术路线**：`window::Settings { transparent: true, decorations: false, level: AlwaysOnTop }` + **全局 style 背景色设为 `Color::TRANSPARENT`** + wgpu 预乘 alpha 后端。
   - 关键教训（W0 第一轮发现）：**仅设 `window.transparent` 不够**——iced 每帧用主题背景色（Light = 白）清屏，表现为白底不透明；必须同时置全局 `theme::Style::background_color = TRANSPARENT`。
2. **点击穿透**：M0 预研用 `FindWindowW`（按唯一窗口标题）+ `SetWindowLongPtrW(GWL_EXSTYLE, |= WS_EX_LAYERED | WS_EX_TRANSPARENT)` 实现整窗穿透，W0 验证生效。正式实现（M1）改为 raw-window-handle 直取 HWND，`FindWindowW` 方案废弃。
3. **降级路径**（计划书 1.4.6）：透明不可用时切不透明主题——`OrbTheme` 的不透明降级版已列入 M2 交付。

## W0 验证记录（用户 Windows 验证机，2026-10-07）

| 项 | 结果 |
|---|---|
| 透明渲染（空白区透出桌面） | **通过**（GLASS 面板悬浮于透明背景，无黑块/闪烁） |
| 无边框 | 通过 |
| 置顶 AlwaysOnTop | 通过 |
| 整窗点击穿透 | **通过**（能点到后面的桌面） |
| Esc 退出（穿透后的退出途径） | 通过（订阅 keyboard Esc → 进程退出） |

## 待办 / 未验证

- 不抢焦点（WS_EX_NOACTIVATE 或 winit focus 策略）未单独验证；
- "显示桌面"时窗口行为未验证（计划书 5.3 案例）；
- 多 GPU / Win10 22H2 上的复现未覆盖（验证机信息待登记）；
- 预研版深色 hint 文字在暗壁纸上不可读——预研程序已知问题（正式版文字落在不透明底板上，不受影响），probe 下一版修。

## 影响

- P0 的"透明 / 叠加窗口能力"从"高风险未验证"转为"通路可行"，剩余验证项列入后续 W 关卡；
- ADR-004（毛玻璃）的 G2 亚克力路线判定未完成：`acrylic applied` 但暗色壁纸下无法目视确认模糊，待复杂壁纸复验。
