// So we need to render the game screen
// For this first version we will stick to the blue background in the terminal and we will draw the map of the game

// Remember that the map of the game is a rectangle constitued by NUM_ROWS and NUM_COLS
// So NUM_ROWS will correspond to the height
// And NUM_COLS will correspond to the width

// The first method we need to do is to create a welcome screen

use std::io::{Stdout, Write};

use crossterm::{ExecutableCommand, terminal::{Clear, ClearType}, style::{SetBackgroundColor, Color}, QueueableCommand, cursor::MoveTo};

use crate::{styles::{
    get_terminal_dimensions, render_background_color, write_menu_options, write_centered_text,
}, game_utils::MENU_ITEMS};

use super::frame::Frame;

// So as for the game rendering,  we will need to have access to a terminal, in that way we will be able to customize the screen
pub fn render_welcome_screen(stdout: &mut Stdout) {
    render_background_color(stdout, true);
    let dimnesions = get_terminal_dimensions();
    let welcome_text = b"Welcome to space-invaders rust";
    let menu_items = MENU_ITEMS;

    write_centered_text(stdout, dimnesions.1 / 10, welcome_text);
    write_menu_options(stdout, menu_items, dimnesions.1 / 7);

    println!("\n");
}

// To render we need to have an access to the terminal
pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    // We only need to render what has changed, so we have a current frame and a last frame
    if force {
        stdout.execute(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.execute(Clear(ClearType::All)).unwrap();
        stdout.execute(SetBackgroundColor(Color::Black)).unwrap();
    }

    for (x, col) in curr_frame.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            if *s != last_frame[x][y] || force {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                println!("{}", *s);
            }
        }
    }
    stdout.flush().unwrap();

}
