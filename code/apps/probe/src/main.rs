#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

//! M0 预研程序（W0）：多模式验证 Windows 通路与视觉/渲染假设（计划书 M0.1～M0.9）。
//!
//! 用法：`probe <mode>`
//!
//! | mode | 验证项 | 计划书原型 |
//! |---|---|---|
//! | glass | 玻璃质感（渐变/阴影/描边 ×20 面板，深浅背景切换） | P1 |
//! | transparent | 透明 + 无边框 + 置顶 + 整窗穿透（1.5s 后生效） | P2 |
//! | acrylic | 整窗亚克力（window-vibrancy，G2） | P3 |
//! | ime | 文本输入（中文输入法候选窗） | P5 |
//! | dpi | 1px/2px 线条 + 圆角 + 字号样例（100%~200% 缩放目视） | P8 |
//! | canvas | 同屏 6 个 canvas（4 缓存 + 2 非缓存，多画布渲染） | P6 |
//! | snapshot | tiny-skia headless 离屏快照 → PNG（多画布 + 玻璃） | P6/P9 |

use iced::theme::Base as _;
use iced::widget::{canvas, column, container, row, text, text_input};
use iced::{Color, Element, Size, Subscription, Task, Theme};
use std::time::Duration;

mod snapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Glass,
    Acrylic,
    Ime,
    Dpi,
    Canvas,
    Anim,
}

impl Mode {
    fn parse(arg: &str) -> Option<Self> {
        match arg {
            "glass" => Some(Self::Glass),
            "acrylic" => Some(Self::Acrylic),
            "ime" => Some(Self::Ime),
            "dpi" => Some(Self::Dpi),
            "canvas" => Some(Self::Canvas),
            "anim" => Some(Self::Anim),
            _ => None,
        }
    }

    fn arg_name(self) -> &'static str {
        match self {
            Self::Glass => "glass",
            Self::Acrylic => "acrylic",
            Self::Ime => "ime",
            Self::Dpi => "dpi",
            Self::Canvas => "canvas",
            Self::Anim => "anim",
        }
    }

    /// 按钮标签（含验证项编号）。
    fn label(self) -> String {
        self.arg_name().to_string()
    }

    /// 窗口标题：唯一化，供 orb-platform 按标题定位 HWND。
    fn title(self) -> String {
        format!("orb-probe-{}", format!("{self:?}").to_lowercase())
    }

    fn hint(self) -> &'static str {
        match self {
            Self::Glass => "P1 玻璃质感：点按钮切换深浅背景；观察 20 面板渐变/阴影/描边",
            Self::Acrylic => "P3 毛玻璃（G2 亚克力）：普通窗口，背景应对桌面模糊；若为纯黑即 G2 在本机不可用",
            Self::Ime => "P5 输入法：点击输入框，切换中文输入法输入，观察候选窗位置与上屏",
            Self::Dpi => "P8 缩放：在系统 100%/125%/150%/200% 下分别运行，检查 1px 线与圆角是否清晰",
            Self::Canvas => "P6 多画布：上排 4 块独立缓存画布 + 下排 2 块每帧重画画布，应全部显示橙圆",
            Self::Anim => "P4 动画：点按钮展开/收起——图标栏展开 + 面板滑入 + 6 点 40ms 错峰；动画结束后应完全静止",
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    BgToggled,
    ImeInput(String),
    Tick,
    Launch(Mode),
    /// 生成快照 PNG（后台子进程，headless 渲染，不弹窗）。
    RunSnapshot,
    /// 按 Esc：退出进程。
    Exit,
    /// P4：展开/收起切换。
    AnimToggle,
    /// P4：帧时钟（window::frames 订阅，动画进行时才有）。
    AnimTick(std::time::Instant),
    Ignored,
}

/// P4 动画状态：rail 展开、面板滑入、6 点错峰（计划书 2.5 的 Tween+Stagger 雏形）。
struct AnimState {
    rail: iced::animation::Animation<f32>,
    slide: iced::animation::Animation<f32>,
    stagger: Vec<iced::animation::Animation<f32>>,
    now: std::time::Instant,
}

