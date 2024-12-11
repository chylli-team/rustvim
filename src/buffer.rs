use std::io;

#[derive(Debug)]
pub struct Buffer {
    lines: Vec<String>,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            lines: Vec::new()
        }
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn get_line(&self, index: usize) -> Option<&String> {
        self.lines.get(index)
    }

    pub fn get_line_mut(&mut self, index: usize) -> Option<&mut String> {
        self.lines.get_mut(index)
    }

    pub fn insert_line(&mut self, index: usize, line: String) {
        while index > self.lines.len() {
            self.lines.push(String::new());
        }
        
        if index == self.lines.len() {
            self.lines.push(line);
        } else {
            self.lines.insert(index, line);
        }
    }

    pub fn insert_char(&mut self, line: usize, col: usize, c: char) {
        while line >= self.lines.len() {
            self.lines.push(String::new());
        }

        if let Some(line_content) = self.get_line_mut(line) {
            while line_content.len() < col {
                line_content.push(' ');
            }
            line_content.insert(col, c);
        }
    }

    pub fn remove_char(&mut self, line: usize, col: usize) -> bool {
        if let Some(line_content) = self.get_line_mut(line) {
            if col < line_content.len() {
                line_content.remove(col);
                return true;
            }
        }
        false
    }

    pub fn line_screen_rows(&self, line_index: usize, term_width: u16) -> io::Result<u16> {
        let line = match self.get_line(line_index) {
            Some(line) => line,
            None => return Ok(1),
        };
        
        let content_width = line.len() + 5;
        Ok((content_width as u16 + term_width - 1) / term_width)
    }

    pub fn get_line_part(&self, line_index: usize, row_index: u16, term_width: u16) -> String {
        let line = match self.get_line(line_index) {
            Some(line) => line,
            None => return String::new(),
        };
        
        let effective_width = term_width - 5;
        let start = row_index as usize * effective_width as usize;
        let end = start + effective_width as usize;
        
        if start >= line.len() {
            String::new()
        } else {
            line[start..line.len().min(end)].to_string()
        }
    }
}
