
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

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
            grid: grid,
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

    fn has_no_duplicates(candidates: &[Option<i32>]) -> bool {
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

}