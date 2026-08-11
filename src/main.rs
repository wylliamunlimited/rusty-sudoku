use crossterm::event::poll;

use rusty_sudoku::sudoku::{Board, Puzzle};
use rusty_sudoku::ui::{App, Request, TerminalGuard};

use std::io;
use std::time::Duration;

const BOARD_SIZE: usize = 9;
const BOX_SIZE: usize = 3;
/// Short enough that the menu animation stays smooth between keypresses.
const POLL_INTERVAL: u64 = 40; // ms

fn main() -> io::Result<()> {
    let mut app: App = App::new();
    let guard: TerminalGuard = TerminalGuard::new()?;

    loop {
        app.tick();
        guard.draw(&app)?;

        if poll(Duration::from_millis(POLL_INTERVAL))?
            && let Some(input) = guard.read_input()?
        {
            match app.handle_input(input) {
                Request::Continue => {}
                // The RNG is an effect, so it stays out here at the edge.
                Request::NewGame => {
                    let puzzle: Puzzle = Puzzle::generate(BOARD_SIZE, BOX_SIZE);
                    app.start_game(Board::from_puzzle(puzzle));
                }
                Request::Exit => break,
            }
        }
    }

    drop(guard);

    Ok(())
}
