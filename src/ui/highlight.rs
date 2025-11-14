use crossterm::style::{Color, Stylize};

pub enum Syntax {
    Rust,
    Plain,
}

pub struct Highlighter {
    pub syntax: Syntax,
}

impl Highlighter {
    pub fn new() -> Self {
        Self { syntax: Syntax::Plain }
    }

    pub fn highlight_line(&self, line: &str) -> String {
        match self.syntax {
            Syntax::Rust => self.highlight_rust(line),
            Syntax::Plain => line.to_string(),
        }
    }

    fn highlight_rust(&self, line: &str) -> String {
        let mut out = String::new();

        let keywords = [
            "fn", "let", "mut", "pub", "impl", "struct", "enum", "use", "mod", "match",
            "if", "else", "while", "for", "loop", "return", "break", "continue",
        ];

        for token in line.split_whitespace() {
            if keywords.contains(&token) {
                out.push_str(&token.blue().bold().to_string());
            } else if token.starts_with("\"") && token.ends_with("\"") {
                out.push_str(&token.green().to_string());
            } else {
                out.push_str(token);
            }

            out.push(' ');
        }

        out
    }
}
