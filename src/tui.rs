use crate::app::{App, Input};
use crossterm::event::{Event, KeyCode, KeyEventKind, read};
use crossterm::{
    execute, terminal,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};

use std::io;
use std::io::Write;

pub struct TerminalGuard;

impl TerminalGuard {
    pub fn new() -> io::Result<Self> {
        execute!(io::stdout(), EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;

        Ok(TerminalGuard)
    }

    pub fn key_to_input(&self, key: KeyCode) -> Option<Input> {
        // Translation Function - keys in, screen-agnostic intent out. What each
        // of these *means* is decided by App, which knows what's on screen.

        match key {
            // Shift
            KeyCode::Left => Some(Input::Left),
            KeyCode::Right => Some(Input::Right),
            KeyCode::Up => Some(Input::Up),
            KeyCode::Down => Some(Input::Down),

            // Ops
            KeyCode::Backspace | KeyCode::Delete | KeyCode::Char('0') => Some(Input::Erase),
            KeyCode::Char(c @ '1'..='9') => {
                let digit = c.to_digit(10).unwrap() as i32;
                Some(Input::Digit(digit))
            }

            // Screen control
            KeyCode::Enter => Some(Input::Confirm),
            KeyCode::Esc | KeyCode::Char('q') => Some(Input::Back),

            _ => None,
        }
    }

    pub fn read_input(&self) -> io::Result<Option<Input>> {
        let input = match read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => {
                self.key_to_input(event.code)
            }
            _ => None,
        };

        Ok(input)
    }

    pub fn draw(&self, app: &App) -> io::Result<()> {
        // Include the redraw logics
        print!("\x1B[2J\x1B[H");
        print!("{}", app.view().replace('\n', "\r\n"));
        io::stdout().flush()?;

        Ok(())
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}
