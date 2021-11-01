use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::time::Duration;
use tokio_stream::Stream;
use velm_core::{Event, Key as VelmKey};

/// Newtype to allow mapping crossterm::event::KeyEvent to VelmKey.
struct Key(VelmKey);

/// EventStream implementation for Crossterm.
pub struct CrosstermEventStream {
    tick_rate: Duration,
}

impl CrosstermEventStream {
    /// Creates a new CrosstermEventStream.
    pub fn new(tick_rate: Duration) -> Self {
        Self { tick_rate }
    }
}

impl Stream for CrosstermEventStream {
    type Item = Event;

    fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        use crossterm::event as ctevent;

        match ctevent::poll(self.tick_rate) {
            Ok(true) => match ctevent::read() {
                Ok(ctevent::Event::Key(key)) => Poll::Ready(Some(Event::Input(Key::from(key).0))),
                Err(e) => Poll::Ready(Some(Event::Error(e))),
                Ok(ctevent::Event::Mouse(_)) | Ok(ctevent::Event::Resize(_, _)) => Poll::Pending,
            },
            Ok(false) => Poll::Ready(Some(Event::Tick)),
            Err(e) => Poll::Ready(Some(Event::Error(e))),
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
