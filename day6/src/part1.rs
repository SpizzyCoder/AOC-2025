pub fn solve(input: &str) -> i128 {
    let mut solution: i128 = 0;
    let mut numbers: Vec<Vec<i128>> = Vec::new();
    let mut operations: Vec<&str> = Vec::new();

    for line in input.lines() {
        let line_splitted: Vec<&str> = line.split_ascii_whitespace().collect();

        if line_splitted[0].parse::<i32>().is_ok() {
            // It's a line with numbers
            numbers.push(line_splitted.iter().map(|x| x.parse::<i128>().unwrap()).collect());
        } else {
            // It's the last line with the operations
            operations = line_splitted.clone();
        }
    }

    for (x_index, operation) in operations.iter().enumerate() {
        let mut result: i128 = 0;

        for y_index in 0..numbers.len() {
            match *operation {
                "*" => if result == 0 { result = numbers[y_index][x_index] } else { result *= numbers[y_index][x_index] },
                "+" => result += numbers[y_index][x_index],
                _ => panic!["Operation {} not supported", operations[x_index]]
            };
        }

        solution += result as i128;
    }

    return solution;
}
