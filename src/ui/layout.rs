use crossterm::terminal;

pub fn center_text(text: &str, term_width: u16) -> u16 {
    let len = text.chars().count() as u16;
    if len >= term_width {
        0
    } else {
        (term_width - len) / 2
    }
}

pub fn center_block(block_height: u16, term_height: u16) -> u16 {
    if block_height >= term_height {
        0
    } else {
        (term_height - block_height) / 2
    }
}
