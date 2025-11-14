#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
}

impl Mode {
    pub fn is_insert(&self) -> bool {
        matches!(self, Mode::Insert)
    }
}
