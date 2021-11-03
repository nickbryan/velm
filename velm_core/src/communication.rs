/// `Message` is an enum that captures all messages that the `Editor` and its `Component`s
/// understand.
#[derive(Debug)]
pub enum Message {
    InsertChar(char),
    Quit,
}

/// `Command` is a type alias for a closure that returns a message. This allows for more powerful
/// `Message` computation such as timers. `Command`s will be executed asynchronously by the `Editor`.
pub type Command = fn() -> Message;
