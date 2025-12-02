pub fn solve(input: String) {
    let ranges: Vec<&str> = input.split(",").collect();
    let mut solution: i128 = 0;

    for range in ranges {
        let range_splitted: Vec<&str> = range.split("-").collect();

        let range_parsed: (i128, i128) = (
            range_splitted[0].parse().unwrap(),
            range_splitted[1].parse().unwrap(),
        );

        for i in range_parsed.0..=range_parsed.1 {
            let num_as_string: String = format!["{}", i];

            if num_as_string.len() % 2 == 0 {
                // Could potentially be a number
                let num_as_string_splitted: (&str, &str) = num_as_string.split_at(num_as_string.len() / 2);

                if num_as_string_splitted.0 == num_as_string_splitted.1 {
                    solution += i;
                }
            }
        }
    }

    println!["{}", solution];
}
