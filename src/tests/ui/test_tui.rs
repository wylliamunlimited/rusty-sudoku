use crate::sudoku::{Board, Puzzle};
use crate::ui::tui::{display_width, fits, too_small_notice};
use crate::ui::{Input, TerminalGuard};
use crossterm::event::KeyCode;

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
        let board = Board::from_puzzle(Puzzle::generate(3));

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

    fn press(c: char) -> Option<Input> {
        TerminalGuard::key_to_input(KeyCode::Char(c))
    }

    #[test]
    fn test_digits_one_through_nine_map_to_their_value() {
        for (i, c) in "123456789".chars().enumerate() {
            assert_eq!(press(c), Some(Input::Digit(i as i32 + 1)), "key {c}");
        }
    }

    #[test]
    fn test_letters_a_through_g_map_to_ten_through_sixteen() {
        for (i, c) in "abcdefg".chars().enumerate() {
            assert_eq!(press(c), Some(Input::Digit(i as i32 + 10)), "key {c}");
        }
    }

    #[test]
    fn test_zero_erases_rather_than_entering_a_value() {
        assert_eq!(press('0'), Some(Input::Erase));
    }

    #[test]
    fn test_q_still_quits_and_is_not_read_as_a_value() {
        assert_eq!(press('q'), Some(Input::Back));
        assert_eq!(TerminalGuard::key_to_input(KeyCode::Esc), Some(Input::Back));
    }

    #[test]
    fn test_letters_past_g_are_ignored() {
        for c in "hijkxyz".chars() {
            assert_eq!(press(c), None, "key {c}");
        }
    }

    #[test]
    fn test_fits_accepts_a_view_that_matches_the_terminal_exactly() {
        let lines = vec!["abcde", "fghij"];
        assert!(fits(&lines, 5, 2));
    }

    #[test]
    fn test_fits_rejects_a_view_wider_or_taller_than_the_terminal() {
        let lines = vec!["abcde", "fghij"];
        assert!(!fits(&lines, 4, 2));
        assert!(!fits(&lines, 5, 1));
    }

    #[test]
    fn test_fits_measures_display_width_not_byte_length() {
        let lines = vec!["\x1B[7m 5 \x1B[0m"];
        assert!(fits(&lines, 3, 1));
    }

    #[test]
    fn test_a_sixteen_board_does_not_fit_a_standard_terminal() {
        let board = Board::with_box_size(4);
        let render = board.render((0, 0), false);
        let lines: Vec<&str> = render.lines().collect();

        assert!(!fits(&lines, 80, 24));
        assert!(fits(&lines, 100, 40));
    }

    #[test]
    fn test_too_small_notice_reports_both_dimensions() {
        let lines = vec!["abcde", "fghij"];
        let notice = too_small_notice(&lines, 4, 1);

        assert!(notice.contains("5×2"));
        assert!(notice.contains("4×1"));
    }
}
