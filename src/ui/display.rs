use std::io::{stdout, Write};
use crossterm::{
    cursor,
    execute,
    style::Print,
    terminal::{self, ClearType},
};

use crate::buffer::Buffer;
use crate::ui::layout::{center_text, center_block};
use crate::ui::{SPLASH, Highlighter};

pub struct Display {
    pub show_splash: bool,
    pub highlighter: Highlighter,
}

impl Display {
    pub fn new() -> Self {
        Self {
            show_splash: true,
            highlighter: Highlighter::new(),
        }
    }

    pub fn draw(
        &mut self,
        buffer: &Buffer,
        cursor_x: usize,
        cursor_y: usize,
    ) -> std::io::Result<()> {
        let mut out = stdout();
        let (cols, rows) = terminal::size()?;

        // Clear screen
        execute!(out, terminal::Clear(ClearType::All))?;

        // Draw ASCII header
        let ascii = SPLASH;
        let ascii_height = ascii.len() as u16;

        for (i, line) in ascii.iter().enumerate() {
            if i as u16 >= rows { break; }

            let x = center_text(line, cols);
            execute!(
                out,
                cursor::MoveTo(x, i as u16),
                Print(*line)
            )?;
        }

        // Editor area starts below ASCII
        let usable_rows = rows.saturating_sub(ascii_height);
        let start_y = ascii_height;

        // Gutter width
        let line_count = buffer.lines().len();
        let gutter_width = (line_count as f32).log10().floor() as usize + 1;

        // Draw text buffer
        for (i, line) in buffer.lines().iter().enumerate() {
            if i as u16 >= usable_rows {
                break;
            }

            let screen_y = start_y + i as u16;

            // Gutter
            let line_no = format!("{:>width$} | ", i + 1, width = gutter_width);

            // Highlighted code
            let highlighted = self.highlighter.highlight_line(line);

            // Print gutter + highlighted line
            execute!(
                out,
                cursor::MoveTo(0, screen_y),
                Print(format!("{}{}", line_no, highlighted))
            )?;
        }

        // Cursor position
        let cursor_screen_x = (gutter_width + 3 + cursor_x) as u16;
        let cursor_screen_y = start_y + cursor_y as u16;

        execute!(
            out,
            cursor::MoveTo(cursor_screen_x, cursor_screen_y)
        )?;

        out.flush()
    }

    pub fn draw_splash(
        &self,
        out: &mut std::io::Stdout,
        cols: u16,
        rows: u16,
    ) -> std::io::Result<()> {
        let ascii = SPLASH;

        let block_height = ascii.len() as u16;
        let start_y = center_block(block_height, rows);

        for (i, line) in ascii.iter().enumerate() {
            let y = start_y + i as u16;
            if y >= rows { break; }

            let x = center_text(line, cols);
            execute!(
                out,
                cursor::MoveTo(x, y),
                Print(*line)
            )?;
        }

        out.flush()
    }
}