impl Default for AnimState {
    fn default() -> Self {
        Self {
            rail: iced::animation::Animation::new(0.0)
                .easing(iced::animation::Easing::EaseInOut)
                .duration(Duration::from_millis(260)),
            slide: iced::animation::Animation::new(0.0)
                .easing(iced::animation::Easing::EaseOut)
                .duration(Duration::from_millis(260)),
            stagger: (0..6)
                .map(|_| {
                    iced::animation::Animation::new(0.0)
                        .easing(iced::animation::Easing::EaseOut)
                        .duration(Duration::from_millis(180))
                })
                .collect(),
            now: std::time::Instant::now(),
        }
    }
}

impl AnimState {
    fn toggle(&mut self) {
        let now = std::time::Instant::now();
        self.now = now;
        let target = if self.rail.value() > 0.5 { 0.0 } else { 1.0 };
        self.rail.go_mut(target, now);
        self.slide.go_mut(target, now);
        for (i, anim) in self.stagger.iter_mut().enumerate() {
            anim.go_mut(target, now + Duration::from_millis(i as u64 * 40));
        }
    }

    fn is_animating(&self) -> bool {
        self.rail.is_animating(self.now)
            || self.slide.is_animating(self.now)
            || self.stagger.iter().any(|a| a.is_animating(self.now))
    }
}

const ORANGE: Color = Color::from_rgba(0.96, 0.65, 0.14, 1.0);

struct Probe {
    mode: Mode,
    dark_bg: bool,
    ime_value: String,
    status: String,
    platform_done: bool,
    /// 4 个独立 Cache 的画布（W0 结论：共享单 Cache 的多画布在 wgpu 下只绘制第一个，
    /// 即计划书引用的 iced issue #3040；独立 Cache 是规范写法，待 Windows 复核）
    canvas_arts: Vec<CanvasArt>,
    anim: AnimState,
}

#[derive(Default)]
struct CanvasArt {
    cache: canvas::Cache,
}

impl canvas::Program<Message> for CanvasArt {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let frame = self.cache.draw(renderer, bounds.size(), |frame| {
            frame.fill(&canvas::Path::circle(frame.center(), 40.0), ORANGE);
        });
        vec![frame]
    }
}

struct HotCanvas;

impl canvas::Program<Message> for HotCanvas {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        frame.fill(&canvas::Path::circle(frame.center(), 40.0), ORANGE);
        vec![frame.into_geometry()]
    }
}

fn main() -> iced::Result {
    // GUI 子系统无控制台：任何错误都要落到 exe 旁的 probe.log，否则表现为"没反应"
    std::panic::set_hook(Box::new(|info| {
        append_log(format!("PANIC: {info}"));
    }));

    let arg = std::env::args().nth(1).unwrap_or_default();
    if arg == "snapshot" {
        return run_logged(snapshot::run);
    }
    // 双击运行无参数：默认 glass，保证"点开就有窗口"
    let mode = Mode::parse(&arg).unwrap_or(Mode::Glass);
    if mode != Mode::Glass || !arg.is_empty() {
        append_log(format!("start mode={mode:?} arg={arg:?}"));
    }
    run_logged(move || {
        // 毛玻璃（G2 亚克力）需要窗口 transparent + 全局背景 TRANSPARENT 露出 DWM 模糊层；
        // 其余模式用主题默认背景。注意：不做无边框 / 置顶 / 点击穿透（用户决策 2026-10-08）
        iced::application(move || Probe::new(mode), Probe::update, Probe::view)
            .title(move |_: &Probe| mode.title())
            .window(window_settings(mode))
            .subscription(Probe::subscription)
            .style(move |_: &Probe, theme| {
                if mode == Mode::Acrylic {
                    iced::theme::Style {
                        background_color: Color::TRANSPARENT,
                        text_color: Color::from_rgb(0.1, 0.1, 0.1),
                    }
                } else {
                    iced::theme::Style {
                        background_color: theme.base().background_color,
                        text_color: theme.base().text_color,
                    }
                }
            })
            .run()
    })
}

fn run_logged(f: impl FnOnce() -> iced::Result) -> iced::Result {
    let result = f();
    if let Err(err) = &result {
        append_log(format!("ERROR: {err:?}"));
    }
    result
}

fn append_log(message: String) {
    let Ok(exe) = std::env::current_exe() else {
        return;
    };
    let path = exe.with_file_name("probe.log");
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or_default();
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        use std::io::Write;
        let _ = writeln!(file, "[{timestamp}] {message}");
    }
}

