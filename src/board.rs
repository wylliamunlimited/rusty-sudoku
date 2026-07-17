use std::fmt;
use std::collections::HashSet;

pub struct Board {
    pub(crate) size: usize,
    pub(crate) box_size: usize,
    pub(crate) cells: Vec<Vec<Option<i32>>>,
}

impl Board {
    pub fn new(size: usize, box_size: usize) -> Self {
        Board {
            size,
            box_size,
            cells: vec![vec![None; size]; size],
        }
    }

    fn has_no_duplicates(candidates: &[Option<i32>]) -> bool {
        let mut seen = HashSet::new();

        for cell in candidates {
            if let Some(d) = cell && !seen.insert(d){
                return false;
            }
        }
        true
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

    pub fn set_cell(&mut self, row: usize, col: usize, value: i32) {
        self.cells[row][col] = Some(value);
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

    fn format_row(&self, row_id: usize, highlight_col: Option<usize>) -> String {
        let mut output = String::new();

        let roster: &Vec<Option<i32>> = &self.cells[row_id];

        output.push('║');

        for i in 0..self.size {
            let cell = match roster[i] {
                None => String::from("   "),
                Some(n) => format!(" {n} "),
            };

            if highlight_col == Some(i) {
                output.push_str("\x1B[7m"); // before
                output.push_str(&cell); // the 3 chars
                output.push_str("\x1B[0m"); // after
            } else {
                output.push_str(&cell);
            }

            if i == self.size - 1 {
                output.push('║');
            } else if (i + 1) % self.box_size == 0 {
                output.push('║');
            } else {
                output.push('│');
            }
        }

        output.push('\n');
        output
    }

    fn border(&self, style: &BorderStyle) -> String {
        let mut output = String::new();

        output.push(style.left);

        for i in 0..self.size {
            output.push_str(style.fill);

            if i == self.size - 1 {
                output.push(style.right);
            } else if (i + 1) % self.box_size == 0 {
                output.push(style.box_junction);
            } else {
                output.push(style.cell);
            }
        }

        output.push('\n');
        output
    }

    fn top_border(&self) -> String {
        self.border(&BorderStyle::TOP)
    }

    fn bottom_border(&self) -> String {
        self.border(&BorderStyle::BOTTOM)
    }

    fn thick_middle_border(&self) -> String {
        self.border(&BorderStyle::THICK)
    }

    fn thin_middle_border(&self) -> String {
        self.border(&BorderStyle::THIN)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_board() -> Board {
        let data: Vec<Vec<Option<i32>>> = vec![vec![Some(1), Some(2), Some(3), None, None, None, None, None, None]; 9];
        let sample: Board = Board {
            size: 9,
            box_size: 3,
            cells: data,
        };

        sample
    }

    #[test]
    fn test_top_border() {
        let rendered_format: String = sample_board().top_border();

        assert_eq!(rendered_format, "╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗\n");
    }

    #[test]
    fn test_bottom_border() {
        let rendered_format: String = sample_board().bottom_border();

        assert_eq!(rendered_format, "╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝\n")
    }

    #[test]
    fn test_thin_border() {
        let rendered_format: String = sample_board().thin_middle_border();

        assert_eq!(rendered_format, "╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n")
    }

    #[test]
    fn test_thick_border() {
        let rendered_format: String = sample_board().thick_middle_border();

        assert_eq!(rendered_format, "╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣\n")
    }

    #[test]
    fn test_row_format() {
        let rendered_format: String = sample_board().format_row(1, None);

        assert_eq!(rendered_format, "║ 1 │ 2 │ 3 ║   │   │   ║   │   │   ║\n");
    }

    #[test]
    fn test_row_format_with_col() {
        // Blink is handled by render(), not format_row — Some(col) always highlights.
        let rendered_format: String = sample_board().format_row(1, Some(1));

        assert_eq!(
            rendered_format,
            "║ 1 │\x1B[7m 2 \x1B[0m│ 3 ║   │   │   ║   │   │   ║\n"
        );
    }

    #[test]
    fn test_board() {
        let data: Vec<Vec<Option<i32>>> = vec![
            vec![Some(5), Some(3), Some(4), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2)],
            vec![Some(6), Some(7), Some(2), Some(1), Some(9), Some(5), Some(3), Some(4), Some(8)],
            vec![Some(1), Some(9), Some(8), Some(3), Some(4), Some(2), Some(5), Some(6), Some(7)],
            vec![Some(8), Some(5), Some(9), Some(7), Some(6), Some(1), Some(4), Some(2), Some(3)],
            vec![Some(4), Some(2), Some(6), Some(8), Some(5), Some(3), Some(7), Some(9), Some(1)],
            vec![Some(7), Some(1), Some(3), Some(9), Some(2), Some(4), Some(8), Some(5), Some(6)],
            vec![Some(9), Some(6), Some(1), Some(5), Some(3), Some(7), Some(2), Some(8), Some(4)],
            vec![Some(2), Some(8), Some(7), Some(4), Some(1), Some(9), Some(6), Some(3), Some(5)],
            vec![Some(3), Some(4), Some(5), Some(2), Some(8), Some(6), Some(1), Some(7), Some(9)],
        ];
        let sample: Board = Board {
            size: 9,
            box_size: 3,
            cells: data,
        };

        let rendered_board: String = sample.to_string();

        assert_eq!(
            rendered_board,
            "╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗\n\
             ║ 5 │ 3 │ 4 ║ 6 │ 7 │ 8 ║ 9 │ 1 │ 2 ║\n\
             ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
             ║ 6 │ 7 │ 2 ║ 1 │ 9 │ 5 ║ 3 │ 4 │ 8 ║\n\
             ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
             ║ 1 │ 9 │ 8 ║ 3 │ 4 │ 2 ║ 5 │ 6 │ 7 ║\n\
             ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣\n\
             ║ 8 │ 5 │ 9 ║ 7 │ 6 │ 1 ║ 4 │ 2 │ 3 ║\n\
             ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
             ║ 4 │ 2 │ 6 ║ 8 │ 5 │ 3 ║ 7 │ 9 │ 1 ║\n\
             ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
             ║ 7 │ 1 │ 3 ║ 9 │ 2 │ 4 ║ 8 │ 5 │ 6 ║\n\
             ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣\n\
             ║ 9 │ 6 │ 1 ║ 5 │ 3 │ 7 ║ 2 │ 8 │ 4 ║\n\
             ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
             ║ 2 │ 8 │ 7 ║ 4 │ 1 │ 9 ║ 6 │ 3 │ 5 ║\n\
             ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
             ║ 3 │ 4 │ 5 ║ 2 │ 8 │ 6 ║ 1 │ 7 │ 9 ║\n\
             ╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝\n"
        );
    }

    #[test]
    fn test_board_new() {
        let board = Board::new(9, 3);

        for r in 0..board.size {
            for c in 0..board.size {
                assert_eq!(board.cells[r][c], None);
            }
        }
    }

    #[test]
    fn test_board_render() {
        let board = Board::new(9, 3);

        // blink=false → no highlight; empty board matches Display
        let rendered_board: String = board.render((3, 3), false);

        assert_eq!(rendered_board, board.to_string());
        assert_eq!(
            rendered_board,
            "╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗\n\
             ║   │   │   ║   │   │   ║   │   │   ║\n\
             ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
             ║   │   │   ║   │   │   ║   │   │   ║\n\
             ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
             ║   │   │   ║   │   │   ║   │   │   ║\n\
             ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣\n\
             ║   │   │   ║   │   │   ║   │   │   ║\n\
             ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
             ║   │   │   ║   │   │   ║   │   │   ║\n\
             ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
             ║   │   │   ║   │   │   ║   │   │   ║\n\
             ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣\n\
             ║   │   │   ║   │   │   ║   │   │   ║\n\
             ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
             ║   │   │   ║   │   │   ║   │   │   ║\n\
             ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
             ║   │   │   ║   │   │   ║   │   │   ║\n\
             ╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝\n"
        );
    }

    #[test]
    fn test_board_render_with_blink() {
        let board = Board::new(9, 3);

        let rendered_board: String = board.render((3, 3), true);

        assert_eq!(
            rendered_board,
            "╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗\n\
             ║   │   │   ║   │   │   ║   │   │   ║\n\
             ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
             ║   │   │   ║   │   │   ║   │   │   ║\n\
             ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
             ║   │   │   ║   │   │   ║   │   │   ║\n\
             ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣\n\
             ║   │   │   ║\x1B[7m   \x1B[0m│   │   ║   │   │   ║\n\
             ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
             ║   │   │   ║   │   │   ║   │   │   ║\n\
             ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
             ║   │   │   ║   │   │   ║   │   │   ║\n\
             ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣\n\
             ║   │   │   ║   │   │   ║   │   │   ║\n\
             ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
             ║   │   │   ║   │   │   ║   │   │   ║\n\
             ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
             ║   │   │   ║   │   │   ║   │   │   ║\n\
             ╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝\n"
        );
    }

    #[test]
    fn test_set_cell() {
        let mut board = Board::new(9, 3);

        board.set_cell(3, 3, 5);

        assert_eq!(board.cells[3][3], Some(5));
    }

    #[test]
    fn test_clear_cell() {
        let mut board = Board::new(9, 3);

        board.set_cell(3, 3, 5);
        board.set_cell(3, 8, 3);
        board.set_cell(8, 4, 6);
        board.set_cell(5, 1, 7);

        board.clear_cell(3, 3);

        assert_eq!(board.cells[3][3], None);
        assert_eq!(board.cells[3][8], Some(3));
        assert_eq!(board.cells[8][4], Some(6));
        assert_eq!(board.cells[5][1], Some(7));
    }

    #[test]
    fn test_has_no_duplicates_with_no_duplicates() {
        let data: Vec<Option<i32>> = vec![Some(5), Some(3), Some(4), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2)];

        assert!(Board::has_no_duplicates(&data));
    }

    #[test]
    fn test_has_no_duplicates_with_duplicates() {
        let data: Vec<Option<i32>> = vec![Some(5), Some(3), Some(4), Some(4), Some(7), Some(8), Some(9), Some(1), Some(2)];

        assert!(!Board::has_no_duplicates(&data));
    }

    #[test]
    fn test_is_valid_move_filled() {
        let mut board = Board::new(9, 3);
        board.set_cell(5, 0, 7);

        assert!(!board.is_valid_move(5, 0, 2));
    }

    #[test]
    fn test_is_valid_move_row_conflict() {
        let mut board = Board::new(9, 3);
        board.set_cell(5, 0, 7);

        assert!(!board.is_valid_move(5, 5, 7));
    }

    #[test]
    fn test_is_valid_move_column_conflict() {
        let mut board = Board::new(9, 3);
        board.set_cell(5, 0, 7);

        assert!(!board.is_valid_move(0, 0, 7));
    }

    #[test]
    fn test_is_valid_move_box_conflict() {
        let mut board = Board::new(9, 3);
        board.set_cell(1, 1, 5);

        assert!(!board.is_valid_move(0, 0, 5));
    }
}
