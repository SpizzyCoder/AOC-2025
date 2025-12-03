// Note: Holy fuck, this was a chokefest like no one has ever seen before.

pub fn solve(input: String) {
    let mut solution: i128 = 0;

    for line in input.lines() {
        let mut numbers_array: Vec<i32> = Vec::new();
        let mut solution_string: String = String::from("");

        for char in line.chars() {
            numbers_array.push(char.to_string().parse().unwrap());
        }

        let mut first_index: usize = 0;
        let mut last_index: usize = numbers_array.len() - 11;
        for _ in 0..12 {
            let mut largest_number_index: usize = first_index;
            for cur_index in first_index..last_index {
                if numbers_array[cur_index] > numbers_array[largest_number_index] {
                    largest_number_index = cur_index;
                }
            }

            solution_string.push_str(&format!["{}", numbers_array[largest_number_index]]);
            first_index = largest_number_index + 1;
            last_index += 1;
        }

        solution += &solution_string.parse().unwrap();
    }

    println!["{}", solution];
}
