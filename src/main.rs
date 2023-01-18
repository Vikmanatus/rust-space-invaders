mod game_utils;
mod styles;
use std::{error::Error, io::stdout, time::Duration};

use crossterm::{
    cursor::{Hide, Show},
    event::{poll, read, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rusty_audio::Audio;

use crate::{
    game_utils::{add_sounds, render::render_welcome_screen, MENU_ITEMS, NUM_COLS, NUM_ROWS, MenuResetRequired},
    styles::style_menu_index,
};

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
    render_welcome_screen(&mut stdout);

    let mut current_menu_index = 0;
    style_menu_index(&mut stdout, current_menu_index, MenuResetRequired::None);
    // We will need a game loop in which we can create the elements of the game
    'gameloop: loop {
        while poll(Duration::default())? {
            if let Event::Key(key_code) = read()? {
                match key_code.code {
                    KeyCode::Up => {
                        if current_menu_index - 1 >= 0 {
                            // Need to update the UI of the menu
                            current_menu_index -= 1;
                            style_menu_index(&mut stdout, current_menu_index, MenuResetRequired::UpKey);
                        }
                    }
                    KeyCode::Down => {
                        if current_menu_index + 1 < MENU_ITEMS.len() as i32 {
                            current_menu_index+=1;
                            style_menu_index(&mut stdout, current_menu_index, MenuResetRequired::DownKey);
                        }
                    }
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

// use crossterm::{event::{read, Event, KeyCode}, terminal::enable_raw_mode};

// fn main() {
//     enable_raw_mode().unwrap();

//     let mut selected_option = 1;

//     loop {
//         println!("Welcome to my game!");
//         println!("1. Start game");
//         println!("2. Options");
//         println!("3. Exit");

//         match read().unwrap() {
//             Event::Key(key) => {
//                 match key.code {
//                     KeyCode::Up => {
//                         if selected_option > 1 {
//                             selected_option -= 1;
//                         }
//                     }
//                     KeyCode::Down => {
//                         if selected_option < 3 {
//                             selected_option += 1;
//                         }
//                     }
//                     KeyCode::Enter => match selected_option {
//                         1 => start_game(),
//                         2 => options(),
//                         3 => exit(),
//                         _ => continue,
//                     },
//                     _ => continue,
//                 }
//             }
//             _ => continue,
//         }
//     }
// }
