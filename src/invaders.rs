use std::{cmp::max, time::Duration};

use rusty_time::timer::Timer;

use crate::{
    frame::{Drawable, Frame},
    NUM_COLS, NUM_ROWS, SPARSE_VAL,
};

pub struct Invader {
    pub x: usize,
    pub y: usize,
}

pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if x > 1
                    && x < NUM_COLS - 2
                    && y > 0
                    && y < 9
                    && y % SPARSE_VAL == 0
                    && x % SPARSE_VAL == 0
                {
                    army.push(Invader { x, y });
                }
            }
        }
        Self {
            army,
            move_timer: Timer::from_millis(2000),
            direction: 1,
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);

        if self.move_timer.ready {
            self.move_timer.reset();
            let mut down = false;
            if self.direction == -1 {
                let min_x = self.army.iter().map(|iv| iv.x).min().unwrap_or(0);
                if min_x == 0 {
                    self.direction = 1;
                    down = true;
                }
            } else {
                let max_x = self.army.iter().map(|iv| iv.x).max().unwrap_or(0);
                if max_x == NUM_COLS - 1 {
                    self.direction = -1;
                    down = true;
                }
            }
            if down {
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }
            return true;
        }
        false
    }
    pub fn all_kill(&self) -> bool {
        self.army.is_empty()
    }
    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|i| i.y).max().unwrap_or(0) >= NUM_ROWS - 1
    }

    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> bool {
        if let Some(idx) = self.army.iter().position(|i| i.x == x && i.y == y) {
            self.army.remove(idx);
            true
        } else {
            false
        }
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for inv in self.army.iter() {
            frame[inv.x][inv.y] = if (self.move_timer.time_left.as_secs_f32()
                / self.move_timer.duration.as_secs_f32())
                > 0.5
            {
                "x"
            } else {
                "+"
            };
        }
    }
}
