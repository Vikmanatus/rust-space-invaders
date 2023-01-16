use std::io::{Stdout, Write};

use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor},
    terminal::{size, Clear, ClearType},
    ExecutableCommand,
};

pub fn render_background_color(stdout: &mut Stdout, clear: bool) {
    stdout.execute(SetBackgroundColor(Color::Blue)).unwrap();
    if clear {
        stdout.execute(Clear(ClearType::All)).unwrap();
    }
}
pub fn get_terminal_dimensions() -> (u16, u16) {
    size().unwrap()
}

pub fn write_text_in_terminal(stdout: &mut Stdout, x_position: u16, y_position: u16, buf: &[u8]) {
    stdout.execute(MoveTo(x_position, y_position)).unwrap();
    stdout.write_all(buf).unwrap();
}
