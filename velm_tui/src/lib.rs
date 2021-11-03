use anyhow::Result;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    style::{Color as CrosstermColor, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Error as IoError, Write};
use velm_core::{Canvas, Cell, Color as VelmColor, Event, EventStream, Key as VelmKey, Rect};

/// Map the events coming from the crossterm EventStream into the events that are expected by the application.
pub fn map_crossterm_event_stream() -> EventStream {
    use futures::StreamExt;

    Box::pin(crossterm::event::EventStream::new().map(|possible_event| {
        use crossterm::event as ctevent;

        match possible_event {
            Ok(ctevent::Event::Key(key)) => Event::KeyPressed(Key::from(key).0),
            Ok(ctevent::Event::Mouse(_)) => Event::MouseInputReceived,
            Ok(ctevent::Event::Resize(x, y)) => Event::WindowResized(x, y),
            Err(e) => Event::ReadFailed(e),
        }
    }))
}

/// Newtype to allow mapping VelmColor to CrosstermColor.
struct Color(VelmColor);

/// Newtype to allow mapping crossterm::event::KeyEvent to VelmKey.
struct Key(VelmKey);

/// Canvas implementation for crossterm.
pub struct CrosstermCanvas<W: Write> {
    out: W,
}

impl<W: Write> CrosstermCanvas<W> {
    /// Creates a new CrosstermCanvas.
    pub fn new(mut out: W) -> Result<Self, IoError> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(out, EnterAlternateScreen)?;

        Ok(Self { out })
    }
}

impl<W: Write> Drop for CrosstermCanvas<W> {
    /// Ensures that we LeaveAlternateScreen and disable_raw_mode before the application ends to
    /// return the user terminal back to normal.
    fn drop(&mut self) {
        crossterm::execute!(self.out, LeaveAlternateScreen)
            .expect("unable to leave alternate screen");
        crossterm::terminal::disable_raw_mode().expect("unable to disable raw mode");
    }
}

impl<W: Write> Canvas for CrosstermCanvas<W> {
    fn clear(&mut self) -> Result<(), IoError> {
        crossterm::queue!(self.out, Clear(ClearType::All))
    }

    fn draw<'a, I>(&mut self, cells: I) -> Result<(), IoError>
    where
        I: Iterator<Item = &'a Cell>,
    {
        let mut prev_background = Color(VelmColor::Reset);
        let mut prev_foreground = Color(VelmColor::Reset);

        for cell in cells {
            self.position_cursor(cell.position().row, cell.position().col)?;

            if cell.background() != prev_background.0 {
                crossterm::queue!(
                    self.out,
                    SetBackgroundColor(CrosstermColor::from(Color(cell.background())))
                )?;

                prev_background = Color(cell.background());
            }

            if cell.foreground() != prev_foreground.0 {
                crossterm::queue!(
                    self.out,
                    SetForegroundColor(CrosstermColor::from(Color(cell.foreground())))
                )?;

                prev_foreground = Color(cell.foreground());
            }

            crossterm::queue!(self.out, Print(cell.symbol()))?;
        }

        crossterm::queue!(
            self.out,
            SetBackgroundColor(CrosstermColor::from(Color(VelmColor::Reset))),
            SetForegroundColor(CrosstermColor::from(Color(VelmColor::Reset))),
        )
    }

    fn flush(&mut self) -> Result<(), IoError> {
        self.out.flush()
    }

    fn hide_cursor(&mut self) -> Result<(), IoError> {
        crossterm::queue!(self.out, Hide)
    }

    fn position_cursor(&mut self, row: usize, col: usize) -> Result<(), IoError> {
        use std::convert::TryFrom;

        let x =
            u16::try_from(col).map_err(|e| IoError::new(io::ErrorKind::Other, format!("{}", e)))?;
        let y =
            u16::try_from(row).map_err(|e| IoError::new(io::ErrorKind::Other, format!("{}", e)))?;

        crossterm::queue!(self.out, MoveTo(x, y))
    }

    fn show_cursor(&mut self) -> Result<(), IoError> {
        crossterm::queue!(self.out, Show)
    }

    fn size(&self) -> Result<Rect, IoError> {
        let (width, height) = crossterm::terminal::size()?;
        Ok(Rect::new(usize::from(width), usize::from(height)))
    }
}

impl From<Color> for CrosstermColor {
    fn from(color: Color) -> Self {
        match color.0 {
            VelmColor::Reset => CrosstermColor::Reset,
            VelmColor::Black => CrosstermColor::Black,
            VelmColor::Red => CrosstermColor::DarkRed,
            VelmColor::Green => CrosstermColor::DarkGreen,
            VelmColor::Yellow => CrosstermColor::DarkYellow,
            VelmColor::Blue => CrosstermColor::DarkBlue,
            VelmColor::Magenta => CrosstermColor::DarkMagenta,
            VelmColor::Cyan => CrosstermColor::DarkCyan,
            VelmColor::Gray => CrosstermColor::Grey,
            VelmColor::DarkGray => CrosstermColor::DarkGrey,
            VelmColor::LightRed => CrosstermColor::Red,
            VelmColor::LightGreen => CrosstermColor::Green,
            VelmColor::LightBlue => CrosstermColor::Blue,
            VelmColor::LightYellow => CrosstermColor::Yellow,
            VelmColor::LightMagenta => CrosstermColor::Magenta,
            VelmColor::LightCyan => CrosstermColor::Cyan,
            VelmColor::White => CrosstermColor::White,
            VelmColor::AnsiValue(v) => CrosstermColor::AnsiValue(v),
            VelmColor::Rgb(r, g, b) => CrosstermColor::Rgb { r, g, b },
        }
    }
}

impl From<crossterm::event::KeyEvent> for Key {
    fn from(event: crossterm::event::KeyEvent) -> Self {
        use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

        match event {
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Enter,
            } => Key(VelmKey::Enter),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Tab,
            } => Key(VelmKey::Tab),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Backspace,
            } => Key(VelmKey::Backspace),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Esc,
            } => Key(VelmKey::Esc),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Left,
            } => Key(VelmKey::Left),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Right,
            } => Key(VelmKey::Right),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Down,
            } => Key(VelmKey::Down),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Up,
            } => Key(VelmKey::Up),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Insert,
            } => Key(VelmKey::Insert),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Delete,
            } => Key(VelmKey::Delete),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Home,
            } => Key(VelmKey::Home),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::End,
            } => Key(VelmKey::End),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::PageUp,
            } => Key(VelmKey::PageUp),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::PageDown,
            } => Key(VelmKey::PageDown),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Char(ch),
            } => Key(VelmKey::Char(ch)),
            KeyEvent {
                modifiers: KeyModifiers::CONTROL,
                code: KeyCode::Char(ch),
            } => Key(VelmKey::Ctrl(ch)),
            _ => Key(VelmKey::Unknown),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CrosstermCanvas;

    #[test]
    fn crossterm_backend_enters_and_leaves_alternate_screen() {
        let mut out: Vec<u8> = Vec::new();

        let backend = CrosstermCanvas::new(&mut out);
        drop(backend);

        assert_eq!(
            "\u{1b}[?1049h\u{1b}[?1049l",
            String::from_utf8(out).unwrap()
        );
    }
}
