mod board;
mod app;

use board::Board;
use app::{App, Direction};

use crossterm::event::{Event, KeyCode, KeyEventKind, poll, read};
use crossterm::{execute, terminal, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};

use std::time::Duration;
use std::io::Write;
use std::io;


fn main() -> io::Result<()> {
    let mut app: App = App::new(Board::new(9, 3));
    
    execute!(io::stdout(), EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    loop {

        app.tick();

        // Redraw the board in place.
        print!("\x1B[2J\x1B[H");
        print!("{}", app.view().replace('\n', "\r\n"));
        print!("\nEnter 'q' to exit this window...");
        io::stdout().flush()?;

        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(event) if event.kind == KeyEventKind::Press => {
                    match event.code {
                        KeyCode::Left => app.shift_cursor(Direction::Left),
                        KeyCode::Right => app.shift_cursor(Direction::Right),
                        KeyCode::Down => app.shift_cursor(Direction::Down),
                        KeyCode::Up => app.shift_cursor(Direction::Up),
                        KeyCode::Char(c @ '1'..='9') => {
                            let digit = c.to_digit(10).unwrap() as i32;
                            app.set_current_cell(digit);
                        },
                        KeyCode::Backspace => app.clear_current_cell(),
                        KeyCode::Char('q') => break,
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
    terminal::disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}
