use crate::sudoku::{Board, Grid};
use crate::ui::tui::display_width;

#[cfg(test)]
mod tests {
    use super::*;

    fn board() -> Board {
        Board::with_box_size(3)
    }

    fn cell_width() -> usize {
        board().cell_width()
    }

    fn stride() -> usize {
        cell_width() + 1
    }

    fn first_column_of(col: usize) -> usize {
        1 + col * stride()
    }

    fn line_of(row: usize) -> usize {
        1 + row * 2
    }

    #[test]
    fn test_cell_at_rejects_the_top_border() {
        assert_eq!(board().cell_at(0, 1), None);
    }

    #[test]
    fn test_cell_at_rejects_the_left_border() {
        assert_eq!(board().cell_at(1, 0), None);
    }

    #[test]
    fn test_cell_at_finds_the_first_cell() {
        let board = board();
        assert_eq!(board.cell_at(1, 1), Some((0, 0)));
        assert_eq!(board.cell_at(1, 2), Some((0, 0)));
        assert_eq!(board.cell_at(1, 3), Some((0, 0)));
    }

    #[test]
    fn test_cell_at_rejects_a_vertical_separator() {
        assert_eq!(board().cell_at(1, 4), None);
    }

    #[test]
    fn test_cell_at_finds_the_second_cell_after_the_separator() {
        assert_eq!(board().cell_at(1, 5), Some((0, 1)));
    }

    #[test]
    fn test_cell_at_rejects_a_separator_line() {
        assert_eq!(board().cell_at(2, 1), None);
    }

    #[test]
    fn test_cell_at_finds_the_second_row() {
        assert_eq!(board().cell_at(3, 1), Some((1, 0)));
    }

    #[test]
    fn test_cell_at_finds_the_last_cell() {
        assert_eq!(board().cell_at(17, 33), Some((8, 8)));
        assert_eq!(board().cell_at(17, 35), Some((8, 8)));
    }

    #[test]
    fn test_cell_at_rejects_the_right_border() {
        assert_eq!(board().cell_at(17, 36), None);
    }

    #[test]
    fn test_cell_at_rejects_past_the_bottom_border() {
        assert_eq!(board().cell_at(18, 1), None);
        assert_eq!(board().cell_at(19, 1), None);
    }

    #[test]
    fn test_cell_at_rejects_past_the_right_edge() {
        assert_eq!(board().cell_at(1, 37), None);
    }

    #[test]
    fn test_cell_at_inverts_the_layout_for_every_cell() {
        let board = board();

        for row in 0..board.size() {
            for col in 0..board.size() {
                let line = line_of(row);
                for offset in 0..cell_width() {
                    let column = first_column_of(col) + offset;
                    assert_eq!(
                        board.cell_at(line, column),
                        Some((row, col)),
                        "line {line}, column {column}"
                    );
                }
                assert_eq!(board.cell_at(line, first_column_of(col) - 1), None);
            }
        }
    }

    #[test]
    fn test_cell_at_rejects_every_border_line() {
        let board = board();

        for row in 0..=board.size() {
            let line = row * 2;
            assert_eq!(board.cell_at(line, 1), None, "line {line}");
        }
    }

    #[test]
    fn test_rendered_layout_matches_the_cell_width_constant() {
        let board = board();
        let rendered = board.render((0, 0), false);
        let lines: Vec<&str> = rendered.lines().collect();

        let expected_width = 1 + board.size() * stride();
        for line in &lines {
            assert_eq!(display_width(line), expected_width);
        }

        assert_eq!(lines.len(), 1 + board.size() * 2);
    }
}
