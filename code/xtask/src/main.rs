//! xtask：本地自动化统一入口（计划书 3.3）。
//!
//! 用法：`cargo xtask <子命令>`（需先 `source dev/env.sh`）
//!
//! | 子命令 | 作用 |
//! |---|---|
//! | doctor | 环境盘点 → `doc/test-reports/env-xtask.md` |
//! | vendor | 依赖落到 `dev/vendor/`（需缓存或联网窗口） |
//! | ci | 本地门禁：fmt → clippy(Linux+Windows 目标) → test → check-layout → win-build |
//! | check-layout | 目录规则检查（code/ 无构建物、dist/ 白名单、命名规则） |
//! | win-check | Windows 目标 `cargo check --workspace`（cfg(windows) 代码必须被编译） |
//! | win-build | 交叉构建 Windows release |
//! | win-pack | 组装 `dist/windows-verify/`（exe + manifest + 清单） |
//! | dist | 组装 `dist/windows-x64/`（release 分发包 + 校验和） |
//! | win-sync | 按 `dev/win-verify.toml` 推送验证包（未配置则提示） |
//! | win-collect | 收回验证结果到 `doc/test-reports/win-verify/<日期>/` |

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const MSVC: &str = "x86_64-pc-windows-msvc";

fn main() -> ExitCode {
    let arg = std::env::args().nth(1);
    match arg.as_deref() {
        Some("doctor") => {
            cmd_doctor();
            ExitCode::SUCCESS
        }
        Some("vendor") => run("cargo", &["vendor", "../dev/vendor"]),
        Some("ci") => cmd_ci(),
        Some("check-layout") => cmd_check_layout(),
        Some("win-check") => run(
            "cargo",
            &[
                "check",
                "--offline",
                "--workspace",
                "--all-targets",
                "--target",
                MSVC,
            ],
        ),
        Some("win-build") => run(
            "cargo",
            &[
                "xwin",
                "build",
                "--offline",
                "--release",
                "--workspace",
                "--target",
                MSVC,
            ],
        ),
        Some("win-pack") => {
            cmd_win_pack();
            ExitCode::SUCCESS
        }
        Some("dist") => {
            cmd_dist();
            ExitCode::SUCCESS
        }
        Some("win-sync") => {
            cmd_win_sync();
            ExitCode::SUCCESS
        }
        Some("win-collect") => {
            cmd_win_collect();
            ExitCode::SUCCESS
        }
        Some("fonts") => cmd_fonts(),
        _ => {
            eprintln!("usage: cargo xtask <doctor|vendor|ci|check-layout|win-check|win-build|win-pack|dist|win-sync|win-collect|fonts>");
            ExitCode::from(2)
        }
    }
}

// ---------- 路径与进程辅助 ----------

/// 仓库根（<root>/code/xtask → <root>）。
fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("xtask lives in <root>/code/xtask")
        .to_path_buf()
}

fn code() -> PathBuf {
    repo().join("code")
}

/// 运行命令（cwd = code/），失败即退出非零（ci 链路语义）。
fn run(cmd: &str, args: &[&str]) -> ExitCode {
    println!("$ {cmd} {}", args.join(" "));
    let status = Command::new(cmd)
        .args(args)
        .current_dir(code())
        .status()
        .unwrap_or_else(|e| panic!("failed to spawn {cmd}: {e}"));
    if status.success() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(status.code().unwrap_or(1) as u8)
    }
}

fn try_out(cmd: &str, args: &[&str]) -> Option<String> {
    Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
}

fn git_sha() -> String {
    try_out("git", &["rev-parse", "--short", "HEAD"]).unwrap_or_else(|| "uncommitted".into())
}

/// 发布版本：取最近 tag 名去掉 v 前缀（无 tag 时回退 0.1.0）。
fn release_version() -> String {
    try_out("git", &["describe", "--tags", "--abbrev=0"])
        .map(|t| t.trim_start_matches('v').to_string())
        .unwrap_or_else(|| "0.1.0".into())
}

// ---------- doctor ----------

