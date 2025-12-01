use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use std::fs::read_to_string;

macro_rules! benchmark {
    ($year:tt $($day:tt),*) => {
        fn $year(c: &mut Criterion) {
            use aoc_25::$year;
            $(
                {
                    let day_num = stringify!($day).trim_start_matches("day").parse::<u32>().unwrap();
                    let path = format!("inputs/{}/day{:02}.txt", stringify!($year), day_num);
                    
                    if let Ok(data) = read_to_string(&path) {
                        let parsed = $year::$day::parse(&data);

                        c.bench_function(&format!("{}_day{:02}_parse", stringify!($year), day_num), |b| {
                            b.iter(|| $year::$day::parse(black_box(&data)))
                        });

                        c.bench_function(&format!("{}_day{:02}_part1", stringify!($year), day_num), |b| {
                            b.iter(|| $year::$day::part1(black_box(&parsed)))
                        });

                        c.bench_function(&format!("{}_day{:02}_part2", stringify!($year), day_num), |b| {
                            b.iter(|| $year::$day::part2(black_box(&parsed)))
                        });
                    }
                }
            )*
        }
    }
}

benchmark!(year2025
    day01 
);

// Add more years as needed:
// benchmark!(year2024
//     day01, day02
// );

criterion_group!(benches, year2025);
criterion_main!(benches);
