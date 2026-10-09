# ADR-006：交叉编译与 Windows 验证流程（MSVC + cargo-xwin）

- 状态：**已接受**（2026-10-07，M0.0 / W0 期间验证）
- 关联：计划书 3.2 / 3.6 / 3.7、R14～R18

## 背景

开发宿主是 Linux（Debian 12），目标平台是 Windows 10 22H2 / 11。需要决定：
1. Windows 目标用 MSVC 还是 GNU 工具链；
2. 交叉编译缓存如何落地与复用；
3. 验证流程如何保证"验证的就是这个版本"。

## 决策

1. **正式目标：`x86_64-pc-windows-msvc`，经 `cargo-xwin` 从 Linux 交叉编译。**
   依据：MSVC ABI 与计划一致、有 PDB 符号、wgpu/DX12 路径为主流组合；
   `cargo-xwin 0.23.0` 已在本机验证可用。
2. **微软 CRT / Windows SDK 缓存落 `dev/xwin-cache/`**（环境变量 `XWIN_CACHE_DIR`，
   由 `dev/env.sh` 设置）。当前 1.1 GB。不入库、不进 `dist/`、不随产物分发；
   与 `dev/vendor/` 同列 `xtask backup-dev` 备份范围。
3. **降级方案**：`x86_64-pc-windows-gnu` + mingw-w64（本机已具备）。
   仅当 xwin 路线不可用时启用，须记录 ABI / 符号差异。
4. **CRT 链接方式**：默认动态链接 VC 运行库。是否改静态链接，
   待 W0 验证机反馈"是否缺 VCRUNTIME140.dll"后决定（本 ADR 届时补充结论）。
5. **验证流程**：`win-pack → win-sync → 人工按清单验证 → win-collect`；
   程序窗口标题与 `manifest.json` 携带版本信息，验证记录存
   `doc/test-reports/win-verify/<日期>/`。Windows 行为结论只以验证机为准（计划书 3.6）。

## 已验证事实

| 事实 | 验证记录 |
|---|---|
| Linux 断网原生构建（vendor + offline） | `doc/test-reports/env-2026-10-07.md` §3 |
| `cargo xwin build` 产出可运行 exe | gallery.exe 已在用户 Windows 环境打开（2026-10-07 用户确认） |
| xwin 首次构建自动下载 CRT/SDK 到 `XWIN_CACHE_DIR` | `dev/xwin-cache/` 1.1 GB |
| `cfg(windows)` 代码必须交叉检查才暴露类型错误 | orb-platform 4 处类型错误仅 Windows 目标 check 可见（2026-10-07） |
| Linux 原生运行需显示服务 | 本机无 X11/Wayland，用 xvfb 替代；真实视觉调试受限 |

## 影响

- 每次提交须通过双目标检查（Linux + windows-msvc 的 check/clippy）；
- `FindWindowW` 按标题定位 HWND 是 M0 原型的临时通道，M1 换 raw-window-handle 正式通道后此 ADR 追加记录；
- 干净 Windows 环境的冒烟（W6）前必须先解决 CRT 依赖问题（装运行库或静态链接）。
