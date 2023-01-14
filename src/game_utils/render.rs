// So we need to render the game screen
// For this first version we will stick to the blue background in the terminal and we will draw the map of the game

// Remember that the map of the game is a rectangle constitued by NUM_ROWS and NUM_COLS
// So NUM_ROWS will correspond to the height
// And NUM_COLS will correspond to the width

// The first method we need to do is to create a welcome screen

use std::io::{Stdout, Write};

use crossterm::{QueueableCommand, style::{SetBackgroundColor, Color}, terminal::{Clear, ClearType,}, ExecutableCommand};

// So as for the game rendering,  we will need to have access to a terminal, in that way we will be able to customize the screen
pub fn render_welcome_screen(stdout: &mut Stdout) {
    // stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
    // stdout.queue(Clear(ClearType::All)).unwrap();
    // stdout.flush().unwrap();
    stdout.write(b"Some value in terminal").unwrap();
    // stdout.write(b"Welcome to rust-space-invaders").unwrap();
}

// To render we need to have an access to the terminal
pub fn render() {
    
}