use std::fs::read_dir;

use regex::Regex;
use rusty_audio::Audio;

use crate::styles::get_terminal_dimensions;


pub mod render;

pub const NUM_ROWS:usize = 20;
pub const NUM_COLS: usize = 40;
pub const MENU_ITEMS:[&str;8] = ["Play game", "Options","Information", "Quit game", "testing", "testing2","testing3","testing4"];
#[derive(PartialEq)]
pub enum MenuResetRequired {
    UpKey,
    DownKey,
    None
}
pub fn add_sounds(audio: &mut Audio, path: String) {
    println!("{}" , path);
    
    for file in read_dir(path).unwrap() {
        let filepath = file.unwrap().path().display().to_string();
        let re = Regex::new(r"(?P<filename>\w+)\.\w+$").unwrap();
        let filename = re
            .captures(&filepath)
            .unwrap()
            .name("filename")
            .unwrap()
            .as_str();
        println!("{} {}" , filename, filepath);
        audio.add::<&str, &str>(filename, filepath.as_ref())
    }
}