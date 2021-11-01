#[derive(Debug)]
pub enum Message {
    InsertChar(char),
    Quit,
}

pub type Command = fn() -> Message;
