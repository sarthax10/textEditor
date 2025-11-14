mod editor;
mod buffer;
mod input;
mod ui;   // <-- correct

fn main() -> std::io::Result<()> {
    editor::run_editor()
}
