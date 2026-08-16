use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

use std::collections::HashSet;

use crate::sudoku::Grid;

pub struct Puzzle {
    pub(crate) solution: Vec<Vec<i32>>,
    pub(crate) mask: Vec<Vec<bool>>,
    pub(crate) box_size: usize,
}

impl Puzzle {
    pub fn new(solution: Vec<Vec<i32>>, mask: Vec<Vec<bool>>, box_size: usize) -> Self {
        Puzzle {
            solution,
            mask,
            box_size,
        }
    }

    pub fn generate(box_size: usize) -> Self {
        let size = box_size * box_size;
        let mut rng = rand::rng();

        let solution = Self::seed(size, box_size, &mut rng);
        let mask = Self::mask(size, &mut rng);

        Puzzle {
            solution,
            mask,
            box_size,
        }
    }

    pub fn seed(size: usize, box_size: usize, rng: &mut ThreadRng) -> Vec<Vec<i32>> {
        let candidates: Vec<i32> = (1..=size).map(|n| n as i32).collect();
        let mut draft: Vec<Vec<i32>> = vec![vec![0; size]; size];
        let mut c: Vec<i32> = candidates.clone();
        c.shuffle(rng);
        let mut stack: Vec<(Vec<i32>, usize)> = vec![(c, 0)];
        draft[0][0] = stack[0].0[stack[0].1];

        loop {
            let depth = stack.len() - 1;
            let (row, col) = (depth / size, depth % size);

            if stack[depth].1 >= size {
                draft[row][col] = 0;
                stack.pop();
                if stack.is_empty() {
                    break;
                }
                let last = stack.len() - 1;
                stack[last].1 += 1;
                continue;
            }

            draft[row][col] = stack[depth].0[stack[depth].1];

            if Self::validate(&draft, row, col, box_size) {
                if stack.len() == size * size {
                    break;
                }
                let mut next = candidates.clone();
                next.shuffle(rng);
                stack.push((next, 0));
            } else {
                stack[depth].1 += 1;
            }
        }
        draft
    }

    pub fn mask(size: usize, rng: &mut ThreadRng) -> Vec<Vec<bool>> {
        let clues = size * size / 2;

        let mut positions: Vec<(usize, usize)> = (0..size)
            .flat_map(|r| (0..size).map(move |c| (r, c)))
            .collect();
        positions.shuffle(rng);

        let mut mask = vec![vec![false; size]; size];
        for &(r, c) in positions.iter().take(clues) {
            mask[r][c] = true;
        }
        mask
    }

    fn has_no_duplicates(candidates: &[i32]) -> bool {
        let mut seen = HashSet::new();

        for &cell in candidates {
            if cell != 0 && !seen.insert(cell) {
                return false;
            }
        }
        true
    }

    pub fn validate(grid: &[Vec<i32>], row: usize, col: usize, box_size: usize) -> bool {
        let aggregate: Vec<i32> = grid[row].clone();
        if !Self::has_no_duplicates(&aggregate) {
            return false;
        }

        let aggregate: Vec<i32> = grid.iter().map(|row| row[col]).collect();
        if !Self::has_no_duplicates(&aggregate) {
            return false;
        }

        let box_row = (row / box_size) * box_size;
        let box_col = (col / box_size) * box_size;

        let aggregate: Vec<i32> = grid[box_row..box_row + box_size]
            .iter()
            .flat_map(|r| &r[box_col..box_col + box_size])
            .copied()
            .collect();
        if !Self::has_no_duplicates(&aggregate) {
            return false;
        }

        true
    }
}

impl Grid for Puzzle {
    fn size(&self) -> usize {
        self.solution.len()
    }
    fn box_size(&self) -> usize {
        self.box_size
    }
    fn cell_str(&self, row: usize, col: usize) -> String {
        let width = self.cell_width();
        match self.mask[row][col] {
            true => match self.solution[row][col] {
                0 => " ".repeat(width),
                n => format!("{n:^width$}"),
            },
            false => format!("{:^width$}", 'x'),
        }
    }
}
