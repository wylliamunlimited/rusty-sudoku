mod board;
mod app;
mod tui;

use board::Board;
use app::App;

use tui::TerminalGuard;
use crossterm::event::poll;

use std::time::Duration;
use std::io;


fn main() -> io::Result<()> {
    let mut app: App = App::new(Board::new(9, 3));
    let guard: TerminalGuard = TerminalGuard::new()?;

    loop {

        app.tick();
        guard.draw(&app)?;

        if poll(Duration::from_millis(500))? {
            if let Some(action) = guard.handle_op()? {
                if !app.handle_action(action) {
                    break
                }
            }
        }
    }

    drop(guard);

    Ok(())
}
