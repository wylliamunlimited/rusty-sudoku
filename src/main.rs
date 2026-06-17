mod board;

use board::Board;
use std::io::{self, Write};

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
//   3. Add navigation commands (arrow keys or hjkl) to move cursor
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
//   Navigation:  Up/Down/Left/Right (or h/j/k/l)
//   Fill:        1-9 sets value at cursor
//   Clear:       Backspace / Delete / 0 clears cursor cell (-1)
//   Quit:        q or Esc
//
// =============================================================================

fn main() -> io::Result<()> {
    let mut board: Board = Board::new(9, 3);

    loop {
        // Redraw the board in place.
        print!("\x1B[2J\x1B[H");
        print!("{board}");
        io::stdout().flush()?;

        // Read one command (temporary: line-based input, requires Enter).
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "q" => break,
            "c" => board.clear_cell(0, 0), // TODO: use cursor col/row
            "5" => board.set_cell(0, 0, 5), // TODO: use cursor col/row
            _ => {}
        }
    }

    Ok(())
}
