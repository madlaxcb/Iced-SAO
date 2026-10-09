# Token 设计规范

> 数据模型在 `code/crates/orb-tokens`；主题消费在 `code/crates/orb-theme`。
> 本文档与代码同步（M2 交付要求）；改 Token 必须同步改这里的表。

## 规则

1. 组件**禁止**硬编码颜色 / 尺寸 / 时长，一律取 `Tokens`；
2. 颜色统一用 `Rgba = [u8; 4]`，转 iced 用 `rgba_f32` / `Color::from_rgba8`；
3. 半透明色必须可一键降级：`OrbTheme::opaque_fallback()` 将 alpha 提升至 255；
4. 文字对比度底线 ≥ 4.5:1；橙底（accent）上必须用深色文字 `on_accent`（约 5:1）。

## Palette（浅色 / 深色）

| Token | 浅色 | 深色 | 用途 |
|---|---|---|---|
| `glass_fill` | #FFFFFF @0.82 | #1E2228 @0.78 | 玻璃面板底色 |
| `glass_fill_hover` | #FFFFFF @0.92 | #1E2228 @0.88 | 面板悬停 |
| `glass_stroke` | #FFFFFF @0.90 | #FFFFFF @0.14 | 内侧高光描边 |
| `glass_edge` | #000000 @0.08 | #000000 @0.24 | 外侧轻描边 |
| `plate` | #F4F4F4 | #262B32 | 文字底板（不透明） |
| `background` | #EAECEF | #15181C | 窗口背景 |
| `text_primary` | #3C3C3C | #E8EAED | 主文字 |
| `text_secondary` | #7A7A7A | #9AA0A6 | 次要文字 |
| `text_disabled` | #B0B0B0 | #5F666D | 禁用文字 |
| `accent` | #F5A623 | #F5A623 | 选中 / 主操作（占屏 < 10%） |
| `on_accent` | #3C3C3C | #201A08 | 橙底文字 |
| `hp_good` | #7CC243 | 同浅色 | 状态条：良好 |
| `hp_warn` | #F5D33D | 同浅色 | 状态条：警告 |
| `hp_bad` | #E5483E | 同浅色 | 状态条：危险 |

## Radius / Spacing / Shadow / Typography / Motion

| 组 | 取值 | 说明 |
|---|---|---|
| Radius | 4 / 8 / 12 / 16 / 24 / 999(px) | 小控件 / 输入框 / 卡片 / 面板 / 大面板 / 全圆 |
| Spacing | 4 / 8 / 12 / 16 / 24 / 32(px) | 4px 基数栅格 |
| Shadow | sm(1,3,0.12) / md(4,12,0.16) / lg(8,24,0.20) | 偏移 y / 模糊 / 不透明度，仅纵向 |
| Typography | 12 / 14 / 16 / 20 / 28 / 40(px) | 标注 / 正文 / 输入 / 小标题 / 页标题 / 展示数字 |
| Motion | 100 / 180 / 260 / 380ms；stagger 40ms、最多 8 项 | 反馈 / 切换 / 面板 / 启动序列 |
| Easing | feedback=EaseOut；panel=EaseInOut；EaseOutBack 过冲 ≤ 4%（仅位移 / 缩放） | 附录 B |

## 渐变

`Gradient { angle_deg, stops: Vec<GradientStop { offset, color }> }`，用于玻璃面板的轻微纵向渐变；幅度保持克制，避免大面积浅色渐变出现色阶分层。

## TOML 与热加载

- `Tokens::from_toml_str` / `to_toml_string` 支持 TOML 往返；
- Gallery（debug 构建）启动时在 `assets/theme/light.toml`、`assets/theme/dark.toml` 自动生成内置样例；
- debug 下 `notify` 监听文件变化，120ms 防抖后发出 `Message::ThemeFileChanged` 即时生效；
- release 构建不监听文件（`cfg(debug_assertions)`）。

## 单元测试

`orb-tokens` 内置 5 项测试：附录 B 初始值、圆角档位、动效时长、Rgba 转换、浅深色 TOML 往返。改 Token 后先跑 `cargo test -p orb-tokens`。
