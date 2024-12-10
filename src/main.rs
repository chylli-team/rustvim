use std::io::{self, stdout, Write};
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use termion::cursor;

fn main() -> io::Result<()> {
    let _raw = stdout().into_raw_mode()?;
    println!("欢迎使用 RustVim! 按 Ctrl-c 退出\r");
    
    let stdin = io::stdin();
    let mut content = String::new();
    let mut cursor_x = 1;
    let mut cursor_y = 2;
    
    for key in stdin.keys() {
        match key? {
            Key::Ctrl('c') => break,
            Key::Char(c) => {
                if c == '\n' {
                    content.push('\n');
                    print!("\r\n");
                    cursor_x = 1;
                    cursor_y += 1;
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
                    print!("\x08 \x08");
                    if cursor_x > 1 {
                        cursor_x -= 1;
                    }
                    stdout().flush()?;
                }
            }
            Key::Left => {
                if cursor_x > 1 {
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
                if cursor_y > 2 {
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
            _ => (),
        }
    }
    
    Ok(())
}
