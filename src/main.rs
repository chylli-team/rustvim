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

// 存储每行的内容
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

fn main() -> io::Result<()> {
    let _raw = stdout().into_raw_mode()?;
    
    print!("{}{}", clear::All, cursor::Goto(1, 1));
    stdout().flush()?;
    
    println!("欢迎使用 RustVim! 按 Ctrl-c 退出\r");
    println!("按 i 进入插入模式，按 ESC 返回普通模式\r");
    
    let stdin = io::stdin();
    let mut lines: Vec<Line> = vec![Line::new()];  // 存储所有行
    let mut current_line = 0;  // 当前行索引
    let mut cursor_x = 5;  // 从行号后面开始
    let mut cursor_y = 3;  // 从第3行开始
    let mut mode = Mode::Normal;
    
    show_mode(&mode)?;
    draw_line_number(cursor_y)?;
    
    print!("{}", cursor::Goto(cursor_x, cursor_y));
    stdout().flush()?;

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
                // 限制光标不能超过当前行的内容长度
                if cursor_x < lines[current_line].len() + 5 {
                    cursor_x += 1;
                    print!("{}", cursor::Right(1));
                    stdout().flush()?;
                }
            }
            Key::Up => {
                if cursor_y > 3 && current_line > 0 {
                    current_line -= 1;
                    cursor_y -= 1;
                    // 确保x坐标不超过新行的长度
                    cursor_x = cursor_x.min(lines[current_line].len() + 5);
                    print!("{}", cursor::Goto(cursor_x, cursor_y));
                    stdout().flush()?;
                }
            }
            Key::Down => {
                if current_line < lines.len() - 1 {
                    current_line += 1;
                    cursor_y += 1;
                    // 确保x坐标不超过新行的长度
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
                                if cursor_y > 3 && current_line > 0 {
                                    current_line -= 1;
                                    cursor_y -= 1;
                                    cursor_x = cursor_x.min(lines[current_line].len() + 5);
                                    print!("{}", cursor::Goto(cursor_x, cursor_y));
                                    stdout().flush()?;
                                }
                            }
                            Key::Char('j') => {
                                if current_line < lines.len() - 1 {
                                    current_line += 1;
                                    cursor_y += 1;
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
                                // 在普通模式下，如果光标在行尾，需要向左移动一格
                                if cursor_x > lines[current_line].len() + 5 {
                                    cursor_x -= 1;
                                }
                                print!("{}", cursor::Goto(cursor_x, cursor_y));
                                stdout().flush()?;
                            }
                            Key::Char(c) => {
                                if c == '\n' {
                                    // 创建新行
                                    lines.insert(current_line + 1, Line::new());
                                    current_line += 1;
                                    cursor_y += 1;
                                    cursor_x = 5;
                                    print!("\r\n");
                                    draw_line_number(cursor_y)?;
                                    print!("{}", cursor::Goto(cursor_x, cursor_y));
                                } else {
                                    let relative_x = (cursor_x - 5) as usize;
                                    let line = &mut lines[current_line];
                                    // 确保字符串长度足够
                                    while line.content.len() < relative_x {
                                        line.content.push(' ');
                                    }
                                    if relative_x == line.content.len() {
                                        line.content.push(c);
                                    } else {
                                        line.content.insert(relative_x, c);
                                    }
                                    print!("{}", c);
                                    cursor_x += 1;
                                }
                                stdout().flush()?;
                            }
                            Key::Backspace => {
                                if cursor_x > 5 {
                                    let relative_x = (cursor_x - 6) as usize;
                                    let line = &mut lines[current_line];
                                    if relative_x < line.content.len() {
                                        line.content.remove(relative_x);
                                        print!("\x08{} \x08", &line.content[relative_x..]);
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
