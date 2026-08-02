use crate::app::{App, Direction};
use crate::board::Board;
use crate::puzzle::Puzzle;

#[cfg(test)]
mod tests {
    use super::*;
    
    fn solution() -> Vec<Vec<i32>> {
        vec![
            vec![1, 2, 3, 4],
            vec![3, 4, 1, 2],
            vec![2, 1, 4, 3],
            vec![4, 3, 2, 1],
        ]
    }

    fn app_with_mask(mask: Vec<Vec<bool>>) -> App {
        App::new(Board::from_puzzle(Puzzle::new(solution(), mask)))
    }

    fn app_all_editable() -> App {
        app_with_mask(vec![vec![false; 4]; 4])
    }

    fn mask_with_row(row_id: usize, row: Vec<bool>) -> Vec<Vec<bool>> {
        let mut mask = vec![vec![false; 4]; 4];
        mask[row_id] = row;
        mask
    }

    #[test]
    fn test_shift_cursor_moves_one_cell_when_all_editable() {
        let mut app = app_all_editable();
        assert_eq!(app.cursor, (0, 0));

        app.shift_cursor(Direction::Right);
        assert_eq!(app.cursor, (0, 1));

        app.shift_cursor(Direction::Down);
        assert_eq!(app.cursor, (1, 1));

        app.shift_cursor(Direction::Left);
        assert_eq!(app.cursor, (1, 0));

        app.shift_cursor(Direction::Up);
        assert_eq!(app.cursor, (0, 0));
    }

    #[test]
    fn test_shift_cursor_clamps_at_top_left() {
        let mut app = app_all_editable();
        app.cursor = (0, 0);

        app.shift_cursor(Direction::Up);
        assert_eq!(app.cursor, (0, 0));

        app.shift_cursor(Direction::Left);
        assert_eq!(app.cursor, (0, 0));
    }

    #[test]
    fn test_shift_cursor_clamps_at_bottom_right() {
        let mut app = app_all_editable();
        app.cursor = (3, 3);

        app.shift_cursor(Direction::Down);
        assert_eq!(app.cursor, (3, 3));

        app.shift_cursor(Direction::Right);
        assert_eq!(app.cursor, (3, 3));
    }

    #[test]
    fn test_shift_cursor_skips_a_clue() {
        let mut app = app_with_mask(mask_with_row(0, vec![false, true, false, false]));
        app.cursor = (0, 0);

        app.shift_cursor(Direction::Right);
        assert_eq!(app.cursor, (0, 2));
    }

    #[test]
    fn test_shift_cursor_skips_consecutive_clues() {
        let mut app = app_with_mask(mask_with_row(0, vec![false, true, true, false]));
        app.cursor = (0, 0);

        app.shift_cursor(Direction::Right);
        assert_eq!(app.cursor, (0, 3));
    }

    #[test]
    fn test_shift_cursor_skips_clues_moving_backwards() {
        let mut app = app_with_mask(mask_with_row(0, vec![false, true, true, false]));
        app.cursor = (0, 3);

        app.shift_cursor(Direction::Left);
        assert_eq!(app.cursor, (0, 0));
    }

    #[test]
    fn test_shift_cursor_skips_clues_vertically() {
        let mut mask = vec![vec![false; 4]; 4];
        mask[1][0] = true;
        let mut app = app_with_mask(mask);
        app.cursor = (0, 0);

        app.shift_cursor(Direction::Down);
        assert_eq!(app.cursor, (2, 0));
    }

    #[test]
    fn test_shift_cursor_stays_put_when_only_clues_ahead() {
        let mut app = app_with_mask(mask_with_row(0, vec![false, true, true, true]));
        app.cursor = (0, 0);

        app.shift_cursor(Direction::Right);
        assert_eq!(app.cursor, (0, 0));
    }

    #[test]
    fn test_shift_cursor_moves_normally_without_a_puzzle() {
        let mut app = App::new(Board::new(9, 3));
        assert_eq!(app.cursor, (0, 0));

        app.shift_cursor(Direction::Right);
        assert_eq!(app.cursor, (0, 1));

        app.shift_cursor(Direction::Down);
        assert_eq!(app.cursor, (1, 1));
    }

    #[test]
    fn test_new_starts_on_first_editable_cell() {
        let app = app_with_mask(mask_with_row(0, vec![true, true, false, false]));
        assert_eq!(app.cursor, (0, 2));
    }

    #[test]
    fn test_new_wraps_to_next_row_when_first_row_is_all_clues() {
        let app = app_with_mask(mask_with_row(0, vec![true; 4]));
        assert_eq!(app.cursor, (1, 0));
    }

    #[test]
    fn test_new_falls_back_to_origin_when_nothing_is_editable() {
        let app = app_with_mask(vec![vec![true; 4]; 4]);
        assert_eq!(app.cursor, (0, 0));
    }
}
