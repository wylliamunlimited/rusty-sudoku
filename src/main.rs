use rusty_sudoku::app::{App, Request};
use rusty_sudoku::board::Board;
use rusty_sudoku::puzzle::Puzzle;

use crossterm::event::poll;
use rusty_sudoku::tui::TerminalGuard;

use std::io;
use std::time::Duration;

const BOARD_SIZE: usize = 9;
const BOX_SIZE: usize = 3;

fn main() -> io::Result<()> {
    let mut app: App = App::new();
    let guard: TerminalGuard = TerminalGuard::new()?;

    loop {
        app.tick();
        guard.draw(&app)?;

        if poll(Duration::from_millis(500))?
            && let Some(input) = guard.read_input()?
        {
            match app.handle_input(input) {
                Request::Continue => {}
                // Generation needs the RNG, which is an effect - so it happens
                // out here at the edge and the finished board goes back in.
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
