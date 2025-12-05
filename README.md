# Advent of Code

Rust solutions for Advent of Code 2025 (planning to backport 2024 solutions from previous repo).

Project structure is heavily inspired by [maneatingape/advent-of-code-rust](https://github.com/maneatingape/advent-of-code-rust).
Logic for running multiple years are more or less directly adapted from there.
Tests have been changed to use txt files for input.
Benchmarks have been changed to use criterion.


All actual task solutions are mine.

## Usage

Add input files in the `inputs/yearXXXX/` directory, named `dayXX.txt` for each day.
Each day's solution should be in `src/yearXXXX/dayXX.rs` with the following structure:
```rust
pub fn parse(input: &str) -> YourType { ... }
pub fn part1(input: &YourType) -> u32 { ... }
pub fn part2(input: &YourType) -> u32 { ... }
```

Update `src/main.rs` to include the new day by adding it to the `run!` macro and the solutions array.

Update `benches/benchmarks.rs` to include benchmarks for the new day by adding it to the `benchmark!` macro and the `criterion_group!`.

Add tests in the `tests/yearXXXX/` directory, creating a new file `dayXX.rs` for each day with test cases for part 1 and part 2,
add the necessary input files in the `tests/yearXXXX/` directory as well (e.g. `dayXX_input.txt`).
Add the new files to the module for the test suite to recognize them.

## Running Solutions

```bash
# Run all solutions from all years
cargo run

# Run all solutions from a specific year
cargo run -- 2025

# Run a specific day (searches all years)
cargo run -- 1

# Run a specific year and day
cargo run -- 2025 1

# Show totals
cargo run -- --totals
```

## Testing

```bash
# Run all tests
cargo test

# Run tests for a specific module
cargo test year2025

# Run a specific test
cargo test year2025:day01::part1_test
```

## Benchmarking

```bash
# Run all benchmarks
cargo bench

# Run benchmarks for a specific year
cargo bench year2025

# Run benchmarks for a specific day
cargo bench year2025_day01

# Report summary of benchmarks
cargo bench --bench summary
```
