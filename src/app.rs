
use crate::board::Board;

use std::time::{Duration, Instant};


pub struct App {
    pub(crate) board: Board,
    pub(crate) cursor: (usize, usize),
    pub(crate) highlight_on: bool,
    pub(crate) last_blink_time: Instant
}

pub enum Direction {
    Up, Down, Left, Right
}

impl App {

    // Game State / Logics Warehouse

    const BLINK_INTERVAL: u64 = 500; // 500ms

    pub fn new(
        board: Board
    ) -> Self {
        App {
            board,
            cursor: (0, 0),
            highlight_on: false, // Started with no blink
            last_blink_time: Instant::now() // Ticking since instantiation
        }
    }

    pub fn tick(&mut self) {
        if self.last_blink_time.elapsed() >= Duration::from_millis(Self::BLINK_INTERVAL) {
            self.highlight_on = !self.highlight_on;
            self.last_blink_time = Instant::now();
        }
    }

    pub fn view(&self) -> String {
        self.board.render(self.cursor, self.highlight_on)
    }

    pub fn shift_cursor(&mut self, op: Direction) {

        match op {
            Direction::Left => if self.cursor.1 > 0 {
                self.cursor.1 -= 1;
            },
            Direction::Right => if self.cursor.1 < self.board.size - 1 {
                self.cursor.1 += 1;
            },
            Direction::Up => if self.cursor.0 > 0 {
                self.cursor.0 -= 1;
            },
            Direction::Down => if self.cursor.0 < self.board.size - 1 {
                self.cursor.0 += 1;
            }
        }

    }

    pub fn set_current_cell(&mut self, value: i32) {
        self.board.set_cell(self.cursor.0, self.cursor.1, value);
    }

    pub fn clear_current_cell(&mut self) {
        self.board.clear_cell(self.cursor.0, self.cursor.1);
    }

}