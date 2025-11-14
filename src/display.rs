use crate::buffer::Buffer;
use crossterm::{cursor, execute, terminal::{self, ClearType}};
use std::io::{stdout, Write};

pub fn draw(buffer: &Buffer, cursor_x: usize, cursor_y: usize) -> std::io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(ClearType::All))?;

    for (i, line) in buffer.lines().iter().enumerate() {
        execute!(stdout, cursor::MoveTo(0, i as u16))?;
        print!("{}", line);
    }

    execute!(stdout, cursor::MoveTo(cursor_x as u16, cursor_y as u16))?;
    stdout.flush()?;
    Ok(())
}
