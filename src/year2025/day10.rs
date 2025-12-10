// Seems like the longest line of lights is 10, so 16 bits is sufficient.
// Using u16 with each bit representing on/off allows bitwise operations for fast checks.
type IndicatorLights = u16;
type ButtonWiring = Vec<u16>;
type JoltageReqs = Vec<u16>;

type ParsedInput = Vec<(IndicatorLights, ButtonWiring, JoltageReqs)>;

pub fn parse(input: &str) -> ParsedInput {
    // On each line:
    // - Indicator lights wrapped in [] (# for on, . for off)
    // - One or more button wiring schematics wrapped in () (e.g. (3) (1,3) (2) etc.)
    // - Joltage requirements wrapped in {} (e.g. {3,5,4,7}). Length matches number of indicator lights
    // All parts are space-separated, and follow the order above.
input.lines().map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        // Parse indicator lights
        let lights_str = parts[0];
        let lights_chars: Vec<char> = lights_str.trim_matches(&['[', ']'][..]).chars().collect();
        let num_lights = lights_chars.len();
        let mut lights: IndicatorLights = 0x0000;
        for ch in &lights_chars {
            lights <<= 1;
            if *ch == '#' {
                lights |= 0x0001;
            }
        }
        // Parse button wiring
        // Button indices are 0-indexed from the LEFT, but our bit representation
        // has the leftmost light as the MSB. So button index i maps to bit (num_lights - 1 - i).
        let mut wiring: ButtonWiring = Vec::new();
        for wiring_str in &parts[1..parts.len()-1] {
            let wiring_clean = wiring_str.trim_matches(&['(', ')'][..]);
            let mut wiring_bits: u16 = 0x0000;
            for ch in wiring_clean.split(',') {
                let btn_index: usize = ch.parse().unwrap();
                let bit_pos = num_lights - 1 - btn_index;
                wiring_bits |= 1 << bit_pos;
            }
            wiring.push(wiring_bits);
        }
        // Parse joltage requirements
        let joltage_str = parts[parts.len()-1];
        let joltage_clean = joltage_str.trim_matches(&['{', '}'][..]);
        let joltage_reqs: JoltageReqs = joltage_clean.split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        (lights, wiring, joltage_reqs)
    }).collect()
}

pub fn part1(_input: &ParsedInput) -> u32{
    // Each light starts at 0.
    // We can ignore the joltage requirements for part 1.
    // For each button press, we toggle the lights according to the wiring.
    // We can press each button any number of times.
    // We have to find the fewest button presses for the lights to match the target state.
    // The result is the sum of the fewest button presses for each input line.
    //
    // Optimization: Use fixed-size array for visited states instead of HashSet.
    let mut total_presses: u32 = 0;
    
    // Reuse visited array across iterations (clear with generation counter)
    let mut visited: Vec<u32> = vec![0; 65536];
    let mut generation: u32 = 0;
    
    for (_idx, (target_lights, wiring, _joltage_reqs)) in _input.iter().enumerate() {
        generation += 1;
        
        if *target_lights == 0 {
            // Already at target
            continue;
        }
        
        use std::collections::VecDeque;
        let mut queue: VecDeque<(u16, u32)> = VecDeque::new();
        
        queue.push_back((0, 0)); // Start with all lights off
        visited[0] = generation;
        
        while let Some((current_lights, presses)) = queue.pop_front() {
            if current_lights == *target_lights {
                total_presses += presses;
                break;
            }
            // Try pressing each button
            for &button in wiring.iter() {
                let next_lights = current_lights ^ button;
                if visited[next_lights as usize] != generation {
                    visited[next_lights as usize] = generation;
                    queue.push_back((next_lights, presses + 1));
                }
            }
        }
    }
    total_presses
}

pub fn part2(_input: &ParsedInput) -> u32 {
    // We need each light to reach EXACTLY its joltage requirement.
    // This is: A * x = b, minimize sum(x), x >= 0
    // Where A[light][button] = 1 if button affects light
    //
    // Use Gaussian elimination to identify dependencies, then search free vars.
    
    let mut total_presses: u32 = 0;

    for (_idx, (_target_lights, wiring, joltage_reqs)) in _input.iter().enumerate() {
        let num_lights = joltage_reqs.len();
        let num_buttons = wiring.len();
        
        // Build matrix: affects[light][button] = 1 if button affects light
        let mut affects: Vec<Vec<i64>> = vec![vec![0; num_buttons]; num_lights];
        for (btn_idx, &button) in wiring.iter().enumerate() {
            for light_idx in 0..num_lights {
                let bit_pos = num_lights - 1 - light_idx;
                if (button & (1 << bit_pos)) != 0 {
                    affects[light_idx][btn_idx] = 1;
                }
            }
        }
        let reqs: Vec<i64> = joltage_reqs.iter().map(|&r| r as i64).collect();
        
        let result = solve_system(&affects, &reqs, num_lights, num_buttons);
        total_presses += result;
    }

    total_presses
}

