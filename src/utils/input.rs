use std::fs;
use super::grid::Grid;

/// Read input file as a single string
pub fn read_input(day: u8, use_test: bool) -> String {
    let filename = if use_test {
        format!("inputs/day{:02}_test.txt", day)
    } else {
        format!("inputs/day{:02}.txt", day)
    };
    fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", filename))
}

/// Read input file as a vector of lines
pub fn read_lines(day: u8, use_test: bool) -> Vec<String> {
    read_input(day, use_test)
        .lines()
        .map(|s| s.to_string())
        .collect()
}

/// Read input file as a character grid
pub fn read_grid(day: u8, use_test: bool) -> Grid<char> {
    Grid::from_string(&read_input(day, use_test))
}

/// Parse input lines into a vector of parsed items
pub fn parse_lines<T, F>(day: u8, use_test: bool, parser: F) -> Vec<T>
where
    F: Fn(&str) -> T,
{
    read_lines(day, use_test)
        .iter()
        .map(|line| parser(line))
        .collect()
}

/// Read input and parse as a vector of integers
pub fn read_ints(day: u8, use_test: bool) -> Vec<i32> {
    read_lines(day, use_test)
        .iter()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect()
}

/// Read input and parse as a vector of integers (i64)
pub fn read_ints64(day: u8, use_test: bool) -> Vec<i64> {
    read_lines(day, use_test)
        .iter()
        .filter_map(|line| line.parse::<i64>().ok())
        .collect()
}

/// Split input by blank lines into groups
pub fn read_groups(day: u8, use_test: bool) -> Vec<Vec<String>> {
    let input = read_input(day, use_test);
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|s| s.to_string())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_functions() {
        // These tests would require actual input files
        // They're here as examples of how to use the functions
    }
}
