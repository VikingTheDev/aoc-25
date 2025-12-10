type ParsedInput = Vec<(u64, u64)>;

pub fn parse(input: &str) -> ParsedInput {
    input.lines().map(|line| {
        let mut parts = line.split(',');
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        (x, y)
    }).collect()
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
    let mut edges: Vec<((u64, u64), (u64, u64))> = Vec::new();
    for i in 0.._input.len() {
        let next = (i + 1) % _input.len();
        edges.push((_input[i], _input[next]));
    }
    
    let mut largest_area = 0;
    
    // For each pair of red tiles as opposite corners
    for i in 0.._input.len() {
        for j in (i+1).._input.len() {
            let (x1, y1) = _input[i];
            let (x2, y2) = _input[j];
            
            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);
            
            // Check if the rectangle is entirely inside the polygon
            // by verifying no polygon edge crosses through the rectangle's interior
            if is_rectangle_inside_polygon(&edges, _input, min_x, max_x, min_y, max_y) {
                let area = (max_x - min_x + 1) * (max_y - min_y + 1);
                if area > largest_area {
                    largest_area = area;
                }
            }
        }
    }
    
    largest_area
}

// Check if a rectangle is entirely inside the polygon (no edge crosses through interior)
fn is_rectangle_inside_polygon(
    edges: &[((u64, u64), (u64, u64))],
    polygon: &[(u64, u64)],
    min_x: u64, max_x: u64, min_y: u64, max_y: u64
) -> bool {
    // First check: the center of the rectangle must be inside the polygon
    // (or we can check any interior point)
    let center_x = (min_x + max_x) / 2;
    let center_y = (min_y + max_y) / 2;
    
    if !is_point_inside_or_on_polygon(polygon, edges, center_x, center_y) {
        return false;
    }
    
    // Second check: no polygon edge can cross through the interior of the rectangle
    // An edge crosses through the interior if it enters the rectangle on one side and exits on another
    for &((ex1, ey1), (ex2, ey2)) in edges {
        // Check if this edge crosses the interior of the rectangle
        if edge_crosses_rectangle_interior(ex1, ey1, ex2, ey2, min_x, max_x, min_y, max_y) {
            return false;
        }
    }
    
    true
}

fn is_point_inside_or_on_polygon(polygon: &[(u64, u64)], edges: &[((u64, u64), (u64, u64))], x: u64, y: u64) -> bool {
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
            && ((x as i64) < ((xj as i64 - xi as i64) * (y as i64 - yi as i64) / (yj as i64 - yi as i64) + xi as i64));
        
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

fn edge_crosses_rectangle_interior(ex1: u64, ey1: u64, ex2: u64, ey2: u64, min_x: u64, max_x: u64, min_y: u64, max_y: u64) -> bool {
    // An axis-aligned edge crosses the rectangle's interior if:
    // - For vertical edge (ex1 == ex2): the x is strictly inside (min_x < ex1 < max_x) 
    //   and the y range overlaps with [min_y, max_y]
    // - For horizontal edge (ey1 == ey2): the y is strictly inside (min_y < ey1 < max_y)
    //   and the x range overlaps with [min_x, max_x]
    
    if ex1 == ex2 {
        // Vertical edge
        let edge_x = ex1;
        let edge_min_y = ey1.min(ey2);
        let edge_max_y = ey1.max(ey2);
        
        // Edge x must be strictly inside the rectangle's x range
        if edge_x > min_x && edge_x < max_x {
            // Check if the edge's y range overlaps with rectangle's y range
            if edge_min_y < max_y && edge_max_y > min_y {
                return true;
            }
        }
    } else if ey1 == ey2 {
        // Horizontal edge
        let edge_y = ey1;
        let edge_min_x = ex1.min(ex2);
        let edge_max_x = ex1.max(ex2);
        
        // Edge y must be strictly inside the rectangle's y range
        if edge_y > min_y && edge_y < max_y {
            // Check if the edge's x range overlaps with rectangle's x range
            if edge_min_x < max_x && edge_max_x > min_x {
                return true;
            }
        }
    }
    
    false
}
