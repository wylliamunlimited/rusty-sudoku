pub struct BorderStyle {
    pub(crate) left: char,
    pub(crate) fill: char,
    pub(crate) cell: char,
    pub(crate) box_junction: char,
    pub(crate) right: char,
}

impl BorderStyle {
    pub(crate) const TOP: BorderStyle = BorderStyle {
        left: '╔',
        fill: '═',
        cell: '╤',
        box_junction: '╦',
        right: '╗',
    };

    pub(crate) const BOTTOM: BorderStyle = BorderStyle {
        left: '╚',
        fill: '═',
        cell: '╧',
        box_junction: '╩',
        right: '╝',
    };

    pub(crate) const THICK: BorderStyle = BorderStyle {
        left: '╠',
        fill: '═',
        cell: '╪',
        box_junction: '╬',
        right: '╣',
    };

    pub(crate) const THIN: BorderStyle = BorderStyle {
        left: '╟',
        fill: '─',
        cell: '┼',
        box_junction: '╫',
        right: '╢',
    };
}

pub trait Grid {
    fn size(&self) -> usize;
    fn box_size(&self) -> usize;
    fn cell_str(&self, row: usize, col: usize) -> String;

    fn cell_width(&self) -> usize {
        self.size().to_string().len() + 2
    }

    fn border(&self, style: &BorderStyle) -> String {
        let mut output = String::new();

        output.push(style.left);

        for i in 0..self.size() {
            for _ in 0..self.cell_width() {
                output.push(style.fill);
            }

            if i == self.size() - 1 {
                output.push(style.right);
            } else if (i + 1).is_multiple_of(self.box_size()) {
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

    fn row_separator(&self, row_id: usize) -> String {
        if row_id == self.size() - 1 {
            self.bottom_border()
        } else if (row_id + 1).is_multiple_of(self.box_size()) {
            self.thick_middle_border()
        } else {
            self.thin_middle_border()
        }
    }

    fn format_row(&self, row_id: usize, highlight_col: Option<usize>) -> String {
        let mut output = String::new();

        output.push('║');

        for i in 0..self.size() {
            let cell_str = self.cell_str(row_id, i);

            if highlight_col == Some(i) {
                output.push_str("\x1B[7m");
                output.push_str(&cell_str);
                output.push_str("\x1B[0m");
            } else {
                output.push_str(&cell_str);
            }

            if i == self.size() - 1 || (i + 1).is_multiple_of(self.box_size()) {
                output.push('║');
            } else {
                output.push('│');
            }
        }

        output.push('\n');
        output
    }

    fn cell_at(&self, line: usize, column: usize) -> Option<(usize, usize)> {
        let cell_width = self.cell_width();
        let stride = cell_width + 1;

        let line_slot = line.checked_sub(1)?;
        if !line_slot.is_multiple_of(2) {
            return None;
        }
        let row = line_slot / 2;

        let column_slot = column.checked_sub(1)?;
        if column_slot % stride == cell_width {
            return None;
        }
        let col = column_slot / stride;

        (row < self.size() && col < self.size()).then_some((row, col))
    }

    fn render_grid(&self, highlight: Option<(usize, usize)>) -> String {
        let mut output = self.top_border();

        for row_id in 0..self.size() {
            let col = highlight.filter(|&(r, _)| r == row_id).map(|(_, c)| c);
            output.push_str(&self.format_row(row_id, col));
            output.push_str(&self.row_separator(row_id));
        }

        output
    }
}
