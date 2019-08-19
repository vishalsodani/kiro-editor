// Refs:
//   Build Your Own Text Editor: https://viewsourcecode.org/snaptoken/kilo/index.html
//   VT100 User Guide: https://vt100.net/docs/vt100-ug/chapter3.html

use kiro::input::StdinRawMode;
use kiro::Editor;
use std::io;

fn main() -> io::Result<()> {
    let input = StdinRawMode::new()?.input_keys();
    let mut editor = Editor::new(term_size::dimensions_stdout(), input);
    if let Some(arg) = std::env::args().skip(1).next() {
        editor.open_file(arg)?;
    }
    editor.run()
}