fn cmd_doctor() {
    let mut md = String::from("# 环境盘点（xtask doctor）\n\n");
    md += &format!("- 生成时间：{}\n", now());
    md += &format!("- git commit：{}\n\n", git_sha());
    md += "| 检查项 | 结果 |\n|---|---|\n";

    let rows: Vec<(String, String)> = vec![
        (
            "rustc".into(),
            try_out("rustc", &["-V"]).unwrap_or_else(|| "MISSING".into()),
        ),
        (
            "cargo".into(),
            try_out("cargo", &["-V"]).unwrap_or_else(|| "MISSING".into()),
        ),
        (
            "windows target".into(),
            match try_out("rustup", &["target", "list", "--installed"]) {
                Some(s) if s.contains(MSVC) => format!("installed ({MSVC})"),
                Some(_) => "MISSING".into(),
                None => "rustup unavailable".into(),
            },
        ),
        (
            "clang".into(),
            try_out("clang", &["--version"])
                .map(|s| s.lines().next().unwrap_or("").to_string())
                .unwrap_or_else(|| "MISSING (仅 C 依赖需要)".into()),
        ),
        (
            "dev/vendor".into(),
            match fs::read_dir(dev().join("vendor")) {
                Ok(entries) => format!("{} crates", entries.count()),
                Err(_) => "MISSING (run: cargo xtask vendor)".into(),
            },
        ),
        (
            "dev/xwin-cache".into(),
            if dev().join("xwin-cache").exists() {
                "present".into()
            } else {
                "MISSING (首次 xwin 构建时自动下载)".into()
            },
        ),
        (
            "dev/fonts-src".into(),
            match fs::read_dir(dev().join("fonts-src")) {
                Ok(entries) => {
                    let names: Vec<String> = entries
                        .filter_map(|e| e.ok())
                        .map(|e| e.file_name().to_string_lossy().to_string())
                        .collect();
                    names.join(", ")
                }
                Err(_) => "MISSING".into(),
            },
        ),
        (
            "dev/win-verify.toml".into(),
            if dev().join("win-verify.toml").exists() {
                "present (字段待补全则 win-sync 走手工通道)".into()
            } else {
                "MISSING".into()
            },
        ),
        (
            "dist/windows-verify/probe.exe".into(),
            if repo().join("dist/windows-verify/probe.exe").exists() {
                "present".into()
            } else {
                "not packed yet".into()
            },
        ),
        (
            "XWIN_CACHE_DIR env".into(),
            std::env::var("XWIN_CACHE_DIR").unwrap_or_else(|_| "unset (source dev/env.sh)".into()),
        ),
    ];

    for (k, v) in &rows {
        md += &format!("| {k} | {v} |\n");
        println!("{k:24} {v}");
    }

    let report = repo().join("doc/test-reports").join("env-xtask.md");
    fs::create_dir_all(report.parent().unwrap()).expect("create doc dir");
    fs::write(&report, md).expect("write env report");
    println!("\nreport -> {}", report.display());
}

fn dev() -> PathBuf {
    repo().join("dev")
}

fn now() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or_default();
    format!("epoch {secs}")
}

// ---------- ci ----------

fn step(name: &str, ok: bool) -> Option<ExitCode> {
    println!("\n=== ci: {name} {} ===", if ok { "ok" } else { "FAILED" });
    (!ok).then_some(ExitCode::FAILURE)
}

fn cmd_ci() -> ExitCode {
    // 1. fmt
    let fmt = Command::new("cargo")
        .args(["fmt", "--all", "--check"])
        .current_dir(code())
        .status()
        .expect("spawn cargo fmt");
    if let Some(code) = step("fmt --check", fmt.success()) {
        eprintln!("hint: cargo fmt --all");
        return code;
    }

    // 2. clippy（Linux 目标）
    let clippy_host = Command::new("cargo")
        .args([
            "clippy",
            "--offline",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ])
        .current_dir(code())
        .status()
        .expect("spawn clippy host");
    if let Some(code) = step("clippy (host)", clippy_host.success()) {
        return code;
    }

    // 3. clippy（Windows 目标，cfg(windows) 代码必须被编译检查）
    let clippy_win = Command::new("cargo")
        .args([
            "clippy",
            "--offline",
            "--workspace",
            "--all-targets",
            "--target",
            MSVC,
            "--",
            "-D",
            "warnings",
        ])
        .current_dir(code())
        .status()
        .expect("spawn clippy windows");
    if let Some(code) = step("clippy (windows-msvc)", clippy_win.success()) {
        return code;
    }

    // 4. test
    let test = Command::new("cargo")
        .args(["test", "--offline", "--workspace"])
        .current_dir(code())
        .status()
        .expect("spawn test");
    if let Some(code) = step("test", test.success()) {
        return code;
    }

    // 5. 目录规则
    if let Some(code) = step("check-layout", run_capture_ok("check-layout")) {
        return code;
    }

    // 6. Windows release 交叉构建
    let build = Command::new("cargo")
        .args([
            "xwin",
            "build",
            "--offline",
            "--release",
            "--workspace",
            "--target",
            MSVC,
        ])
        .current_dir(code())
        .status()
        .expect("spawn xwin build");
    if let Some(code) = step("win-build (release)", build.success()) {
        return code;
    }

    println!("\nci: ALL GREEN (commit {})", git_sha());
    ExitCode::SUCCESS
}

