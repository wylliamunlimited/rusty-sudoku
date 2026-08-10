use crate::app::{App, Input, MenuItem, Request, Screen};
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

    /// A board with no clues, so every cell is editable and the cursor starts
    /// at (0, 0). Built without the RNG - App never generates puzzles itself.
    fn board() -> Board {
        Board::from_puzzle(Puzzle::new(solution(), vec![vec![false; 4]; 4]))
    }

    fn app_in_game() -> App {
        let mut app = App::new();
        app.start_game(board());
        app
    }

    // --- Opening state ---

    #[test]
    fn test_starts_on_the_menu_with_no_game() {
        let app = App::new();

        assert_eq!(app.screen(), Screen::Menu);
        assert_eq!(app.selected(), MenuItem::NewGame);
        assert!(!app.has_game());
    }

    // --- Menu navigation ---

    #[test]
    fn test_menu_skips_continue_when_there_is_no_game() {
        let mut app = App::new();

        app.handle_input(Input::Down);
        assert_eq!(app.selected(), MenuItem::Quit);
    }

    #[test]
    fn test_menu_skips_continue_moving_back_up() {
        let mut app = App::new();
        app.handle_input(Input::Down);

        app.handle_input(Input::Up);
        assert_eq!(app.selected(), MenuItem::NewGame);
    }

    #[test]
    fn test_menu_lands_on_continue_once_a_game_exists() {
        let mut app = app_in_game();
        app.handle_input(Input::Back); // back to the menu

        app.handle_input(Input::Down);
        assert_eq!(app.selected(), MenuItem::Continue);
    }

    #[test]
    fn test_menu_selection_clamps_at_both_ends() {
        let mut app = App::new();

        app.handle_input(Input::Up);
        assert_eq!(app.selected(), MenuItem::NewGame);

        app.handle_input(Input::Down);
        app.handle_input(Input::Down);
        assert_eq!(app.selected(), MenuItem::Quit);
    }

    #[test]
    fn test_digits_do_nothing_on_the_menu() {
        let mut app = App::new();

        assert_eq!(app.handle_input(Input::Digit(5)), Request::Continue);
        assert_eq!(app.screen(), Screen::Menu);
        assert!(!app.has_game());
    }

    // --- Menu actions ---

    #[test]
    fn test_confirm_on_new_game_asks_main_for_a_board() {
        let mut app = App::new();

        // App requests the effect rather than performing it, so it's still on
        // the menu until main hands a board back.
        assert_eq!(app.handle_input(Input::Confirm), Request::NewGame);
        assert_eq!(app.screen(), Screen::Menu);
        assert!(!app.has_game());
    }

    #[test]
    fn test_confirm_on_quit_exits() {
        let mut app = App::new();
        app.handle_input(Input::Down); // Continue is skipped -> Quit

        assert_eq!(app.handle_input(Input::Confirm), Request::Exit);
    }

    #[test]
    fn test_back_on_the_menu_exits() {
        let mut app = App::new();

        assert_eq!(app.handle_input(Input::Back), Request::Exit);
    }

    #[test]
    fn test_confirm_on_continue_does_nothing_without_a_game() {
        // Continue can't be reached by navigation with no game, but confirming
        // it must not strand the app on an empty Game screen either.
        let mut app = App::new();

        assert_eq!(app.handle_input(Input::Confirm), Request::NewGame);
        assert_eq!(app.screen(), Screen::Menu);
    }

    // --- Entering and leaving a game ---

    #[test]
    fn test_start_game_enters_the_game_screen() {
        let app = app_in_game();

        assert_eq!(app.screen(), Screen::Game);
        assert!(app.has_game());
    }

    #[test]
    fn test_back_from_a_game_returns_to_the_menu_without_discarding_it() {
        let mut app = app_in_game();

        assert_eq!(app.handle_input(Input::Back), Request::Continue);
        assert_eq!(app.screen(), Screen::Menu);
        assert!(app.has_game());
    }

    #[test]
    fn test_continue_resumes_the_same_game() {
        let mut app = app_in_game();
        app.handle_input(Input::Right);
        app.handle_input(Input::Down);
        let left_at = app.game().unwrap().cursor;

        app.handle_input(Input::Back); // to the menu
        app.handle_input(Input::Down); // onto Continue
        app.handle_input(Input::Confirm);

        assert_eq!(app.screen(), Screen::Game);
        assert_eq!(app.game().unwrap().cursor, left_at);
    }

    #[test]
    fn test_new_game_from_the_menu_replaces_the_running_game() {
        let mut app = app_in_game();
        app.handle_input(Input::Right);
        app.handle_input(Input::Back);

        assert_eq!(app.handle_input(Input::Confirm), Request::NewGame);
        app.start_game(board());

        assert_eq!(app.screen(), Screen::Game);
        assert_eq!(app.game().unwrap().cursor, (0, 0));
    }

    // --- Game input still reaches the board ---

    #[test]
    fn test_arrows_move_the_cursor_in_a_game() {
        let mut app = app_in_game();

        app.handle_input(Input::Right);
        assert_eq!(app.game().unwrap().cursor, (0, 1));

        app.handle_input(Input::Down);
        assert_eq!(app.game().unwrap().cursor, (1, 1));
    }

    #[test]
    fn test_digits_reach_the_board_in_a_game() {
        let mut app = app_in_game();

        app.handle_input(Input::Digit(1)); // (0, 0) is a 1 in the solution
        assert_eq!(app.game().unwrap().board.cells[0][0], Some(1));

        app.handle_input(Input::Erase);
        assert_eq!(app.game().unwrap().board.cells[0][0], None);
    }

    #[test]
    fn test_a_wrong_digit_is_refused_and_reported() {
        let mut app = app_in_game();

        app.handle_input(Input::Digit(9));
        assert_eq!(app.game().unwrap().board.cells[0][0], None);
        assert!(app.game().unwrap().last_error.is_some());

        // Moving away clears the message - it described the old cell.
        app.handle_input(Input::Right);
        assert!(app.game().unwrap().last_error.is_none());
    }

    // --- Rendering ---

    #[test]
    fn test_menu_view_marks_the_selection_and_dead_entries() {
        let app = App::new();
        let view = app.view();

        assert!(view.contains("▸ New Game"));
        assert!(view.contains("Continue  ·  no game in progress"));
    }

    #[test]
    fn test_game_screen_renders_the_board_not_the_menu() {
        let app = app_in_game();
        let view = app.view();

        assert!(!view.contains("RUSTY SUDOKU"));
        assert!(view.contains('║'));
    }
}
