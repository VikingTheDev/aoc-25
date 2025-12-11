//! # Day 1: Dial Puzzle

pub fn parse(input: &str) -> Vec<i32> {
    let mut result = Vec::new();

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        if chars.len() < 2 {
            continue;
        }

        let direction = match chars[0] {
            'L' => false,
            'R' => true,
            _ => continue,
        };

        let number: String = chars[1..].iter().collect();
        if let Ok(steps) = number.parse::<i32>() {
            result.push(if direction { steps } else { -steps });
        }
    }

    result
}

pub fn part1(input: &[i32]) -> u32 {
    let mut dial_position: i32 = 50;
    let mut zero_count: u32 = 0;

    for &steps in input {
        // Turn dial, we use rem_euclid to handle negative wrap-around
        dial_position = (dial_position + steps).rem_euclid(100);

        // Check for zero
        if dial_position == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

pub fn part2(input: &[i32]) -> u32 {
    let mut dial_position: i32 = 50;
    let mut zero_count: u32 = 0;

    for &steps in input {
        let abs_steps = steps.abs();

        // Count complete wraps around the dial (each wrap crosses 0 once)
        zero_count += (abs_steps / 100) as u32;

        // Check if we cross 0 in the remaining partial rotation
        let remaining = abs_steps % 100;

        if remaining > 0 {
            if steps > 0 {
                // Moving right: we cross 0 if the range (position, position + remaining] contains a multiple of 100
                // This happens when position + remaining >= 100
                if dial_position + remaining >= 100 {
                    zero_count += 1;
                }
            } else {
                // Moving left: we cross 0 if the range [position - remaining, position) contains 0
                // This happens when position - remaining < 0 AND position > 0
                // Equivalently: position > 0 AND position <= remaining
                if dial_position > 0 && dial_position <= remaining {
                    zero_count += 1;
                }
            }
        }

        // Turn dial
        dial_position = (dial_position + steps).rem_euclid(100);
    }

    zero_count
}
