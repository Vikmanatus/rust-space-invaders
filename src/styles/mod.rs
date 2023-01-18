use std::io::{Stdout, Write};

use crossterm::{
    cursor::{MoveTo, MoveToNextLine},
    style::{self, style, Color, SetBackgroundColor, Stylize},
    terminal::{self, size, Clear, ClearType},
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
pub fn write_menu_options(stdout: &mut Stdout, menu_options: [&str; 3], initial_y: u16) {
    let mut y_position = initial_y + 1;
    for item in menu_options {
        write_centered_text(stdout, y_position, item.as_bytes());
        y_position += 1;
    }
}
pub fn write_text_in_terminal(stdout: &mut Stdout, x_position: u16, y_position: u16, buf: &[u8]) {
    stdout.execute(MoveTo(x_position, y_position)).unwrap();
    stdout.write_all(buf).unwrap();
}

pub fn write_centered_text(stdout: &mut Stdout, y_position: u16, buf: &[u8]) {
    let x = (get_terminal_dimensions().0 - buf.len() as u16) / 2;
    stdout.execute(MoveTo(x, y_position)).unwrap();
    stdout.write_all(buf).unwrap();
}
