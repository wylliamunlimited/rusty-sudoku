fn print_board(matrix: &[&[i32]]) {
    println!("{}", format_board(matrix));
}

fn format_board(matrix: &[&[i32]]) -> String {
    let mut output = String::new();
    for row in matrix {
        output.push_str("| ");
        for col in *row {
            output.push_str(&format!("{} | ", col));
        }
        output.push('\n');
    }
    output
}

fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_board() {
        let row1 = [1, 2, 3];
        let row2 = [4, 5, 6];
        let row3 = [7, 8, 9];
        let matrix: &[&[i32]] = &[&row1, &row2, &row3];

        let formatted_board: String = format_board(matrix);
        assert_eq!(
            formamated_board(matrix),
            "| 1 | 2 | 3 | \n| 4 | 5 | 6 | \n| 7 | 8 | 9 | \n"
        );

    }
}