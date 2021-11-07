use crate::communication::{Command, Message};
use crate::component::status_bar::StatusBar;
use crate::component::{Component, TextInput};
use crate::mode::Mode;
use crate::render::{Frame, View};
use crate::ui::{Position, Rect};

/// `Window` is the default root component for the `Editor`.
pub struct Window {
    command_prompt: TextInput,
    mode: Mode,
    size: Rect,
}

impl Window {
    pub fn new(size: Rect, mode: Mode) -> Self {
        let mut command_prompt = TextInput::new(
            ":",
            " Press : to enter a command...",
            Position::new(0, size.bottom()),
        );

        if let Mode::Execute(_) = mode {
            command_prompt.focus();
        }

        Self {
            command_prompt,
            mode,
            size,
        }
    }
}

impl Component for Window {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Message::EnterMode(mode) = msg.clone() {
            if let Mode::Execute(_) = mode {
                self.command_prompt.focus();
            } else {
                self.command_prompt.unfocus();
            }

            self.mode = mode;
        }

        if let Mode::Execute(_) = self.mode {
            return self.command_prompt.update(msg);
        }

        None
    }
}

impl View for Window {
    fn render_to(&self, frame: &mut Frame) {
        if let Mode::Normal(_) | Mode::Insert(_) = self.mode {
            frame.set_cursor_position(Position::default());
        }

        StatusBar {
            area: Rect::positioned(self.size.width, 1, self.size.left(), self.size.bottom() - 1),
            mode: self.mode.to_string(),
            line_count: 0,
            cursor_position: frame.cursor_position(),
            file_name: "".to_string(),
        }
        .render_to(frame);

        self.command_prompt.render_to(frame);
    }
}
