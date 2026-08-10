use rusty_sudoku::app::App;
use rusty_sudoku::board::Board;
use rusty_sudoku::puzzle::Puzzle;

use crossterm::event::poll;
use rusty_sudoku::tui::TerminalGuard;

use std::io;
use std::time::Duration;

fn main() -> io::Result<()> {
    let puzzle: Puzzle = Puzzle::generate(9, 3);
    let mut app: App = App::new(Board::from_puzzle(puzzle));
    let guard: TerminalGuard = TerminalGuard::new()?;

    loop {
        app.tick();
        guard.draw(&app)?;

        if poll(Duration::from_millis(500))?
            && let Some(input) = guard.read_input()?
            && !app.handle_input(input)
        {
            break;
        }
    }

    drop(guard);

    Ok(())
}
