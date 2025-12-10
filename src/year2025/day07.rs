use crate::utils::{Grid, Point};

type ParsedInput = Grid<char>;

pub fn parse(input: &str) -> ParsedInput {
    let cells = input.lines().map(|s| s.chars().collect()).collect::<Vec<Vec<char>>>();

    Grid::from_vec(cells)
}

pub fn part1(_input: &ParsedInput) -> u32 {
    // We start at the top line, and move down.
    // When we encounter a 'S', we place a beam (|) directly below it. (only one S)
    // The beam continues until it hits a splitter (^), or the bottom of the grid.
    // When it hits a splitter, two beams are created, directly left and right of
    // the splitter (i-1, i+1), unless that position is already occupied by a beam.
    // We should count the total amount of splits.
    let mut grid = _input.clone();
    let mut split_count: u32 = 0;

    for x in 0..grid.width {
        if grid.get(&Point { x: x as i32, y: 0 }) == Some(&'S') {
            let mut beam_positions: Vec<Point> = vec![Point { x: x as i32, y: 1 }];
            while !beam_positions.is_empty() {
                let mut new_beam_positions: Vec<Point> = Vec::new();
                for beam_pos in beam_positions {
                    if beam_pos.y >= grid.height as i32 {
                        continue; // Beam has reached the bottom
                    }
                    match grid.get(&beam_pos) {
                        Some(&'^') => {
                            // Split the beam
                            let left_pos = Point { x: beam_pos.x - 1, y: beam_pos.y + 1 };
                            let right_pos = Point { x: beam_pos.x + 1, y: beam_pos.y + 1 };
                            if grid.get(&left_pos) != Some(&'|') && left_pos.x >= 0 && (left_pos.x as usize) < grid.width {
                                new_beam_positions.push(left_pos);
                            }
                            if grid.get(&right_pos) != Some(&'|') && (right_pos.x as usize) < grid.width {
                                new_beam_positions.push(right_pos);
                            }
                            split_count += 1;
                        }
                        Some(&'.') => {
                            // Continue the beam downwards
                            new_beam_positions.push(Point { x: beam_pos.x, y: beam_pos.y + 1 });
                        }
                        _ => {
                            // Beam is blocked or out of bounds
                        }
                    }
                    // Mark the beam position
                    grid.set(&beam_pos, '|');
                }
                beam_positions = new_beam_positions;
            }
        }
    }
    
    split_count
}

pub fn part2(_input: &ParsedInput) -> u64{
    // Count all distinct paths from S to the bottom using dynamic programming
    // We track the number of paths that reach each position
    use std::collections::HashMap;
    
    let grid = _input;
    let mut path_counts: HashMap<Point, u64> = HashMap::new();
    
    // Find the starting position
    let mut start_x = None;
    for x in 0..grid.width {
        if grid.get(&Point { x: x as i32, y: 0 }) == Some(&'S') {
            start_x = Some(x as i32);
            break;
        }
    }
    
    let start_x = match start_x {
        Some(x) => {
            x
        },
        None => {
            return 0;
        }
    };
    
    // Initialize: the position below S has 1 path
    let start_pos = Point { x: start_x, y: 1 };
    path_counts.insert(start_pos, 1);
    
    // Process layer by layer from top to bottom
    // Go one layer beyond to count exits
    for y in 1..=(grid.height as i32) {
        // Collect all positions at current layer that have paths
        let current_layer: Vec<(Point, u64)> = path_counts
            .iter()
            .filter(|(p, _)| p.y == y)
            .map(|(p, &count)| (*p, count))
            .collect();
        
        if !current_layer.is_empty() {
        }
        
        for (pos, count) in current_layer {
            let cell = grid.get(&pos);
            match cell {
                Some(&'^') => {
                    // Split: paths go left and right, then down
                    let left_pos = Point { x: pos.x - 1, y: pos.y + 1 };
                    let right_pos = Point { x: pos.x + 1, y: pos.y + 1 };
                    
                    if left_pos.x >= 0 && (left_pos.x as usize) < grid.width {
                        *path_counts.entry(left_pos).or_insert(0) += count;
                    }
                    if right_pos.x >= 0 && (right_pos.x as usize) < grid.width {
                        *path_counts.entry(right_pos).or_insert(0) += count;
                    }
                }
                Some(&'.') => {
                    // Continue downwards
                    let next_pos = Point { x: pos.x, y: pos.y + 1 };
                    *path_counts.entry(next_pos).or_insert(0) += count;
                }
                _ => {
                    // Blocked, invalid, or out of bounds - paths end here
                }
            }
        }
    }
    
    // Count all paths that reached y == height (just past the bottom row)
    let mut total_paths = 0;
    for (pos, count) in path_counts.iter() {
        if pos.y == grid.height as i32 {
            total_paths += count;
        }
    }
    
    total_paths
}
