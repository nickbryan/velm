use crate::communication::{Command, Message};

mod text_input;
mod window;

use text_input::TextInput;
pub use window::Window;

/// `Component` is the foundation for all interactivity within the `Editor`. You can view it as the
/// model in elm architecture.
pub trait Component {
    fn update(&mut self, msg: Message) -> Option<Command>;
}
