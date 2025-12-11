use crate::utils::{Grid, Point};

type ParsedInput = Vec<(char, Grid<char>)>;

pub fn parse(input: &str) -> ParsedInput {
    // we have several lines, all except the last line consist of numbers
    // the lines are space-separated columns, but there could be varying amounts of spaces
    // The last line consists of a single char per column, and this will always be at the
    // first "index" of that column.
    // So, our approach will be to read the last line first to get the characters,
    // and at the same time we will find the width of each column.
    // Then we read each column into a grid of chars.

    // First, find the height of the grid (number of lines - 1)
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len() - 1;

    // Iterate over the last line to find the characters and their positions
    let last_line = lines.last().unwrap();
    let mut char_positions: Vec<(char, usize)> = Vec::new();
    for (i, ch) in last_line.chars().enumerate() {
        if ch != ' ' {
            char_positions.push((ch, i));
        }
    }

    // Now determine column boundaries
    // Each column starts at the character position and ends before the next column starts
    let mut columns: Vec<(char, Vec<Vec<char>>)> = Vec::new();
    for (idx, &(ch, pos)) in char_positions.iter().enumerate() {
        let col_start = pos;
        let col_end = if idx + 1 < char_positions.len() {
            char_positions[idx + 1].1
        } else {
            last_line.len()
        };

        // Now read the column data from the previous lines
        let mut grid_data: Vec<Vec<char>> = Vec::new();
        for line in &lines[0..height] {
            let col_str = if col_end <= line.len() {
                &line[col_start..col_end]
            } else {
                &line[col_start..]
            };
            let row: Vec<char> = col_str.chars().collect();
            grid_data.push(row);
        }
        columns.push((ch, grid_data));
    }

    // Convert columns to ParsedInput
    columns
        .into_iter()
        .map(|(ch, grid_data)| (ch, Grid::from_vec(grid_data)))
        .collect()
}

pub fn part1(_input: &ParsedInput) -> u64 {
    // Iterate over each (char, Grid<char>) pair
    // We should look at the chat to decide if we should sum (+) or multiply (*)
    // Then we add all chars per row and convert to u64, i.e. '3', '4', '5' -> 345
    let mut total: u64 = 0;
    for (ch, grid) in _input {
        let mut column_numbers: Vec<u64> = Vec::new();
        for y in 0..grid.height {
            let mut row_value: u64 = 0;
            for x in 0..grid.width {
                if let Some(c) = grid.get(&Point {
                    x: x as i32,
                    y: y as i32,
                }) {
                    if c.is_ascii_digit() {
                        row_value = row_value * 10 + (*c as u64 - '0' as u64);
                    }
                }
            }
            column_numbers.push(row_value);
        }
        // Now we have all row values for this column
        // Depending on ch, we either sum or multiply
        let column_total: u64 = if *ch == '+' {
            column_numbers.iter().sum()
        } else if *ch == '*' {
            column_numbers.iter().product()
        } else {
            0
        };
        total += column_total;
    }

    total
}

pub fn part2(_input: &ParsedInput) -> u64 {
    // Iterate over the input similarly to part1
    let mut total: u64 = 0;
    for (ch, grid) in _input {
        let mut column_numbers: Vec<u64> = Vec::new();
        // Now we have to produce the numbers to "operate" on
        // from top to bottom, instead of left to right.
        // The easiest way is to push chars into a string per column
        // and then parse that string as a u64.
        for x in 0..grid.width {
            let mut col_string = String::new();
            for y in 0..grid.height {
                if let Some(c) = grid.get(&Point {
                    x: x as i32,
                    y: y as i32,
                }) {
                    if c.is_ascii_digit() {
                        col_string.push(*c);
                    }
                }
            }
            if !col_string.is_empty() {
                if let Ok(value) = col_string.parse::<u64>() {
                    column_numbers.push(value);
                }
            }
        }
        // Now we have all column values for this column
        // Depending on ch, we either sum or multiply
        let column_total: u64 = if *ch == '+' {
            column_numbers.iter().sum()
        } else if *ch == '*' {
            column_numbers.iter().product()
        } else {
            0
        };
        total += column_total;
    }
    total
}
