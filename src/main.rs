mod editor;
mod buffer;
mod input;
mod display;

fn main() -> std::io::Result<()> {
    editor::run_editor()
}
