use rayon::prelude::*;

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
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            // Parse indicator lights
            let lights_str = parts[0];
            let lights_chars: Vec<char> =
                lights_str.trim_matches(&['[', ']'][..]).chars().collect();
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
            for wiring_str in &parts[1..parts.len() - 1] {
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
            let joltage_str = parts[parts.len() - 1];
            let joltage_clean = joltage_str.trim_matches(&['{', '}'][..]);
            let joltage_reqs: JoltageReqs = joltage_clean
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            (lights, wiring, joltage_reqs)
        })
        .collect()
}

pub fn part1(_input: &ParsedInput) -> u32 {
    // Each light starts at 0.
    // We can ignore the joltage requirements for part 1.
    // For each button press, we toggle the lights according to the wiring.
    // We can press each button any number of times.
    // We have to find the fewest button presses for the lights to match the target state.
    // The result is the sum of the fewest button presses for each input line.
    let mut total_presses: u32 = 0;
    for (_idx, (target_lights, wiring, _joltage_reqs)) in _input.iter().enumerate() {
        // BFS to find fewest presses
        use std::collections::{HashSet, VecDeque};
        let mut queue: VecDeque<(IndicatorLights, u32)> = VecDeque::new();
        let mut visited: HashSet<IndicatorLights> = HashSet::new();
        queue.push_back((0x0000, 0)); // Start with all lights off
        visited.insert(0x0000);
        let mut found_presses: u32 = 0;
        while let Some((current_lights, presses)) = queue.pop_front() {
            if &current_lights == target_lights {
                found_presses = presses;
                break;
            }
            // Try pressing each button
            for &button in wiring.iter() {
                let next_lights = current_lights ^ button; // Toggle lights
                if !visited.contains(&next_lights) {
                    visited.insert(next_lights);
                    queue.push_back((next_lights, presses + 1));
                }
            }
        }
        total_presses += found_presses;
    }
    total_presses
}

pub fn part2(input: &ParsedInput) -> u32 {
    // Process all machines in parallel
    input
        .par_iter()
        .map(|(_target_lights, wiring, joltage_reqs)| solve_joltage_ilp(wiring, joltage_reqs))
        .sum()
}

/// Solve the joltage problem using Gaussian elimination + bounded search.
fn solve_joltage_ilp(wiring: &[u16], joltage_reqs: &[u16]) -> u32 {
    let num_counters = joltage_reqs.len();
    let num_buttons = wiring.len();

    // Build matrix A where A[i][j] = 1 if button j affects counter i
    let mut matrix: Vec<Vec<i64>> = vec![vec![0; num_buttons + 1]; num_counters];
    for (btn_idx, &button) in wiring.iter().enumerate() {
        for counter_idx in 0..num_counters {
            let bit_pos = num_counters - 1 - counter_idx;
            if (button & (1 << bit_pos)) != 0 {
                matrix[counter_idx][btn_idx] = 1;
            }
        }
    }
    for (counter_idx, &req) in joltage_reqs.iter().enumerate() {
        matrix[counter_idx][num_buttons] = req as i64;
    }

    // Gaussian elimination
    let mut pivot_row = 0;
    let mut pivot_info: Vec<(usize, usize)> = Vec::new();
    let mut is_pivot_col = vec![false; num_buttons];

    for col in 0..num_buttons {
        let mut pivot = None;
        for row in pivot_row..num_counters {
            if matrix[row][col] != 0 {
                pivot = Some(row);
                break;
            }
        }

        let Some(found_row) = pivot else { continue };

        matrix.swap(pivot_row, found_row);
        pivot_info.push((pivot_row, col));
        is_pivot_col[col] = true;

        let pivot_val = matrix[pivot_row][col];
        for row in 0..num_counters {
            if row != pivot_row && matrix[row][col] != 0 {
                let factor = matrix[row][col];
                for c in 0..=num_buttons {
                    matrix[row][c] = matrix[row][c] * pivot_val - matrix[pivot_row][c] * factor;
                }
                let g = matrix[row].iter().fold(0i64, |acc, &x| gcd(acc, x));
                if g > 1 {
                    for x in matrix[row].iter_mut() {
                        *x /= g;
                    }
                }
            }
        }
        pivot_row += 1;
    }

    for row in matrix.iter_mut() {
        let g = row.iter().fold(0i64, |acc, &x| gcd(acc, x));
        if g > 1 {
            for x in row.iter_mut() {
                *x /= g;
            }
        }
    }

    let free_cols: Vec<usize> = (0..num_buttons).filter(|&c| !is_pivot_col[c]).collect();

    // Try free vars = 0 first as baseline
    let base_solution = try_solve_bounded(
        &matrix,
        &pivot_info,
        &free_cols,
        num_buttons,
        &vec![0; free_cols.len()],
    );
    let mut best = base_solution.unwrap_or(u32::MAX);

    if free_cols.is_empty() {
        return best;
    }

    // Bound: each free variable shouldn't exceed max joltage requirement
    let max_req = *joltage_reqs.iter().max().unwrap_or(&0) as i64;
    let bound = max_req.min(500);

    // Search over free variables
    let mut free_vals = vec![0i64; free_cols.len()];

    search_free_vars_with_bounds(
        &matrix,
        &pivot_info,
        &free_cols,
        num_buttons,
        bound,
        0,
        &mut free_vals,
        &mut best,
        0,
    );

    best
}

fn gcd(a: i64, b: i64) -> i64 {
    let (a, b) = (a.abs(), b.abs());
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn try_solve_bounded(
    matrix: &[Vec<i64>],
    pivot_info: &[(usize, usize)],
    free_cols: &[usize],
    num_buttons: usize,
    free_vals: &[i64],
) -> Option<u32> {
    let mut presses = vec![0i64; num_buttons];

    // Set free variables
    for (i, &col) in free_cols.iter().enumerate() {
        presses[col] = free_vals[i];
    }

    // Back-substitute to find pivot variables
    for &(row, pivot_col) in pivot_info.iter().rev() {
        let mut rhs = matrix[row][num_buttons];
        for col in 0..num_buttons {
            if col != pivot_col {
                rhs -= matrix[row][col] * presses[col];
            }
        }

        let coef = matrix[row][pivot_col];
        if coef == 0 {
            return None;
        }
        if rhs % coef != 0 {
            return None;
        }
        presses[pivot_col] = rhs / coef;
    }

    // Check non-negativity
    if presses.iter().any(|&p| p < 0) {
        return None;
    }

    let total: i64 = presses.iter().sum();
    if total < 0 || total > u32::MAX as i64 {
        return None;
    }

    Some(total as u32)
}

fn search_free_vars_with_bounds(
    matrix: &[Vec<i64>],
    pivot_info: &[(usize, usize)],
    free_cols: &[usize],
    num_buttons: usize,
    bound: i64,
    idx: usize,
    free_vals: &mut Vec<i64>,
    best: &mut u32,
    current_free_sum: i64,
) {
    if current_free_sum as u32 >= *best {
        return;
    }

    if idx == free_cols.len() {
        if let Some(total) =
            try_solve_bounded(matrix, pivot_info, free_cols, num_buttons, free_vals)
        {
            if total < *best {
                *best = total;
            }
        }
        return;
    }

    for v in 0..=bound {
        let new_sum = current_free_sum + v;
        if new_sum as u32 >= *best {
            break;
        }
        free_vals[idx] = v;
        search_free_vars_with_bounds(
            matrix,
            pivot_info,
            free_cols,
            num_buttons,
            bound,
            idx + 1,
            free_vals,
            best,
            new_sum,
        );
    }
}
