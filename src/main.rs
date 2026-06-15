mod board;

use board::Board;

fn print_board(matrix: &Board) {
    println!("{}", format_board(matrix));
}

// fn format_board(matrix: &Board) -> String {
//     let mut output = String::new();
//     for row in matrix {
//         output.push_str(&"| - ".repeat(9));
//         output.push_str("|\n");
//         output.push_str("| ");
//         for col in *row {
//             output.push_str(&format!("{} | ", col));
//         }
//         output.push('\n');
//     }
//     output.push_str(&"| - ".repeat(9));
//     output.push_str("|\n");
//     output
// }

fn format_row(matrix: &Board, row_id: usize) -> String {
    let mut output = String::new();

    let roster: &Vec<i32> = &matrix.cells[row_id];

    output.push('║');

    for i in 0..matrix.size {

        if roster[i] < 0 {
            output.push_str(&format!("   "));
        } else {
            output.push_str(&format!(" {} ", roster[i]));
        }

        if i == matrix.size - 1 {
            output.push('║');
        } else if (i + 1) % matrix.box_size == 0 {
            output.push('║');
        } else {
            output.push('│');
        }
    }
    
    output.push('\n');
    output
}

fn top_border(matrix: &Board) -> String {
    let mut output = String::new();

    output.push('╔');
    for i in 0..matrix.size {
        output.push_str("═══");

        if i == matrix.size - 1 {
            output.push('╗');
        } else if (i + 1) % matrix.box_size == 0 {
            output.push('╦');
        } else {
            output.push('╤');
        }
    }
    output.push('\n');
    output
}

fn bottom_border(matrix: &Board) -> String {
    let mut output = String::new();

    output.push('╚');
    for i in 0..matrix.size {
        output.push_str("═══");

        if i == matrix.size - 1 {
            output.push('╝');
        } else if (i + 1) % matrix.box_size == 0 {
            output.push('╩');
        } else {
            output.push('╧');
        }
    }
    output.push('\n');
    output
}

fn thick_middle_border(matrix: &Board) -> String {
    let mut output = String::new();

    output.push('╠');

    for i in 0..matrix.size {
        output.push_str("═══");

        if i == matrix.size - 1 {
            output.push('╣');
        } else if (i + 1) % matrix.box_size == 0 {
            output.push('╬');
        } else {
            output.push('╪');
        }
    }
    
    output.push('\n');
    output
}

fn thin_middle_border(matrix: &Board) -> String {
    let mut output = String::new();

    output.push('╟');

    for i in 0..matrix.size {
        output.push_str("───");

        if i == matrix.size - 1 {
            output.push('╢');
        } else if (i + 1) % matrix.box_size == 0 {
            output.push('╫');
        } else {
            output.push('┼');
        }
    }
    
    output.push('\n');
    output
}

fn format_board(matrix: &Board) -> String {
    // Characters: 
    //     ╔ ╗ ╚ ╝
    //     ═ ║
    //     ╦ ╩ ╠ ╣ ╬

    let mut output = String::new();

    output.push_str(&top_border(matrix));

    for row_id in 0..matrix.size {
        output.push_str(&format_row(matrix, row_id));

        if row_id == matrix.size - 1 {
            output.push_str(&bottom_border(matrix));
        } else if (row_id + 1) % matrix.box_size == 0 {
            output.push_str(&thick_middle_border(matrix));
        } else {
            output.push_str(&thin_middle_border(matrix));
        }
    }

    output
}

fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_border() {
        // Make a Board Object
        let data: Vec<Vec<i32>> = vec![vec![1, 2, 3, -1, -1, -1, -1, -1, -1]; 9];
        let sample: Board = Board {
            size: 9,
            box_size: 3,
            cells: data,
        };
        
        // Call the top_border() function
        let rendered_format: String = top_border(&sample);

        // assert_eq!
        assert_eq!(
            rendered_format,
            "╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗\n"
        );
    }

    #[test]
    fn test_bottom_border() {
        let data: Vec<Vec<i32>> = vec![vec![1, 2, 3, -1, -1, -1, -1, -1, -1]; 9];
        let sample: Board = Board {
            size: 9,
            box_size: 3,
            cells: data
        };

        let rendered_format: String = bottom_border(&sample);

        assert_eq!(
            rendered_format,
            "╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝\n"
        )
    } 

    #[test]
    fn test_thin_border() {
        let data: Vec<Vec<i32>> = vec![vec![1, 2, 3, -1, -1, -1, -1, -1, -1]; 9];
        let sample: Board = Board {
            size: 9,
            box_size: 3,
            cells: data
        };

        let rendered_format: String = thin_middle_border(&sample);

        assert_eq!(
            rendered_format,
            "╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n"
        )
    }

    #[test]
    fn test_thick_border() {
        let data: Vec<Vec<i32>> = vec![vec![1, 2, 3, -1, -1, -1, -1, -1, -1]; 9];
        let sample: Board = Board {
            size: 9,
            box_size: 3,
            cells: data
        };

        let rendered_format: String = thick_middle_border(&sample);

        assert_eq!(
            rendered_format,
            "╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣\n"
        )
    }

    #[test]
    fn test_row_format() {
        let data: Vec<Vec<i32>> = vec![vec![1, 2, 3, -1, -1, -1, -1, -1, -1]; 9];
        let sample: Board = Board {
            size: 9,
            box_size: 3,
            cells: data
        };

        let rendered_format: String = format_row(&sample, 1);

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

        let rendered_board: String = format_board(&sample);

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