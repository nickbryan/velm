use crate::communication::{Command, Message};
use crate::render::{Frame, View};
use crate::Color;

/// `Component` is the foundation for all interactivity within the `Editor`. You can view it as the
/// model in elm architecture.
pub trait Component {
    fn update(&mut self, msg: Message) -> Option<Command>;
}

/// `Window` is the default root component for the `Editor`.
pub struct Window;

impl Component for Window {
    fn update(&mut self, _msg: Message) -> Option<Command> {
        None
    }
}

impl View for Window {
    fn render_to(&self, frame: &mut Frame) {
        frame.write_line(0, "abcdefg123", Color::White, Color::Black);
    }
}
