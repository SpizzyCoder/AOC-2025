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

            for chunk_size in 1..=num_as_string.len() / 2 {
                let first_chunk: &[u8] = &num_as_string.as_bytes()[..chunk_size];

                let mut okay: bool = true;
                for chunk in num_as_string.as_bytes().chunks(chunk_size) {
                    if chunk != first_chunk {
                        okay = false;
                        break;
                    }
                }

                if okay {
                    solution += i;
                    break;
                }
            }
        }
    }

    println!["{}", solution];
}
