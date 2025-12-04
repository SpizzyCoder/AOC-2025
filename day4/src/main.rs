use std::{env, fs};

mod part1;
mod part2;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!["Usage: {} <part> <input-file>", args[0]];
        return;
    }

    let input: String = fs::read_to_string(&args[2]).unwrap();

    match args[1].as_str() {
        "1" => part1::solve(input),
        "2" => part2::solve(input),
        _ => panic!["Invalid part -> can be either 1 or 2"],
    }
}
