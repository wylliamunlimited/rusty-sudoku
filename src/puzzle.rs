
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

use std::collections::HashSet;

pub struct Puzzle {
    pub(crate) solution: Vec<Vec<i32>>,
    pub(crate) mask: Vec<Vec<bool>>,
}

impl Puzzle {

    const lookup_range: Vec<i32> = (1..=9).collect();

    pub fn generate(size: usize, box_size: usize) {

        let grid: Vec<Vec<i32>> = Self::seed();
        let mask: Vec<Vec<bool>> = Self::mask();
        
        Puzzle {
            solution: grid,
            mask: mask,
        }

    }

    pub fn mask(size: usize, rng: &mut ThreadRng) -> Vec<Vec<bool> {

        let clues = size * size / 2; // default difficulty for now

        let mut positions: Vec<(usize, usize)> =
            (0..size).flat_map(|r| (0..size).map(move |c| (r, c))).collect();
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

    pub fn validate(
        grid: &[Vec<i32>], row: usize, col: usize, box_size: usize
    ) -> bool {

        // row check
        let aggregate: Vec<i32> = grid[row].clone();
        if !Self::has_no_duplicates(&aggregate) {
            return false; // Row conflicts
        }

        // col check
        let aggregate: Vec<i32> = grid.iter().map(|row| row[col]).collect();
        if !Self::has_no_duplicates(&aggregate) {
            return false; // Col conflicts
        }
        
        // Box
        let mut aggregate: Vec<i32> = Vec::new();
        let box_row = (row / box_size) * box_size;
        let box_col = (col / box_size) * box_size;

        for r in box_row..box_row + box_size {
            for c in box_col..box_col + box_size {
                aggregate.push(grid[r][c]);
            }
        }
        if !Self::has_no_duplicates(&aggregate) {
            return false; // Box conflicts
        }

        true

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask() {
        let size: usize = 9;
        let mut rng = rand::rng();

        let mask: Vec<Vec<bool>> = Puzzle::mask(size, &mut rng);

        assert_eq!(mask.len(), size);
        assert!(mask.iter().all(|row| row.len() == size));

        let revealed = mask.iter().flatten().filter(|&&shown| shown).count();
        assert_eq!(revealed, size * size / 2);
    }
}