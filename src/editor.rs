use crate::buffer::Buffer;
use crate::display;
use crate::input;
use crossterm::terminal;
use std::io;

pub fn run_editor() -> io::Result<()> {
    terminal::enable_raw_mode()?;

    let mut buffer = Buffer::new();
    let mut cursor_x = 0;
    let mut cursor_y = 0;

    loop {
        // Draw everything
        display::draw(&buffer, cursor_x, cursor_y)?;

        // Read input
        if let Some(key) = input::read_key()? {
            match key {
                input::Key::Quit => break,
                input::Key::Char(c) => {
                    buffer.insert_char(cursor_y, cursor_x, c);
                    cursor_x += 1;
                }
                input::Key::Enter => {
                    buffer.insert_newline(cursor_y, cursor_x);
                    cursor_y += 1;
                    cursor_x = 0;
                }
                input::Key::Backspace => buffer.backspace(&mut cursor_x, &mut cursor_y),
                input::Key::Left => buffer.move_left(&mut cursor_x, &mut cursor_y),
                input::Key::Right => buffer.move_right(&mut cursor_x, &mut cursor_y),
                input::Key::Up => buffer.move_up(&mut cursor_x, &mut cursor_y),
                input::Key::Down => buffer.move_down(&mut cursor_x, &mut cursor_y),
            }

        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
