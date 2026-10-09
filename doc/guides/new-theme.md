# 如何做新主题

> 面向新开发者的操作指南。前置阅读：[tokens.md](../design/tokens.md)。
>
> 核心结论：**orb 的主题 = 数据（`Tokens`）+ 纯函数样式（`orb-theme/src/styles.rs`）**。
> 因为所有样式函数只读 `theme.tokens`，所以做新主题原则上**不需要写任何 Rust 代码**——
> 只需要一份 TOML。只有当默认两套（light/dark）之外还想内置第三套预设时才动代码。

## 1. 主题系统的三件事

| 层 | 位置 | 职责 |
|---|---|---|
| Token 数据 | `orb-tokens`（`Tokens`/`Palette` 等） | 颜色、圆角、间距、阴影、字号、动效时长 |
| 样式函数 | `orb-theme/src/styles.rs` | 把 `Tokens` 映射为各 widget 的 `Style`（纯函数，禁止字面值） |
| Theme 类型 | `orb-theme/src/lib.rs`（`OrbTheme`） | 实现 iced `Base` + 13 个 widget Catalog，对外暴露 `glass_fill()` 等降级感知 API |

依赖方向单向：`orb-tokens → orb-core → orb-theme → orb-widgets`。
`OrbTheme` 由三部分组成：

```rust
pub struct OrbTheme {
    pub tokens: Tokens,          // 全量设计 Token，整体可替换（热加载入口）
    pub variant: Variant,        // Light / Dark（影响 iced mode）
    pub opaque_fallback: bool,   // 不透明降级开关（透明/模糊不可用时置 true）
}
```

## 2. 路线 A：TOML 文件主题（推荐，零代码）

### 步骤 1：导出现有主题作为模板

`Tokens` 支持完整 TOML 往返（有单测保证 `light_dark_roundtrip`）：

```rust
// 一次性调试代码，或写在 testkit 里
std::fs::write("theme-light.toml", orb_tokens::Tokens::light().to_toml_string()).unwrap();
```

得到的 TOML 顶层就是六个节：`palette`、`radius`、`spacing`、`shadow`、`typography`、`motion`，
字段含义见 [tokens.md](../design/tokens.md) 的对照表。

### 步骤 2：改字段，不动结构

- 只改**值**，不改字段名/嵌套结构（否则 `from_toml_str` 解析失败）；
- 颜色格式为 `[r, g, b, a]`，各分量 0–255，alpha 直接写 0–255（如 `glass_fill = [255,255,255,209]` 即 #FFFFFF @0.82）；
- **每个主题必须自带可用作降级版的值**：把半透明玻璃色（`glass_fill`、`glass_fill_hover` 等）同步准备一份不透明值备用。

### 步骤 3：加载

```rust
let s = std::fs::read_to_string("theme-light.toml")?;
let theme = orb_theme::OrbTheme::from_toml_str(orb_theme::Variant::Light, &s)?;
```

- `Variant::Light/Dark` 由调用方指定，它只影响 iced 的 `mode()` 报告与语义；
- `from_toml_str` 初始 `opaque_fallback = false`；需要降级时 `theme.opaque_fallback = true` 或链式 `.opaque_fallback()`。

### 步骤 4：验证

1. TOML 往返：`Tokens::from_toml_str(&s)?` 解析成功且字段与预期一致；
2. 不透明降级开关两种状态下跑 gallery，确认玻璃面板、按钮、浮层全部正常；
3. 交叉目标检查（样式代码含 cfg 差异时只有 Windows 目标能暴露问题）：

```sh
cd code
cargo check --offline -p <改动包>
cargo check --offline --target x86_64-pc-windows-msvc -p <改动包>
```

4. 最终以 Windows 验证机截图为准（W 关卡），未验证必须标注"未验证"。

## 3. 路线 B：新增内置预设（需要写代码）

只有当新主题要**编译进库**（如未来新增"高对比度"预设）时才走这条路。涉及两处：

1. `orb-tokens/src/palette.rs`：新增 `Palette::high_contrast()`（TDD：先在 `mod tests` 写断言
   关键色的失败测试，再填实现）；
2. `orb-tokens/src/lib.rs`：`Tokens::high_contrast()` 组合新 Palette + 默认 metrics；
3. （可选）`orb-theme/src/lib.rs`：`OrbTheme::high_contrast()`，并在 `name()` 的 match 里登记
   新名字——注意 `name()` 目前只覆盖 light/dark × opaque 四种组合，新变体需要同步扩展。

### 硬性规则（计划书 6.2）

- **禁止字面值**：样式函数（`styles.rs`）和组件里一律引用 `theme.tokens`，新预设也不例外；
- **降级必须成对**：每个新主题都要确认 `opaque()` 提升后的效果（`glass_fill()` 已自动感知）；
- **附录 B 是基准**：修改 light/dark 的既有值必须先改计划书附录 B 并在 PR 里说明，否则
  `appendix_b_initial_values` 单测会红——这是故意的，防止悄悄漂移。

## 4. 常见坑

| 现象 | 原因 | 处理 |
|---|---|---|
| `from_toml_str` 报错 | TOML 多写/少写字段 | 以 `to_toml_string()` 导出为准，只改值 |
| 深色主题下文字看不清 | 新 Palette 忘了 `text_secondary`/`text_disabled` | 对照 light/dark 的字段清单逐项检查 |
| 玻璃面板在验证机上发白/花 | 透明不可用但没开降级 | 运行时置 `opaque_fallback = true`（P8 验证项） |
| `pick_list` 菜单样式没变 | 菜单走独立的 `overlay::menu::Catalog` | 改 `style_pick_list_menu`，两个都要改 |
| 改了色但 gallery 没变 | 看的是旧进程 / notify 防抖未触发 | 确认 120ms 防抖与热加载日志 |

## 5. 速查

```
新主题流程：
导出模板（to_toml_string）→ 改值 → from_toml_str 加载
→ 双目标 check → gallery 两种降级状态过一遍 → Windows 验证机确认

动代码时：先写失败测试 → 最小实现 → cargo xtask ci 全绿
```
