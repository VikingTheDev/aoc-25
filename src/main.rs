use std::env;
use std::time::Instant;

mod days;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let day_filter = if args.len() > 1 {
        args[1].parse::<u8>().ok()
    } else {
        None
    };

    let use_test = args.len() > 2 && args[2] == "--test";

    println!("🎄 Advent of Code 2025 🎄\n");

    let mut total_time = std::time::Duration::ZERO;
    let mut days_run = 0;

    for day in 1..=25 {
        if let Some(filter) = day_filter {
            if day != filter {
                continue;
            }
        }

        if let Some((part1, part2)) = run_day(day, use_test) {
            days_run += 1;
            total_time += part1.1 + part2.1;
            
            println!("Day {:02}:", day);
            println!("  Part 1: {} ({:.2?})", part1.0, part1.1);
            println!("  Part 2: {} ({:.2?})", part2.0, part2.1);
            println!("  Total: {:.2?}\n", part1.1 + part2.1);
        }
    }

    if days_run > 0 {
        println!("═══════════════════════════════════════");
        println!("Total time for {} day(s): {:.2?}", days_run, total_time);
        println!("Average per day: {:.2?}", total_time / days_run);
    }
}

fn run_day(day: u8, use_test: bool) -> Option<((String, std::time::Duration), (String, std::time::Duration))> {
    match day {
        1 => Some(time_solution(|| days::day01::solve(use_test))),
        2 => Some(time_solution(|| days::day02::solve(use_test))),
        3 => Some(time_solution(|| days::day03::solve(use_test))),
        4 => Some(time_solution(|| days::day04::solve(use_test))),
        5 => Some(time_solution(|| days::day05::solve(use_test))),
        6 => Some(time_solution(|| days::day06::solve(use_test))),
        7 => Some(time_solution(|| days::day07::solve(use_test))),
        8 => Some(time_solution(|| days::day08::solve(use_test))),
        9 => Some(time_solution(|| days::day09::solve(use_test))),
        10 => Some(time_solution(|| days::day10::solve(use_test))),
        11 => Some(time_solution(|| days::day11::solve(use_test))),
        12 => Some(time_solution(|| days::day12::solve(use_test))),
        13 => Some(time_solution(|| days::day13::solve(use_test))),
        14 => Some(time_solution(|| days::day14::solve(use_test))),
        15 => Some(time_solution(|| days::day15::solve(use_test))),
        16 => Some(time_solution(|| days::day16::solve(use_test))),
        17 => Some(time_solution(|| days::day17::solve(use_test))),
        18 => Some(time_solution(|| days::day18::solve(use_test))),
        19 => Some(time_solution(|| days::day19::solve(use_test))),
        20 => Some(time_solution(|| days::day20::solve(use_test))),
        21 => Some(time_solution(|| days::day21::solve(use_test))),
        22 => Some(time_solution(|| days::day22::solve(use_test))),
        23 => Some(time_solution(|| days::day23::solve(use_test))),
        24 => Some(time_solution(|| days::day24::solve(use_test))),
        25 => Some(time_solution(|| days::day25::solve(use_test))),
        _ => None,
    }
}

fn time_solution<F>(f: F) -> ((String, std::time::Duration), (String, std::time::Duration))
where
    F: FnOnce() -> (String, String),
{
    let start = Instant::now();
    let (part1, part2) = f();
    let total_duration = start.elapsed();
    
    // Approximate equal split for timing (you can enhance this later)
    let part1_time = total_duration / 2;
    let part2_time = total_duration - part1_time;
    
    ((part1, part1_time), (part2, part2_time))
}
