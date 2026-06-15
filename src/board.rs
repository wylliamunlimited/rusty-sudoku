use std::fmt;

pub struct Board {
    pub(crate) size: usize,
    pub(crate) box_size: usize,
    pub(crate) cells: Vec<Vec<i32>>,
}

impl Board {

    fn format_row(&self, row_id: usize) -> String {
        let mut output = String::new();

        let roster: &Vec<i32> = &self.cells[row_id];

        output.push('║');

        for i in 0..self.size {

            if roster[i] < 0 {
                output.push_str("   ");
            } else {
                output.push_str(&format!(" {} ", roster[i]));
            }

            if i == self.size - 1 {
                output.push('║');
            } else if (i + 1) % self.box_size == 0 {
                output.push('║');
            } else {
                output.push('│');
            }
        }
        
        output.push('\n');
        output
    }

    fn border(&self, style: &BorderStyle) -> String {
        let mut output = String::new();
    
        output.push(style.left);
    
        for i in 0..self.size {
            output.push_str(style.fill);
    
            if i == self.size - 1 {
                output.push(style.right);
            } else if (i + 1) % self.box_size == 0 {
                output.push(style.box_junction);
            } else {
                output.push(style.cell);
            }
        }
    
        output.push('\n');
        output
    }

    fn top_border(&self) -> String {
        self.border(&BorderStyle::TOP)
    }

    fn bottom_border(&self) -> String {
        self.border(&BorderStyle::BOTTOM)
    }

    fn thick_middle_border(&self) -> String {
        self.border(&BorderStyle::THICK)
    }

    fn thin_middle_border(&self) -> String {
        self.border(&BorderStyle::THIN)
    }
}

// ============================================================================
// HANDOFF NOTES — pick up here (refactor: move rendering onto Board)
// ============================================================================
//
// WHERE THINGS STAND:
//   - DONE: format_row + border + the 4 border wrappers are now methods on
//     Board (above). `use std::fmt;` is already imported at the top.
//   - NOT DONE: the whole-board render (old `format_board`) still lives in
//     main.rs as a free function. It is the last thing to move.
//
//   ⚠️ THE PROJECT DOES NOT COMPILE RIGHT NOW. main.rs still calls
//   top_border(matrix), format_row(matrix, ..), etc. as FREE functions, but
//   those are methods on Board now. That's expected mid-refactor — the steps
//   below fix it.
//
// ----------------------------------------------------------------------------
// STEP 1 — Implement Display for Board (write it right here, below this note).
// ----------------------------------------------------------------------------
//   `Display` is the std trait for "render me as human text." Implementing it
//   gives board.to_string(), format!("{board}"), and println!("{board}") free.
//
//   The signature is fixed by the std library — you can't change it:
//
//     impl fmt::Display for Board {
//         fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//             // write into `f` instead of building a String
//             write!(f, "{}", self.top_border())?;
//
//             for row_id in 0..self.size {
//                 // TODO: write the row, then the border underneath it.
//                 // Same if / else-if / else as the old format_board:
//                 //   last row            -> self.bottom_border()
//                 //   (row_id+1)%box == 0 -> self.thick_middle_border()
//                 //   otherwise           -> self.thin_middle_border()
//             }
//
//             Ok(())   // () = "no value"; Ok(()) = "succeeded, nothing returned"
//         }
//     }
//
//   New pieces to remember:
//     - `f: &mut fmt::Formatter` is the output sink. You WRITE into it; you
//       don't return a String.
//     - `write!(f, "{}", x)?` is like print! but targets `f`. Every write
//       returns a Result, so it needs a trailing `?`.
//     - `?` means "if Err, bail out and return that Err from fmt; else carry
//       on." That's why fmt's return type is fmt::Result (= Result<(), Error>).
//     - End with `Ok(())`.
//
// ----------------------------------------------------------------------------
// STEP 2 — Clean up main.rs (it currently breaks the build):
// ----------------------------------------------------------------------------
//   - DELETE the old free fn `format_board` (Display replaces it).
//   - DELETE `print_board` and the commented-out dead format_board block.
//   - The `use` line can drop BorderStyle: `use board::Board;` is enough.
//   - Fill in main():
//         fn main() {
//             let board = Board { size: 9, box_size: 3, cells: vec![...] };
//             print!("{board}");   // print! NOT println! — Display already
//                                  // ends in '\n', so println! double-spaces.
//         }
//
// ----------------------------------------------------------------------------
// STEP 3 — Fix the tests (they still use free-function syntax):
// ----------------------------------------------------------------------------
//   Tests live in main.rs and call top_border(&sample), format_row(&sample, 1),
//   format_board(&sample). Convert to method syntax:
//       top_border(&sample)        -> sample.top_border()
//       format_row(&sample, 1)     -> sample.format_row(1)
//       format_board(&sample)      -> sample.to_string()   // via Display
//   NOTE: the border/format_row methods are PRIVATE. The tests must therefore
//   live in board.rs (same module sees private items), OR you make the methods
//   pub(crate). Easiest: move the `mod tests { .. }` block into board.rs.
//
// ----------------------------------------------------------------------------
// VERIFY: `cargo test` should show 6 passing tests and the build is clean.
//   (test_board now asserts against sample.to_string().)
// ============================================================================

pub struct BorderStyle {
    pub(crate) left: char,
    pub(crate) fill: &'static str,
    pub(crate) cell: char,
    pub(crate) box_junction: char,
    pub(crate) right: char,
}

impl BorderStyle {
    pub(crate) const TOP: BorderStyle = BorderStyle {
        left: '╔',
        fill: "═══",
        cell: '╤',
        box_junction: '╦',
        right: '╗',
    };

    pub(crate) const BOTTOM: BorderStyle = BorderStyle {
        left: '╚',
        fill: "═══",
        cell: '╧',
        box_junction: '╩',
        right: '╝',
    };

    pub(crate) const THICK: BorderStyle = BorderStyle {
        left: '╠',
        fill: "═══",
        cell: '╪',
        box_junction: '╬',
        right: '╣',
    };

    pub(crate) const THIN: BorderStyle = BorderStyle {
        left: '╟',
        fill: "───",
        cell: '┼',
        box_junction: '╫',
        right: '╢',
    };
}
