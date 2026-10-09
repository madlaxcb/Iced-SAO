//! snapshot 模式：tiny-skia headless 离屏渲染多画布 + 玻璃面板 → PNG（P6 Linux 半边 / P9 两侧对比素材）。

use crate::{glass_style, CanvasArt, HotCanvas, Message, ORANGE};
use iced::advanced::renderer::{Headless, Style};
use iced::theme::Base;
use iced::widget::{canvas, column, container, row, text};
use iced::{Font, Pixels, Size, Theme};
use iced_futures::backend::default::Executor;
use iced_renderer::Renderer;
use iced_runtime::user_interface::{Cache, UserInterface};

const LOGICAL_SIZE: Size<f32> = Size::new(900.0, 600.0);

/// 构建快照页面：4 缓存画布 + 2 热画布 + 2 玻璃面板 + 橙色样例。
fn page(dark: bool) -> iced::Element<'static, Message> {
    // 'static 借用：headless 构建期间持有；每页泄漏一个 Cache（本模式仅调用 2 次）
    let art: &'static CanvasArt = Box::leak(Box::new(CanvasArt::default()));
    let glass_panel = |label: String| {
        container(text(label).size(14))
            .width(180)
            .height(90)
            .center_x(180)
            .center_y(90)
            .style(move |_| glass_style(dark))
    };
    column![
        text("probe snapshot: tiny-skia headless").size(14),
        row![
            canvas::Canvas::new(art),
            canvas::Canvas::new(art),
            canvas::Canvas::new(art),
            canvas::Canvas::new(art),
        ]
        .height(140)
        .spacing(8),
        row![
            canvas::Canvas::new(&HotCanvas),
            canvas::Canvas::new(&HotCanvas)
        ]
        .height(140)
        .spacing(8),
        row![
            glass_panel(String::from("glass light")),
            glass_panel(String::from("glass dark"))
        ]
        .spacing(8),
        container(text("ORANGE").size(16))
            .padding(8)
            .style(|_| container::Style {
                background: Some(iced::Background::Color(ORANGE)),
                ..container::Style::default()
            }),
    ]
    .spacing(10)
    .padding(12)
    .into()
}

/// 渲染浅色 / 深色两页到当前目录 `probe-snapshot-{light,dark}.png`。
pub fn run() -> iced::Result {
    let executor = Executor::new().map_err(|err| {
        iced::Error::ExecutorCreationFailed(std::io::Error::other(err.to_string()))
    })?;
    executor.block_on(async {
        let mut renderer = Renderer::new(Font::DEFAULT, Pixels(16.0), Some("tiny-skia"))
            .await
            .ok_or_else(|| {
                iced::Error::ExecutorCreationFailed(std::io::Error::other(
                    "tiny-skia headless renderer is unavailable",
                ))
            })?;

        for (name, theme, dark) in [("light", Theme::Light, false), ("dark", Theme::Dark, true)] {
            let mut interface =
                UserInterface::build(page(dark), LOGICAL_SIZE, Cache::new(), &mut renderer);
            interface.draw(
                &mut renderer,
                &theme,
                &Style {
                    text_color: theme.base().text_color,
                },
                iced::mouse::Cursor::Unavailable,
            );
            let physical = Size::new(
                (LOGICAL_SIZE.width * 1.0) as u32,
                (LOGICAL_SIZE.height * 1.0) as u32,
            );
            let pixels = renderer.screenshot(physical, 1.0, theme.base().background_color);
            // 输出到 exe 旁（不能用 CARGO_MANIFEST_DIR——那是交叉编译机的路径，Windows 上不存在）
            let dir = std::env::current_exe()
                .ok()
                .and_then(|exe| exe.parent().map(std::path::Path::to_path_buf))
                .unwrap_or_default();
            let path = dir.join(format!("probe-snapshot-{name}.png"));
            write_png(&path, physical, &pixels)?;
            println!("written: {}", path.display());
        }
        Ok(())
    })
}

fn write_png(path: &std::path::Path, size: Size<u32>, pixels: &[u8]) -> iced::Result {
    let file = std::fs::File::create(path).map_err(io_err)?;
    let mut encoder = png::Encoder::new(file, size.width, size.height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().map_err(enc_err)?;
    writer.write_image_data(pixels).map_err(enc_err)?;
    Ok(())
}

fn io_err(err: std::io::Error) -> iced::Error {
    iced::Error::ExecutorCreationFailed(err)
}

fn enc_err(err: png::EncodingError) -> iced::Error {
    iced::Error::ExecutorCreationFailed(std::io::Error::other(err.to_string()))
}