fn run_capture_ok(_name: &str) -> bool {
    // check-layout 的进程内调用：直接执行函数并捕获结果
    layout_ok()
}

fn layout_ok() -> bool {
    let mut errors = Vec::new();
    let code_dir = code();

    // code/ 禁止：构建产物与第三方二进制
    let deny_ext = ["exe", "dll", "pdb", "lib", "msi"];
    visit(&code_dir, &mut |path, name| {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if deny_ext.contains(&ext) {
                errors.push(format!("build artifact in code/: {}", path.display()));
            }
        }
        if path.is_dir() && name == "target" {
            errors.push(format!("target dir in code/: {}", path.display()));
        }
        // 命名规则：小写、数字、连字符、下划线、点（计划书 3.1）；
        // 豁免 Rust 生态固定名
        if name != "Cargo.toml"
            && name != "Cargo.lock"
            && name
                .chars()
                .any(|c| c.is_ascii_uppercase() || " :*?\"<>|".contains(c))
        {
            errors.push(format!("naming violation: {}", path.display()));
        }
    });

    // dist/ 白名单
    let dist = repo().join("dist");
    let allow = [
        "windows-verify/probe.exe",
        "windows-verify/gallery.exe",
        "windows-verify/sample-launcher.exe",
        "windows-verify/manifest.json",
        "windows-verify/README.txt",
        "windows-verify/win-verify-checklist.md",
        "windows-x64/gallery.exe",
        "windows-x64/sample-launcher.exe",
        "windows-x64/manifest.json",
        "windows-x64/README.txt",
        "windows-x64/SHA256SUMS.txt",
        "windows-x64/THIRD_PARTY_LICENSES.txt",
        "windows-x64/VIRUS-SCAN.md",
    ];
    visit(&dist, &mut |path, _name| {
        if path.is_file() {
            let rel = path
                .strip_prefix(&dist)
                .unwrap_or(path)
                .to_string_lossy()
                .replace('\\', "/");
            // 版本化发布 zip（iced-sao-vX.Y.Z-windows-x64.zip）
            let is_release_zip = rel.starts_with("iced-sao-v") && rel.ends_with("-windows-x64.zip");
            let is_release_checksum = rel.starts_with("SHA256SUMS-v") && rel.ends_with(".txt");
            if !allow.contains(&rel.as_str()) && !is_release_zip && !is_release_checksum {
                errors.push(format!("unexpected file in dist/: {rel}"));
            }
        }
    });

    // 微软 SDK/CRT 不入库不分发（R18）：典型文件名检查
    for marker in ["kernel32.lib", "ConsoleGameEngine", "msvcrt.lib"] {
        if code_dir.join(marker).exists() || dist.join(marker).exists() {
            errors.push(format!("MS SDK file leaked: {marker}"));
        }
    }

    if errors.is_empty() {
        true
    } else {
        for e in &errors {
            eprintln!("layout: {e}");
        }
        false
    }
}

fn visit(dir: &Path, f: &mut impl FnMut(&Path, &str)) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') {
            continue;
        }
        if path.is_dir() {
            // 跳过构建缓存（正常情况下 target 都指向 dev/，此处兜底）
            if name == "target" {
                f(&path, &name);
                continue;
            }
            visit(&path, f);
        }
        f(&path, &name);
    }
}

