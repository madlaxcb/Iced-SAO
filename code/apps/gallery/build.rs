//! 构建脚本：注入 Git 提交号环境变量。
use std::process::Command;

/// 注入 Git 短提交号：窗口标题与 About 页显示，用于 W 关卡的版本一致性校验
/// （计划书 3.6：窗口内显示提交号，确认"验证的就是这个版本"）。
fn main() {
    let sha = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "uncommitted".to_string());
    println!("cargo:rustc-env=GIT_SHA={sha}");
}