fn window_settings(mode: Mode) -> iced::window::Settings {
    let base = iced::window::Settings {
        size: Size::new(760.0, 520.0),
        ..iced::window::Settings::default()
    };
    match mode {
        // 毛玻璃：保留系统边框、不置顶、不穿透（用户决策 2026-10-08）；
        // 仅 transparent = true 露出 DWM 模糊层
        Mode::Acrylic => iced::window::Settings {
            transparent: true,
            ..base
        },
        _ => base,
    }
}

impl Probe {
    fn new(mode: Mode) -> Self {
        Self {
            mode,
            dark_bg: false,
            ime_value: String::new(),
            status: String::from("started"),
            platform_done: false,
            canvas_arts: (0..4).map(|_| CanvasArt::default()).collect(),
            anim: AnimState::default(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::BgToggled => self.dark_bg = !self.dark_bg,
            Message::ImeInput(value) => self.ime_value = value,
            // 启动器：拉起自身 exe 带模式参数（各模式需要不同的窗口设置，进程级切换最可靠）
            Message::Launch(mode) => {
                if let Ok(exe) = std::env::current_exe() {
                    let _ = std::process::Command::new(exe).arg(mode.arg_name()).spawn();
                }
            }
            Message::RunSnapshot => {
                if let Ok(exe) = std::env::current_exe() {
                    let _ = std::process::Command::new(exe).arg("snapshot").spawn();
                }
                self.status = String::from(
                    "snapshot: rendering in background -> probe-snapshot-light/dark.png (next to probe.exe)",
                );
            }
            Message::Exit => {
                std::process::exit(0);
            }
            Message::Ignored => {}
            Message::AnimToggle => self.anim.toggle(),
            Message::AnimTick(now) => self.anim.now = now,
            Message::Tick => {
                // 毛玻璃：窗口就绪后按标题定位 HWND，应用亚克力（同步调用，微秒级）
                if !self.platform_done {
                    self.platform_done = true;
                    let title = self.mode.title();
                    let result = match self.mode {
                        Mode::Acrylic => {
                            orb_platform::apply_acrylic_by_title(&title, [240, 244, 248, 120])
                                .map(|()| String::from("acrylic applied"))
                        }
                        _ => Ok(String::new()),
                    };
                    self.status = match result {
                        Ok(msg) => msg,
                        Err(err) => format!("PLATFORM_FAIL: {err}"),
                    };
                }
            }
        }
        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        // Esc 退出：transparent 整窗穿透后唯一可靠退出途径
        let esc = iced::keyboard::listen().map(|event| match event {
            iced::keyboard::Event::KeyPressed { key, .. } => {
                if matches!(
                    key,
                    iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape)
                ) {
                    Message::Exit
                } else {
                    Message::Ignored
                }
            }
            _ => Message::Ignored,
        });
        let timer = if self.mode == Mode::Acrylic && !self.platform_done {
            iced::time::every(Duration::from_millis(1500)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        };
        // P4：仅在动画进行时订阅帧流（计划书 2.5「无动画不订阅」）
        let frames = if self.mode == Mode::Anim && self.anim.is_animating() {
            iced::window::frames().map(Message::AnimTick)
        } else {
            Subscription::none()
        };
        Subscription::batch([esc, timer, frames])
    }

    fn view(&self) -> Element<'_, Message> {
        let body: Element<'_, Message> = match self.mode {
            Mode::Glass => self.view_glass(),
            Mode::Acrylic => self.view_transparent(),
            Mode::Ime => self.view_ime(),
            Mode::Dpi => Self::view_dpi(),
            Mode::Canvas => self.view_canvas(),
            Mode::Anim => self.view_anim(),
        };
        // 毛玻璃模式下文字直接浮在模糊层上，加深色底板保证可读
        let needs_plate = self.mode == Mode::Acrylic;
        let hint = Self::plate(text(self.mode.hint()).size(13).into(), needs_plate);
        let status = Self::plate(
            text(format!("status: {}", self.status)).size(12).into(),
            needs_plate,
        );
        let mut root = column![hint];
        root = root.push(Self::mode_launcher());
        root = root.push(body).push(status);
        root.spacing(8).padding(12).into()
    }

    /// 需要时给文字加深色半透明底板（透明窗口在暗壁纸上保证可读）。
    fn plate(inner: Element<'static, Message>, plate: bool) -> Element<'static, Message> {
        if plate {
            container(inner)
                .padding(6)
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(Color::from_rgba(
                        0.08, 0.08, 0.1, 0.72,
                    ))),
                    border: iced::Border {
                        radius: 6.0.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    text_color: Some(Color::from_rgba(0.95, 0.95, 0.95, 1.0)),
                    ..container::Style::default()
                })
                .into()
        } else {
            inner
        }
    }

    /// 模式启动器：拉起对应模式的新窗口（各模式窗口设置不同，进程级切换）。
    fn mode_launcher() -> Element<'static, Message> {
        let buttons: Vec<Element<'static, Message>> = [
            Mode::Glass,
            Mode::Acrylic,
            Mode::Ime,
            Mode::Dpi,
            Mode::Canvas,
            Mode::Anim,
        ]
        .into_iter()
        .map(|mode| {
            iced::widget::button(text(mode.label()).size(13))
                .on_press(Message::Launch(mode))
                .into()
        })
        .chain(std::iter::once(
            iced::widget::button(text("snapshot").size(13))
                .on_press(Message::RunSnapshot)
                .into(),
        ))
        .collect();
        row(buttons).spacing(6).into()
    }

    fn view_glass(&self) -> Element<'_, Message> {
        let panel = |label: String| -> Element<'_, Message> {
            container(text(label).size(14))
                .width(120)
                .height(64)
                .center_x(120)
                .center_y(64)
                .style(|_| glass_style(self.dark_bg))
                .into()
        };
        let mut rows = column![].spacing(10);
        for chunk in (1..=20usize).collect::<Vec<_>>().chunks(5) {
            let cells: Vec<Element<'_, Message>> = chunk
                .iter()
                .map(|i| panel(format!("panel {i:02}")))
                .collect();
            rows = rows.push(row(cells).spacing(10));
        }
        column![
            iced::widget::button(
                text(if self.dark_bg {
                    "bg: dark (click)"
                } else {
                    "bg: light (click)"
                })
                .size(13)
            )
            .on_press(Message::BgToggled),
            rows,
        ]
        .spacing(10)
        .into()
    }

    fn view_transparent(&self) -> Element<'_, Message> {
        // 窗口透明：内容区只放一块半透明玻璃面板，四周留空白让用户看桌面
        container(
            container(
                column![
                    text("GLASS").size(22),
                    text("transparent + borderless + always-on-top").size(12),
                ]
                .spacing(4),
            )
            .padding(18)
            .style(|_| glass_style(false)),
        )
        .width(iced::Fill)
        .height(iced::Fill)
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
    }

    fn view_ime(&self) -> Element<'_, Message> {
        column![
            text("中文输入验证（微软拼音 / 五笔等）").size(15),
            text_input("在此输入中文…", &self.ime_value)
                .on_input(Message::ImeInput)
                .padding(8)
                .size(15),
            text(format!("value: {}", self.ime_value)).size(13),
        ]
        .spacing(10)
        .into()
    }

    fn view_dpi() -> Element<'static, Message> {
        let line = |h: f32| {
            let empty: Element<'static, Message> = column![].into();
            container(empty)
                .width(iced::Fill)
                .height(h)
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(Color::BLACK)),
                    ..container::Style::default()
                })
        };
        column![
            text("1px line below:").size(13),
            line(1.0),
            text("2px line below:").size(13),
            line(2.0),
            text("12px 字号：永远 12px").size(12),
            text("16px 字号：永远 16px").size(16),
            text("24px 字号：永远 24px").size(24),
            container(text("radius 12").size(14))
                .padding(10)
                .style(|_| container::Style {
                    border: iced::Border {
                        radius: 12.0.into(),
                        width: 1.0,
                        color: Color::BLACK,
                    },
                    ..container::Style::default()
                }),
        ]
        .spacing(6)
        .into()
    }

    fn view_canvas(&self) -> Element<'_, Message> {
        let hot = || canvas::Canvas::new(&HotCanvas);
        let cached = |i: usize| canvas::Canvas::new(&self.canvas_arts[i]);
        column![
            text("cached x4 (independent caches):").size(13),
            row![cached(0), cached(1), cached(2), cached(3)]
                .height(120)
                .spacing(8),
            text("hot x2:").size(13),
            row![hot(), hot()].height(120).spacing(8),
        ]
        .spacing(8)
        .into()
    }

    /// P4 动画：图标栏展开 + 面板滑入 + 6 点错峰（40ms 间隔）。
    fn view_anim(&self) -> Element<'_, Message> {
        let now = self.anim.now;
        let rail_open = self.anim.rail.interpolate_with(|v| v, now);
        let slide = self.anim.slide.interpolate_with(|v| v, now);

        let rail_w = 64.0 + rail_open * 156.0;
        let rail = container(
            column![
                container(text("●").size(18))
                    .width(40)
                    .height(40)
                    .center_x(40)
                    .center_y(40),
                container(text("menu").size(14)),
                container(text("panel").size(14)),
                container(text("setup").size(14)),
            ]
            .spacing(10)
            .padding(12),
        )
        .width(rail_w)
        .height(240)
        .clip(true)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(Color::from_rgba(
                1.0, 1.0, 1.0, 0.9,
            ))),
            border: iced::Border {
                radius: 16.0.into(),
                width: 1.0,
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
            },
            ..container::Style::default()
        });

        let panel = container(
            column![
                text("SLIDE-IN PANEL").size(16),
                text("padding-animated (layout cost on purpose)").size(11),
            ]
            .spacing(4),
        )
        .padding(16)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(Color::from_rgba(
                0.96, 0.65, 0.14, 0.9,
            ))),
            border: iced::Border {
                radius: 12.0.into(),
                width: 1.0,
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.6),
            },
            ..container::Style::default()
        });

        let dots: Vec<Element<'_, Message>> = self
            .anim
            .stagger
            .iter()
            .map(|anim| {
                let t = anim.interpolate_with(|v| v, now);
                let r = 6.0 + 18.0 * t;
                container(column![])
                    .width(r * 2.0)
                    .height(r * 2.0)
                    .center_x(r * 2.0)
                    .center_y(r * 2.0)
                    .style(move |_| container::Style {
                        background: Some(iced::Background::Color(Color::from_rgba(
                            0.96,
                            0.65,
                            0.14,
                            0.25 + 0.75 * t,
                        ))),
                        ..container::Style::default()
                    })
                    .width(r * 2.0)
                    .height(r * 2.0)
                    .into()
            })
            .collect();

        let panel_off = (1.0 - slide) * 260.0;
        column![
            iced::widget::button(text("toggle expand").size(13)).on_press(Message::AnimToggle),
            row![rail].height(240),
            container(panel).padding(iced::Padding {
                top: 0.0,
                bottom: 0.0,
                left: panel_off,
                right: 0.0,
            }),
            row(dots).spacing(12).height(56),
        ]
        .spacing(12)
        .into()
    }
}

