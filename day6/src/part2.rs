pub fn solve(input: &str) -> i128 {
    let mut solution: i128 = 0;

    let operator_line: &str = input.lines().last().unwrap();
    let mut col_pos_widths: Vec<(usize, usize)> = Vec::new();

    let mut cur_col_width: usize = 0;
    let mut cur_col_pos: usize = 0;
    for (index, ch) in operator_line.chars().enumerate() {
        match ch {
            '*' => {
                if cur_col_width != 0 {
                    col_pos_widths.push((cur_col_pos, cur_col_width - 1));
                }
                cur_col_width = 1;
                cur_col_pos = index;
            },
            '+' => {
                if cur_col_width != 0 {
                    col_pos_widths.push((cur_col_pos, cur_col_width - 1));
                }
                cur_col_width = 1;
                cur_col_pos = index;
            },
            ' ' => cur_col_width += 1,
            _ => panic!["Fuck"],
        };
    }

    col_pos_widths.push((cur_col_pos, cur_col_width));

    let mut matrix: Vec<Vec<&str>> = Vec::new();

    for line in input.lines() {
        let mut slices: Vec<&str> = Vec::new();

        for col_pos_width in col_pos_widths.iter() {
            slices.push(&line[col_pos_width.0..col_pos_width.0 + col_pos_width.1]);
        }

        matrix.push(slices);
    }
    
    let mut ops: Vec<(&str, Vec<&str>)> = Vec::new();
    for cur_col in 0..col_pos_widths.len() {
        let mut nums: Vec<&str> = Vec::new();

        for cur_num_row in 0..matrix.len() - 1 {
            nums.push(matrix[cur_num_row][cur_col]);
        }

        ops.push((matrix[matrix.len() - 1][cur_col].trim(), nums));
    }

    for op in ops.iter() {
        let mut result: i128 = 0;
        let converted_nums: Vec<i128> = get_cephalopod_numbers(&op.1);

        for converted_num in converted_nums.iter() {
            match op.0 {
                "*" => if result == 0 { result = *converted_num } else { result *= *converted_num },
                "+" => result += *converted_num,
                _ => panic!["Operation {} not supported", op.0]
            }
        }

        solution += result;
    }

    return solution;
}

fn get_cephalopod_numbers(raw_numbers: &[&str]) -> Vec<i128> {
    let mut converted_numbers: Vec<i128> = Vec::new();
    let mut owned_raw_numbers: Vec<String> = raw_numbers.iter().map(|x| x.to_string()).collect();

    loop {
        let mut all_converted: bool = false;
        let mut tmp_string: String = String::new();

        for i in owned_raw_numbers.iter_mut() {
            if let Some(ch) = i.pop() {
                if ch != ' ' {
                    tmp_string.push(ch);
                }
            } else {
                all_converted = true;
                break;
            }
        }

        if all_converted {
            break;
        }

        converted_numbers.push(tmp_string.parse().unwrap());
    }

    return converted_numbers;
}