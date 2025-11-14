pub struct Buffer {
    lines: Vec<String>,
}

impl Buffer {
    pub fn new() -> Self {
        Self { lines: vec![String::new()] }
    }

    pub fn insert_char(&mut self, y: usize, x: usize, c: char) {
        if let Some(line) = self.lines.get_mut(y) {
            line.insert(x, c);
        }
    }

    pub fn insert_newline(&mut self, y: usize, x: usize) {
        if let Some(line) = self.lines.get_mut(y) {
            let new_line = line.split_off(x);
            self.lines.insert(y + 1, new_line);
        }
    }

    pub fn backspace(&mut self, cursor_x: &mut usize, cursor_y: &mut usize) {
        if *cursor_x > 0 {
            if let Some(line) = self.lines.get_mut(*cursor_y) {
                line.remove(*cursor_x - 1);
                *cursor_x -= 1; // move cursor back
            }
        } else if *cursor_y > 0 {
            let prev_len = self.lines[*cursor_y - 1].len();
            let current_line = self.lines.remove(*cursor_y);
            *cursor_y -= 1;
            *cursor_x = prev_len;
            self.lines[*cursor_y].push_str(&current_line);
        }
    }


    pub fn move_left(&self, cursor_x: &mut usize, cursor_y: &mut usize) {
        if *cursor_x > 0 {
            *cursor_x -= 1;
        } else if *cursor_y > 0 {
            *cursor_y -= 1;
            *cursor_x = self.lines[*cursor_y].len();
        }
    }

    pub fn move_right(&self, cursor_x: &mut usize, cursor_y: &mut usize) {
        if *cursor_x < self.lines[*cursor_y].len() {
            *cursor_x += 1;
        } else if *cursor_y + 1 < self.lines.len() {
            *cursor_y += 1;
            *cursor_x = 0;
        }
    }

    pub fn move_up(&self, cursor_x: &mut usize, cursor_y: &mut usize) {
        if *cursor_y > 0 {
            *cursor_y -= 1;
            *cursor_x = (*cursor_x).min(self.lines[*cursor_y].len());
        }
    }

    pub fn move_down(&self, cursor_x: &mut usize, cursor_y: &mut usize) {
        if *cursor_y + 1 < self.lines.len() {
            *cursor_y += 1;
            *cursor_x = (*cursor_x).min(self.lines[*cursor_y].len());
        }
    }

    pub fn lines(&self) -> &Vec<String> {
        &self.lines
    }
}
