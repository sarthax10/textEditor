use crate::buffer::Buffer;
use crate::ui::Display;
use crate::input;
use crossterm::terminal;
use std::io;

use super::commands::handle_key;
use super::mode::Mode;

pub struct EditorState {
    pub buffer: Buffer,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub mode: Mode,
    pub display: Display,
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            cursor_x: 0,
            cursor_y: 0,
            mode: Mode::Insert,
            display: Display::new(),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;

        loop {
            
            self.display.draw(
                &self.buffer,
                self.cursor_x,
                self.cursor_y
            )?;

            if let Some(key) = input::read_key()? {
                if handle_key(self, key)? {
                    break; // quit
                }
            }
        }

        terminal::disable_raw_mode()?;
        Ok(())
    }
}
