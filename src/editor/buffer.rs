pub struct Buffer {
    lines: Vec<String>,
    cursor_x: usize,
    cursor_y: usize,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            lines: vec![String::new()],
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn insert_char(&mut self, c: char) {
        if c == '\n' {
            let current_line = self.lines[self.cursor_y].clone();
            let rest_of_line = current_line[self.cursor_x..].to_string();
            self.lines[self.cursor_y].truncate(self.cursor_x);
            self.lines.insert(self.cursor_y + 1, rest_of_line);
            self.cursor_y += 1;
            self.cursor_x = 0;
        } else {
            self.lines[self.cursor_y].insert(self.cursor_x, c);
            self.cursor_x += 1;
        }
    }

    pub fn delete_char(&mut self) {
        if self.cursor_x > 0 {
            self.lines[self.cursor_y].remove(self.cursor_x - 1);
            self.cursor_x -= 1;
        } else if self.cursor_y > 0 {
            let current_line = self.lines.remove(self.cursor_y);
            self.cursor_y -= 1;
            self.cursor_x = self.lines[self.cursor_y].len();
            self.lines[self.cursor_y].push_str(&current_line);
        }
    }

    pub fn move_to_start_of_line(&mut self) {
        self.cursor_x = 0;
    }

    pub fn move_to_end_of_line(&mut self) {
        self.cursor_x = self.lines[self.cursor_y].len();
    }

    pub fn move_cursor(&mut self, dx: isize, dy: isize) {
        let new_y = (self.cursor_y as isize + dy).max(0).min(self.lines.len() as isize - 1) as usize;
        
        let line_len = self.lines[new_y].len();
        let new_x = if dx == 0 {
            self.cursor_x.min(line_len)
        } else {
            (self.cursor_x as isize + dx).max(0).min(line_len as isize) as usize
        };
        
        self.cursor_x = new_x;
        self.cursor_y = new_y;
    }

    pub fn get_content(&self) -> String {
        self.lines.join("\n")
    }

    pub fn get_cursor_position(&self) -> (usize, usize) {
        (self.cursor_x, self.cursor_y)
    }

    pub fn get_line(&self, index: usize) -> Option<&str> {
        self.lines.get(index).map(|s| s.as_str())
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn get_lines(&self) -> &Vec<String> {
        &self.lines
    }
}
