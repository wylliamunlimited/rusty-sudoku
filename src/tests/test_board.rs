use crate::board::Board;
use crate::grid::Grid;
use crate::puzzle::Puzzle;

#[cfg(test)]
mod tests {
    use super::*;

    fn board_with(cells: Vec<Vec<Option<i32>>>) -> Board {
        Board {
            cells,
            ..Board::new(9, 3)
        }
    }

    fn sample_board() -> Board {
        board_with(vec![
            vec![
                Some(1),
                Some(2),
                Some(3),
                None,
                None,
                None,
                None,
                None,
                None
            ];
            9
        ])
    }

    fn create_toy_puzzle(solution: Option<Vec<Vec<i32>>>, mask: Option<Vec<Vec<bool>>>) -> Puzzle {
        let solution = solution.unwrap_or_else(|| vec![vec![3, 2], vec![5, 4]]);
        let mask = mask.unwrap_or_else(|| vec![vec![false, true], vec![true, false]]);
        Puzzle::new(solution, mask)
    }

    #[test]
    fn test_from_puzzle_maps_mask() {
        let puzzle = Puzzle {
            solution: vec![vec![1, 2], vec![3, 4]],
            mask: vec![vec![true, false], vec![false, true]],
        };

        let board = Board::from_puzzle(puzzle);

        assert_eq!(board.cells[0][0], Some(1));
        assert_eq!(board.cells[0][1], None);
        assert_eq!(board.cells[1][0], None);
        assert_eq!(board.cells[1][1], Some(4));
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
            vec![
                Some(5),
                Some(3),
                Some(4),
                Some(6),
                Some(7),
                Some(8),
                Some(9),
                Some(1),
                Some(2),
            ],
            vec![
                Some(6),
                Some(7),
                Some(2),
                Some(1),
                Some(9),
                Some(5),
                Some(3),
                Some(4),
                Some(8),
            ],
            vec![
                Some(1),
                Some(9),
                Some(8),
                Some(3),
                Some(4),
                Some(2),
                Some(5),
                Some(6),
                Some(7),
            ],
            vec![
                Some(8),
                Some(5),
                Some(9),
                Some(7),
                Some(6),
                Some(1),
                Some(4),
                Some(2),
                Some(3),
            ],
            vec![
                Some(4),
                Some(2),
                Some(6),
                Some(8),
                Some(5),
                Some(3),
                Some(7),
                Some(9),
                Some(1),
            ],
            vec![
                Some(7),
                Some(1),
                Some(3),
                Some(9),
                Some(2),
                Some(4),
                Some(8),
                Some(5),
                Some(6),
            ],
            vec![
                Some(9),
                Some(6),
                Some(1),
                Some(5),
                Some(3),
                Some(7),
                Some(2),
                Some(8),
                Some(4),
            ],
            vec![
                Some(2),
                Some(8),
                Some(7),
                Some(4),
                Some(1),
                Some(9),
                Some(6),
                Some(3),
                Some(5),
            ],
            vec![
                Some(3),
                Some(4),
                Some(5),
                Some(2),
                Some(8),
                Some(6),
                Some(1),
                Some(7),
                Some(9),
            ],
        ];
        let sample: Board = board_with(data);

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
        let data: Vec<Option<i32>> = vec![
            Some(5),
            Some(3),
            Some(4),
            Some(6),
            Some(7),
            Some(8),
            Some(9),
            Some(1),
            Some(2),
        ];

        assert!(Board::has_no_duplicates(&data));
    }

    #[test]
    fn test_has_no_duplicates_with_duplicates() {
        let data: Vec<Option<i32>> = vec![
            Some(5),
            Some(3),
            Some(4),
            Some(4),
            Some(7),
            Some(8),
            Some(9),
            Some(1),
            Some(2),
        ];

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

    #[test]
    fn test_is_correct_move() {
        let puzzle: Puzzle = create_toy_puzzle(None, None);
        let board: Board = Board::from_puzzle(puzzle);

        assert!(board.is_correct_move(0, 0, 3));
        assert!(!board.is_correct_move(1, 1, 3));
    }

    #[test]
    fn test_is_editable() {
        let puzzle: Puzzle = create_toy_puzzle(None, None);
        let board: Board = Board::from_puzzle(puzzle);

        assert!(board.is_editable(0, 0));
        assert!(!board.is_editable(1, 0));
        assert!(board.is_editable(1, 1));
        assert!(!board.is_editable(0, 1));
    }

    #[test]
    fn test_set_cell_gated_accepts_correct_value_on_editable_cell() {
        let puzzle: Puzzle = create_toy_puzzle(None, None);
        let mut board: Board = Board::from_puzzle(puzzle);

        assert!(board.set_cell_gated(0, 0, 3));
        assert_eq!(board.cells[0][0], Some(3));
    }

    #[test]
    fn test_set_cell_gated_rejects_incorrect_value_on_editable_cell() {
        let puzzle: Puzzle = create_toy_puzzle(None, None);
        let mut board: Board = Board::from_puzzle(puzzle);

        assert!(!board.set_cell_gated(0, 0, 4));
        assert_eq!(board.cells[0][0], None);
    }

    #[test]
    fn test_set_cell_gated_rejects_write_to_clue() {
        let puzzle: Puzzle = create_toy_puzzle(None, None);
        let mut board: Board = Board::from_puzzle(puzzle);

        assert!(!board.set_cell_gated(0, 1, 2));
        assert_eq!(board.cells[0][1], Some(2));
    }

    #[test]
    fn test_clear_cell_gated_clears_editable_cell() {
        let puzzle: Puzzle = create_toy_puzzle(None, None);
        let mut board: Board = Board::from_puzzle(puzzle);
        board.set_cell(0, 0, 3);

        assert!(board.clear_cell_gated(0, 0));
        assert_eq!(board.cells[0][0], None);
    }

    #[test]
    fn test_clear_cell_gated_rejects_clearing_clue() {
        let puzzle: Puzzle = create_toy_puzzle(None, None);
        let mut board: Board = Board::from_puzzle(puzzle);

        assert!(!board.clear_cell_gated(0, 1));
        assert_eq!(board.cells[0][1], Some(2));
    }
}
