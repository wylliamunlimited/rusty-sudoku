use crate::ui::{App, Input};
use crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseButton, MouseEvent,
    MouseEventKind, read,
};
use crossterm::{
    execute, terminal,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};

use std::cell::Cell;
use std::io;
use std::io::Write;

const MAX_VALUE_RADIX: u32 = 17;

pub struct TerminalGuard {
    origin: Cell<(u16, u16)>,
}

impl TerminalGuard {
    pub fn new() -> io::Result<Self> {
        execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        terminal::enable_raw_mode()?;
        print!("\x1B[?25l");
        io::stdout().flush()?;

        Ok(TerminalGuard {
            origin: Cell::new((0, 0)),
        })
    }

    pub fn key_to_input(key: KeyCode) -> Option<Input> {
        match key {
            KeyCode::Left => Some(Input::Left),
            KeyCode::Right => Some(Input::Right),
            KeyCode::Up => Some(Input::Up),
            KeyCode::Down => Some(Input::Down),

            KeyCode::Enter => Some(Input::Confirm),
            KeyCode::Esc | KeyCode::Char('q') => Some(Input::Back),

            KeyCode::Backspace | KeyCode::Delete | KeyCode::Char('0') => Some(Input::Erase),
            KeyCode::Char(c) => c
                .to_digit(MAX_VALUE_RADIX)
                .filter(|&d| d > 0)
                .map(|d| Input::Digit(d as i32)),

            _ => None,
        }
    }

    pub fn mouse_to_input(&self, event: MouseEvent) -> Option<Input> {
        if event.kind != MouseEventKind::Down(MouseButton::Left) {
            return None;
        }

        let (top, left) = self.origin.get();
        let row = event.row.checked_sub(top)?;
        let col = event.column.checked_sub(left)?;

        Some(Input::Click(row, col))
    }

    pub fn read_input(&self) -> io::Result<Option<Input>> {
        let input = match read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => {
                Self::key_to_input(event.code)
            }
            Event::Mouse(event) => self.mouse_to_input(event),
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
        let left = cols.saturating_sub(width) / 2;
        self.origin.set((top, left));

        let pad = " ".repeat(left as usize);

        let mut out = String::from("\x1B[H");
        for screen_row in 0..rows {
            if screen_row > 0 {
                out.push_str("\r\n");
            }
            if let Some(line) = screen_row
                .checked_sub(top)
                .and_then(|i| lines.get(i as usize))
            {
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
        let _ = execute!(io::stdout(), DisableMouseCapture, LeaveAlternateScreen);
    }
}
