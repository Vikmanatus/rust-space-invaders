use super::{
    frame::{Drawable, Frame},
    NUM_COLS, NUM_ROWS,
};

// What do we need to create our player in our 40 x 20 map
pub struct Player {
    x: usize,
    y: usize,
}

// What does the player need to be used ?
// A method to create a new player
// Move to the left
// Move to the right
// Draw the player so we can render it
impl Player {
    pub fn new() -> Self {
        Self {
            // We put it at the middle of the screen
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
        }
    }
    pub fn move_left(&mut self) {
        // As the map start at x=0, we need to check if the current x greater than 0, otherwise the player will go out of the screen
        if self.x > 0 {
            self.x -= 1;
        }
    }
    pub fn move_right(&mut self) {
        // We check if x is lower than the number of columns (40) that are playable (-1)
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A";
    }
}
