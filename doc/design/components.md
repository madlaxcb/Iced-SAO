# 组件 API 一览

> 实现：`code/crates/orb-widgets`（组件）、`code/crates/orb-core`（动画 / 状态机 / 像素对齐）。
> 所有组件的 Theme 泛型均为自定义 `OrbTheme`（ADR-002）。

## 基础与容器（M4.1）

| 函数 | 用途 |
|---|---|
| `themed_button(content, ButtonVariant, Option<Message>)` | 按钮；变体 Primary / Secondary / Ghost / Danger |
| `panel(content)` / `card(content)` | 面板 / 卡片容器 |
| `divider()` | 主题化分隔线 |
| `section_header(title)` | 区块标题容器 |
| `glass_panel(content, GlassPanelStyle)` | 玻璃面板（fill / stroke / radius / shadow_alpha） |

## 表单控件（M4.2）

| 函数 | 用途 |
|---|---|
| `text_input_control(placeholder, value, on_input)` | 单行输入 |
| `checkbox_control(label, checked, on_toggle)` | 复选框 |
| `radio_control(label, value, selected, on_click)` | 单选 |
| `switch_control(label, checked, on_toggle)` | 开关 |
| `slider_control(range, value, on_change)` | 滑块 |
| `scroll_area(content)` | 滚动区域 |

## 浮层（M4.3 / M5）

| 函数 | 用途 |
|---|---|
| `overlay(content)` | 浮层容器 |
| `tooltip(content, hint)` | 悬停提示 |
| `context_menu(items)` | 上下文菜单 |
| `modal(content)` | 模态对话框 |
| `toast(message, ToastLevel)` | 轻量通知（Info / Success / Warning / Error） |

## 数据展示与反馈（M4.4 / M5，v0.2.0）

| 函数 | 用途 |
|---|---|
| `list_item(title, subtitle, selected, on_press)` | 单行列表项（P0 List 的行组件；选中态强调描边）；行文本格式见 `list_item_line` |
| `readout(value, label, unit)` | 数值读出（大号数值 + 小号标签）；文本格式见 `readout_text` |
| `loading(phase, size)` | 环形旋转指示；相位由调用方 Tick 推进（`loading_normalize` 归一化），保持确定性动画架构 |
| `tabs(labels, selected, on_select)` | 胶囊标签页（越界选择自动钳制 `tabs_clamp_select`） |
| `badge(label, BadgeLevel)` | 胶囊徽标；Neutral / Accent / Good / Warn / Bad 五档，颜色取主题 Token |

## 招牌组件（M3 / M5）

| 函数 | 用途 |
|---|---|
| `circle_button(content, size, Option<Message>)` | 圆形图标按钮 |
| `ring(progress, color, size)` | 环形进度（独立 `canvas::Cache`，进度钳制 [0,1]） |
| `hp_bar(value, width, height)` | 胶囊 HP 条；`hp_bar_band` 分档 Good(≥0.5) / Warn(≥0.2) / Bad |
| `menu_rail(icons, selected, on_select)` | SAO 风格圆形图标导航栏（选中描边强调色） |
| `menu_rail_next / menu_rail_prev` | MenuRail 键盘步进（环形 + None 兜底；app 层接 `keyboard::listen` 使用） |
| `menu_panel(title, content)` | 与 rail 配套的展开面板 |
| `loop_scroll(content)` | 水平循环滚动条带；索引回绕用 `loop_scroll_next / loop_scroll_prev` |

## 演示页交互架构（gallery Components 页）

控件演示不是静态展示：状态由 `gallery::ComponentDemo` 持有（input / checked / radio /
switch_on / slider / tab / list_selected / loading_phase），经 `ComponentMessage` 回写。
Loading 相位仅在 Components 页活动时由 40ms tick 推进（2s / 圈）。新增演示控件时必须
接入该状态，禁止在 view 里硬编码值（v0.1.2 修复的教训）。

## orb-core：动画与状态

| 类型 | 用途 |
|---|---|
| `Easing` / `Tween` | 缓动与补间（Linear / EaseOut / EaseInOut / EaseOutBack） |
| `Sequence` / `Stagger` | 顺序播放 / 错峰（间隔与上限取 Motion Token） |
| `ManualClock` | 可注入假时钟，动画确定性单测的基础 |
| `StartupSequence` | 启动序列 Enter→Brand→Content→Complete（Token 时长） |
| `ToastState` | Toast 生命周期（到期自动隐藏） |
| `InteractionState` | hover / press / focus / disable 状态机（禁用清空其余状态） |
| `snap / snap_rect / pixel_scale` | 100/125/150/200% 缩放像素对齐 |

## 组件完成定义（DoD）

1. 状态齐全（default / hover / pressed / focus / disabled，按组件适用）；
2. 键盘可操作、焦点样式可见；
3. 全部数值来自 Token；
4. 纯逻辑（分档 / 索引 / 状态机）有确定性单测；
5. `cargo xtask ci` 双目标全绿。
