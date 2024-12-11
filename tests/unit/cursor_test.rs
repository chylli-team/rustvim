use rustvim::buffer::Buffer;
use rustvim::cursor::Cursor;

#[test]
fn test_new_cursor() {
    let cursor = Cursor::new(4);
    assert_eq!(cursor.row, 0);
    assert_eq!(cursor.col, 0);
    assert_eq!(cursor.screen_row, 4);
    assert_eq!(cursor.screen_col, 5);
}

#[test]
fn test_move_right() {
    let mut buffer = Buffer::new();
    buffer.insert_line(0, String::from("first line"));
    let mut cursor = Cursor::new(4);
    
    // 移动到第一行的末尾
    for _ in 0..10 {
        cursor.move_right(&buffer).unwrap();
    }
    assert_eq!(cursor.col, 10);
    
    // 尝试移动超过行尾
    cursor.move_right(&buffer).unwrap();
    assert_eq!(cursor.col, 10, "光标不应该移动超过行尾");
}

#[test]
fn test_move_left() {
    let mut buffer = Buffer::new();
    buffer.insert_line(0, String::from("first line"));
    let mut cursor = Cursor::new(4);
    
    // 先移动到右边
    for _ in 0..5 {
        cursor.move_right(&buffer).unwrap();
    }
    assert_eq!(cursor.col, 5);
    
    // 然后向左移动
    cursor.move_left(&buffer).unwrap();
    assert_eq!(cursor.col, 4);
    
    // 移动到开始
    for _ in 0..4 {
        cursor.move_left(&buffer).unwrap();
    }
    assert_eq!(cursor.col, 0);
    
    // 尝试移动超过行首
    cursor.move_left(&buffer).unwrap();
    assert_eq!(cursor.col, 0, "光标不应该移动超过行首");
}

#[test]
fn test_move_down() {
    let mut buffer = Buffer::new();
    buffer.insert_line(0, String::from("first line"));
    buffer.insert_line(1, String::from("second line"));
    buffer.insert_line(2, String::from("third line"));
    let mut cursor = Cursor::new(4);
    
    assert_eq!(cursor.row, 0, "光标应该从第0行开始");
    
    cursor.move_down(&buffer).unwrap();
    assert_eq!(cursor.row, 1, "光标应该移动到第1行");
    
    cursor.move_down(&buffer).unwrap();
    assert_eq!(cursor.row, 2, "光标应该移动到第2行");
    
    let row_before = cursor.row;
    cursor.move_down(&buffer).unwrap();
    assert_eq!(cursor.row, row_before, "光标不应该移动超过最后一行");
}

#[test]
fn test_move_up() {
    let mut buffer = Buffer::new();
    buffer.insert_line(0, String::from("first line"));
    buffer.insert_line(1, String::from("second line"));
    buffer.insert_line(2, String::from("third line"));
    let mut cursor = Cursor::new(4);
    
    cursor.move_down(&buffer).unwrap();
    cursor.move_down(&buffer).unwrap();
    assert_eq!(cursor.row, 2, "光标应该在最后一行");
    
    cursor.move_up(&buffer).unwrap();
    assert_eq!(cursor.row, 1, "光标应该移动到第1行");
    
    cursor.move_up(&buffer).unwrap();
    assert_eq!(cursor.row, 0, "光标应该移动到第0行");
    
    cursor.move_up(&buffer).unwrap();
    assert_eq!(cursor.row, 0, "光标不应该移动超过第一行");
}

#[test]
fn test_move_to_start_end() {
    let mut buffer = Buffer::new();
    buffer.insert_line(0, String::from("first line"));
    let mut cursor = Cursor::new(4);
    
    for _ in 0..5 {
        cursor.move_right(&buffer).unwrap();
    }
    assert_eq!(cursor.col, 5, "光标应该在第5列");
    
    cursor.move_to_start(&buffer).unwrap();
    assert_eq!(cursor.col, 0, "光标应该在行首");
    
    cursor.move_to_end(&buffer).unwrap();
    assert_eq!(cursor.col, 10, "光标应该在行尾");
}

#[test]
fn test_cursor_column_adjustment() {
    let mut buffer = Buffer::new();
    buffer.insert_line(0, String::from("first line"));     // 长度10
    buffer.insert_line(1, String::from("second line"));    // 长度11
    buffer.insert_line(2, String::from("short"));          // 长度5
    let mut cursor = Cursor::new(4);
    
    cursor.move_to_end(&buffer).unwrap();
    assert_eq!(cursor.col, 10, "光标应该在第一行末尾");
    
    cursor.move_down(&buffer).unwrap();
    assert_eq!(cursor.col, 10, "光标列位置应该保持不变，因为第二行足够长");
    
    cursor.move_down(&buffer).unwrap();
    assert_eq!(cursor.col, 5, "光标列位置应该调整到短行的末尾");
}
