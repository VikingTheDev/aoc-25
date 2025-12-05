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

                        // Benchmark parse, part1, and part2
                        let mut group = c.benchmark_group(format!("{}_day{:02}", stringify!($year), day_num));
                        group.sample_size(40);
                        
                        group.bench_function("parse", |b| {
                            b.iter(|| $year::$day::parse(black_box(&data)))
                        });
                        
                        group.bench_function("part1", |b| {
                            b.iter(|| $year::$day::part1(black_box(&parsed)))
                        });
                        
                        group.bench_function("part2", |b| {
                            b.iter(|| $year::$day::part2(black_box(&parsed)))
                        });
                        group.finish();
                    }
                }
            )*
        }
    }
}

benchmark!(year2025
    day01, day02, day03, day04, day05
);

fn custom_criterion() -> Criterion {
    Criterion::default()
        .with_output_color(true)
        .plotting_backend(criterion::PlottingBackend::Plotters)
}

criterion_group! {
    name = benches;
    config = custom_criterion();
    targets = year2025
}
criterion_main!(benches);
