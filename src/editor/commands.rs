use crate::input::Key;
use std::io;

use super::state::EditorState;

pub fn handle_key(editor: &mut EditorState, key: Key) -> io::Result<bool> {
    match key {
        Key::Quit => return Ok(true),

        Key::Char(c) => {
            if editor.mode.is_insert() {
                editor.buffer.insert_char(editor.cursor_y, editor.cursor_x, c);
                editor.cursor_x += 1;
            }
        }

        Key::Enter => {
            editor.buffer.insert_newline(editor.cursor_y, editor.cursor_x);
            editor.cursor_y += 1;
            editor.cursor_x = 0;
        }

        Key::Backspace => {
            editor.buffer.backspace(&mut editor.cursor_x, &mut editor.cursor_y);
        }

        Key::Left => editor.buffer.move_left(&mut editor.cursor_x, &mut editor.cursor_y),
        Key::Right => editor.buffer.move_right(&mut editor.cursor_x, &mut editor.cursor_y),
        Key::Up => editor.buffer.move_up(&mut editor.cursor_x, &mut editor.cursor_y),
        Key::Down => editor.buffer.move_down(&mut editor.cursor_x, &mut editor.cursor_y),

        _ => {}
    }

    Ok(false)
}
