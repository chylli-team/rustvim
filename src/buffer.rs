use std::io;

#[derive(Default)]
pub struct Buffer {
    lines: Vec<String>,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
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
        self.lines.insert(index, line);
    }

    pub fn insert_char(&mut self, line: usize, col: usize, c: char) {
        if let Some(line) = self.get_line_mut(line) {
            // 如果插入位置超过当前行长度，先用空格填充
            while line.len() < col {
                line.push(' ');
            }
            line.insert(col, c);
        }
    }

    pub fn remove_char(&mut self, line: usize, col: usize) -> bool {
        if let Some(line) = self.get_line_mut(line) {
            if col < line.len() {
                line.remove(col);
                return true;
            }
        }
        false
    }

    // 计算一行文本在屏幕上实际占用的行数
    pub fn line_screen_rows(&self, line_index: usize, term_width: u16) -> io::Result<u16> {
        let empty_line = String::new();
        let line = self.get_line(line_index).unwrap_or(&empty_line);
        let content_width = line.len() + 5; // +5 是因为行号占用的空间
        Ok((content_width as u16 + term_width - 1) / term_width)
    }

    // 获取指定行的指定屏幕行的内容
    pub fn get_line_part(&self, line_index: usize, row_index: u16, term_width: u16) -> String {
        let empty_line = String::new();
        let line = self.get_line(line_index).unwrap_or(&empty_line);
        let effective_width = term_width - 5;  // 减去行号占用的空间
        let start = row_index as usize * effective_width as usize;
        let end = start + effective_width as usize;
        
        if start >= line.len() {
            String::new()
        } else {
            line[start..line.len().min(end)].to_string()
        }
    }
}
