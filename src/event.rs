use crossterm::event::{self, Event as CrosstermEvent};
use std::time::Duration;

use crate::key::Key;

/// Application events.
///
/// These are the events the app can handle. Key events come from
/// the terminal, tick events come from a timer, and message events
/// come from background tasks.
#[derive(Debug)]
pub enum Event {
    Key(Key),
    Tick,
    #[allow(dead_code)]
    Message(String),
}

/// Read an event from the terminal.
///
/// This blocks for up to 250ms waiting for input, then returns
/// a Tick event if nothing happened. This gives the UI a chance
/// to refresh periodically.
pub fn read_event() -> anyhow::Result<Event> {
    if event::poll(Duration::from_millis(250))? {
        match event::read()? {
            CrosstermEvent::Key(key_event) => Ok(Event::Key(Key::from(key_event))),
            _ => Ok(Event::Tick),
        }
    } else {
        Ok(Event::Tick)
    }
}
