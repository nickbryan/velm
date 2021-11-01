use velm_core::{Event, EventStream, Key as VelmKey};

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

/// Newtype to allow mapping crossterm::event::KeyEvent to VelmKey.
struct Key(VelmKey);

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
