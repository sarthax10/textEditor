use crossterm::event::{self, Event, KeyCode};
use std::io;

pub enum Key {
    Quit,
    Char(char),
    Enter,
    Backspace,
    Left,
    Right,
    Up,
    Down,
}

pub fn read_key() -> io::Result<Option<Key>> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key_event) = event::read()? {
            let key = match key_event.code {
                KeyCode::Char('q') => Key::Quit,
                KeyCode::Char(c) => Key::Char(c),
                KeyCode::Enter => Key::Enter,
                KeyCode::Backspace => Key::Backspace,
                KeyCode::Left => Key::Left,
                KeyCode::Right => Key::Right,
                KeyCode::Up => Key::Up,
                KeyCode::Down => Key::Down,
                _ => return Ok(None),
            };
            return Ok(Some(key));
        }
    }
    Ok(None)
}
