
use crate::app::{Action, Direction, App};
use crossterm::event::{Event, KeyCode, KeyEventKind, read};
use crossterm::{execute, terminal, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};

use std::io;
use std::io::Write;

pub struct TerminalGuard;

impl TerminalGuard {

    pub fn new() -> io::Result<Self> {
        execute!(io::stdout(), EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;

        Ok(TerminalGuard)
    }

    pub fn key_to_action(&self, key: KeyCode) -> Option<Action> {
        // Translation Function

        match key {
            // Shift
            KeyCode::Left => Some(Action::Move(Direction::Left)),
            KeyCode::Right => Some(Action::Move(Direction::Right)),
            KeyCode::Up => Some(Action::Move(Direction::Up)),
            KeyCode::Down => Some(Action::Move(Direction::Down)),
            
            // Exit
            KeyCode::Char('q') => Some(Action::Quit),

            // Ops
            KeyCode::Backspace => Some(Action::ClearCell),
            KeyCode::Char(c @ '1'..='9') => {
                let digit = c.to_digit(10).unwrap() as i32;
                Some(Action::SetDigit(digit))
            },
            _ => None
        }

    }

    pub fn handle_op(&self) -> io::Result<Option<Action>> {

        let action = match read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => {
                self.key_to_action(event.code)
            },
            _ => None
        };

        Ok(action)

    }

    pub fn draw(&self, app: &App) -> io::Result<()> {
        // Include the redraw logics
        print!("\x1B[2J\x1B[H");
        print!("{}", app.view().replace('\n', "\r\n"));
        print!("\nEnter 'q' to exit this window...");
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