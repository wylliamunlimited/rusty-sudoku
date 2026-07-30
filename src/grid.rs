use crate::board::BorderStyle;

pub trait Grid {

    // Common Usage between Board.rs and Puzzle.rs
    // 1. rendering
    // 2. data

    // --- Required: the per-type pieces (you implement these on Board & Puzzle) ---
    fn size(&self) -> usize;
    fn box_size(&self) -> usize;
    fn cell_str(&self, row: usize, col: usize) -> String;

    // --- Provided (default): shared rendering, relocated from Board ---
    // These call the required methods above, so they work for any implementor.

    fn border(&self, style: &BorderStyle) -> String {
        let mut output = String::new();

        output.push(style.left);

        for i in 0..self.size() {
            output.push_str(style.fill);

            if i == self.size() - 1 {
                output.push(style.right);
            } else if (i + 1) % self.box_size() == 0 {
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

    fn format_row(&self, row_id: usize, highlight_col: Option<usize>) -> String {
        let mut output = String::new();

        output.push('║');

        for i in 0..self.size() {
            let cell_str = self.cell_str(row_id, i);

            if highlight_col == Some(i) {
                output.push_str("\x1B[7m"); // before
                output.push_str(&cell_str); // the 3 chars
                output.push_str("\x1B[0m"); // after
            } else {
                output.push_str(&cell_str);
            }

            if i == self.size() - 1 || (i + 1) % self.box_size() == 0 {
                output.push('║');
            } else {
                output.push('│');
            }
        }

        output.push('\n');
        output
    }

    fn render_grid(&self) -> String {
        let mut output = String::new();
        output.push_str(&self.top_border());

        for row_id in 0..self.size() {
            output.push_str(&self.format_row(row_id, None));

            if row_id == self.size() - 1 {
                output.push_str(&self.bottom_border());
            } else if (row_id + 1) % self.box_size() == 0 {
                output.push_str(&self.thick_middle_border());
            } else {
                output.push_str(&self.thin_middle_border());
            }
        }

        output
    }
}
