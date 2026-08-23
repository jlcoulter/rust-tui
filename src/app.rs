use crate::event::Event;
use crate::key::Key;

/// Application state.
///
/// This owns all data the TUI displays. Keep it separate from
/// rendering and event handling so it's easy to unit test.
pub struct App {
    /// Should the app quit?
    pub running: bool,

    /// Example state: a counter displayed in the UI.
    pub counter: i32,

    /// Current page/tab for multi-page UIs.
    pub tab: Tab,
}

/// Pages/tabs in the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Home,
    Example,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            counter: 0,
            tab: Tab::Home,
        }
    }

    /// Handle an event. Returns false if the app should quit.
    pub fn handle_event(&mut self, event: Event) -> bool {
        match event {
            Event::Key(key) => self.handle_key(key),
            Event::Tick => {
                // Periodic update — polling, status refresh, etc.
            }
            Event::Message(msg) => {
                // Async message from background task
                tracing::debug!("received message: {}", msg);
            }
        }
        self.running
    }

    fn handle_key(&mut self, key: Key) {
        match key {
            Key::Quit => {
                self.running = false;
            }
            Key::Up => {
                self.counter += 1;
            }
            Key::Down => {
                self.counter -= 1;
            }
            Key::Tab => {
                self.tab = match self.tab {
                    Tab::Home => Tab::Example,
                    Tab::Example => Tab::Home,
                };
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_starts_on_home_tab() {
        let app = App::new();
        assert_eq!(app.tab, Tab::Home);
        assert!(app.running);
    }

    #[test]
    fn quit_stops_app() {
        let mut app = App::new();
        let still_running = app.handle_event(Event::Key(Key::Quit));
        assert!(!still_running);
        assert!(!app.running);
    }

    #[test]
    fn up_increments_counter() {
        let mut app = App::new();
        app.handle_event(Event::Key(Key::Up));
        assert_eq!(app.counter, 1);
        app.handle_event(Event::Key(Key::Up));
        assert_eq!(app.counter, 2);
    }

    #[test]
    fn down_decrements_counter() {
        let mut app = App::new();
        app.handle_event(Event::Key(Key::Down));
        assert_eq!(app.counter, -1);
    }

    #[test]
    fn tab_cycles_pages() {
        let mut app = App::new();
        assert_eq!(app.tab, Tab::Home);
        app.handle_event(Event::Key(Key::Tab));
        assert_eq!(app.tab, Tab::Example);
        app.handle_event(Event::Key(Key::Tab));
        assert_eq!(app.tab, Tab::Home);
    }
}
