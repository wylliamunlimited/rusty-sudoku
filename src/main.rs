mod board;

use board::Board;

use crossterm::terminal;
use crossterm::event::{Event, KeyCode, KeyEventKind, poll, read};
use crossterm::{execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};

use std::time::{Duration, Instant};
use std::io::Write;
use std::io;


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
    let mut highlight_on: bool = false;
    let mut last_blink_time = Instant::now();
    const BLINK_INTERVAL: u64 = 500; // ms 
    
    execute!(io::stdout(), EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    loop {

        if last_blink_time.elapsed() >= Duration::from_millis(BLINK_INTERVAL) {
            highlight_on = !highlight_on;
            last_blink_time = Instant::now();
        }

        // Redraw the board in place.
        print!("\x1B[2J\x1B[H");
        print!("{}", board.render((row_id, col_id), highlight_on).replace('\n', "\r\n"));
        print!("\nEnter 'q' to exit this window...");
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
