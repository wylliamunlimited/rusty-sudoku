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

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.top_border())?;

        for row_id in 0..self.size {
            write!(f, "{}", self.format_row(row_id))?;

            if row_id == self.size - 1 {
                write!(f, "{}", self.bottom_border())?;
            } else if (row_id + 1) % self.box_size == 0 {
                write!(f, "{}", self.thick_middle_border())?;
            } else {
                write!(f, "{}", self.thin_middle_border())?;
            }
        }

        Ok(())
    }
}

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


#[cfg(test)]
mod tests {
    use super::*;

    fn sample_board() -> Board {
        let data: Vec<Vec<i32>> = vec![vec![1, 2, 3, -1, -1, -1, -1, -1, -1]; 9];
        let sample: Board = Board {
            size: 9,
            box_size: 3,
            cells: data,
        };

        sample
    }

    #[test]
    fn test_top_border() {
        let rendered_format: String = sample_board().top_border();

        assert_eq!(
            rendered_format,
            "╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗\n"
        );
    }

    #[test]
    fn test_bottom_border() {
        let rendered_format: String = sample_board().bottom_border();

        assert_eq!(
            rendered_format,
            "╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝\n"
        )
    } 

    #[test]
    fn test_thin_border() {
        let rendered_format: String = sample_board().thin_middle_border();

        assert_eq!(
            rendered_format,
            "╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n"
        )
    }

    #[test]
    fn test_thick_border() {
        let rendered_format: String = sample_board().thick_middle_border();

        assert_eq!(
            rendered_format,
            "╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣\n"
        )
    }

    #[test]
    fn test_row_format() {
        let rendered_format: String = sample_board().format_row(1);

        assert_eq!(
            rendered_format,
            "║ 1 │ 2 │ 3 ║   │   │   ║   │   │   ║\n"
        );
    }

    #[test]
    fn test_board() {
        let data: Vec<Vec<i32>> = vec![
            vec![5, 3, 4, 6, 7, 8, 9, 1, 2],
            vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
            vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
            vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
            vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
            vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
            vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
            vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
            vec![3, 4, 5, 2, 8, 6, 1, 7, 9],
        ];
        let sample: Board = Board {
            size: 9,
            box_size: 3,
            cells: data
        };

        let rendered_board: String = sample.to_string();

        assert_eq!(
            rendered_board,
            "╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗\n\
             ║ 5 │ 3 │ 4 ║ 6 │ 7 │ 8 ║ 9 │ 1 │ 2 ║\n\
             ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
             ║ 6 │ 7 │ 2 ║ 1 │ 9 │ 5 ║ 3 │ 4 │ 8 ║\n\
             ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
             ║ 1 │ 9 │ 8 ║ 3 │ 4 │ 2 ║ 5 │ 6 │ 7 ║\n\
             ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣\n\
             ║ 8 │ 5 │ 9 ║ 7 │ 6 │ 1 ║ 4 │ 2 │ 3 ║\n\
             ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
             ║ 4 │ 2 │ 6 ║ 8 │ 5 │ 3 ║ 7 │ 9 │ 1 ║\n\
             ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
             ║ 7 │ 1 │ 3 ║ 9 │ 2 │ 4 ║ 8 │ 5 │ 6 ║\n\
             ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣\n\
             ║ 9 │ 6 │ 1 ║ 5 │ 3 │ 7 ║ 2 │ 8 │ 4 ║\n\
             ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
             ║ 2 │ 8 │ 7 ║ 4 │ 1 │ 9 ║ 6 │ 3 │ 5 ║\n\
             ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n\
             ║ 3 │ 4 │ 5 ║ 2 │ 8 │ 6 ║ 1 │ 7 │ 9 ║\n\
             ╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝\n"
        );
    }
}