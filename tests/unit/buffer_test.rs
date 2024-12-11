use rustvim::buffer::Buffer;

#[test]
fn test_new_buffer() {
    let buffer = Buffer::new();
    assert_eq!(buffer.line_count(), 0, "新创建的buffer应该是空的");
    assert_eq!(buffer.get_line(0), None, "空buffer不应该有任何行");
}

#[test]
fn test_insert_char() {
    let mut buffer = Buffer::new();
    
    // 测试在空buffer中插入字符（应该自动创建第一行）
    buffer.insert_char(0, 0, 'a');
    assert_eq!(buffer.get_line(0), Some(&String::from("a")));

    // 测试在已有字符后插入
    buffer.insert_char(0, 1, 'b');
    assert_eq!(buffer.get_line(0), Some(&String::from("ab")));

    // 测试在中间插入字符
    buffer.insert_char(0, 1, 'c');
    assert_eq!(buffer.get_line(0), Some(&String::from("acb")));

    // 测试在超出当前行长度的位置插入字符（应该自动填充空格）
    buffer.insert_char(0, 5, 'd');
    assert_eq!(buffer.get_line(0), Some(&String::from("acb  d")));

    // 测试在不存在的行中插入字符（应该自动创建新行）
    buffer.insert_char(2, 0, 'x');
    assert_eq!(buffer.line_count(), 3, "应该自动创建中间的空行");
    assert_eq!(buffer.get_line(1), Some(&String::from("")), "中间行应该是空的");
    assert_eq!(buffer.get_line(2), Some(&String::from("x")));
}

#[test]
fn test_remove_char() {
    let mut buffer = Buffer::new();
    
    // 准备测试数据
    buffer.insert_char(0, 0, 'a');
    buffer.insert_char(0, 1, 'b');
    buffer.insert_char(0, 2, 'c');
    assert_eq!(buffer.get_line(0), Some(&String::from("abc")));

    // 测试删除中间字符
    assert!(buffer.remove_char(0, 1));
    assert_eq!(buffer.get_line(0), Some(&String::from("ac")));

    // 测试删除最后一个字符
    assert!(buffer.remove_char(0, 1));
    assert_eq!(buffer.get_line(0), Some(&String::from("a")));

    // 测试删除第一个字符
    assert!(buffer.remove_char(0, 0));
    assert_eq!(buffer.get_line(0), Some(&String::from("")));

    // 测试删除空行中的字符（应该返回false）
    assert!(!buffer.remove_char(0, 0));

    // 测试删除不存在的行中的字符
    assert!(!buffer.remove_char(1, 0));
}

#[test]
fn test_insert_line() {
    let mut buffer = Buffer::new();
    
    // 测试在空buffer中插入行
    buffer.insert_line(0, String::from("first line"));
    assert_eq!(buffer.line_count(), 1);
    assert_eq!(buffer.get_line(0), Some(&String::from("first line")));

    // 测试在末尾插入行
    buffer.insert_line(1, String::from("second line"));
    assert_eq!(buffer.line_count(), 2);
    assert_eq!(buffer.get_line(1), Some(&String::from("second line")));

    // 测试在中间插入行
    buffer.insert_line(1, String::from("middle line"));
    assert_eq!(buffer.line_count(), 3);
    assert_eq!(buffer.get_line(0), Some(&String::from("first line")));
    assert_eq!(buffer.get_line(1), Some(&String::from("middle line")));
    assert_eq!(buffer.get_line(2), Some(&String::from("second line")));

    // 测试在间隔位置插入行（应该自动填充空行）
    buffer.insert_line(5, String::from("gap line"));
    assert_eq!(buffer.line_count(), 6);
    assert_eq!(buffer.get_line(3), Some(&String::from("")));
    assert_eq!(buffer.get_line(4), Some(&String::from("")));
    assert_eq!(buffer.get_line(5), Some(&String::from("gap line")));
}

#[test]
fn test_line_screen_rows() {
    let mut buffer = Buffer::new();
    let term_width = 20; // 假设终端宽度为20，减去行号占用的5个字符，实际可用宽度为15

    // 测试空buffer
    assert_eq!(buffer.line_screen_rows(0, term_width).unwrap(), 1);

    // 测试不需要折行的短行
    buffer.insert_line(0, String::from("short line"));
    assert_eq!(buffer.line_screen_rows(0, term_width).unwrap(), 1);

    // 测试刚好一行的文本
    buffer.insert_line(1, String::from("exactly15chars!"));
    assert_eq!(buffer.line_screen_rows(1, term_width).unwrap(), 1);

    // 测试需要折行的长行
    buffer.insert_line(2, String::from("this is a very long line that needs to be wrapped"));
    assert_eq!(buffer.line_screen_rows(2, term_width).unwrap(), 3);

    // 测试不存在的行
    assert_eq!(buffer.line_screen_rows(5, term_width).unwrap(), 1);
}

#[test]
fn test_get_line_part() {
    let mut buffer = Buffer::new();
    let term_width = 20; // 实际可用宽度为15（减去行号占用的5个字符）
    
    // 测试空buffer
    assert_eq!(buffer.get_line_part(0, 0, term_width), "");
    
    buffer.insert_line(0, String::from("this is a long line that needs to be wrapped"));

    // 测试第一行
    assert_eq!(buffer.get_line_part(0, 0, term_width), "this is a long ");

    // 测试第二行
    assert_eq!(buffer.get_line_part(0, 1, term_width), "line that needs");

    // 测试第三行
    assert_eq!(buffer.get_line_part(0, 2, term_width), " to be wrapped");

    // 测试超出范围的行号
    assert_eq!(buffer.get_line_part(0, 3, term_width), "");
    
    // 测试不存在的行
    assert_eq!(buffer.get_line_part(1, 0, term_width), "");
}
