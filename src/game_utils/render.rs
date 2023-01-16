// So we need to render the game screen
// For this first version we will stick to the blue background in the terminal and we will draw the map of the game

// Remember that the map of the game is a rectangle constitued by NUM_ROWS and NUM_COLS
// So NUM_ROWS will correspond to the height
// And NUM_COLS will correspond to the width

// The first method we need to do is to create a welcome screen

use std::io::{Stdout, Write};

use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor, SetStyle},
    terminal::{self, Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};

use crate::styles::{get_terminal_dimensions, render_background_color, write_text_in_terminal};

// So as for the game rendering,  we will need to have access to a terminal, in that way we will be able to customize the screen
pub fn render_welcome_screen(stdout: &mut Stdout) {
    // stdout.execute(SetBackgroundColor(Color::Blue)).unwrap();
    // stdout.execute(Clear(ClearType::All)).unwrap();
    render_background_color(stdout, true);
    let dimnesions = get_terminal_dimensions();
    let welcome_text = b"Welcome to space-invaders rust";
    write_text_in_terminal(stdout, dimnesions.0 / 2, dimnesions.1 / 10, welcome_text);
    stdout.execute(SetBackgroundColor(Color::White)).unwrap();
    write_text_in_terminal(stdout, dimnesions.0 / 4, dimnesions.1 / 8, b"Play game");
    render_background_color(stdout, false);
    write_text_in_terminal(stdout, dimnesions.0 / 4, dimnesions.1 / 4, b"Options");
    write_text_in_terminal(stdout, dimnesions.0 / 4, dimnesions.1 / 3, b"Quit game");
    // To get the center we need to devide the values by 2
    println!("\n");
    // stdout.write(b"Welcome to rust-space-invaders").unwrap();
}

// To render we need to have an access to the terminal
pub fn render() {}
