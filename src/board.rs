use std::collections::HashSet;
use std::fmt;

use crate::grid::Grid;
use crate::puzzle::Puzzle;

pub struct Board {
    pub(crate) size: usize,
    pub(crate) box_size: usize,
    pub(crate) cells: Vec<Vec<Option<i32>>>,
    pub(crate) puzzle: Option<Puzzle>,
}

pub enum OpError {
    NotEditable,
    Occupied,
    Conflicts,
    Incorrect,
}

impl Board {
    pub fn from_puzzle(puzzle: Puzzle) -> Self {
        let size = puzzle.size();
        let box_size = (puzzle.size() as f64).sqrt() as usize;
        let cells: Vec<Vec<Option<i32>>> = puzzle
            .mask
            .iter()
            .zip(&puzzle.solution)
            .map(|(mask_row, sol_row)| {
                mask_row
                    .iter()
                    .zip(sol_row)
                    .map(|(&shown, &val)| shown.then_some(val))
                    .collect()
            })
            .collect();
        Board {
            size,
            box_size,
            cells,
            puzzle: Some(puzzle),
        }
    }

    pub fn new(size: usize, box_size: usize) -> Self {
        Board {
            size,
            box_size,
            cells: vec![vec![None; size]; size],
            puzzle: None,
        }
    }

    pub fn has_no_duplicates(candidates: &[Option<i32>]) -> bool {
        let mut seen = HashSet::new();

        for cell in candidates {
            if let Some(d) = cell
                && !seen.insert(d)
            {
                return false;
            }
        }
        true
    }

    pub fn is_editable(&self, row: usize, col: usize) -> bool {
        match &self.puzzle {
            None => true,
            Some(n) => !n.mask[row][col],
        }
    }

    pub fn is_valid_move(&self, row: usize, col: usize, val: i32) -> bool {
        // Row
        let mut trial_list: Vec<Option<i32>> = self.cells[row].clone();
        if trial_list[col].is_some() {
            return false; // A value already exists
        }
        trial_list[col] = Some(val);
        if !Self::has_no_duplicates(&trial_list) {
            return false; // Row conflicts
        }

        // Column
        let mut trial_list: Vec<Option<i32>> = self.cells.iter().map(|row| row[col]).collect();
        if trial_list[row].is_some() {
            return false; // A value already exists
        }
        trial_list[row] = Some(val);
        if !Self::has_no_duplicates(&trial_list) {
            return false; // Row conflicts
        }

        // Box
        let mut trial_list: Vec<Option<i32>> = Vec::new();
        let box_row = (row / self.box_size) * self.box_size;
        let box_col = (col / self.box_size) * self.box_size;

        for r in box_row..box_row + self.box_size {
            for c in box_col..box_col + self.box_size {
                if r == row && c == col {
                    trial_list.push(Some(val));
                } else {
                    trial_list.push(self.cells[r][c]);
                }
            }
        }
        if !Self::has_no_duplicates(&trial_list) {
            return false;
        }

        true
    }

    pub fn is_correct_move(&self, row: usize, col: usize, val: i32) -> bool {
        self.puzzle
            .as_ref()
            .is_some_and(|p| p.solution[row][col] == val)
    }

    pub fn set_cell(&mut self, row: usize, col: usize, value: i32) {
        self.cells[row][col] = Some(value);
    }

    pub fn set_cell_gated(&mut self, row: usize, col: usize, value: i32) -> Result<(), OpError> {
        if !self.is_editable(row, col) {
            return Err(OpError::NotEditable);
        }
        if self.cells[row][col].is_some() {
            return Err(OpError::Occupied);
        }
        if !self.is_valid_move(row, col, value) {
            return Err(OpError::Conflicts);
        }
        if !self.is_correct_move(row, col, value) {
            return Err(OpError::Incorrect);
        }
        self.set_cell(row, col, value);
        Ok(())
    }

    pub fn clear_cell(&mut self, row: usize, col: usize) {
        self.cells[row][col] = None;
    }

    pub fn clear_cell_gated(&mut self, row: usize, col: usize) -> Result<(), OpError> {
        if !self.is_editable(row, col) {
            return Err(OpError::NotEditable);
        }
        self.clear_cell(row, col);
        Ok(())
    }

    pub fn first_editable(&self) -> Option<(usize, usize)> {
        (0..self.size)
            .flat_map(|r| (0..self.size).map(move |c| (r, c)))
            .find(|&(r, c)| self.is_editable(r, c))
    }

    pub fn render(&self, cursor: (usize, usize), blink: bool) -> String {
        self.render_grid(blink.then_some(cursor))
    }
}

impl Grid for Board {
    fn size(&self) -> usize {
        self.size
    }
    fn box_size(&self) -> usize {
        self.box_size
    }
    fn cell_str(&self, row: usize, col: usize) -> String {
        match self.cells[row][col] {
            None => String::from("   "),
            Some(n) => format!(" {n} "),
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render_grid(None))
    }
}

impl fmt::Display for OpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            OpError::NotEditable => "That cell is part of the puzzle and can't be changed.",
            OpError::Occupied => "That cell already has a number in it.",
            OpError::Conflicts => "That number already appears in this row, column, or box.",
            OpError::Incorrect => "That's not the right number for this cell.",
        };
        write!(f, "{message}")
    }
}
