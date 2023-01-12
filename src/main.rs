mod game_utils;
use std::{error::Error, io::stdout};

use crossterm::{terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, cursor::{Hide, Show}, ExecutableCommand};

use crate::game_utils::{NUM_COLS, NUM_ROWS};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to space invaders Rust version");
    println!("Number of rows: {}", NUM_ROWS);
    println!("Number of columns: {}", NUM_COLS);

    // As we will need access to the terminal we need to instantiate access to it with the io module
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
