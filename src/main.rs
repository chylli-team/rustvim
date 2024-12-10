use std::io::{self, stdout, Write};
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use termion::cursor;
use termion::clear;

enum Mode {
    Normal,
    Insert,
}

fn show_mode(mode: &Mode) -> io::Result<()> {
    let mode_text = match mode {
        Mode::Normal => "-- NORMAL --",
        Mode::Insert => "-- INSERT --",
    };
    // 移动到第一行开始位置，清除该行，显示模式
    print!("{}{}{}", 
        cursor::Goto(1, 1),
        clear::CurrentLine,
        mode_text
    );
    stdout().flush()?;
    Ok(())
}

fn draw_line_number(line: u16) -> io::Result<()> {
    // 移动到对应行的开始位置，显示行号
    print!("{}{:3} ", 
        cursor::Goto(1, line),
        line - 2
    );
    stdout().flush()?;
    Ok(())
}

fn main() -> io::Result<()> {
    let _raw = stdout().into_raw_mode()?;
    
    // 清屏并移动光标到左上角
    print!("{}{}", clear::All, cursor::Goto(1, 1));
    stdout().flush()?;
    
    // 显示欢迎信息
    println!("欢迎使用 RustVim! 按 Ctrl-c 退出\r");
    println!("按 i 进入插入模式，按 ESC 返回普通模式\r");
    
    let stdin = io::stdin();
    let mut content = String::new();
    let mut cursor_x = 5;  // 从行号后面开始
    let mut cursor_y = 3;  // 从第3行开始，因为有两行提示信息
    let mut mode = Mode::Normal;
    
    // 显示初始模式和行号
    show_mode(&mode)?;
    draw_line_number(cursor_y)?;
    
    // 移动到编辑区域的初始位置
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
                cursor_x += 1;
                print!("{}", cursor::Right(1));
                stdout().flush()?;
            }
            Key::Up => {
                if cursor_y > 3 {
                    cursor_y -= 1;
                    print!("{}", cursor::Up(1));
                    stdout().flush()?;
                }
            }
            Key::Down => {
                cursor_y += 1;
                print!("{}", cursor::Down(1));
                stdout().flush()?;
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
                                cursor_x += 1;
                                print!("{}", cursor::Right(1));
                                stdout().flush()?;
                            }
                            Key::Char('k') => {
                                if cursor_y > 3 {
                                    cursor_y -= 1;
                                    print!("{}", cursor::Up(1));
                                    stdout().flush()?;
                                }
                            }
                            Key::Char('j') => {
                                cursor_y += 1;
                                print!("{}", cursor::Down(1));
                                stdout().flush()?;
                            }
                            _ => ()
                        }
                    }
                    Mode::Insert => {
                        match key {
                            Key::Esc => {
                                mode = Mode::Normal;
                                show_mode(&mode)?;
                                print!("{}", cursor::Goto(cursor_x, cursor_y));
                                stdout().flush()?;
                            }
                            Key::Char(c) => {
                                if c == '\n' {
                                    content.push('\n');
                                    cursor_y += 1;
                                    cursor_x = 5;
                                    print!("\r\n");
                                    draw_line_number(cursor_y)?;
                                    print!("{}", cursor::Goto(cursor_x, cursor_y));
                                } else {
                                    content.push(c);
                                    print!("{}", c);
                                    cursor_x += 1;
                                }
                                stdout().flush()?;
                            }
                            Key::Backspace => {
                                if !content.is_empty() {
                                    content.pop();
                                    if cursor_x > 5 {
                                        cursor_x -= 1;
                                        print!("\x08 \x08");
                                    }
                                    stdout().flush()?;
                                }
                            }
                            _ => ()
                        }
                    }
                }
            }
        }
    }
    
    // 退出前清屏
    print!("{}", clear::All);
    stdout().flush()?;
    
    Ok(())
}
