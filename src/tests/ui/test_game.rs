use crate::sudoku::{Board, Puzzle};
use crate::ui::{Direction, Game};

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

    fn game_with_mask(mask: Vec<Vec<bool>>) -> Game {
        Game::new(Board::from_puzzle(Puzzle::new(solution(), mask)))
    }

    fn game_all_editable() -> Game {
        game_with_mask(vec![vec![false; 4]; 4])
    }

    fn mask_with_row(row_id: usize, row: Vec<bool>) -> Vec<Vec<bool>> {
        let mut mask = vec![vec![false; 4]; 4];
        mask[row_id] = row;
        mask
    }

    #[test]
    fn test_shift_cursor_moves_one_cell_when_all_editable() {
        let mut game = game_all_editable();
        assert_eq!(game.cursor, (0, 0));

        game.shift_cursor(Direction::Right);
        assert_eq!(game.cursor, (0, 1));

        game.shift_cursor(Direction::Down);
        assert_eq!(game.cursor, (1, 1));

        game.shift_cursor(Direction::Left);
        assert_eq!(game.cursor, (1, 0));

        game.shift_cursor(Direction::Up);
        assert_eq!(game.cursor, (0, 0));
    }

    #[test]
    fn test_shift_cursor_clamps_at_top_left() {
        let mut game = game_all_editable();
        game.cursor = (0, 0);

        game.shift_cursor(Direction::Up);
        assert_eq!(game.cursor, (0, 0));

        game.shift_cursor(Direction::Left);
        assert_eq!(game.cursor, (0, 0));
    }

    #[test]
    fn test_shift_cursor_clamps_at_bottom_right() {
        let mut game = game_all_editable();
        game.cursor = (3, 3);

        game.shift_cursor(Direction::Down);
        assert_eq!(game.cursor, (3, 3));

        game.shift_cursor(Direction::Right);
        assert_eq!(game.cursor, (3, 3));
    }

    #[test]
    fn test_shift_cursor_skips_a_clue() {
        let mut game = game_with_mask(mask_with_row(0, vec![false, true, false, false]));
        game.cursor = (0, 0);

        game.shift_cursor(Direction::Right);
        assert_eq!(game.cursor, (0, 2));
    }

    #[test]
    fn test_shift_cursor_skips_consecutive_clues() {
        let mut game = game_with_mask(mask_with_row(0, vec![false, true, true, false]));
        game.cursor = (0, 0);

        game.shift_cursor(Direction::Right);
        assert_eq!(game.cursor, (0, 3));
    }

    #[test]
    fn test_shift_cursor_skips_clues_moving_backwards() {
        let mut game = game_with_mask(mask_with_row(0, vec![false, true, true, false]));
        game.cursor = (0, 3);

        game.shift_cursor(Direction::Left);
        assert_eq!(game.cursor, (0, 0));
    }

    #[test]
    fn test_shift_cursor_skips_clues_vertically() {
        let mut mask = vec![vec![false; 4]; 4];
        mask[1][0] = true;
        let mut game = game_with_mask(mask);
        game.cursor = (0, 0);

        game.shift_cursor(Direction::Down);
        assert_eq!(game.cursor, (2, 0));
    }

    #[test]
    fn test_shift_cursor_stays_put_when_only_clues_ahead() {
        let mut game = game_with_mask(mask_with_row(0, vec![false, true, true, true]));
        game.cursor = (0, 0);

        game.shift_cursor(Direction::Right);
        assert_eq!(game.cursor, (0, 0));
    }

    #[test]
    fn test_shift_cursor_moves_normally_without_a_puzzle() {
        let mut game = Game::new(Board::new(9, 3));
        assert_eq!(game.cursor, (0, 0));

        game.shift_cursor(Direction::Right);
        assert_eq!(game.cursor, (0, 1));

        game.shift_cursor(Direction::Down);
        assert_eq!(game.cursor, (1, 1));
    }

    fn cell_line(row: usize) -> usize {
        1 + row * 2
    }

    fn cell_column(col: usize) -> usize {
        1 + col * 4
    }

    #[test]
    fn test_click_moves_the_cursor_to_the_clicked_cell() {
        let mut game = game_all_editable();
        game.cursor = (0, 0);

        game.click(cell_line(2), cell_column(3));
        assert_eq!(game.cursor, (2, 3));
    }

    #[test]
    fn test_click_lands_anywhere_within_the_cell() {
        let mut game = game_all_editable();

        for offset in 0..3 {
            game.cursor = (0, 0);
            game.click(cell_line(1), cell_column(2) + offset);
            assert_eq!(game.cursor, (1, 2), "offset {offset}");
        }
    }

    #[test]
    fn test_click_on_a_border_is_ignored() {
        let mut game = game_all_editable();
        game.cursor = (1, 1);

        game.click(0, cell_column(2));
        assert_eq!(game.cursor, (1, 1));

        game.click(cell_line(2), 0);
        assert_eq!(game.cursor, (1, 1));

        game.click(cell_line(2), cell_column(2) + 3);
        assert_eq!(game.cursor, (1, 1));
    }

    #[test]
    fn test_click_outside_the_board_is_ignored() {
        let mut game = game_all_editable();
        game.cursor = (1, 1);

        game.click(cell_line(4), cell_column(0));
        assert_eq!(game.cursor, (1, 1));

        game.click(cell_line(0), cell_column(4));
        assert_eq!(game.cursor, (1, 1));
    }

    #[test]
    fn test_click_on_a_clue_is_ignored() {
        let mut game = game_with_mask(mask_with_row(0, vec![false, true, false, false]));
        game.cursor = (0, 0);

        game.click(cell_line(0), cell_column(1));
        assert_eq!(game.cursor, (0, 0));
    }

    #[test]
    fn test_click_clears_the_last_error() {
        let mut game = game_all_editable();
        game.cursor = (0, 0);
        game.set_current_cell(9);
        assert!(game.last_error.is_some());

        game.click(cell_line(1), cell_column(1));
        assert!(game.last_error.is_none());
    }

    #[test]
    fn test_ignored_click_keeps_the_last_error() {
        let mut game = game_all_editable();
        game.cursor = (0, 0);
        game.set_current_cell(9);
        assert!(game.last_error.is_some());

        game.click(0, cell_column(1));
        assert!(game.last_error.is_some());
    }

    #[test]
    fn test_new_starts_on_first_editable_cell() {
        let game = game_with_mask(mask_with_row(0, vec![true, true, false, false]));
        assert_eq!(game.cursor, (0, 2));
    }

    #[test]
    fn test_new_wraps_to_next_row_when_first_row_is_all_clues() {
        let game = game_with_mask(mask_with_row(0, vec![true; 4]));
        assert_eq!(game.cursor, (1, 0));
    }

    #[test]
    fn test_new_falls_back_to_origin_when_nothing_is_editable() {
        let game = game_with_mask(vec![vec![true; 4]; 4]);
        assert_eq!(game.cursor, (0, 0));
    }
}
