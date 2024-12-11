use std::io;
use termion::terminal_size;

use crate::buffer::Buffer;

#[derive(Debug)]
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

    pub fn update_screen_position(&mut self, buffer: &Buffer) -> io::Result<()> {
        let (term_width, _) = terminal_size()?;
        
        let mut screen_row = 4;
        for i in 0..self.row {
            screen_row += buffer.line_screen_rows(i, term_width)?;
        }
        
        let effective_width = term_width - 5;
        let screen_row_offset = self.col / effective_width as usize;
        let screen_col = self.col % effective_width as usize;
        
        self.screen_row = screen_row + screen_row_offset as u16;
        self.screen_col = screen_col as u16 + 5;
        
        Ok(())
    }

    pub fn move_left(&mut self, buffer: &Buffer) -> io::Result<()> {
        if self.col > 0 {
            self.col -= 1;
            self.update_screen_position(buffer)?;
        }
        Ok(())
    }

    pub fn move_right(&mut self, buffer: &Buffer) -> io::Result<()> {
        if let Some(line) = buffer.get_line(self.row) {
            if self.col < line.len() {
                self.col += 1;
                self.update_screen_position(buffer)?;
            }
        }
        Ok(())
    }

    pub fn move_up(&mut self, buffer: &Buffer) -> io::Result<()> {
        if self.row > 0 {
            self.row -= 1;
            if let Some(line) = buffer.get_line(self.row) {
                self.col = self.col.min(line.len());
            }
            self.update_screen_position(buffer)?;
        }
        Ok(())
    }

    pub fn move_down(&mut self, buffer: &Buffer) -> io::Result<()> {
        if buffer.line_count() == 0 {
            return Ok(());
        }

        if self.row >= buffer.line_count() - 1 {
            return Ok(());
        }

        self.row += 1;
        
        if let Some(line) = buffer.get_line(self.row) {
            self.col = self.col.min(line.len());
        }
        
        self.update_screen_position(buffer)?;
        Ok(())
    }

    pub fn move_to_start(&mut self, buffer: &Buffer) -> io::Result<()> {
        self.col = 0;
        self.update_screen_position(buffer)
    }

    pub fn move_to_end(&mut self, buffer: &Buffer) -> io::Result<()> {
        if let Some(line) = buffer.get_line(self.row) {
            self.col = line.len();
            self.update_screen_position(buffer)?;
        }
        Ok(())
    }
}
