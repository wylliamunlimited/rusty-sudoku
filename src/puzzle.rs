

pub struct Puzzle {
    pub(crate) solution: Vec<Vec<i32>>,
    pub(crate) mask: Vec<Vec<bool>>,
}

impl Puzzle {

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

    pub fn is_valid_move(&self, row: usize, col: usize, val: i32) -> bool {
        // Row
        let mut trial_list: Vec<Option<i32>> = self.solution[row].clone();
        if trial_list[col].is_some() {
            return false; // A value already exists
        }
        trial_list[col] = Some(val);
        if !Self::has_no_duplicates(&trial_list) {
            return false; // Row conflicts
        }

        // Column
        let mut trial_list: Vec<Option<i32>> = self.solution.iter().map(|row| row[col]).collect();
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
                    trial_list.push(self.solution[r][c]);
                }
            }
        }
        if !Self::has_no_duplicates(&trial_list) {
            return false;
        }

        true
    }

}