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
        let (cols, rows) = terminal::size()?;
        let view = app.view();
        let lines: Vec<&str> = view.lines().collect();

        let height = lines.len() as u16;
        let width = lines.iter().copied().map(display_width).max().unwrap_or(0) as u16;

        let top = rows.saturating_sub(height) / 2;
        let pad = " ".repeat((cols.saturating_sub(width) / 2) as usize);

        let mut out = String::from("\x1B[H");
        for screen_row in 0..rows {
            if screen_row > 0 {
                out.push_str("\r\n");
            }
            if let Some(line) = screen_row.checked_sub(top).and_then(|i| lines.get(i as usize)) {
                out.push_str(&pad);
                out.push_str(line);
            }
            out.push_str("\x1B[K");
        }

        print!("{out}");
        io::stdout().flush()?;

        Ok(())
    }
}

pub(crate) fn display_width(line: &str) -> usize {
    let mut width = 0;
    let mut chars = line.chars();

    while let Some(c) = chars.next() {
        if c == '\x1B' {
            for esc in chars.by_ref() {
                if esc.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            width += 1;
        }
    }

    width
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = io::stdout().write_all(b"\x1B[?25h");
        let _ = terminal::disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}
