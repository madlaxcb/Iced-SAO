# Windows 验证清单（W0：M0 预研包）

- 验证包：`probe.exe`（多模式）+ 本清单
- 产物提交号：见 `manifest.json`
- 记录方式：每项在"结果"列填 `通过 / 失败 / 未测`，失败附现象描述与截图文件名
- 截图命名：`<模式>-<序号>.png`（如 `transparent-1.png`），与清单同目录

## 运行前提

1. 把验证包解压到任意目录（建议路径不含中文空格）。
2. 双击 `probe.exe` 打开 glass 模式；窗口顶部有一排模式按钮，
   点击即打开对应模式的新窗口（transparent / acrylic 打开后按 Esc 退出）。
3. 若双击报"缺少 VCRUNTIME140.dll"：安装 VC++ Redistributable（x64）后在清单记录"缺 VC 运行库"（这本身是 W0 的有效结论，影响 ADR-006 的 CRT 静态链接决策）。

## 逐项验证

| # | 模式 | 命令 | 验证内容 | 通过标准 | 结果 |
|---|---|---|---|---|---|
| 1 | glass | `probe glass` | 20 个玻璃面板的渐变/圆角/描边/阴影；点按钮切深浅背景 | 面板呈半透明玻璃质感，两种背景下文字可读 | |
| 2 | transparent | `probe transparent` | 无边框透明窗口；1.5s 后整窗穿透 | 窗口四周透明区能看到桌面；穿透生效后鼠标点击空白处落到桌面（如选中桌面图标）；状态行显示 `click-through enabled` | |
| 3 | acrylic | `probe acrylic` | 整窗亚克力（G2） | 窗口对背后桌面内容呈现模糊；状态行显示 `acrylic applied`；记录是否卡顿 | |
| 4 | ime | `probe ime` | 中文输入法 | 微软拼音等可输入中文，候选窗位置正确，上屏内容与显示一致 | |
| 5 | dpi | `probe dpi` | 线条与缩放 | 在系统 100% / 125% / 150% / 200% 下分别运行：1px 线始终 1 物理像素清晰、无发虚；圆角无变形 | |
| 6 | canvas | `probe canvas` | 同屏 6 画布（wgpu 后端） | 上排 4 块 + 下排 2 块全部显示橙圆，无黑块/缺失/错位 | |
| 7 | snapshot | `probe snapshot` | 软件渲染离屏快照 | 当前目录生成 `probe-snapshot-light.png` 与 `probe-snapshot-dark.png`，内容与 Linux 基线对比（由开发机完成比对） | |

## 附加观察（可选但有价值）

- [ ] transparent 模式：任务栏是否出现图标；Alt+Tab 表现；"显示桌面"时窗口是否被隐藏
- [ ] transparent/acrylic 模式：拖动窗口是否正常（无边框拖动未实现，应不可拖动——记录即可）
- [ ] acrylic 模式：窗口拖动/缩放时的卡顿程度（计划书引用的已知性能问题）
- [ ] glass 模式：浅色背景下面板可读性主观评价（1~5 分）
- [ ] 全部模式：启动速度体感、是否有闪屏/黑块
- [ ] dpi 模式：每档缩放截图各一张

## 收尾

1. 把截图与填好的本清单放回一个目录，交回开发机（`doc/test-reports/win-verify/<日期>/`）。
2. 若状态行出现 `PLATFORM_FAIL: ...`，原样抄录。
