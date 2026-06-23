mod board;

use board::Board;

use crossterm::event;
use crossterm::terminal;
use crossterm::event::{Event, KeyCode, KeyEventKind, poll, read};

use std::time::Duration;
use std::io::Write;
use std::io;

// =============================================================================
// EXPECTED BEHAVIOR — interactive Sudoku (work-in-progress)
// =============================================================================
//
// GOAL
//   When the process starts, the terminal becomes a locked game view:
//   - the board redraws in place (same screen area, not scrolling)
//   - stdin only drives the game (navigate, fill, clear, quit)
//   - the process exits cleanly on quit
//
// CURRENT STATE (stdlib read_line prototype)
//   - starts with an empty 9x9 board (all cells = -1, rendered as blank)
//   - redraws the full board each loop iteration
//   - waits for one line of input + Enter
//   - hardcoded test commands on cell (0, 0):
//       "q" -> quit
//       "5" -> set (0, 0) to 5
//       "c" -> clear (0, 0)
//   - cursor does not exist yet; row/col are not tracked
//
// NEXT STEPS
//   1. Track cursor state (row, col) — probably an App struct wrapping Board
//   2. Replace hardcoded (0, 0) with cursor position for set/clear
//   3. Add navigation commands (arrow keys) to move cursor
//   4. Accept any digit 1-9 at cursor, not just "5"
//   5. Distinguish fixed clue cells from editable cells (later)
//
// FUTURE (raw input / Ratatui)
//   - switch from read_line (needs Enter) to crossterm event loop
//   - arrow keys move cursor without pressing Enter
//   - highlight the selected cell visually
//   - enable raw mode on startup, restore terminal on exit
//
// REDRAW CONTRACT
//   - use print!("\x1B[2J\x1B[H") to clear screen + move cursor home
//   - use print!("{board}") to render via Board's Display impl
//   - use io::stdout().flush()? so output appears before waiting for input
//   - do NOT use println! for the board — Display already ends lines with \n
//
// INPUT CONTRACT (target)
//   Navigation:  Up/Down/Left/Right arrow keys
//   Fill:        1-9 sets value at cursor
//   Clear:       Backspace / Delete / 0 clears cursor cell (-1)
//   Quit:        q or Esc
//
// =============================================================================

fn shift_cell(board: &Board, row_id: &mut usize, col_id: &mut usize, op: KeyCode) {
    
    match op {
        KeyCode::Left => if *col_id > 0 {
            *col_id -= 1;
        },
        KeyCode::Right => if *col_id < board.size - 1 {
            *col_id += 1;
        },
        KeyCode::Up => if *row_id > 0 {
            *row_id -= 1;
        },
        KeyCode::Down => if *row_id < board.size - 1 {
            *row_id += 1;
        },
        _ => {}
    }
}

fn main() -> io::Result<()> {
    let mut board: Board = Board::new(9, 3);

    let mut row_id: usize = 0;
    let mut col_id: usize = 0;
    
    terminal::enable_raw_mode()?;
    loop {
        // Redraw the board in place.
        print!("\x1B[2J\x1B[H");
        print!("{}", board.to_string().replace('\n', "\r\n"));
        io::stdout().flush()?;

        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(event) if event.kind == KeyEventKind::Press => {
                    match event.code {
                        KeyCode::Left => shift_cell(&board, &mut row_id, &mut col_id, KeyCode::Left),
                        KeyCode::Right => shift_cell(&board, &mut row_id, &mut col_id, KeyCode::Right),
                        KeyCode::Down => shift_cell(&board, &mut row_id, &mut col_id, KeyCode::Down),
                        KeyCode::Up => shift_cell(&board, &mut row_id, &mut col_id, KeyCode::Up),
                        KeyCode::Char(c @ '1'..='9') => {
                            let digit = c.to_digit(10).unwrap() as i32;
                            board.set_cell(row_id, col_id, digit);
                        },
                        KeyCode::Backspace => board.clear_cell(row_id, col_id),
                        KeyCode::Esc => break,
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
    terminal::disable_raw_mode()?;

    Ok(())
}
