mod board;

use board::Board;

fn main() {
    
    loop {

        print!("\x1B[2J\x1B[H");
        print!("{board}");

    }
    
}
