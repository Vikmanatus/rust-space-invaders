mod game_utils;
use std::{error::Error, io::stdout, time::Duration};

use crossterm::{
    cursor::{Hide, Show},
    event::{poll, read, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rusty_audio::Audio;

use crate::game_utils::{add_sounds, render::render_welcome_screen, NUM_COLS, NUM_ROWS};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to space invaders Rust version");
    println!("Number of rows: {}", NUM_ROWS);
    println!("Number of columns: {}", NUM_COLS);
    let mut audio = Audio::new();
    // Files path of the audio files required for the game
    let audio_files_path = "./src/audio".to_string();
    // Adding required sound in module
    add_sounds(&mut audio, audio_files_path);
    // As we will need access to the terminal we need to instantiate access to it with the io module
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    // Hiding content of users terminal and providing a similar result as a vim editor in which we can draw and render the elements of the game
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    audio.play("welcome");



    // We will need a game loop in which we can create the elements of the game
    'gameloop: loop {
        while poll(Duration::default())? {
            if let Event::Key(key_code) = read()? {
                match key_code.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }
    }

    // audio.wait(); // Block until sounds welcome sound finishes playing

    // Killing the app and terminating the program
    // Need to create a render engine so we can draw elements into the terminal
    stdout.execute(Show)?;
    // Return to the original terminal
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
