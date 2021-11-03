use crate::communication::{Command, Message};

/// `Component` is the foundation for all interactivity within the `Editor`. You can view it as the
/// model in elm architecture.
pub trait Component {
    fn update(&mut self, msg: Message) -> Option<Command>;
}

/// `Window` is the default root component for the `Editor`.
pub struct Window;

impl Component for Window {
    fn update(&mut self, msg: Message) -> Option<Command> {
        println!("{:?}", msg);

        Some(|| Message::Quit)
    }
}
