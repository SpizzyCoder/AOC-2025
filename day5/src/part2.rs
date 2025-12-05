pub fn solve(input: String) {
    let mut fresh_id_ranges_from_file: Vec<(i128, i128)> = Vec::new();
    let mut fresh_id_ranges: Vec<(i128, i128)> = Vec::new();
    let mut solution: i128 = 0;

    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        let line_splitted: Vec<&str> = line.split("-").collect();
        let num1_parsed: i128 = line_splitted[0].parse().unwrap();
        let num2_parsed: i128 = line_splitted[1].parse().unwrap();

        fresh_id_ranges_from_file.push((num1_parsed, num2_parsed));
    }

    fresh_id_ranges_from_file.sort_by(|a, b| a.0.cmp(&b.0));

    for fresh_id_range in fresh_id_ranges_from_file.iter() {
        let mut num1: i128 = fresh_id_range.0;
        let mut num2: i128 = fresh_id_range.1;

        let mut discard: bool = false;
        for range in fresh_id_ranges.iter() {
            if num1 >= range.0 && num1 <= range.1 {
                num1 = range.1 + 1;
            }

            if num2 >= range.0 && num2 <= range.1 {
                num2 = range.0 - 1;
            }

            if num1 > num2 {
                // Discard whole range
                discard = true;
                break;
            }
        }

        if !discard {
            fresh_id_ranges.push((num1, num2));

            solution += num2 - num1 + 1;
        }
    }

    println!["{}", solution];
}
