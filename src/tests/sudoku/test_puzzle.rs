use crate::sudoku::Puzzle;
use std::collections::HashSet;

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

    #[test]
    fn test_seed() {
        let size: usize = 9;
        let box_size: usize = 3;
        let mut rng = rand::rng();

        let solution = Puzzle::seed(size, box_size, &mut rng);

        let expected: HashSet<i32> = (1..=size as i32).collect();

        for row in &solution {
            assert_eq!(row.iter().copied().collect::<HashSet<_>>(), expected);
        }

        for col in 0..size {
            let col: HashSet<i32> = solution.iter().map(|r| r[col]).collect();
            assert_eq!(col, expected);
        }

        let solution = &solution;
        for br in (0..size).step_by(box_size) {
            for bc in (0..size).step_by(box_size) {
                let boxed: HashSet<i32> = (br..br + box_size)
                    .flat_map(|r| (bc..bc + box_size).map(move |c| solution[r][c]))
                    .collect();
                assert_eq!(boxed, expected);
            }
        }
    }

    #[test]
    fn test_generate_solution_valid() {
        let puzzle: Puzzle = Puzzle::generate(9, 3);
        let sol = &puzzle.solution;

        assert_eq!(sol.len(), 9);
        assert!(sol.iter().all(|r| r.len() == 9));

        for r in 0..9 {
            for c in 0..9 {
                assert_ne!(sol[r][c], 0, "cell ({r},{c}) left empty");
                assert!(Puzzle::validate(sol, r, c, 3), "conflict at ({r},{c})");
            }
        }
    }
}
