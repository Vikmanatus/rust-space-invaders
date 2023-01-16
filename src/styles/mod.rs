use std::io::Stdout;

use crossterm::{
    style::{Color, SetBackgroundColor},
    terminal::{self, size, Clear, ClearType},
    ExecutableCommand,
};

pub fn render_background_color(stdout: &mut Stdout) {
    stdout.execute(SetBackgroundColor(Color::Blue)).unwrap();
    stdout.execute(Clear(ClearType::All)).unwrap();
}
pub fn get_terminal_dimensions() -> (u16, u16) {
    size().unwrap()
}
