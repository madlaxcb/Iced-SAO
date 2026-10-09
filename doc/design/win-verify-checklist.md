# Windows 验证清单（当前：W2～W5）

- 对应产物：**v0.3.0**（gallery.exe + sample-launcher.exe，内嵌中文字体；提交号以发布产物为准）
- 产物来源：GitHub Releases zip，或 `cargo xtask win-pack` → `dist/windows-verify/`
- 记录方式：每项在"结果"列填 `通过 / 失败 / 未测`，失败附现象描述与截图文件名
- 截图命名：`w<关卡>-<序号>.png`（如 `w2-1.png`），与清单同目录
- W0 预研清单已完成（2026-10-07/08），历史版见文末附录

## 运行前提

1. 解压到任意目录（建议路径不含中文与空格）。
2. 打开 gallery.exe：窗口标题、侧栏、About 页三处提交号应一致且等于 `6b5cd5f`，与 manifest.json 一致（W1 检查项）。
3. **内嵌字体**：界面中文应为 Noto Sans SC 观感（清晰、字重分明）。若出现方块、豆腐或与默认系统字体（微软雅黑）明显不同的观感，记 `失败`——这是 P9 的验证点。
4. 双击报"缺少 VCRUNTIME140.dll"：安装 VC++ Redistributable（x64）并记录（影响 ADR-006）。

## W2：字体渲染与主题（gallery · Tokens 页）

| # | 操作 | 通过标准 | 结果 |
|---|---|---|---|
| 1 | 查看色板 / 渐变 / 阴影 / 圆角 / 字号阶梯各区 | 与 `doc/design/tokens.md` 数值一致，无缺色、无错档 | |
| 2 | 观察中文 / 西文 / 数字混排 | 同一内嵌字体渲染，基线对齐，无字体回退混杂观感 | |
| 3 | 对比标题（Bold）与正文（Regular） | 字重区分明显 | |
| 4 | 切换深色 / 浅色主题（侧栏按钮） | 两套主题文字与背景对比度均可读 | |
| 5 | 系统缩放 100% / 125% / 150% 各运行一次 | 文字清晰不虚、边线 1 物理像素、布局无错位；每档截图 | |

## W3：视觉原语与动画（gallery · Primitives 页）

| # | 操作 | 通过标准 | 结果 |
|---|---|---|---|
| 1 | 查看玻璃面板（渐变 / 描边 / 阴影 / 伪毛玻璃 G1） | 半透明玻璃质感，深浅背景下文字可读 | |
| 2 | CircleButton / Ring 展示 | 圆形无变形；Ring 进度与数值一致 | |
| 3 | 触发入场 / 交互动画 | 流畅、无跳帧；"减少动画"开启后动画跳过或简化 | |
| 4 | 空闲 1 分钟后看任务管理器 CPU | 记录占用数值（预期 <1%，作为性能基线） | |

## W4：通用组件与输入（gallery · Components / Overlays 页）

| # | 操作 | 通过标准 | 结果 |
|---|---|---|---|
| 1 | 逐个走查 Button / TextInput / Checkbox / Radio / Switch / Slider / ScrollArea 各状态 | hover / active / disabled / focused 视觉与交互正确；输入与拖动的值**保持不回弹** | |
| 2 | Tooltip / ContextMenu / Modal / Toast 逐一触发 | 位置正确、层级正确、Esc / 点击外部可关闭 Modal 与菜单 | |
| 3 | TextInput 中用微软拼音输入中文 | 候选窗位置正确、上屏一致（W0 ime 结论的组件级复核） | |
| 4 | Tab 键在组件间移动焦点 | 焦点环可见、顺序合理 | |
| 5 | List 三项点击切换选中 | 选中项强调描边随点击移动 | |
| 6 | Tabs（Status / Equipment / Skills）点击切换 | 选中项橙色底，切换正确 | |
| 7 | Badge 五档观感 | Neutral / Accent / Good / Warn / Bad 颜色语义正确 | |
| 8 | Loading 旋转 | 连续旋转约 2s / 圈，无卡顿、无闪烁 | |
| 9 | Table 三行点击选中 | 选中行强调描边随点击移动，空单元格显示 `—` | |
| 10 | RadialMenu 点击扇区 | 外环四个扇区选择正确，中心空洞点击不改变选中项 | |
| 11 | Avatar 中英文首字符 | `Kirito` 显示 `K`，`桐人` 显示 `桐`，圆形布局不变形 | |
| 12 | Glow 降级与视觉 | accent 柔光可见；Opaque 开启后内容与布局保持可读 | |
| 13 | TitleBar 按钮 | 标题、最小化、关闭按钮可见；Gallery 演示按钮不关闭应用 | |

## W5：招牌组件、动效与降级（sample-launcher + gallery）

| # | 操作 | 通过标准 | 结果 |
|---|---|---|---|
| 1 | sample-launcher Launcher 页：点击 MenuRail 图标切换面板 | 选中态橙色指示、右侧面板标题随之切换 | |
| 2 | 键盘 ↑↓（或 ←→）操作 MenuRail | 选中项环形移动并联动面板（无选中时按 ↑ 落到最后一项） | |
| 3 | LoopScroll 区域滚动 | 循环顺序正确（选中标记 ◉ 依 index 移动） | |
| 4 | Status 页 HpBar / Ring / Readout | 分档颜色正确（≥0.5 绿 / ≥0.2 黄 / <0.2 红），读数与色一致 | |
| 5 | Settings 页 Toast 触发 | 滑入、停留、自动消失时序符合 Motion Token（100/180/260/380ms） | |
| 6 | gallery 侧栏 Opaque 按钮（降级开关） | 开启后半透明材质变不透明，布局与内容**不**变化 | |
| 7 | 启动序列动画（打开 sample-launcher） | Enter→Brand→Content 顺序入场，无闪烁 | |

## W6（预置：M7 最终冒烟）

干净 Windows 环境运行 `dist/windows-x64/` 全量走查 + 性能数据。在 W2～W5 通过后细化执行项。

## 收尾

1. 截图与填好的清单放回一个目录，交回开发机 `doc/test-reports/win-verify/<日期>/`。
2. 若出现 `PLATFORM_FAIL: ...` 字样，原样抄录。
3. 未执行项一律保留 `未测`，不得留空。

---

# 附录：W0 预研清单（已完成，2026-10-07/08）

- 验证包：`probe.exe`（多模式）

| # | 模式 | 验证内容 | 结果 |
|---|---|---|---|
| 1 | glass | 20 个玻璃面板的渐变/圆角/描边/阴影 | 通过 |
| 2 | transparent | 无边框透明窗口 + 1.5s 后穿透 | 通过 |
| 3 | acrylic | 整窗亚克力（G2） | 通过 |
| 4 | ime | 中文输入法 | 通过 |
| 5 | dpi | 100/125/150/200% 缩放下 1px 线条 | 通过 |
| 6 | canvas | 同屏 6 画布（wgpu） | 通过 |
| 7 | snapshot | 软件渲染离屏快照 | 通过 |

详细记录：`doc/test-reports/win-verify/2026-10-07-w0.md`。