fn solve_system(
    affects: &[Vec<i64>],
    reqs: &[i64],
    num_lights: usize,
    num_buttons: usize,
) -> u32 {
    // Create augmented matrix and perform row reduction
    let mut matrix: Vec<Vec<i64>> = vec![vec![0; num_buttons + 1]; num_lights];
    for i in 0..num_lights {
        for j in 0..num_buttons {
            matrix[i][j] = affects[i][j];
        }
        matrix[i][num_buttons] = reqs[i];
    }
    
    // Gaussian elimination to get row echelon form (fully reduced)
    let mut pivot_row = 0;
    let mut pivot_cols: Vec<usize> = Vec::new();
    let mut free_cols: Vec<usize> = Vec::new();
    
    for col in 0..num_buttons {
        // Find row with non-zero in this column
        let mut found = false;
        for row in pivot_row..num_lights {
            if matrix[row][col] != 0 {
                matrix.swap(pivot_row, row);
                found = true;
                break;
            }
        }
        
        if found {
            pivot_cols.push(col);
            // Eliminate this column from ALL other rows (not just below)
            for row in 0..num_lights {
                if row != pivot_row && matrix[row][col] != 0 {
                    let factor = matrix[row][col];
                    let pivot_val = matrix[pivot_row][col];
                    for c in 0..=num_buttons {
                        matrix[row][c] = matrix[row][c] * pivot_val - matrix[pivot_row][c] * factor;
                    }
                }
            }
            pivot_row += 1;
        } else {
            free_cols.push(col);
        }
    }
    
    let mut best = u32::MAX;
    
    if free_cols.is_empty() {
        // Unique solution - just compute it
        let mut presses = vec![0i64; num_buttons];
        for (row, &pivot_col) in pivot_cols.iter().enumerate() {
            let rhs = matrix[row][num_buttons];
            let coef = matrix[row][pivot_col];
            if coef != 0 {
                if rhs % coef != 0 {
                    return u32::MAX;
                }
                presses[pivot_col] = rhs / coef;
            }
        }
        if presses.iter().all(|&p| p >= 0) {
            return presses.iter().sum::<i64>() as u32;
        }
        return u32::MAX;
    }
    
    // Compute per-free-variable bounds based on constraints
    // For each row: pivot_coef * pivot_var = rhs - sum(other_coefs * other_vars)
    // Since all vars >= 0, we can derive bounds
    let mut free_bounds: Vec<i64> = vec![i64::MAX; free_cols.len()];
    
    for (row, &pivot_col) in pivot_cols.iter().enumerate() {
        let pivot_coef = matrix[row][pivot_col];
        let rhs = matrix[row][num_buttons];
        
        // For pivot_var >= 0: rhs - sum(other terms) must have same sign as pivot_coef
        // This gives constraints on free variables
        for (fi, &free_col) in free_cols.iter().enumerate() {
            let free_coef = matrix[row][free_col];
            if free_coef != 0 {
                // If free_coef and pivot_coef have opposite signs, free var contributes positively to pivot
                // If same signs, free var contributes negatively - bound the free var
                if (free_coef > 0) == (pivot_coef > 0) {
                    // Same sign: increasing free var decreases pivot var
                    // pivot = (rhs - free_coef * free) / pivot_coef
                    // For pivot >= 0: free <= rhs / free_coef (if both positive)
                    let bound = rhs.abs() / free_coef.abs();
                    free_bounds[fi] = free_bounds[fi].min(bound + 1);
                }
            }
        }
    }
    
    // Also bound by total requirement
    let total_req: i64 = reqs.iter().sum();
    for fb in free_bounds.iter_mut() {
        *fb = (*fb).min(total_req + 1);
    }
    
    // Search over free variables with pruning
    try_free_vars(&matrix, &pivot_cols, &free_cols, &free_bounds, num_buttons, 0, &mut vec![0i64; free_cols.len()], &mut best, 0);
    
    best
}

fn try_free_vars(
    matrix: &[Vec<i64>],
    pivot_cols: &[usize],
    free_cols: &[usize],
    free_bounds: &[i64],
    num_buttons: usize,
    idx: usize,
    free_vals: &mut Vec<i64>,
    best: &mut u32,
    current_sum: i64,
) {
    // Early pruning: if current sum already exceeds best, stop
    if current_sum as u32 >= *best {
        return;
    }
    
    if idx == free_cols.len() {
        // Solve for pivot variables given free variable values
        let mut presses = vec![0i64; num_buttons];
        
        // Set free variables
        for (i, &col) in free_cols.iter().enumerate() {
            presses[col] = free_vals[i];
        }
        
        // Back-substitute to find pivot variables
        for (row, &pivot_col) in pivot_cols.iter().enumerate() {
            let mut rhs = matrix[row][num_buttons];
            for col in 0..num_buttons {
                if col != pivot_col {
                    rhs -= matrix[row][col] * presses[col];
                }
            }
            let coef = matrix[row][pivot_col];
            if coef != 0 {
                if rhs % coef != 0 {
                    return; // No integer solution
                }
                presses[pivot_col] = rhs / coef;
            }
        }
        
        // Check all non-negative
        if presses.iter().all(|&p| p >= 0) {
            let total: i64 = presses.iter().sum();
            if total >= 0 && (total as u32) < *best {
                *best = total as u32;
            }
        }
        return;
    }
    
    // Try values for this free variable up to its bound
    let bound = free_bounds[idx];
    for v in 0..=bound {
        let new_sum = current_sum + v;
        if new_sum as u32 >= *best {
            break;
        }
        free_vals[idx] = v;
        try_free_vars(matrix, pivot_cols, free_cols, free_bounds, num_buttons, idx + 1, free_vals, best, new_sum);
    }
}
