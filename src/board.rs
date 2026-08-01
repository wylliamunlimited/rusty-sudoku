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

    pub fn set_cell_gated(&mut self, row: usize, col: usize, value: i32) -> bool {
        if self.is_valid_move(row, col, value) && self.is_correct_move(row, col, value) {
            self.set_cell(row, col, value);
            true
        } else {
            false
        }
    }

    pub fn clear_cell(&mut self, row: usize, col: usize) {
        self.cells[row][col] = None;
    }

    pub fn render(&self, cursor: (usize, usize), blink: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.top_border());

        for row_id in 0..self.size {
            let hl = if row_id == cursor.0 && blink {
                Some(cursor.1)
            } else {
                None
            };
            output.push_str(&self.format_row(row_id, hl));

            if row_id == self.size - 1 {
                output.push_str(&self.bottom_border());
            } else if (row_id + 1) % self.box_size == 0 {
                output.push_str(&self.thick_middle_border());
            } else {
                output.push_str(&self.thin_middle_border());
            }
        }

        output
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
        write!(f, "{}", self.top_border())?;

        for row_id in 0..self.size {
            write!(f, "{}", self.format_row(row_id, None))?;

            if row_id == self.size - 1 {
                write!(f, "{}", self.bottom_border())?;
            } else if (row_id + 1) % self.box_size == 0 {
                write!(f, "{}", self.thick_middle_border())?;
            } else {
                write!(f, "{}", self.thin_middle_border())?;
            }
        }

        Ok(())
    }
}

pub struct BorderStyle {
    pub(crate) left: char,
    pub(crate) fill: &'static str,
    pub(crate) cell: char,
    pub(crate) box_junction: char,
    pub(crate) right: char,
}

impl BorderStyle {
    pub(crate) const TOP: BorderStyle = BorderStyle {
        left: '╔',
        fill: "═══",
        cell: '╤',
        box_junction: '╦',
        right: '╗',
    };

    pub(crate) const BOTTOM: BorderStyle = BorderStyle {
        left: '╚',
        fill: "═══",
        cell: '╧',
        box_junction: '╩',
        right: '╝',
    };

    pub(crate) const THICK: BorderStyle = BorderStyle {
        left: '╠',
        fill: "═══",
        cell: '╪',
        box_junction: '╬',
        right: '╣',
    };

    pub(crate) const THIN: BorderStyle = BorderStyle {
        left: '╟',
        fill: "───",
        cell: '┼',
        box_junction: '╫',
        right: '╢',
    };
}
