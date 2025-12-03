use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, measurement::WallTime};
use criterion::black_box;
use std::fs::read_to_string;
use std::collections::BTreeMap;

macro_rules! benchmark {
    ($year:tt $($day:tt),*) => {
        fn $year(c: &mut Criterion) {
            use aoc_25::$year;
            
            let mut results: BTreeMap<u32, (f64, f64)> = BTreeMap::new();
            
            $(
                {
                    let day_num = stringify!($day).trim_start_matches("day").parse::<u32>().unwrap();
                    let path = format!("inputs/{}/day{:02}.txt", stringify!($year), day_num);
                    
                    if let Ok(data) = read_to_string(&path) {
                        let parsed = $year::$day::parse(&data);

                        // Benchmark part1
                        let mut group = c.benchmark_group(format!("{}_day{:02}", stringify!($year), day_num));
                        group.bench_function("part1", |b| {
                            b.iter(|| $year::$day::part1(black_box(&parsed)))
                        });
                        
                        // Benchmark part2
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
    day01, day02, day3
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
