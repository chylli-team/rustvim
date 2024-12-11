use std::io::{self, stdout, Write};
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use termion::cursor;
use termion::clear;
use termion::terminal_size;

enum Mode {
    Normal,
    Insert,
}

struct Line {
    content: String,
}

impl Line {
    fn new() -> Self {
        Line {
            content: String::new(),
        }
    }

    fn len(&self) -> u16 {
        self.content.len() as u16
    }

    // 计算这行文本实际占用的屏幕行数
    fn screen_lines(&self, term_width: u16) -> u16 {
        let content_width = self.len() + 5; // +5 是因为行号占用的空间
        if content_width == 0 {
            1
        } else {
            (content_width + term_width - 1) / term_width
        }
    }
}

fn show_mode(mode: &Mode) -> io::Result<()> {
    let mode_text = match mode {
        Mode::Normal => "-- NORMAL --",
        Mode::Insert => "-- INSERT --",
    };
    
    let (width, _) = terminal_size()?;
    print!("{}{}{}",
        cursor::Goto(width - 12, 1),
        clear::CurrentLine,
        mode_text
    );
    stdout().flush()?;
    Ok(())
}

fn draw_line_number(line: u16) -> io::Result<()> {
    print!("{}{:3} ", 
        cursor::Goto(1, line),
        line - 2
    );
    stdout().flush()?;
    Ok(())
}

// 重新绘制从指定行开始的所有内容
fn redraw_from(lines: &[Line], start_line: u16) -> io::Result<()> {
    let (term_width, _) = terminal_size()?;
    let mut screen_line = start_line;
    
    for (i, line) in lines.iter().enumerate() {
        // 清除从当前行开始的内容
        print!("{}", cursor::Goto(1, screen_line));
        print!("{}", clear::CurrentLine);
        
        // 绘制行号
        draw_line_number(screen_line)?;
        
        // 绘制内容
        print!("{}", line.content);
        
        // 更新屏幕行号，考虑长行折行的情况
        screen_line += line.screen_lines(term_width);
    }
    stdout().flush()?;
    Ok(())
}

