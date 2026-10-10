use iced::Task;
use orb_core::WindowCommand;

pub fn apply(command: WindowCommand) -> Task<super::Message> {
    iced::window::latest().then(move |id| orb_core::window::task(command, id))
}
