use crate::{
    component::Component,
    render::View,
    ui::{Position, Rect},
};

pub struct Buffer {
    cursor_position: Position,
    focused: bool,
    offset: Position,
    viewport: Rect,
}

impl Buffer {
    pub fn new(viewport: Rect) -> Self {
        Self {
            cursor_position: Position::default(),
            focused: false,
            offset: Position::default(),
            viewport,
        }
    }

    pub fn cursor_position(&self) -> Position {
        self.cursor_position
    }
}

impl Component for Buffer {
    fn update(
        &mut self,
        msg: crate::communication::Message,
    ) -> Option<crate::communication::Command> {
        None
    }
}

impl View for Buffer {
    fn render_to(&self, frame: &mut crate::render::Frame) {
        if self.focused {
            frame.set_cursor_position(self.cursor_position);
        }
    }
}
