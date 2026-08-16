use crate::sudoku::{Board, Grid, OpError};

use std::time::{Duration, Instant};

pub struct Game {
    pub(crate) board: Board,
    pub(crate) cursor: (usize, usize),
    pub(crate) highlight_on: bool,
    pub(crate) last_blink_time: Instant,
    pub(crate) last_error: Option<OpError>,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Game {
    const BLINK_INTERVAL: Duration = Duration::from_millis(500);

    pub fn new(board: Board) -> Self {
        let s: (usize, usize) = board.first_editable().unwrap_or((0, 0));
        Game {
            board,
            cursor: s,
            highlight_on: false,
            last_blink_time: Instant::now(),
            last_error: None,
        }
    }

    pub fn tick(&mut self) {
        if self.last_blink_time.elapsed() >= Self::BLINK_INTERVAL {
            self.highlight_on = !self.highlight_on;
            self.last_blink_time = Instant::now();
        }
    }

    pub fn view(&self) -> String {
        let mut out = self.board.render(self.cursor, self.highlight_on);
        if let Some(err) = &self.last_error {
            out.push_str(&format!("\n{err}\n"));
        }
        out.push_str("\n↑↓←→ move · 1-9 fill · ⌫ clear · Esc menu\n");
        out
    }

    pub fn move_cursor(&mut self, op: Direction) {
        self.last_error = None;
        self.shift_cursor(op);
    }

    pub fn click(&mut self, line: usize, column: usize) {
        let Some((row, col)) = self.board.cell_at(line, column) else {
            return;
        };
        if !self.board.is_editable(row, col) {
            return;
        }
        self.last_error = None;
        self.cursor = (row, col);
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
        self.last_error = self
            .board
            .set_cell_gated(self.cursor.0, self.cursor.1, value)
            .err();
    }

    pub fn clear_current_cell(&mut self) {
        self.last_error = self
            .board
            .clear_cell_gated(self.cursor.0, self.cursor.1)
            .err();
    }
}
