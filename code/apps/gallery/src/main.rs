//! Gallery 入口：全部逻辑在 lib（供 lib 与集成测试共享）。

fn main() -> iced::Result {
    gallery::run()
}
