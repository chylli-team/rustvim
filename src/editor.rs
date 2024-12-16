use std::io::{self, stdout, Write};
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use termion::cursor;
use termion::clear;
use termion::terminal_size;

use crate::buffer::Buffer;
use crate::cursor::Cursor;

#[derive(Debug, Copy, Clone)]
pub enum Mode {
    Normal,
    Insert,
}

impl Mode {
    pub fn display_name(&self) -> &str {
        match self {
            Mode::Normal => "-- NORMAL --",
            Mode::Insert => "-- INSERT --",
        }
    }
}

pub struct Editor {
    pub buffer: Buffer,
    pub cursor: Cursor,
    pub mode: Mode,
}

impl Editor {
    pub fn new() -> io::Result<Editor> {
        Ok(Editor {
            buffer: Buffer::new(),
            cursor: Cursor::new(4),  // 从第4行开始（前面有3行提示信息）
            mode: Mode::Normal,
        })
    }

    pub fn run(&mut self) -> io::Result<()> {
        let _raw = stdout().into_raw_mode()?;
        
        self.init_screen()?;
        self.draw()?;
        
        let stdin = io::stdin();
        for key in stdin.keys() {
            if !self.handle_key(key?)? {
                break;
            }
        }
        
        self.clear_screen()?;
        Ok(())
    }

    fn init_screen(&self) -> io::Result<()> {
        print!("{}{}", clear::All, cursor::Goto(1, 1));
        println!("欢迎使用 RustVim! 按 Ctrl-c 退出\r");
        println!("按 i 进入插入模式，按 ESC 返回普通模式\r");
        println!("普通模式命令: x(删除字符) o/O(插入新行) 0/$(行首/尾)\r");
        stdout().flush()
    }

    fn clear_screen(&self) -> io::Result<()> {
        print!("{}", clear::All);
        stdout().flush()
    }

    fn show_mode(&self) -> io::Result<()> {
        let (width, _) = terminal_size()?;
        print!("{}{}{}",
            cursor::Goto(width - 12, 1),
            clear::CurrentLine,
            self.mode.display_name()
        );
        stdout().flush()
    }

    fn draw_line(&self, line_num: usize, screen_row: u16) -> io::Result<()> {
        let (term_width, _) = terminal_size()?;
        let screen_lines = self.buffer.line_screen_rows(line_num, term_width)?;
        
        // 绘制第一行（包含行号）
        print!("{}", cursor::Goto(1, screen_row));
        print!("{}", clear::CurrentLine);
        print!("{:3} {}", line_num + 1, self.buffer.get_line_part(line_num, 0, term_width));
        
        // 绘制后续折行（不包含行号，用空格对齐）
        for i in 1..screen_lines {
            print!("{}", cursor::Goto(1, screen_row + i));
            print!("{}", clear::CurrentLine);
            print!("    {}", self.buffer.get_line_part(line_num, i, term_width));
        }
        
        stdout().flush()
    }

    fn draw(&self) -> io::Result<()> {
        self.show_mode()?;
        
        // 绘制所有行
        let mut screen_row = 4;  // 从第4行开始
        for line_num in 0..self.buffer.line_count() {
            self.draw_line(line_num, screen_row)?;
            screen_row += self.buffer.line_screen_rows(line_num, terminal_size()?.0)?;
        }
        
        // 更新光标位置
        print!("{}", cursor::Goto(self.cursor.screen_col, self.cursor.screen_row));
        stdout().flush()
    }

    pub fn handle_key(&mut self, key: Key) -> io::Result<bool> {
        match key {
            Key::Ctrl('c') => return Ok(false),
            Key::Left => self.cursor.move_left(&self.buffer)?,
            Key::Right => self.cursor.move_right(&self.buffer)?,
            Key::Up => self.cursor.move_up(&self.buffer)?,
            Key::Down => self.cursor.move_down(&self.buffer)?,
            key => {
                match self.mode {
                    Mode::Normal => self.handle_normal_mode(key)?,
                    Mode::Insert => self.handle_insert_mode(key)?,
                }
            }
        }
        self.draw()?;
        Ok(true)
    }

    pub fn handle_normal_mode(&mut self, key: Key) -> io::Result<()> {
        match key {
            Key::Char('i') => {
                self.mode = Mode::Insert;
            }
            Key::Char('h') => self.cursor.move_left(&self.buffer)?,
            Key::Char('l') => self.cursor.move_right(&self.buffer)?,
            Key::Char('k') => self.cursor.move_up(&self.buffer)?,
            Key::Char('j') => self.cursor.move_down(&self.buffer)?,
            Key::Char('0') => self.cursor.move_to_start(&self.buffer)?,
            Key::Char('$') => self.cursor.move_to_end(&self.buffer)?,
            _ => (),
        }
        Ok(())
    }

    pub fn handle_insert_mode(&mut self, key: Key) -> io::Result<()> {
        match key {
            Key::Esc => {
                self.mode = Mode::Normal;
            }
            Key::Char(c) => {
                if c == '\n' {
                    // 获取当前行的剩余内容
                    let current_line = self.buffer.get_line(self.cursor.row)
                        .unwrap_or(&String::new())
                        .to_string();
                    let (_, remainder) = current_line.split_at(self.cursor.col);
                    
                    // 在当前行后插入新行
                    self.buffer.insert_line(self.cursor.row + 1, remainder.to_string());
                    
                    // 更新当前行，移除已经移动到新行的内容
                    if let Some(line) = self.buffer.get_line_mut(self.cursor.row) {
                        line.truncate(self.cursor.col);
                    }
                    
                    // 移动光标到新行的开始
                    self.cursor.row += 1;
                    self.cursor.col = 0;
                    self.cursor.update_screen_position(&self.buffer)?;
                } else {
                    self.buffer.insert_char(self.cursor.row, self.cursor.col, c);
                    self.cursor.col += 1;
                    self.cursor.update_screen_position(&self.buffer)?;
                }
            }
            Key::Backspace => {
                if self.cursor.col > 0 {
                    self.cursor.col -= 1;
                    self.buffer.remove_char(self.cursor.row, self.cursor.col);
                    self.cursor.update_screen_position(&self.buffer)?;
                }
            }
            _ => (),
        }
        Ok(())
    }
}
