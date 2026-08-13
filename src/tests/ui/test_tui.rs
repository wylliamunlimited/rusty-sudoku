use crate::sudoku::{Board, Puzzle};
use crate::ui::tui::display_width;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_width_counts_plain_ascii() {
        assert_eq!(display_width("New Game"), 8);
        assert_eq!(display_width(""), 0);
    }

    #[test]
    fn test_display_width_counts_box_drawing_as_one_column_each() {
        assert_eq!(display_width("╔═══╗"), 5);
    }

    #[test]
    fn test_display_width_ignores_reverse_video_escapes() {
        assert_eq!(display_width("\x1B[7m 5 \x1B[0m"), 3);
    }

    #[test]
    fn test_display_width_ignores_dim_escapes() {
        assert_eq!(display_width("\x1B[2m⠿\x1B[0m⠿"), 2);
    }

    #[test]
    fn test_display_width_handles_escape_only_line() {
        assert_eq!(display_width("\x1B[0m"), 0);
    }

    #[test]
    fn test_board_width_is_unchanged_by_the_cursor_highlight() {
        let board = Board::from_puzzle(Puzzle::generate(9, 3));

        let widest = |blink: bool| {
            board
                .render((0, 0), blink)
                .lines()
                .map(display_width)
                .max()
                .unwrap()
        };

        assert_eq!(widest(false), widest(true));
    }
}
