pub fn solve(input: String) {
    let mut cur_pos: i32 = 50;
    let mut amount_of_zeros: i32 = 0;

    for line in input.lines() {
        let line_splitted: (&str, &str) = line.split_at(1);
        let number_parsed: i32 = line_splitted.1.parse().unwrap();

        if line_splitted.0 == "L" {
            for _ in 0..number_parsed {
                cur_pos -= 1;

                if cur_pos == 0 {
                    amount_of_zeros += 1;
                }

                if cur_pos == 100 {
                    cur_pos = 0;
                } else if cur_pos == -1 {
                    cur_pos = 99;
                }
            }
        } else if line_splitted.0 == "R" {
            for _ in 0..number_parsed {
                cur_pos += 1;

                if cur_pos == 0 {
                    amount_of_zeros += 1;
                }

                if cur_pos == 100 {
                    amount_of_zeros += 1; // Bro, this one line costed me like a whole fucking hour
                    cur_pos = 0;
                } else if cur_pos == -1 {
                    cur_pos = 99;
                }
            }
        }
    }

    println!["{}", amount_of_zeros];
}
