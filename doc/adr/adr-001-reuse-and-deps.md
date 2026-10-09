# ADR-001：复用与依赖策略

- 状态：**已接受**（2026-10-07，M0.0 期间实施）
- 关联：计划书 2.3、3.2、附录 A R11/R12

## 背景

本机已有上一项目（鹰角风格，`/home/Iced-Arknights`）：同一技术栈（Rust + iced 0.14）、相近的 workspace 结构、已验证的依赖组合。需要决定复用方式与依赖落地策略。

## 决策

### 复用方式：**「拷贝后改名」，不用 path 依赖**

已实际复用并验证的部分：

| 复用物 | 来源 | 本项目落地 |
|---|---|---|
| workspace 配置模式（profiles / lints / 精确锁版本） | `Iced-Arknights/code/Cargo.toml` | `code/Cargo.toml`（orb-* 命名） |
| iced 0.14 feature 组合与子 crate 锁版本 | 同上 | 同上（含 `iced_futures =0.14.0` 的原因注释） |
| 主题方案 B 结构（Theme + Catalog + TOML） | `hud-theme` | ADR-002，M2 实现 orb-theme 时照此结构 |
| headless 快照渲染写法（Headless trait + UserInterface + screenshot + PNG） | `apps/gallery/src/snapshot.rs` | `apps/probe/src/snapshot.rs` |
| 目录规范（dev/code/doc/dist）与 env 脚本模式 | 上一项目 | 本项目（Linux shell 版） |

理由：两个项目独立演进（hud-* vs orb-*、暗色 vs 浅色、Windows 交叉 vs 本机 Windows），path 依赖会把两仓库锁死在一起；拷贝后按新项目规范演化，成本一次性。

不复用的部分：hud-* 的组件实现（风格体系完全不同）、其脚本（本项目改用 bash + xtask）。

### 依赖落地：**cargo vendor 全量落 `dev/vendor/` + 离线配置**

1. `cargo vendor` → `dev/vendor/`（560 crate，772 MB，**已验证含 Windows 目标全部依赖**：windows 全系、wgpu、window-vibrancy）；
2. `code/.cargo/config.toml`：vendor 源替换 + `offline = true` + `target-dir = ../dev/target`；
3. `Cargo.lock` 入库，作为依赖权威清单；
4. 缺失 crate（palette / image-compare / tracing-subscriber / criterion 等）已在"联网补齐窗口"内补齐；
5. **备份义务**：`dev/vendor`（772 MB）与 `dev/xwin-cache`（1.1 GB）丢失即无法重建，`xtask backup-dev`（M1 实现）前先手工备份。

### 工具链复用

- 全局 rustup（stable 1.97.1）直接复用，`rust-toolchain.toml` 锁 `channel = "stable"`；
- Windows 目标标准库已装（msvc/gnu 双 target）；
- 音频 crate 按用户决策**永不引入**（音效不做）。

## 影响

- 断网构建已验证（M0.0，`--offline --locked`）；
- 上一项目脚本依赖 PowerShell 的风险不存在（其为 bash/rust）；
- 复用清单随实现推进在 `doc/` 持续登记，避免"拷来没看"的死代码。
