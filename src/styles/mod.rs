use std::io::{Stdout, Write};

use crossterm::{
    cursor::{MoveTo, MoveToNextLine},
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
pub fn write_menu_options(
    stdout: &mut Stdout,
    menu_options: [&str; 3],
    initial_x: u16,
    initial_y: u16,
) {
    stdout.execute(MoveTo(0, initial_y)).unwrap();
    let mut loop_number = 0;
    for item in menu_options {
        loop_number += 1;
        if loop_number == 1 {
            stdout.execute(SetBackgroundColor(Color::White)).unwrap();
        } else {
            stdout.execute(SetBackgroundColor(Color::Blue)).unwrap();
        }
        stdout.write_all(item.as_bytes()).unwrap();
        stdout.execute(MoveToNextLine(1)).unwrap();
    }
}
pub fn write_text_in_terminal(stdout: &mut Stdout, x_position: u16, y_position: u16, buf: &[u8]) {
    stdout.execute(MoveTo(x_position, y_position)).unwrap();
    stdout.write_all(buf).unwrap();
}
