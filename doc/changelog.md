# Changelog

格式参考 [Keep a Changelog](https://keepachangelog.com/)，版本号遵循 Semver。
每个版本的完整发布说明与校验和见 GitHub Releases。

## [v0.1.1] — 2026-10-09

### Added

- 内嵌子集化 Noto Sans CJK SC（Regular + Bold，两字重合计 1.88MB）：`cargo xtask fonts` 从 `dev/fonts-src/` 生成，字符集 = GB2312 一级常用字 + 常用标点 + 源码实际用字，覆盖源码非 ASCII 字符 100%（P9）
- `orb-theme::fonts`：内嵌字体字节与默认字体句柄；gallery / sample-launcher 设为默认字体
- `doc/licenses/`：Noto CJK 的 SIL OFL 许可证文本
- dist 产物附 `VIRUS-SCAN.md`（ClamAV 扫描报告）

### Changed

- 导航高亮符号 `▸`（U+25B8）改为 `▶`（U+25B6）：原字体不含 U+25B8 字形，此前依赖系统字体回退
- `manifest.json` 与 dist README 的版本号改为从 git tag 动态推导（不再硬编码）

### Fixed

- gallery build.rs 增加 `.git/HEAD` 与 `.git/refs/heads` 的 rerun 依赖：commit 后重新构建即可嵌入新提交号（此前 exe 永远嵌上次构建时的提交号）
- check-layout：dist 白名单补充 `VIRUS-SCAN.md` 与版本化发布 zip

## [v0.1.0] — 2026-10-09

### Added

- 首个 Windows x64 演示发布：gallery（组件陈列馆）+ sample-launcher（四页演示）
- orb 组件库 M0～M6 全部成果：tokens / core（动画与状态机）/ theme / widgets（原语、通用组件、招牌组件）
- `cargo xtask` 自动化：doctor / vendor / ci / check-layout / win-check / win-build / win-pack / dist / win-sync / win-collect
- 文档：总览、ADR 001–006、Token 与组件规范、开发指南（新组件 / 新主题）

### Known limitations

- 多显示器行为有意未验证（用户决策）
- 二进制未签名，杀毒软件可能启发式误报（Release 说明含 VirusTotal 复核提示）

[v0.1.1]: https://github.com/madlaxcb/Iced-SAO/releases/tag/v0.1.1
[v0.1.0]: https://github.com/madlaxcb/Iced-SAO/releases/tag/v0.1.0
