use crate::board::Board;

use std::time::{Duration, Instant};

pub struct App {
    pub(crate) board: Board,
    pub(crate) cursor: (usize, usize),
    pub(crate) highlight_on: bool,
    pub(crate) last_blink_time: Instant,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum Action {
    Move(Direction),
    SetDigit(i32),
    ClearCell,
    Quit,
}

impl App {
    // Game State / Logics Warehouse

    const BLINK_INTERVAL: u64 = 500; // 500ms

    pub fn new(board: Board) -> Self {
        let s: (usize, usize) = board.first_editable().unwrap_or((0, 0));
        App {
            board,
            cursor: s,
            highlight_on: false,             // Started with no blink
            last_blink_time: Instant::now(), // Ticking since instantiation
        }
    }

    pub fn handle_action(&mut self, action: Action) -> bool {
        match action {
            Action::Move(d) => {
                self.shift_cursor(d);
                true
            }
            Action::SetDigit(val) => {
                self.set_current_cell(val);
                true
            }
            Action::ClearCell => {
                self.clear_current_cell();
                true
            }
            Action::Quit => {
                false // Terminate
            }
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

    fn step(&self, (r, c): (usize, usize), op: Direction) -> (usize, usize) {
        let max = self.board.size - 1;
        match op {
            Direction::Up => (r.saturating_sub(1), c),
            Direction::Down => ((r + 1).min(max), c),
            Direction::Left => (r, c.saturating_sub(1)),
            Direction::Right => (r, (c + 1).min(max)),
        }
    }

    pub fn shift_cursor(&mut self, op: Direction) {
        let mut candidate = self.step(self.cursor, op);
        while !self.board.is_editable(candidate.0, candidate.1) {
            let next = self.step(candidate, op);
            if next == candidate {
                return;
            }
            candidate = next;
        }
        self.cursor = candidate;
    }

    pub fn set_current_cell(&mut self, value: i32) {
        self.board
            .set_cell_gated(self.cursor.0, self.cursor.1, value);
    }

    pub fn clear_current_cell(&mut self) {
        self.board.clear_cell_gated(self.cursor.0, self.cursor.1);
    }
}
