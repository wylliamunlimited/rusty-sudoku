
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

use std::collections::HashSet;

pub struct Puzzle {
    pub(crate) solution: Vec<Vec<i32>>,
    pub(crate) mask: Vec<Vec<bool>>,
}

impl Puzzle {

    pub fn generate(size: usize, box_size: usize) -> Puzzle {
        let mut rng = rand::rng();

        let solution = Self::seed(size, box_size, &mut rng);
        let mask = Self::mask(size, &mut rng);

        Puzzle { solution, mask }
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

            if (stack[depth].1 >= size) {
                draft[row][col] = 0;
                stack.pop();
                if stack.len() == 0 { break; }
                let last = stack.len() - 1;
                stack[last].1 += 1;
                continue;
            }

            draft[row][col] = stack[depth].0[stack[depth].1];

            if Self::validate(&draft, row, col, box_size) {
                if stack.len() == size * size { break; }
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

    #[test]
    fn test_validate() {
        let grid: Vec<Vec<i32>> = vec![
            vec![5, 5, 4, 6, 7, 8, 9, 1, 2],
            vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
            vec![1, 9, 0, 3, 4, 2, 5, 6, 7],
            vec![8, 5, 9, 7, 6, 1, 4, 0, 3],
            vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
            vec![7, 1, 3, 9, 2, 4, 0, 5, 6],
            vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
            vec![2, 8, 7, 0, 1, 9, 6, 3, 0],
            vec![3, 4, 5, 2, 8, 6, 1, 7, 9],
        ];

        assert!(!Puzzle::validate(&grid, 0, 1, 3));
    }
}