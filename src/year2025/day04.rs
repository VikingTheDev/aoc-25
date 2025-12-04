use crate::utils::{Grid, Point};
use std::collections::HashSet;

type ParsedInput = Grid<char>;

pub fn parse(input: &str) -> ParsedInput {
    // We create a 2d vector of chars
    let grid: Vec<Vec<char>> = input.trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // Use 2d vector to initialize Grid
    Grid::from_vec(grid)
}

pub fn part1(_input: &ParsedInput) -> u32{
    // Iterate over all points. For each point, check if it is a "toilet paper roll" (@)
    // If it is, we should count it if less than 4 of its 8 neighbors are also toilet paper rolls
    let mut count: u32 = 0;
    for y in 0.._input.height {
        for x in 0.._input.width {
            let point = Point { x: x as i32, y: y as i32 };
            if _input.get(&point) == Some(&'@') {
                let neighbors = _input.neighbors8(&point);
                let mut tp_count = 0;
                for neighbor in neighbors {
                    if _input.get(&neighbor) == Some(&'@') {
                        tp_count += 1;
                    }
                }
                if tp_count < 4 {
                    count += 1;
                }
            }
        }
    }
    count
}

pub fn part2(_input: &ParsedInput) -> u32{
    // Initial pass, collect all points to be removed
    let mut to_be_removed: Vec<Point> = Vec::new();
    for y in 0.._input.height {
        for x in 0.._input.width {
            let point = Point { x: x as i32, y: y as i32 };
            if _input.get(&point) == Some(&'@') {
                let neighbors = _input.neighbors8(&point);
                let mut tp_count = 0;
                for neighbor in neighbors {
                    if _input.get(&neighbor) == Some(&'@') {
                        tp_count += 1;
                    }
                }
                if tp_count < 4 {
                    to_be_removed.push(point);
                }
            }
        }
    }
    
    let mut count = to_be_removed.len() as u32;
    let mut grid = _input.clone();
    // Iterate until no more to be removed
    while !to_be_removed.is_empty() {
        to_be_removed = iterate(&mut grid, to_be_removed);
        count += to_be_removed.len() as u32;
    }

    count
}

pub fn iterate(grid: &mut Grid<char>, to_be_removed: Vec<Point>) -> Vec<Point> {
    // Step one, remove all points in to_be_removed
    for point in &to_be_removed {
        grid.set(point, 'x');
    }

    // Step two, go over to be removed points, and check their neighbors.
    // This should be fairly efficient, and should produce the correct result
    // as only neighbors will be affected by removal anyways. No need to check entire grid again.
    let mut new_to_be_removed = HashSet::new();
    for point in to_be_removed {
        let removed_neighbors = check_if_neighbours_should_be_removed(grid, &point);
        new_to_be_removed.extend(removed_neighbors);
    }
    new_to_be_removed.into_iter().collect()
}

pub fn check_if_neighbours_should_be_removed(grid: &mut Grid<char>, point: &Point) -> Vec<Point> {
    let mut to_be_removed = Vec::new();
    // Point should already be removed
    if grid.get(point) == Some(&'x') {
        let neighbors = grid.neighbors8(point);
        for neighbor in neighbors {
            if grid.get(&neighbor) == Some(&'@') {
                grid.neighbors8(&neighbor);
                // Check if this neighbor now has less than 4 '@' neighbors
                let mut tp_count = 0;
                for nn in grid.neighbors8(&neighbor) {
                    if grid.get(&nn) == Some(&'@') {
                        tp_count += 1;
                    }
                }
                if tp_count < 4 {
                    to_be_removed.push(neighbor);
                }
            }
        }
    }
    to_be_removed
}
