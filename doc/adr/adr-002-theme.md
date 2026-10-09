# ADR-002：主题架构方案

- 状态：**已接受**（2026-10-07，M0 期间）
- 关联：计划书 2.4、M2

## 背景

三个候选（沿用上一份计划书的比较）：
- A：直接用 `iced::Theme`
- B：自定义 Theme 类型并实现各 widget 的 `Catalog`
- C：`iced::Theme` + 全局 Token

## 决策

**采用方案 B，直接沿用上一项目（Iced-Arknights `hud-theme`）的已验证结构**，按 orb 需求扩展：

```rust
pub struct OrbTheme {
    pub tokens: Tokens,          // 全量 Token，整体可替换（热更新）
}
impl theme::Base for OrbTheme { ... }          // 全局背景/文字
impl button::Catalog for OrbTheme { ... }      // 逐 widget Catalog
// ... container / text_input / checkbox / toggler / radio / slider /
//      progress_bar / scrollable / rule / pick_list / tooltip / text / menu
```

依据：
1. 上一项目 `hud-theme` 已用方案 B 完整落地（15+ widget 的 Catalog、`fn(&Theme, Status) -> Style` 纯函数样式、TOML 加载、thiserror 错误链），在 iced 0.14 本机验证可编译可运行——迁移成本远低于重新验证 A/C；
2. SAO 风格需要 Token 承载**渐变（多色标）、阴影（偏移/模糊/颜色）、不透明度、圆角档位**，A/C 的 `iced::Theme` 没有这些槽位；
3. B 的样式函数是纯函数，可独立单测（计划书 DoD"每个 widget × 每个状态有断言"）。

## 相对上一项目的扩展点（M2 实现）

1. Tokens 增加：渐变色标数组、阴影档位（sm/md/lg）、不透明度、圆角档位、动效时长/缓动；
2. **浅色为默认主题**（上一项目是暗色唯一）+ 深色，每套附"不透明降级版"；
3. 主题文件热加载沿用 notify + mpsc 桥接（上一项目 gallery 已验证）；
4. `theme::Mode` 返回 Light（浅色默认时）。

## 影响

- `orb-theme` 的骨架在 M1 就位（空壳已建），M2 按此实现；
- 组件目录禁止字面颜色/尺寸的门禁由 Token 单测 + 走查保障（同上一项目）。
