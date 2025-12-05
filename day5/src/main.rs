use std::{
    env, fs,
    time::{Duration, Instant},
};

mod part1;
mod part2;

const BENCH_RUNS: u32 = 10_000;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args.len() > 4 {
        eprintln!["Usage: {} <part> <input-file> [bench]", args[0]];
        return;
    }

    let input: String = fs::read_to_string(&args[2]).unwrap();

    let mut solution: i128 = 0;

    let runs: u32 = if let Some(arg) = args.get(3) {
        if arg == "bench" { BENCH_RUNS } else { 1 }
    } else {
        1
    };
    let mut average_time: Duration = Duration::from_secs(0);
    match args[1].as_str() {
        "1" => {
            for _ in 0..runs {
                let now: Instant = Instant::now();
                solution = part1::solve(&input);
                average_time += now.elapsed();
            }
        }
        "2" => {
            for _ in 0..runs {
                let now: Instant = Instant::now();
                solution = part2::solve(&input);
                average_time += now.elapsed();
            }
        }
        _ => panic!["Invalid part -> can be either 1 or 2"],
    };
    println!["{}", solution];
    println!["Elapsed: {:?}", average_time / runs as u32];
}