fn cmd_check_layout() -> ExitCode {
    if layout_ok() {
        println!("check-layout: OK");
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

// ---------- win-pack ----------

fn cmd_win_pack() {
    let out_dir = repo().join("dist/windows-verify");
    fs::create_dir_all(&out_dir).expect("create dist/windows-verify");

    let src = dev().join("target").join(MSVC).join("release");
    for exe in ["gallery.exe", "sample-launcher.exe"] {
        let from = src.join(exe);
        if from.exists() {
            fs::copy(&from, out_dir.join(exe)).expect("copy exe");
            println!("packed: {exe}");
        }
    }

    let sha = git_sha();
    let manifest = format!(
        "{{\n  \"package\": \"orb-windows-verify\",\n  \"git_commit\": \"{sha}\",\n  \"target\": \"{MSVC}\",\n  \"profile\": \"release\",\n  \"rustc\": \"{}\",\n  \"built_at_epoch\": {},\n  \"contents\": [\"gallery.exe\", \"sample-launcher.exe\"]\n}}\n",
        try_out("rustc", &["-V"]).unwrap_or_default(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or_default(),
    );
    fs::write(out_dir.join("manifest.json"), manifest).expect("write manifest");

    let doc = "win-verify-checklist.md";
    let from = repo().join("doc/design").join(doc);
    if from.exists() {
        fs::copy(&from, out_dir.join(doc)).expect("copy checklist");
    }
    println!("win-pack done (commit {sha}) -> {}", out_dir.display());
}

// ---------- dist ----------

fn cmd_dist() {
    let out_dir = repo().join("dist/windows-x64");
    fs::create_dir_all(&out_dir).expect("create dist/windows-x64");
    let src = dev().join("target").join(MSVC).join("release");
    let mut contents = Vec::new();
    for exe in ["gallery.exe", "sample-launcher.exe"] {
        let from = src.join(exe);
        if from.exists() {
            fs::copy(&from, out_dir.join(exe)).expect("copy release executable");
            contents.push(exe);
            println!("dist: packed {exe}");
        }
    }
    assert!(
        !contents.is_empty(),
        "no Windows release executables found; run cargo xtask win-build first"
    );

    let licenses = "Third-party license source: cargo metadata --format-version 1\n\nWorkspace crates use the licenses declared in code/Cargo.toml and deny.toml.\nFor the complete dependency license report, run cargo deny check licenses.\n";
    fs::write(out_dir.join("THIRD_PARTY_LICENSES.txt"), licenses).expect("write licenses");

    let mut sums = String::new();
    for name in &contents {
        let path = out_dir.join(name);
        let hash = try_out("sha256sum", &[path.to_string_lossy().as_ref()])
            .and_then(|line| line.split_whitespace().next().map(str::to_owned))
            .expect("sha256sum is required for dist");
        sums.push_str(&format!("{hash}  {name}\n"));
    }
    fs::write(out_dir.join("SHA256SUMS.txt"), sums).expect("write checksums");

    let sha = git_sha();
    let version = release_version();
    let manifest = format!(
        "{{\n  \"package\": \"orb-windows-x64\",\n  \"version\": \"{version}\",\n  \"git_commit\": \"{sha}\",\n  \"target\": \"{MSVC}\",\n  \"profile\": \"release\",\n  \"multi_monitor\": \"not verified by request\",\n  \"contents\": [{}]\n}}\n",
        contents.iter().map(|name| format!("\"{name}\"")).collect::<Vec<_>>().join(", ")
    );
    fs::write(out_dir.join("manifest.json"), manifest).expect("write dist manifest");
    fs::write(
        out_dir.join("README.txt"),
        format!(
            "orb Windows x64 release {version}\n\nDemos: sample-launcher.exe (showcase app), gallery.exe (component gallery).\nMulti-monitor behavior is intentionally not verified in this release.\nVerify with: sha256sum -c SHA256SUMS.txt\nSee manifest.json and THIRD_PARTY_LICENSES.txt.\n"
        ),
    )
    .expect("write dist readme");
    println!("dist done -> {}", out_dir.display());
}

// ---------- fonts（P9：字体子集化）----------

/// 字符集生成脚本：GB2312 一级常用字 + 常用中文标点 + ASCII 可打印 + code/ 源码实际用字。
const SUBSET_PY: &str = r#"
import glob, sys

chars = set()
for hi in range(0xB0, 0xD8):
    for lo in range(0xA1, 0xFF):
        try:
            chars.add(bytes([hi, lo]).decode('gb2312'))
        except UnicodeDecodeError:
            pass
chars.update('，。、；：？！""\u2018\u2019（）《》【】…—·～％＋－＊／＝％°℃¥')
chars.update(chr(c) for c in range(0x20, 0x7F))
for path in glob.glob('code/**/*.rs', recursive=True):
    with open(path, encoding='utf-8') as f:
        for ch in f.read():
            if ord(ch) > 0x7F and not ch.isspace():
                chars.add(ch)
chars.discard('\n'); chars.discard('\r'); chars.discard('\t')
with open(sys.argv[1], 'w', encoding='utf-8') as f:
    f.write(''.join(sorted(chars)))
print('subset charset:', len(chars), 'chars')
"#;

/// 子集化 Noto Sans CJK SC（ttc face 2）两个字重到 code/assets/fonts/，预算合计 ≤ 6MB。
fn cmd_fonts() -> ExitCode {
    let chars_file = std::env::temp_dir().join("orb-subset-chars.txt");
    let chars_arg = chars_file.to_string_lossy().into_owned();
    let py = run("python3", &["-c", SUBSET_PY, &chars_arg]);
    if py != ExitCode::SUCCESS {
        return py;
    }

    let fonts_src = repo().join("dev/fonts-src");
    let fonts_out = code().join("assets/fonts");
    fs::create_dir_all(&fonts_out).expect("create assets/fonts");
    let mut total: u64 = 0;
    for (src, out) in [
        ("NotoSansCJK-Regular.ttc", "noto-sans-sc-regular.ttf"),
        ("NotoSansCJK-Bold.ttc", "noto-sans-sc-bold.ttf"),
    ] {
        let src = fonts_src.join(src);
        let out = fonts_out.join(out);
        let status = run(
            "pyftsubset",
            &[
                &src.to_string_lossy(),
                "--font-number=2",
                &format!("--text-file={chars_arg}"),
                &format!("--output-file={}", out.display()),
                "--layout-features=",
                "--name-IDs=*",
                "--name-legacy",
                "--desubroutinize",
            ],
        );
        if status != ExitCode::SUCCESS {
            return status;
        }
        total += fs::metadata(&out).expect("subset output").len();
    }
    let budget: u64 = 6 * 1024 * 1024;
    println!(
        "fonts: total {} bytes (budget {} bytes, {})",
        total,
        budget,
        if total <= budget { "OK" } else { "OVER BUDGET" }
    );
    if total > budget {
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

// ---------- win-sync / win-collect ----------

#[derive(serde::Deserialize, Default)]
struct WinVerify {
    #[serde(default)]
    transfer: Transfer,
}

#[derive(serde::Deserialize, Default)]
struct Transfer {
    #[serde(default)]
    linux_cmd: String,
    #[serde(default)]
    collect_cmd: String,
}

fn read_transfer() -> WinVerify {
    let path = dev().join("win-verify.toml");
    fs::read_to_string(path)
        .map(|s| toml::from_str(&s).unwrap_or_default())
        .unwrap_or_default()
}

fn cmd_win_sync() {
    let cfg = read_transfer();
    if cfg.transfer.linux_cmd.trim().is_empty() {
        println!(
            "win-sync: dev/win-verify.toml 未配置 transfer.linux_cmd。\n\
             手工通道：把 {} 整个目录拷到验证机即可（zip / 共享目录 / U 盘）。",
            repo().join("dist/windows-verify").display()
        );
        return;
    }
    let status = Command::new("sh")
        .args(["-c", cfg.transfer.linux_cmd.trim()])
        .status()
        .expect("run sync cmd");
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

fn cmd_win_collect() {
    let cfg = read_transfer();
    if cfg.transfer.collect_cmd.trim().is_empty() {
        let today = now_date();
        println!(
            "win-collect: dev/win-verify.toml 未配置 transfer.collect_cmd。\n\
             手工通道：把验证机上的截图与清单放入 doc/test-reports/win-verify/{today}/"
        );
        return;
    }
    let status = Command::new("sh")
        .args(["-c", cfg.transfer.collect_cmd.trim()])
        .status()
        .expect("run collect cmd");
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

fn now_date() -> String {
    // 本地时区日期近似（UTC+8 由使用者校正；正式实现 M1 内部用 chrono 前先手算）
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or_default();
    let days = secs / 86_400;
    // 1970-01-01 起的天数转日期（简化算法，够用于目录名）
    let (y, m, d) = civil_from_days(days as i64);
    format!("{y:04}-{m:02}-{d:02}")
}

/// Howard Hinnant 的 civil_from_days（公历天数 → 年月日）。
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}
