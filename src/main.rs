use rusty_sudoku::app::App;
use rusty_sudoku::board::Board;

use crossterm::event::poll;
use rusty_sudoku::tui::TerminalGuard;

use std::io;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut app: App = App::new(Board::new(9, 3));
    let guard: TerminalGuard = TerminalGuard::new()?;

    loop {
        app.tick();
        guard.draw(&app)?;

        if poll(Duration::from_millis(500))?
            && let Some(action) = guard.handle_op()?
            && !app.handle_action(action)
        {
            break;
        }
    }

    drop(guard);

    Ok(())
}
