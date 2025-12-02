use std::env::args;
use std::fs::read_to_string;
use std::time::{Duration, Instant};

mod year2025;
mod utils;

struct Solution {
    year: u32,
    day: u32,
    wrapper: fn(&str) -> (String, String),
}

fn main() {
    // Check for --test flag
    let use_test = args().any(|arg| arg == "--test");
    
    // Parse command line options
    let mut iter = args().skip(1).flat_map(|arg| arg.parse::<u32>().ok());
    let (year, day) = (iter.next(), iter.next());

    // Build list of all solutions
    let solutions = [year2025()];

    // Run selected solutions
    let (stars, duration) = solutions
        .iter()
        .flatten()
        .filter(|s| year.is_none() || year == Some(s.year))
        .filter(|s| day.is_none() || day == Some(s.day))
        .fold((0, Duration::ZERO), |acc, s| run_solution(acc, s, use_test));

    // Print totals
    if args().any(|arg| arg == "--totals") {
        println!("⭐ {stars}");
        println!("🕒 {duration:?}");
    }
}

fn run_solution((stars, duration): (u32, Duration), solution: &Solution, use_test: bool) -> (u32, Duration) {
    let Solution { year, day, wrapper } = solution;
    let path = if use_test {
        format!("tests/year{year}/day{day:02}_input.txt")
    } else {
        format!("inputs/year{year}/day{day:02}.txt")
    };

    if let Ok(data) = read_to_string(&path) {
        let instant = Instant::now();
        let (part1, part2) = wrapper(&data);
        let elapsed = instant.elapsed();

        println!("{year} Day {day:02}{}", if use_test { " (TEST)" } else { "" });
        println!("    Part 1: {part1}");
        println!("    Part 2: {part2}");

        (stars + 2, duration + elapsed)
    } else {
        eprintln!("{year} Day {day:02}");
        eprintln!("    Missing input!");
        eprintln!("    Place input file in {path}");

        (stars, duration)
    }
}

macro_rules! run {
    ($year:tt $($day:tt),*) => {
        fn $year() -> Vec<Solution> {
            vec![$(
                Solution {
                    year: stringify!($year).trim_start_matches("year").parse().unwrap(),
                    day: stringify!($day).trim_start_matches("day").parse().unwrap(),
                    wrapper: |data: &str| {
                        use $year::$day::*;

                        let input = parse(data);
                        let part1 = part1(&input).to_string();
                        let part2 = part2(&input).to_string();

                        (part1, part2)
                    }
                }
            ,)*]
        }
    }
}

run!(year2025
    day01, day02
);
