use crossterm::event::KeyEvent;

/// Key bindings.
///
/// Centralise all key bindings here so they're easy to find and change.
/// The App handles these — rendering code never checks keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Up,
    Down,
    Tab,
    Quit,
    Other,
}

impl From<KeyEvent> for Key {
    fn from(key_event: KeyEvent) -> Self {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('q') | KeyCode::Esc => Key::Quit,
            KeyCode::Up | KeyCode::Char('k') => Key::Up,
            KeyCode::Down | KeyCode::Char('j') => Key::Down,
            KeyCode::Tab => Key::Tab,
            _ => Key::Other,
        }
    }
}
