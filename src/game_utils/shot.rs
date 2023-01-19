use std::time::Duration;

use rusty_time::timer::Timer;

use super::frame::{Drawable, Frame};

pub struct Shot {
    pub x: usize,
    pub y: usize,
    // Required for invaders
    pub exploding: bool,
    // The timer will keep track of our movment
    timer: Timer,
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
            // The laser will move up, one cell every 50 ms
            timer: Timer::from_millis(50),
        }
    }
    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready && !self.exploding {
            if self.y > 0 {
                self.y -= 1;
            }
            self.timer.reset();
        }
    }
    pub fn explode(&mut self) {
        self.exploding = true;
        // Changing timer so we can see the explosion
        self.timer = Timer::from_millis(250);
    }
    pub fn dead(&self) -> bool {
        (self.exploding && self.timer.ready) || (self.y == 0)
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = if self.exploding { "*" } else { "|" };
    }
}
