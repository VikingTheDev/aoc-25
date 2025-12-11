use rayon::prelude::*;

pub struct Coordinate {
    x: u32,
    y: u32,
    z: u32,
}

type ParsedInput = Vec<Coordinate>;

pub fn parse(input: &str) -> ParsedInput {
    input
        .trim()
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(',');
            if let (Some(x_str), Some(y_str), Some(z_str)) =
                (parts.next(), parts.next(), parts.next())
            {
                if let (Ok(x), Ok(y), Ok(z)) = (
                    x_str.parse::<u32>(),
                    y_str.parse::<u32>(),
                    z_str.parse::<u32>(),
                ) {
                    return Some(Coordinate { x, y, z });
                }
            }
            None
        })
        .collect()
}

// Calculate squared euclidean distance (avoids sqrt - faster and sufficient for comparisons)
#[inline]
fn squared_distance(a: &Coordinate, b: &Coordinate) -> u64 {
    let dx = (a.x as i64) - (b.x as i64);
    let dy = (a.y as i64) - (b.y as i64);
    let dz = (a.z as i64) - (b.z as i64);
    (dx * dx + dy * dy + dz * dz) as u64
}

/// Compute all pairwise distances and return them sorted
fn compute_sorted_distances(input: &ParsedInput) -> Vec<(usize, usize, u64)> {
    let n = input.len();
    let mut distances: Vec<(usize, usize, u64)> = (0..n)
        .into_par_iter()
        .flat_map(|i| {
            ((i + 1)..n)
                .map(move |j| (i, j, squared_distance(&input[i], &input[j])))
                .collect::<Vec<_>>()
        })
        .collect();

    // Use parallel sort for better performance on large arrays
    distances.par_sort_unstable_by_key(|&(_, _, d)| d);
    distances
}

pub fn part1(input: &ParsedInput) -> u32 {
    let mut connections_to_make = 1000;
    if input.len() == 20 {
        connections_to_make = 10;
    }

    let mut parent: Vec<usize> = (0..input.len()).collect();
    let mut size: Vec<usize> = vec![1; input.len()];

    let distances = compute_sorted_distances(input);

    // Connect the closest pairs
    for &(i, j, _) in distances.iter().take(connections_to_make) {
        union(&mut parent, &mut size, i, j);
    }

    // Count circuit sizes
    use std::collections::HashMap;
    let mut circuit_sizes: HashMap<usize, usize> = HashMap::new();
    for i in 0..input.len() {
        let root = find(&mut parent, i);
        *circuit_sizes.entry(root).or_insert(0) += 1;
    }

    // Get three largest
    let mut sizes: Vec<usize> = circuit_sizes.values().copied().collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    sizes.iter().take(3).map(|&s| s as u32).product()
}

pub fn part2(input: &ParsedInput) -> u64 {
    let n = input.len();
    let mut parent: Vec<usize> = (0..n).collect();
    let mut size: Vec<usize> = vec![1; n];

    let distances = compute_sorted_distances(input);

    // Track last two connected points
    let mut last_i = 0;
    let mut last_j = 0;

    for &(i, j, _) in distances.iter() {
        let root_i = find(&mut parent, i);
        let root_j = find(&mut parent, j);
        if root_i != root_j {
            union(&mut parent, &mut size, i, j);
            last_i = i;
            last_j = j;
            // Check if all connected
            let first_root = find(&mut parent, 0);
            if size[first_root] == n {
                break;
            }
        }
    }

    (input[last_i].x as u64) * (input[last_j].x as u64)
}

// Union-Find union operation
pub fn union(parent: &mut Vec<usize>, size: &mut Vec<usize>, x: usize, y: usize) {
    let root_x = find(parent, x);
    let root_y = find(parent, y);

    if root_x != root_y {
        // Union by size
        if size[root_x] < size[root_y] {
            parent[root_x] = root_y;
            size[root_y] += size[root_x];
        } else {
            parent[root_y] = root_x;
            size[root_x] += size[root_y];
        }
    }
}

// Union-Find find operation with path compression
pub fn find(parent: &mut Vec<usize>, x: usize) -> usize {
    if parent[x] != x {
        parent[x] = find(parent, parent[x]); // Path compression
    }
    parent[x]
}
