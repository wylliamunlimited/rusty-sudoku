use rusty_sudoku::ui::App;
use std::io::Write;
use std::{env, fs, thread, time::Duration};

fn main() {
    let frames: usize = env::args()
        .nth(1)
        .and_then(|a| a.parse().ok())
        .unwrap_or(105);
    let out = env::args().nth(2).unwrap_or_else(|| "frames.txt".into());

    let mut app = App::new();
    let mut file = fs::File::create(&out).unwrap();

    for i in 0..frames {
        app.tick();
        write!(file, "\x0C{}", app.view()).unwrap();
        if i + 1 < frames {
            thread::sleep(Duration::from_millis(85));
        }
    }
    eprintln!("wrote {frames} frames to {out}");
}
