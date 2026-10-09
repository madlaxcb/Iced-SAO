//! 主题文件加载与 debug 热更新（计划书 2.4：只在 debug 与 Gallery）。
//!
//! 文件布局：`assets/theme/{light,dark}.toml`（相对运行目录；debug 启动时自动导出内置默认）。

#[cfg(debug_assertions)]
use crate::Message;
use crate::ThemeVariant;
use orb_theme::OrbTheme;
#[cfg(debug_assertions)]
use orb_tokens::Tokens;
use std::path::PathBuf;

fn theme_dir() -> PathBuf {
    PathBuf::from("assets/theme")
}

fn file_name(variant: ThemeVariant) -> &'static str {
    match variant {
        ThemeVariant::Light => "light.toml",
        ThemeVariant::Dark => "dark.toml",
    }
}

fn builtin(variant: ThemeVariant) -> OrbTheme {
    match variant {
        ThemeVariant::Light => OrbTheme::light(),
        ThemeVariant::Dark => OrbTheme::dark(),
    }
}

fn orb_variant(v: ThemeVariant) -> orb_theme::Variant {
    match v {
        ThemeVariant::Light => orb_theme::Variant::Light,
        ThemeVariant::Dark => orb_theme::Variant::Dark,
    }
}

/// 加载主题：文件存在且合法 → 用文件；否则回退内置默认。
pub fn load_or_default(variant: ThemeVariant) -> OrbTheme {
    let path = theme_dir().join(file_name(variant));
    match std::fs::read_to_string(&path) {
        Ok(s) => OrbTheme::from_toml_str(orb_variant(variant), &s).unwrap_or_else(|err| {
            eprintln!("theme file parse failed ({path:?}): {err}; using built-in");
            builtin(variant)
        }),
        Err(_) => builtin(variant),
    }
}

/// debug 启动时导出内置默认 TOML（供用户修改后观察热更新）。
#[cfg(debug_assertions)]
pub fn ensure_samples() {
    let _ = std::fs::create_dir_all(theme_dir());
    for (variant, tokens) in [
        (ThemeVariant::Light, Tokens::light()),
        (ThemeVariant::Dark, Tokens::dark()),
    ] {
        let path = theme_dir().join(file_name(variant));
        if !path.exists() {
            let _ = std::fs::write(&path, tokens.to_toml_string());
        }
    }
}

/// notify 文件监听 → 空消息流（std 线程 + futures mpsc 桥接，上一项目已验证模式）。
#[cfg(debug_assertions)]
pub fn theme_stream() -> impl iced::futures::Stream<Item = Message> {
    use iced::futures::channel::mpsc;
    use notify::Watcher;

    let (mut tx, rx) = mpsc::channel::<Message>(16);
    std::thread::spawn(move || {
        let (evt_tx, evt_rx) = std::sync::mpsc::channel::<()>();
        let watcher =
            notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
                if res.is_ok() {
                    let _ = evt_tx.send(());
                }
            });
        let mut watcher = match watcher {
            Ok(w) => w,
            Err(_) => return,
        };
        if watcher
            .watch(&theme_dir(), notify::RecursiveMode::NonRecursive)
            .is_err()
        {
            return;
        }
        while evt_rx.recv().is_ok() {
            // 防抖：编辑器常一次写多事件，简单延时合并
            std::thread::sleep(std::time::Duration::from_millis(120));
            while evt_rx.try_recv().is_ok() {}
            if tx.try_send(Message::ThemeFileChanged).is_err() {
                return;
            }
        }
    });
    rx
}
