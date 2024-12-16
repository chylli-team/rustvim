use rustvim::editor::{Editor, Mode};
use termion::event::Key;

#[test]
fn test_mode_display_name() {
    assert_eq!(Mode::Normal.display_name(), "-- NORMAL --");
    assert_eq!(Mode::Insert.display_name(), "-- INSERT --");
}

#[test]
fn test_new_editor() {
    let editor = Editor::new().unwrap();
    // 验证编辑器初始状态
    assert_eq!(editor.cursor.row, 0, "初始光标行应该是0");
    assert_eq!(editor.cursor.col, 0, "初始光标列应该是0");
    assert_eq!(editor.cursor.screen_row, 4, "初始屏幕行应该是4");
    assert_eq!(editor.cursor.screen_col, 5, "初始屏幕列应该是5");
    assert!(matches!(editor.mode, Mode::Normal), "初始模式应该是Normal");
    assert_eq!(editor.buffer.line_count(), 0, "初始buffer应该是空的");
}

#[test]
fn test_mode_switching() {
    let mut editor = Editor::new().unwrap();
    
    // 测试从Normal到Insert模式的切换
    editor.handle_normal_mode(Key::Char('i')).unwrap();
    assert!(matches!(editor.mode, Mode::Insert), "应该切换到Insert模式");
    
    // 测试从Insert到Normal模式的切换
    editor.handle_insert_mode(Key::Esc).unwrap();
    assert!(matches!(editor.mode, Mode::Normal), "应该切换回Normal模式");
}

#[test]
fn test_normal_mode_movement() {
    let mut editor = Editor::new().unwrap();
    
    // 在buffer中添加一些内容以便测试移动
    editor.buffer.insert_line(0, String::from("first line"));
    editor.buffer.insert_line(1, String::from("second line"));
    
    // 测试h,l,j,k移动
    editor.handle_normal_mode(Key::Char('l')).unwrap();
    assert_eq!(editor.cursor.col, 1, "l应该向右移动一列");
    
    editor.handle_normal_mode(Key::Char('h')).unwrap();
    assert_eq!(editor.cursor.col, 0, "h应该向左移动一列");
    
    editor.handle_normal_mode(Key::Char('j')).unwrap();
    assert_eq!(editor.cursor.row, 1, "j应该向下移动一行");
    
    editor.handle_normal_mode(Key::Char('k')).unwrap();
    assert_eq!(editor.cursor.row, 0, "k应该向上移动一行");
    
    // 测试0和$移动
    editor.handle_normal_mode(Key::Char('$')).unwrap();
    assert_eq!(editor.cursor.col, 10, "$应该移动到行尾");
    
    editor.handle_normal_mode(Key::Char('0')).unwrap();
    assert_eq!(editor.cursor.col, 0, "0应该移动到行首");
}

#[test]
fn test_insert_mode_typing() {
    let mut editor = Editor::new().unwrap();
    
    // 切换到插入模式
    editor.handle_normal_mode(Key::Char('i')).unwrap();
    
    // 测试输入字符
    editor.handle_insert_mode(Key::Char('H')).unwrap();
    editor.handle_insert_mode(Key::Char('i')).unwrap();
    
    assert_eq!(editor.buffer.get_line(0), Some(&String::from("Hi")));
    assert_eq!(editor.cursor.col, 2, "光标应该在输入的字符后面");
    
    // 测试退格键
    editor.handle_insert_mode(Key::Backspace).unwrap();
    assert_eq!(editor.buffer.get_line(0), Some(&String::from("H")));
    assert_eq!(editor.cursor.col, 1, "光标应该回退一格");
    
    // 测试换行
    editor.handle_insert_mode(Key::Char('\n')).unwrap();
    assert_eq!(editor.buffer.line_count(), 2, "应该创建新行");
    assert_eq!(editor.cursor.row, 1, "光标应该移动到新行");
    assert_eq!(editor.cursor.col, 0, "光标应该在新行开始");
}

#[test]
fn test_cursor_movement_boundaries() {
    let mut editor = Editor::new().unwrap();
    
    // 测试空buffer时的移动
    editor.handle_normal_mode(Key::Char('j')).unwrap();
    assert_eq!(editor.cursor.row, 0, "空buffer时不应该能向下移动");
    
    editor.handle_normal_mode(Key::Char('k')).unwrap();
    assert_eq!(editor.cursor.row, 0, "空buffer时不应该能向上移动");
    
    editor.handle_normal_mode(Key::Char('h')).unwrap();
    assert_eq!(editor.cursor.col, 0, "空buffer时不应该能向左移动");
    
    editor.handle_normal_mode(Key::Char('l')).unwrap();
    assert_eq!(editor.cursor.col, 0, "空buffer时不应该能向右移动");
    
    // 添加一行内容后测试边界
    editor.buffer.insert_line(0, String::from("test"));
    
    editor.handle_normal_mode(Key::Char('$')).unwrap();
    let col = editor.cursor.col;
    editor.handle_normal_mode(Key::Char('l')).unwrap();
    assert_eq!(editor.cursor.col, col, "不应该能移动超过行尾");
}

#[test]
fn test_insert_mode_line_handling() {
    let mut editor = Editor::new().unwrap();
    
    // 切换到插入模式
    editor.handle_normal_mode(Key::Char('i')).unwrap();
    
    // 输入第一行
    editor.handle_insert_mode(Key::Char('H')).unwrap();
    editor.handle_insert_mode(Key::Char('i')).unwrap();
    editor.handle_insert_mode(Key::Char('\n')).unwrap();
    
    // 输入第二行
    editor.handle_insert_mode(Key::Char('t')).unwrap();
    editor.handle_insert_mode(Key::Char('h')).unwrap();
    editor.handle_insert_mode(Key::Char('e')).unwrap();
    editor.handle_insert_mode(Key::Char('r')).unwrap();
    editor.handle_insert_mode(Key::Char('e')).unwrap();
    
    assert_eq!(editor.buffer.line_count(), 2, "应该有两行");
    assert_eq!(editor.buffer.get_line(0), Some(&String::from("Hi")));
    assert_eq!(editor.buffer.get_line(1), Some(&String::from("there")));
    
    // 在行中间换行
    editor.cursor.col = 2;  // 移动到"there"中的h后面
    editor.handle_insert_mode(Key::Char('\n')).unwrap();
    
    assert_eq!(editor.buffer.line_count(), 3, "应该有三行");
    assert_eq!(editor.buffer.get_line(1), Some(&String::from("th")));
    assert_eq!(editor.buffer.get_line(2), Some(&String::from("ere")));
}
