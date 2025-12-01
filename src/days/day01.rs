use crate::utils::{read_lines};

pub fn solve(use_test: bool) -> (String, String) {    
    let _lines = read_lines(1, use_test);

    let part1 = solve_part1(&_lines);
    let part2 = solve_part2(&_lines);

    (part1, part2)
}

fn solve_part1(_input: &Vec<String>) -> String {
    // Parse the input
    let _parsed = parse_input(_input);

    // Now we need to "turn" a dial, bounded between 0 and 99. 
    // Each parsed instruction tells us to turn left or right a certain number of steps.
    // We should count the number of times the dial lands on 0.
    let mut dial_position: i32 = 50;
    let mut zero_count: u32 = 0;

    for (direction, steps) in _parsed {
        if direction {
            // Turn right
            dial_position = (dial_position + steps as i32) % 100;
        } else {
            // Turn left (use rem_euclid to handle negative wrap-around)
            dial_position = (dial_position - steps as i32).rem_euclid(100);
        }

        if dial_position == 0 {
            zero_count += 1;
        }
    }

    zero_count.to_string()
}

fn solve_part2(_input: &Vec<String>) -> String {
    // Parse the input
    let _parsed = parse_input(_input);

    // Now we need to "turn" a dial, bounded between 0 and 99. 
    // Each parsed instruction tells us to turn left or right a certain number of steps.
    // We should count the number of times the dial *points* at 0. Meaning if we pass 0 we should count it.
    let mut dial_position: i32 = 50;
    let mut zero_count: u32 = 0;

    for (direction, steps) in _parsed {
        if direction {
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

    zero_count.to_string()
}

// Takes a line, checks the first character for L or R (L is false, R is true), and the rest as a number
fn parse_input(_input: &Vec<String>) -> Vec<(bool, u32)> {
    let mut result = Vec::new();

    for line in _input {
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
        if let Ok(steps) = number.parse::<u32>() {
            result.push((direction, steps));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_lines;

    #[test]
    fn test_part1() {
        let input = read_lines(1, true);
        assert_eq!(solve_part1(&input), "3");
    }

    #[test]
    fn test_part2() {
        let input = read_lines(1, true);
        assert_eq!(solve_part2(&input), "6");
    }
}