fn main() -> io::Result<()> {
    let _raw = stdout().into_raw_mode()?;
    
    print!("{}{}", clear::All, cursor::Goto(1, 1));
    stdout().flush()?;
    
    println!("欢迎使用 RustVim! 按 Ctrl-c 退出\r");
    println!("按 i 进入插入模式，按 ESC 返回普通模式\r");
    println!("普通模式命令: x(删除字符) o/O(插入新行) 0/$(行首/尾)\r");
    
    let stdin = io::stdin();
    let mut lines: Vec<Line> = vec![Line::new()];
    let mut current_line = 0;
    let mut cursor_x = 5;
    let mut cursor_y = 4;  // 从第4行开始，因为有三行提示信息
    let mut mode = Mode::Normal;
    
    show_mode(&mode)?;
    redraw_from(&lines, cursor_y)?;
    
    print!("{}", cursor::Goto(cursor_x, cursor_y));
    stdout().flush()?;

    let (term_width, _) = terminal_size()?;

    for key in stdin.keys() {
        match key? {
            Key::Ctrl('c') => break,
            Key::Left => {
                if cursor_x > 5 {
                    cursor_x -= 1;
                    print!("{}", cursor::Left(1));
                    stdout().flush()?;
                }
            }
            Key::Right => {
                if cursor_x < lines[current_line].len() + 5 {
                    cursor_x += 1;
                    print!("{}", cursor::Right(1));
                    stdout().flush()?;
                }
            }
            Key::Up => {
                if cursor_y > 4 && current_line > 0 {
                    // 计算上一行占用的屏幕行数
                    let prev_lines = lines[current_line - 1].screen_lines(term_width);
                    current_line -= 1;
                    cursor_y -= prev_lines;
                    cursor_x = cursor_x.min(lines[current_line].len() + 5);
                    print!("{}", cursor::Goto(cursor_x, cursor_y));
                    stdout().flush()?;
                }
            }
            Key::Down => {
                if current_line < lines.len() - 1 {
                    // 计算当前行占用的屏幕行数
                    let current_lines = lines[current_line].screen_lines(term_width);
                    current_line += 1;
                    cursor_y += current_lines;
                    cursor_x = cursor_x.min(lines[current_line].len() + 5);
                    print!("{}", cursor::Goto(cursor_x, cursor_y));
                    stdout().flush()?;
                }
            }
            key => {
                match mode {
                    Mode::Normal => {
                        match key {
                            Key::Char('i') => {
                                mode = Mode::Insert;
                                show_mode(&mode)?;
                                print!("{}", cursor::Goto(cursor_x, cursor_y));
                                stdout().flush()?;
                            }
                            Key::Char('x') => {
                                let relative_x = (cursor_x - 5) as usize;
                                let line = &mut lines[current_line];
                                if relative_x < line.content.len() {
                                    line.content.remove(relative_x);
                                    redraw_from(&lines, cursor_y)?;
                                    print!("{}", cursor::Goto(cursor_x, cursor_y));
                                    stdout().flush()?;
                                }
                            }
                            Key::Char('o') => {
                                mode = Mode::Insert;
                                show_mode(&mode)?;
                                lines.insert(current_line + 1, Line::new());
                                
                                // 计算当前行占用的屏幕行数
                                let current_lines = lines[current_line].screen_lines(term_width);
                                current_line += 1;
                                cursor_y += current_lines;
                                cursor_x = 5;
                                
                                redraw_from(&lines[current_line..], cursor_y)?;
                                print!("{}", cursor::Goto(cursor_x, cursor_y));
                                stdout().flush()?;
                            }
                            Key::Char('O') => {
                                mode = Mode::Insert;
                                show_mode(&mode)?;
                                lines.insert(current_line, Line::new());
                                cursor_x = 5;
                                redraw_from(&lines[current_line..], cursor_y)?;
                                print!("{}", cursor::Goto(cursor_x, cursor_y));
                                stdout().flush()?;
                            }
                            Key::Char('0') => {
                                cursor_x = 5;
                                print!("{}", cursor::Goto(cursor_x, cursor_y));
                                stdout().flush()?;
                            }
                            Key::Char('$') => {
                                cursor_x = lines[current_line].len() + 5;
                                print!("{}", cursor::Goto(cursor_x, cursor_y));
                                stdout().flush()?;
                            }
                            Key::Char('h') => {
                                if cursor_x > 5 {
                                    cursor_x -= 1;
                                    print!("{}", cursor::Left(1));
                                    stdout().flush()?;
                                }
                            }
                            Key::Char('l') => {
                                if cursor_x < lines[current_line].len() + 5 {
                                    cursor_x += 1;
                                    print!("{}", cursor::Right(1));
                                    stdout().flush()?;
                                }
                            }
                            Key::Char('k') => {
                                if cursor_y > 4 && current_line > 0 {
                                    let prev_lines = lines[current_line - 1].screen_lines(term_width);
                                    current_line -= 1;
                                    cursor_y -= prev_lines;
                                    cursor_x = cursor_x.min(lines[current_line].len() + 5);
                                    print!("{}", cursor::Goto(cursor_x, cursor_y));
                                    stdout().flush()?;
                                }
                            }
                            Key::Char('j') => {
                                if current_line < lines.len() - 1 {
                                    let current_lines = lines[current_line].screen_lines(term_width);
                                    current_line += 1;
                                    cursor_y += current_lines;
                                    cursor_x = cursor_x.min(lines[current_line].len() + 5);
                                    print!("{}", cursor::Goto(cursor_x, cursor_y));
                                    stdout().flush()?;
                                }
                            }
                            _ => ()
                        }
                    }
                    Mode::Insert => {
                        match key {
                            Key::Esc => {
                                mode = Mode::Normal;
                                show_mode(&mode)?;
                                if cursor_x > lines[current_line].len() + 5 {
                                    cursor_x -= 1;
                                }
                                print!("{}", cursor::Goto(cursor_x, cursor_y));
                                stdout().flush()?;
                            }
                            Key::Char(c) => {
                                if c == '\n' {
                                    lines.insert(current_line + 1, Line::new());
                                    let current_lines = lines[current_line].screen_lines(term_width);
                                    current_line += 1;
                                    cursor_y += current_lines;
                                    cursor_x = 5;
                                    redraw_from(&lines[current_line..], cursor_y)?;
                                    print!("{}", cursor::Goto(cursor_x, cursor_y));
                                } else {
                                    let relative_x = (cursor_x - 5) as usize;
                                    let line = &mut lines[current_line];
                                    while line.content.len() < relative_x {
                                        line.content.push(' ');
                                    }
                                    if relative_x == line.content.len() {
                                        line.content.push(c);
                                    } else {
                                        line.content.insert(relative_x, c);
                                    }
                                    redraw_from(&lines[current_line..], cursor_y)?;
                                    cursor_x += 1;
                                    print!("{}", cursor::Goto(cursor_x, cursor_y));
                                }
                                stdout().flush()?;
                            }
                            Key::Backspace => {
                                if cursor_x > 5 {
                                    let relative_x = (cursor_x - 6) as usize;
                                    let line = &mut lines[current_line];
                                    if relative_x < line.content.len() {
                                        line.content.remove(relative_x);
                                        redraw_from(&lines[current_line..], cursor_y)?;
                                        cursor_x -= 1;
                                        print!("{}", cursor::Goto(cursor_x, cursor_y));
                                        stdout().flush()?;
                                    }
                                }
                            }
                            _ => ()
                        }
                    }
                }
            }
        }
    }
    
    print!("{}", clear::All);
    stdout().flush()?;
    
    Ok(())
}
