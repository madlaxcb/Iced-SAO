# ADR-005：渲染后端策略

- 状态：**已接受**（2026-10-08，W0 判定）
- 关联：计划书 2.1、2.6、M0-P6/P9、R8

## 背景

iced 0.14 双后端：wgpu（DX12 / Vulkan）与 tiny-skia（CPU 软件渲染）。需决定：主后端、软件回退是否保留、快照后端、已知缺陷的规避。

## W0 事实（两平台验证）

| 事实 | 来源 |
|---|---|
| `iced_tiny_skia` 0.14.1 多画布（缓存 + 非缓存混合 6 块）渲染**全部正常** | Linux headless 快照（probe-snapshot-*.png）；0.14.0 的裁剪缺陷确认已修 |
| **wgpu 下共享单 Cache 的多画布只渲染第一个**（iced issue #3040 复现） | Windows canvas 模式截图（4 块共享 Cache 仅 1 圆） |
| wgpu 下**独立 Cache** 多画布全部正常 | Windows canvas 模式复验截图 |
| 透明窗口在 wgpu + DX12 可用（需全局 style 背景 = TRANSPARENT） | ADR-003 |
| G2 亚克力与 wgpu DX12 组合黑块 + 拖动严重迟滞 | ADR-004 |
| headless 渲染仅 tiny-skia 可用（`Renderer::new(.., Some("tiny-skia"))`） | 上一项目 + probe snapshot |

## 决策

1. **主后端：wgpu**（Windows 上 DX12，Linux 开发宿主上 Vulkan/lavapipe）——产品渲染路径。
2. **tiny-skia 软件渲染保留**，两个用途：
   - **快照测试**（P9）：内嵌字体 + tiny-skia headless，基线在 Linux 生成、Windows 复核（P9 结论待两平台 PNG 比对后定稿）；
   - 无 GPU / GPU 初始化失败时的运行时回退（iced 内建 fallback）。
3. **门禁**：`Cargo.lock` 检查 `iced_tiny_skia >= 0.14.1`（0.14.0 多画布裁剪缺陷）；升级 iced 走独立分支并重跑多画布快照。
4. **组件规范（来自 #3040）**：每个 `canvas::Canvas` widget 持有**独立 `canvas::Cache`**，禁止多 widget 共享同一 Cache；写进组件 DoD 与 code review 检查项。
5. 性能验收只采 Windows + wgpu 数据（计划书 3.6）；tiny-skia 数字仅作回退路径参考。

## 影响

- 视觉回归基线以 tiny-skia 快照为准（M7 建），GPU 渲染抽样人工比对；
- orb-testkit 的快照设施固定 tiny-skia headless（probe 已验证的写法）。
