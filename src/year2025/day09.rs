use rayon::prelude::*;

type ParsedInput = Vec<(u64, u64)>;

pub fn parse(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect()
}

pub fn part1(_input: &ParsedInput) -> u64 {
    let mut largest_area: u64 = 0;

    // We have to use two points as opposite corners of a rectangle
    // and find the area of the largest rectangle
    for i in 0.._input.len() {
        for j in 0.._input.len() {
            if i != j {
                let (x1, y1) = _input[i];
                let (x2, y2) = _input[j];
                let width = if x2 > x1 { x2 - x1 } else { x1 - x2 };
                let height = if y2 > y1 { y2 - y1 } else { y1 - y2 };
                let area = (width + 1) * (height + 1);
                if area > largest_area {
                    largest_area = area;
                }
            }
        }
    }

    largest_area
}

pub fn part2(_input: &ParsedInput) -> u64 {
    // Build list of polygon edges (each edge is a line segment)
    let edges: Vec<((u64, u64), (u64, u64))> = (0.._input.len())
        .map(|i| {
            let next = (i + 1) % _input.len();
            (_input[i], _input[next])
        })
        .collect();

    // Pre-compute and sort vertical and horizontal edges for faster lookup
    let vertical_edges: Vec<(u64, u64, u64)> = edges
        .iter()
        .filter(|&&((ex1, _), (ex2, _))| ex1 == ex2)
        .map(|&((ex1, ey1), (_, ey2))| (ex1, ey1.min(ey2), ey1.max(ey2)))
        .collect();

    let horizontal_edges: Vec<(u64, u64, u64)> = edges
        .iter()
        .filter(|&&((_, ey1), (_, ey2))| ey1 == ey2)
        .map(|&((ex1, ey1), (ex2, _))| (ey1, ex1.min(ex2), ex1.max(ex2)))
        .collect();

    // Generate all pairs of indices
    let n = _input.len();
    let pairs: Vec<(usize, usize)> = (0..n)
        .flat_map(|i| ((i + 1)..n).map(move |j| (i, j)))
        .collect();

    // Process pairs in parallel
    pairs
        .par_iter()
        .filter_map(|&(i, j)| {
            let (x1, y1) = _input[i];
            let (x2, y2) = _input[j];

            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);

            if is_rectangle_inside_polygon_fast(
                &vertical_edges,
                &horizontal_edges,
                &edges,
                _input,
                min_x,
                max_x,
                min_y,
                max_y,
            ) {
                Some((max_x - min_x + 1) * (max_y - min_y + 1))
            } else {
                None
            }
        })
        .max()
        .unwrap_or(0)
}

// Faster version using pre-sorted edges
fn is_rectangle_inside_polygon_fast(
    vertical_edges: &[(u64, u64, u64)],
    horizontal_edges: &[(u64, u64, u64)],
    edges: &[((u64, u64), (u64, u64))],
    polygon: &[(u64, u64)],
    min_x: u64,
    max_x: u64,
    min_y: u64,
    max_y: u64,
) -> bool {
    // First check: the center of the rectangle must be inside the polygon
    let center_x = (min_x + max_x) / 2;
    let center_y = (min_y + max_y) / 2;

    if !is_point_inside_or_on_polygon(polygon, edges, center_x, center_y) {
        return false;
    }

    // Check vertical edges that cross interior
    for &(edge_x, edge_min_y, edge_max_y) in vertical_edges {
        if edge_x > min_x && edge_x < max_x {
            if edge_min_y < max_y && edge_max_y > min_y {
                return false;
            }
        }
    }

    // Check horizontal edges that cross interior
    for &(edge_y, edge_min_x, edge_max_x) in horizontal_edges {
        if edge_y > min_y && edge_y < max_y {
            if edge_min_x < max_x && edge_max_x > min_x {
                return false;
            }
        }
    }

    true
}

fn is_point_inside_or_on_polygon(
    polygon: &[(u64, u64)],
    edges: &[((u64, u64), (u64, u64))],
    x: u64,
    y: u64,
) -> bool {
    // Check if point is on boundary
    for &((ex1, ey1), (ex2, ey2)) in edges {
        if point_on_segment(x, y, ex1, ey1, ex2, ey2) {
            return true;
        }
    }

    // Ray casting for interior
    let mut inside = false;
    let n = polygon.len();

    for i in 0..n {
        let j = (i + 1) % n;
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];

        let intersect = ((yi > y) != (yj > y))
            && ((x as i64)
                < ((xj as i64 - xi as i64) * (y as i64 - yi as i64) / (yj as i64 - yi as i64)
                    + xi as i64));

        if intersect {
            inside = !inside;
        }
    }

    inside
}

fn point_on_segment(px: u64, py: u64, x1: u64, y1: u64, x2: u64, y2: u64) -> bool {
    // For axis-aligned segments only
    if x1 == x2 {
        // Vertical segment
        px == x1 && py >= y1.min(y2) && py <= y1.max(y2)
    } else if y1 == y2 {
        // Horizontal segment
        py == y1 && px >= x1.min(x2) && px <= x1.max(x2)
    } else {
        false
    }
}
