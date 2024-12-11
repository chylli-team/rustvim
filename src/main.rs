mod buffer;
mod cursor;
mod editor;

use std::io;
use editor::Editor;

fn main() -> io::Result<()> {
    let mut editor = Editor::new()?;
    editor.run()
}
