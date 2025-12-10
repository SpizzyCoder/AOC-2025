pub fn solve(input: &str) -> i128 {
    let mut solution: i128 = 0;
    let mut matrix: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    for cur_row in 0..matrix.len() {
        for cur_col in 0..matrix[cur_row].len() {
            match matrix[cur_row][cur_col] {
                'S' => matrix[cur_row + 1][cur_col] = '|',
                '^' => {
                    if matrix[cur_row - 1][cur_col] == '|' {
                        if matrix[cur_row][cur_col - 1] != '^' {
                            matrix[cur_row][cur_col - 1] = '|';
                        }

                        if matrix[cur_row][cur_col + 1] != '^' {
                            matrix[cur_row][cur_col + 1] = '|';
                        }

                        solution += 1;
                    }
                },
                '.' => {
                    if cur_row != 0 {
                        if matrix[cur_row - 1][cur_col] == '|' {
                            matrix[cur_row][cur_col] = '|';
                        }
                    }
                }
                _ => {} // Do nothing
            };
        }
    }

    return solution;
}