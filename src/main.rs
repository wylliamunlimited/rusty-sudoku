mod board;

use board::Board;

fn main() {

    let mut board: Board = Board::new(9, 3);
    
    loop {

        print!("\x1B[2J\x1B[H");
        print!("{board}");

        break;
    }
    
}
