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
        if steps > 0 {
            // We brute force check for crossings
            for step in 1..=steps {
                let check_position = (dial_position + step as i32) % 100;
                if check_position == 0 {
                    zero_count += 1;
                }
            }
            dial_position = (dial_position + steps as i32) % 100;
        } else {
            // We brute force check for crossings
            for step in 1..=steps {
                let check_position = (dial_position - step as i32).rem_euclid(100);
                if check_position == 0 {
                    zero_count += 1;
                }
            }
            // Turn left (use rem_euclid to handle negative wrap-around)
            dial_position = (dial_position - steps as i32).rem_euclid(100);
        }
    }

    zero_count
}
