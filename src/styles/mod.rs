use std::io::{Stdout, Write};

use crossterm::{
    cursor::MoveTo,
    style::{
        Color, PrintStyledContent, ResetColor, SetBackgroundColor, SetForegroundColor, Stylize,
    },
    terminal::{size, Clear, ClearType},
    ExecutableCommand,
};

use crate::game_utils::{MenuResetRequired, MENU_ITEMS};

pub fn render_background_color(stdout: &mut Stdout, clear: bool) {
    stdout.execute(SetBackgroundColor(Color::Blue)).unwrap();
    if clear {
        stdout.execute(Clear(ClearType::All)).unwrap();
    }
}
pub fn get_terminal_dimensions() -> (u16, u16) {
    size().unwrap()
}
pub fn write_menu_options(stdout: &mut Stdout, menu_options: [&str; 9], initial_y: u16) {
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
    let x = calculate_x_center_text(buf);
    stdout.execute(MoveTo(x, y_position)).unwrap();
    stdout.write_all(buf).unwrap();
}
pub fn calculate_x_center_text(buf: &[u8]) -> u16 {
    (get_terminal_dimensions().0 - buf.len() as u16) / 2
}

fn create_spaces_string(num_spaces: i32) -> String {
    let mut spaces = String::new();
    for _ in 0..num_spaces {
        spaces.push(' ');
    }
    spaces
}
pub fn style_menu_index(stdout: &mut Stdout, index: i32, is_reset_required: MenuResetRequired) {
    let dimensions = get_terminal_dimensions();

    if is_reset_required == MenuResetRequired::DownKey {
        let previous_menu_item = MENU_ITEMS[index as usize - 1];
        let previous_y_center = dimensions.1 / 7 + index as u16;

        style_menu_item(previous_menu_item, previous_y_center, stdout, Color::Blue);
    }

    if is_reset_required == MenuResetRequired::UpKey {
        let next_menu_item = MENU_ITEMS[index as usize + 1];
        let next_y_center = dimensions.1 / 7 + index as u16 + 1 + 1;

        style_menu_item(next_menu_item, next_y_center, stdout, Color::Blue);
    }

    let menu_item = MENU_ITEMS[index as usize];
    let y_center = dimensions.1 / 7 + index as u16 + 1;

    style_menu_item(menu_item, y_center, stdout, Color::White);
}

pub fn style_menu_item(
    menu_item: &str,
    y_center: u16,
    stdout: &mut Stdout,
    background_color: Color,
) {
    let x_center = calculate_x_center_text(menu_item.as_bytes());

    stdout.execute(MoveTo(x_center, y_center)).unwrap();
    stdout
        .execute(PrintStyledContent(
            menu_item.with(Color::Black).on(background_color),
        ))
        .unwrap();
}
