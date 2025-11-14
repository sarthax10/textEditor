use std::io;

mod state;
mod commands;
mod mode;

pub use state::EditorState;

pub fn run_editor() -> io::Result<()> {
    let mut editor = EditorState::new();
    editor.run()
}
