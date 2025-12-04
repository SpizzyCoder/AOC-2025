pub fn solve(input: String) {
    let mut solution: i32 = 0;
    let mut map: Vec<Vec<bool>> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let mut map_line: Vec<bool> = Vec::new();

        for char in line.chars() {
            if char == '@' {
                map_line.push(true);
            } else {
                map_line.push(false);
            }
        }

        map.push(map_line);
    }

    for (line_index, line) in map.iter().enumerate() {
        for (pos_index, &cur_pos) in line.iter().enumerate() {
            if cur_pos {
                let mut counter: i32 = 0;

                // Upper line
                if line_index != 0 {
                    for i in (if pos_index == 0 { 0 } else { pos_index - 1 })..pos_index + 2 {
                        if let Some(&t) = map[line_index - 1].get(i) {
                            if t {
                                counter += 1;
                            }
                        }
                    }
                }

                // Left
                if pos_index != 0 {
                    if map[line_index][pos_index - 1] {
                        counter += 1;
                    }
                }

                // Right
                if let Some(&right) = map[line_index].get(pos_index + 1) {
                    if right {
                        counter += 1;
                    }
                }

                // Bottom line
                if let Some(bottom_section) = map.get(line_index + 1) {
                    for i in (if pos_index == 0 { 0 } else { pos_index - 1 })..pos_index + 2 {
                        if let Some(&t) = bottom_section.get(i) {
                            if t {
                                counter += 1;
                            }
                        }
                    }
                }

                if counter < 4 {
                    solution += 1;
                }
            }
        }
    }

    println!["{}", solution];
}
