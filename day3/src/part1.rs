pub fn solve(input: String) {
    let mut solution: i32 = 0;

    for line in input.lines() {
        let mut numbers_array: Vec<i32> = Vec::new();

        for char in line.chars() {
            numbers_array.push(char.to_string().parse().unwrap());
        }

        let mut max_left: (i32, usize) = (numbers_array[0], 0);
        for (index, number) in numbers_array[..numbers_array.len() - 1].iter().enumerate() {
            if *number > max_left.0 {
                max_left = (*number, index);
            }
        }

        let mut max_right: (i32, usize) = (
            numbers_array[numbers_array.len() - 1],
            numbers_array.len() - 1,
        );
        for (index, number) in numbers_array[max_left.1 + 1..].iter().enumerate() {
            if *number > max_right.0 {
                max_right = (*number, max_left.1 + 1 + index);
            }
        }

        solution += &format!["{}{}", max_left.0, max_right.0].parse().unwrap();
    }

    println!["{}", solution];
}
