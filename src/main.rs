mod game_utils;
mod styles;
use std::{error::Error, io::stdout, sync::mpsc, thread::sleep, time::Duration};

use crossterm::{
    cursor::{Hide, Show},
    event::{poll, read, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rusty_audio::Audio;

use crate::{
    game_utils::{
        add_sounds, frame::new_frame, lauch_game_thread, render::render_welcome_screen,
        MenuResetRequired, MENU_ITEMS, NUM_COLS, NUM_ROWS,
    },
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
    let mut game_quit_required = false;
    // We will need a game loop in which we can create the elements of the game
    'menu_control_loop: loop {
        while poll(Duration::default())? {
            if let Event::Key(key_code) = read()? {
                match key_code.code {
                    KeyCode::Up => {
                        if current_menu_index - 1 >= 0 {
                            // Need to update the UI of the menu
                            current_menu_index -= 1;
                            style_menu_index(
                                &mut stdout,
                                current_menu_index,
                                MenuResetRequired::UpKey,
                            );
                        }
                    }
                    KeyCode::Down => {
                        if current_menu_index + 1 < MENU_ITEMS.len() as i32 {
                            current_menu_index += 1;
                            style_menu_index(
                                &mut stdout,
                                current_menu_index,
                                MenuResetRequired::DownKey,
                            );
                        }
                    }
                    KeyCode::Enter => match current_menu_index {
                        0 => {
                            audio.play("play");
                            audio.wait();
                            break 'menu_control_loop;
                        }
                        1 => {
                            game_quit_required = true;
                            play_goodbye_song(&mut audio);
                            break 'menu_control_loop;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
    if !game_quit_required {
        // Render loop in a separate thread for speedup
        let (render_tx, render_rx) = mpsc::channel();
        let render_handler = lauch_game_thread(render_rx);
        audio.play("start_game");
        audio.wait();
        'gameloop: loop {
            // Per-frame init
            let curr_frame = new_frame();
            while poll(Duration::default())? {
                if let Event::Key(key_code) = read()? {
                    match key_code.code {
                        KeyCode::Esc => {
                            play_goodbye_song(&mut audio);
                            break 'gameloop;
                        }
                        _ => {}
                    }
                }
            }
            // Draw & render
            let _ = render_tx.send(curr_frame);
            sleep(Duration::from_millis(1));
        }

        drop(render_tx);
        render_handler.join().unwrap();
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

fn play_goodbye_song(audio: &mut Audio) {
    audio.play("goodbye");
    audio.wait();
}
