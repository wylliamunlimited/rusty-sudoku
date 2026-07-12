
use crate::app::{Action, Direction, App};
use crossterm::event::{Event, KeyCode, KeyEventKind, poll, read};
use crossterm::{execute, terminal, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};

use std::io;
use std::io::Write;

pub struct TerminalGuard;

impl TerminalGuard {

    fn new() -> io::Result<Self> {
        execute!(io::stdout(), EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;

        Ok(TerminalGuard)
    }
    
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}