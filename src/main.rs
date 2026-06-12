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
        output.push_str(&format!(" {} ", roster[i]));

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
        output.push_str('═══');

        if i == matrix.size - 1 {
            output.push_str('╣');
        } else if (i + 1) % matrix.box_size == 0 {
            output.push_str('╬');
        } else {
            output.push_str('╪');
        }
    }
    
    output.push('\n');
    output
}

fn thin_middle_border(matrix: &Board) -> String {
    let mut output = String::new();

    output.push('╟');

    for i in 0..matrix.size {
        output.push_str('═══');

        if i == matrix.size - 1 {
            output.push_str('╢');
        } else if (i + 1) % matrix.box_size == 0 {
            output.push_str('╫');
        } else {
            output.push_str('┼');
        }
    }
    
    output.push('\n');
    output
}

fn format_board(matrix: &Board) -> String {

    '''
        Characters: 
            ╔ ╗ ╚ ╝
            ═ ║
            ╦ ╩ ╠ ╣ ╬
    '''
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
    let mut row = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let matrix: &[[i32; 9]; 9] = &[row; 9];
    print_board(matrix);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bottom_border() {
        // Make a Board object


        // Call the bottom_border() function

        // assert_eq!
    }

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
}