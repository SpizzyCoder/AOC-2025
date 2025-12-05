pub fn solve(input: &str) -> i128 {
    let mut fresh_id_ranges: Vec<(i128, i128)> = Vec::new();
    let mut ids_to_check: Vec<i128> = Vec::new();
    let mut solution: i128 = 0;

    // Parse file
    let mut switch: bool = false;
    for line in input.lines() {
        if line.is_empty() {
            switch = true;
            continue;
        }

        if !switch {
            let line_splitted: Vec<&str> = line.split("-").collect();
            let num1_parsed: i128 = line_splitted[0].parse().unwrap();
            let num2_parsed: i128 = line_splitted[1].parse().unwrap();
            fresh_id_ranges.push((num1_parsed, num2_parsed));
        } else {
            let num_parsed: i128 = line.parse().unwrap();

            ids_to_check.push(num_parsed);
        }
    }

    // Check nums
    for id in ids_to_check {
        for fresh_id_range in fresh_id_ranges.iter() {
            if id >= fresh_id_range.0 && id <= fresh_id_range.1 {
                solution += 1;
                break;
            }
        }
    }

    return solution;
}
