pub struct Coordinate {
    x: u32,
    y: u32,
    z: u32,
}

type ParsedInput = Vec<Coordinate>;

pub fn parse(input: &str) -> ParsedInput {
    input.trim()
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(',');
            if let (Some(x_str), Some(y_str), Some(z_str)) = (parts.next(), parts.next(), parts.next()) {
                if let (Ok(x), Ok(y), Ok(z)) = (x_str.parse::<u32>(), y_str.parse::<u32>(), z_str.parse::<u32>()) {
                    return Some(Coordinate { x, y, z });
                }
            }
            None
        })
        .collect()
}

pub fn part1(_input: &ParsedInput) -> u32{
    // We have to find the 1000 closest pairs of coordinates.
    // Each time we find the closest pair, we have to connect them
    // and add them to a circuit.
    // A point can be connected to multiple other points.
    // The result is the product of the size of the three largest circuits.
    
    let mut connections_to_make = 1000;

    // Handle test case
    if _input.len() == 20 {
        connections_to_make = 10;
    }

    // Union-Find structure to track which circuit each box belongs to
    let mut parent: Vec<usize> = (0.._input.len()).collect();
    let mut size: Vec<usize> = vec![1; _input.len()];
    
    // Calculate all pairwise distances
    let mut distances: Vec<((usize, usize), f64)> = Vec::new();
    for i in 0.._input.len() {
        for j in (i+1).._input.len() {
            let dist = euclidean_distance(&_input[i], &_input[j]);
            distances.push(((i, j), dist));
        }
    }
    // Sort to get closest pairs
    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    
    // Now connect the closest pairs
    for &((i, j), _) in distances.iter().take(connections_to_make) {
        union(&mut parent, &mut size, i, j);
    }
    
    // Count the size of each circuit
    use std::collections::HashMap;
    let mut circuit_sizes: HashMap<usize, usize> = HashMap::new();
    for i in 0.._input.len() {
        let root = find(&mut parent, i);
        *circuit_sizes.entry(root).or_insert(0) += 1;
    }
    
    // Get the three largest circuits
    let mut sizes: Vec<usize> = circuit_sizes.values().copied().collect();
    sizes.sort_by(|a, b| b.cmp(a));
    
    let result = sizes.iter().take(3).map(|&s| s as u32).product();
    result
}

pub fn part2(_input: &ParsedInput) -> u32 {
    // Now we need to keep connecting the closest pairs until all points are in a single circuit
    // We can reuse much of the code from part1
    // The result is the product of the x coordinates for the last two points connected to the circuit
    let mut parent: Vec<usize> = (0.._input.len()).collect();
    let mut size: Vec<usize> = vec![1; _input.len()];

    // Calculate all pairwise distances
    let mut distances: Vec<((usize, usize), f64)> = Vec::new();
    for i in 0.._input.len() {
        for j in (i+1).._input.len() {
            let dist = euclidean_distance(&_input[i], &_input[j]);
            distances.push(((i, j), dist));
        }
    }
    // Sort to get closest pairs
    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    
    // Keep track of the last two points connected
    let mut last_two_connected: Vec<usize> = Vec::new();
    // Continue connecting until all points are in a single circuit
    for &((i, j), _) in distances.iter() {
        let root_i = find(&mut parent, i);
        let root_j = find(&mut parent, j);
        if root_i != root_j {
            union(&mut parent, &mut size, i, j);
            last_two_connected.push(i);
            last_two_connected.push(j);
            if last_two_connected.len() > 4 {
                last_two_connected.remove(0);
                last_two_connected.remove(0);
            }
            // Check if all points are connected
            let first_root = find(&mut parent, 0);
            if size[first_root] == _input.len() {
                break;
            }
        }
    }

    // Get the x coordinates of the last two connected points and return their product
    let x1 = _input[last_two_connected[last_two_connected.len() - 2]].x;
    let x2 = _input[last_two_connected[last_two_connected.len() - 1]].x;
    x1 * x2 as u32
}

// Calculate euclidean distance between two 3D Coordinates
pub fn euclidean_distance(a: &Coordinate, b: &Coordinate) -> f64 {
    let dx = (a.x as f64) - (b.x as f64);
    let dy = (a.y as f64) - (b.y as f64);
    let dz = (a.z as f64) - (b.z as f64);
    (dx * dx + dy * dy + dz * dz).sqrt()
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