/// 玻璃面板样式：半透明渐变 + 1px 高光描边 + 柔和阴影（P1 目标观感，M0 原型字面值）。
fn glass_style(dark: bool) -> container::Style {
    let (fill, text_color, stroke) = if dark {
        (
            Color::from_rgba(0.12, 0.14, 0.17, 0.78),
            Color::from_rgba(0.91, 0.92, 0.93, 1.0),
            Color::from_rgba(1.0, 1.0, 1.0, 0.14),
        )
    } else {
        (
            Color::from_rgba(1.0, 1.0, 1.0, 0.82),
            Color::from_rgb(0.24, 0.24, 0.24),
            Color::from_rgba(1.0, 1.0, 1.0, 0.9),
        )
    };
    container::Style {
        text_color: Some(text_color),
        background: Some(iced::Background::Gradient(iced::Gradient::Linear(
            iced::gradient::Linear::new(iced::Degrees(160.0))
                .add_stop(0.0, fill)
                .add_stop(
                    1.0,
                    iced::Color {
                        a: fill.a * 0.75,
                        ..fill
                    },
                ),
        ))),
        border: iced::Border {
            radius: 12.0.into(),
            width: 1.0,
            color: stroke,
        },
        shadow: iced::Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.16),
            offset: iced::Vector::new(0.0, 4.0),
            blur_radius: 12.0,
        },
        snap: false,
    }
}
