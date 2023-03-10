use std::{fs::read_dir, thread::{self, JoinHandle}, io, sync::mpsc::Receiver};

use regex::Regex;
use rusty_audio::Audio;

use self::{render::render, frame::Frame};

pub mod frame;
pub mod render;
pub mod player;
pub const NUM_ROWS: usize = 20;
pub const NUM_COLS: usize = 40;
pub const MENU_ITEMS: [&str; 2] = ["Play game", "Quit game"];
#[derive(PartialEq)]
pub enum MenuResetRequired {
    UpKey,
    DownKey,
    None,
}
pub fn add_sounds(audio: &mut Audio, path: String) {
    println!("{}", path);

    for file in read_dir(path).unwrap() {
        let filepath = file.unwrap().path().display().to_string();
        let re = Regex::new(r"(?P<filename>\w+)\.\w+$").unwrap();
        let filename = re
            .captures(&filepath)
            .unwrap()
            .name("filename")
            .unwrap()
            .as_str();
        println!("{} {}", filename, filepath);
        audio.add::<&str, &str>(filename, filepath.as_ref())
    }
}

pub fn lauch_game_thread(render_rx: Receiver<Frame>)-> JoinHandle<()> {
    let render_handler = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });
    render_handler
}
