# 如何写一个新组件

> 目标读者：首次接触本仓库的开发者。步骤按 TDD，全部命令可在断网的 Linux 开发机上执行。

## 前置阅读

- `doc/design/tokens.md`（哪些值能用）
- `doc/design/components.md`（已有组件，避免重复造）
- `doc/adr/adr-005-renderer.md`（每个 Canvas widget 必须持有**独立** `canvas::Cache`）

## 步骤

### 1. 先写失败测试

放在 `code/crates/orb-widgets/src/lib.rs` 的 `mod tests`。测**纯逻辑**：
稳定标识（`xxx_label` 返回 `&'static str`）、分档 / 索引 / 状态机等。样例参考
`hp_bar_band_thresholds_are_deterministic`。

```sh
cargo test --offline -p orb-widgets   # 确认编译失败（RED）
```

### 2. 最小实现

- 组件函数签名统一：`pub fn xxx<'a, Message>(...) -> iced 组件类型`，Theme 泛型为 `OrbTheme`；
- 样式走 `orb_theme::style_*` 或闭包内取 `theme.tokens.*`；
- **禁止**出现魔法数字颜色 / 尺寸；圆角、间距、时长取 Token；
- 需要新的样式函数时加在 `code/crates/orb-theme/src/styles.rs`。

```sh
cargo test --offline -p orb-widgets   # 转绿（GREEN）
```

### 3. 接入展示

- Gallery 对应页面加预览（`code/apps/gallery/src/`）；
- Sample Launcher 视场景接入（`code/apps/sample-launcher/src/main.rs`）。

### 4. 门禁

```sh
cargo fmt --all
cargo check --offline -p <改动包>
cargo check --offline --target x86_64-pc-windows-msvc -p <改动包>
cargo xtask ci          # fmt + 双目标 clippy + test + check-layout + win-build
```

### 5. Windows 行为确认

涉及绘制 / 命中 / 动画的新组件，在下次 Windows 验证关卡按
`doc/design/win-verify-checklist.md` 走查；未验证前在文档标注"未验证"。

## 规则速查

| 规则 | 原因 |
|---|---|
| 每个 Canvas 独立 Cache | 共享 Cache 会导致多画布只绘制一个（W0 实测） |
| `update` 不做阻塞操作 | 卡帧；定时器用 Subscription Tick |
| 命中区域显式实现 | 圆形按钮按圆形命中（计划书 5.1） |
| 依赖只允许 widgets → theme → core → tokens | 禁止反向依赖 |
