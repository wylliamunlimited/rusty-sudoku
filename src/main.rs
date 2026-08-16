use crossterm::event::poll;

use rusty_sudoku::sudoku::{Board, Puzzle};
use rusty_sudoku::ui::{App, Request, TerminalGuard};

use std::io;
use std::time::Duration;

const POLL_INTERVAL: Duration = Duration::from_millis(40);

fn main() -> io::Result<()> {
    let mut app: App = App::new();
    let guard: TerminalGuard = TerminalGuard::new()?;

    loop {
        app.tick();
        guard.draw(&app)?;

        if poll(POLL_INTERVAL)?
            && let Some(input) = guard.read_input()?
        {
            match app.handle_input(input) {
                Request::Continue => {}
                Request::NewGame => {
                    let puzzle: Puzzle = Puzzle::generate(app.box_size());
                    app.start_game(Board::from_puzzle(puzzle));
                }
                Request::Exit => break,
            }
        }
    }

    drop(guard);

    Ok(())
}
