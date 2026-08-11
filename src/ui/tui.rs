use crate::ui::{App, Input};
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
        print!("\x1B[?25l");
        io::stdout().flush()?;

        Ok(TerminalGuard)
    }

    pub fn key_to_input(&self, key: KeyCode) -> Option<Input> {
        match key {
            KeyCode::Left => Some(Input::Left),
            KeyCode::Right => Some(Input::Right),
            KeyCode::Up => Some(Input::Up),
            KeyCode::Down => Some(Input::Down),

            KeyCode::Backspace | KeyCode::Delete | KeyCode::Char('0') => Some(Input::Erase),
            KeyCode::Char(c @ '1'..='9') => {
                let digit = c.to_digit(10).unwrap() as i32;
                Some(Input::Digit(digit))
            }

            KeyCode::Enter => Some(Input::Confirm),
            KeyCode::Esc | KeyCode::Char('q') => Some(Input::Back),

            _ => None,
        }
    }

    pub fn read_input(&self) -> io::Result<Option<Input>> {
        let input = match read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => self.key_to_input(event.code),
            _ => None,
        };

        Ok(input)
    }

    pub fn draw(&self, app: &App) -> io::Result<()> {
        let mut out = String::from("\x1B[H");
        for line in app.view().lines() {
            out.push_str(line);
            out.push_str("\x1B[K\r\n");
        }
        out.push_str("\x1B[J");

        print!("{out}");
        io::stdout().flush()?;

        Ok(())
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = io::stdout().write_all(b"\x1B[?25h");
        let _ = terminal::disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}
