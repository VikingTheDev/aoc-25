use std::fs::read_to_string;
use std::time::{Duration, Instant};
use std::hint::black_box;

macro_rules! benchmark_year {
    ($year:tt $($day:tt),*) => {{
        use aoc_25::$year;
        
        let year_num = stringify!($year).trim_start_matches("year").parse::<u32>().unwrap();
        println!("{}:", year_num);
        
        let mut total = Duration::ZERO;
        
        $(
            {
                let day_num = stringify!($day).trim_start_matches("day").parse::<u32>().unwrap();
                let path = format!("inputs/{}/day{:02}.txt", stringify!($year), day_num);
                
                if let Ok(data) = read_to_string(&path) {
                    let parsed = $year::$day::parse(&data);
                    
                    // Single run benchmark for timing
                    let start = Instant::now();
                    black_box($year::$day::part1(black_box(&parsed)));
                    let part1_time = start.elapsed();
                    
                    let start = Instant::now();
                    black_box($year::$day::part2(black_box(&parsed)));
                    let part2_time = start.elapsed();
                    
                    let day_total = part1_time + part2_time;
                    total += day_total;
                    
                    println!("  Day {:02}: Part 1: {:>8.2} μs, Part 2: {:>8.2} μs", 
                        day_num,
                        part1_time.as_secs_f64() * 1_000_000.0,
                        part2_time.as_secs_f64() * 1_000_000.0
                    );
                }
            }
        )*
        
        println!("\nTotal {}: {:.2} μs\n", year_num, total.as_secs_f64() * 1_000_000.0);
    }}
}

fn main() {
    benchmark_year!(year2025 day01, day02, day03, day04, day05, day06, day07, day08);
}
