use std::io;
use termion::terminal_size;

use crate::buffer::Buffer;

pub struct Cursor {
    pub row: usize,    // 文档中的行号（从0开始）
    pub col: usize,    // 列号（从0开始）
    pub screen_row: u16, // 屏幕上的行号（考虑到折行）
    pub screen_col: u16, // 屏幕上的列号（考虑到行号占用的空间）
}

impl Cursor {
    pub fn new(start_row: u16) -> Self {
        Self {
            row: 0,
            col: 0,
            screen_row: start_row,
            screen_col: 5,  // 从行号后面开始
        }
    }

    // 更新光标的屏幕位置
    pub fn update_screen_position(&mut self, buffer: &Buffer) -> io::Result<()> {
        let (term_width, _) = terminal_size()?;
        
        // 计算之前所有行占用的屏幕行数
        let mut screen_row = 4; // 从第4行开始（前面有3行提示信息）
        for i in 0..self.row {
            screen_row += buffer.line_screen_rows(i, term_width)?;
        }
        
        // 计算当前行的光标位置
        let effective_width = term_width - 5;
        
        // 计算当前光标位置占用的屏幕行数和列数
        let screen_row_offset = self.col / effective_width as usize;
        let screen_col = self.col % effective_width as usize;
        
        self.screen_row = screen_row + screen_row_offset as u16;
        self.screen_col = screen_col as u16 + 5;  // +5 是因为行号占用的空间
        
        Ok(())
    }

    // 向左移动光标
    pub fn move_left(&mut self, buffer: &Buffer) -> io::Result<()> {
        if self.col > 0 {
            self.col -= 1;
            self.update_screen_position(buffer)?;
        }
        Ok(())
    }

    // 向右移动光标
    pub fn move_right(&mut self, buffer: &Buffer) -> io::Result<()> {
        if let Some(line) = buffer.get_line(self.row) {
            if self.col < line.len() {
                self.col += 1;
                self.update_screen_position(buffer)?;
            }
        }
        Ok(())
    }

    // 向上移动光标
    pub fn move_up(&mut self, buffer: &Buffer) -> io::Result<()> {
        if self.row > 0 {
            self.row -= 1;
            // 确保列位置不超过新行的长度
            if let Some(line) = buffer.get_line(self.row) {
                self.col = self.col.min(line.len());
            }
            self.update_screen_position(buffer)?;
        }
        Ok(())
    }

    // 向下移动光标
    pub fn move_down(&mut self, buffer: &Buffer) -> io::Result<()> {
        if self.row + 1 < buffer.line_count() {
            self.row += 1;
            // 确保列位置不超过新行的长度
            if let Some(line) = buffer.get_line(self.row) {
                self.col = self.col.min(line.len());
            }
            self.update_screen_position(buffer)?;
        }
        Ok(())
    }

    // 移动到行首
    pub fn move_to_start(&mut self, buffer: &Buffer) -> io::Result<()> {
        self.col = 0;
        self.update_screen_position(buffer)
    }

    // 移动到行尾
    pub fn move_to_end(&mut self, buffer: &Buffer) -> io::Result<()> {
        if let Some(line) = buffer.get_line(self.row) {
            self.col = line.len();
            self.update_screen_position(buffer)?;
        }
        Ok(())
    }
}
