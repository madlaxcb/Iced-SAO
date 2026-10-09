# Iced-SAO

基于 [iced](https://github.com/iced-rs/iced) 0.14 的「SAO 风格」UI 组件库，目标平台 Windows 10 22H2 / 11。在 Linux 上开发（MSVC 交叉编译），视觉行为以真实 Windows 验证机为准。

[English](README.md)

## 特性

- **Token 驱动主题** — 颜色 / 圆角 / 间距 / 阴影 / 动效时长全部在 `orb-tokens`，支持 TOML 热加载；组件禁止硬编码字面值
- **G1 伪毛玻璃**面板，每个主题自带不透明降级模式
- **确定性动画** — tween / sequence / stagger 由可注入的手动时钟驱动，完全可单测
- **SAO 招牌组件** — 圆形菜单导轨、带分档阈值的 HP 条、循环滚动、圆环
- **断网可复现构建** — vendor 依赖 + `cargo xtask` 自动化（`ci` / `win-pack` / `dist` / `doctor` / `vendor`）

## Workspace 结构

```
code/
├─ crates/  orb-tokens → orb-core → orb-theme → orb-widgets（旁支 orb-icons / orb-platform / orb-testkit）
├─ apps/    gallery（组件陈列馆）、sample-launcher（演示应用）
└─ xtask/   本地构建自动化
```

依赖方向严格单向：`tokens → core → theme → widgets`。

## 快速开始

```sh
# 断网构建（依赖 vendor 在本地，不入库）
cd code
cargo build --offline --locked
cargo run --offline -p gallery

# 全量门禁：fmt + 双目标 clippy + 测试 + Windows release 构建
cargo xtask ci
```

Linux 宿主交叉构建 Windows 版本：

```sh
cargo xwin build --offline --release --target x86_64-pc-windows-msvc -p gallery
```

## 文档

- `doc/00-总览.md` — 项目总览与当前状态
- `doc/design/` — Token 规范、组件 API 目录、Windows 验证清单
- `doc/guides/` — 如何新增组件 / 新主题
- `doc/adr/` — 架构决策记录（001–006）

## 状态

M0–M6 里程碑全部完成；分发打包（M7）由 `cargo xtask dist` 自动化。Windows 验证关卡 W2–W5 在专用验证机上执行，未验证项一律显式标注「未验证」。

预构建的 Windows x64 二进制发布在 [Releases](https://github.com/madlaxcb/Iced-SAO/releases) 页面，附 SHA-256 校验和与第三方许可说明。
