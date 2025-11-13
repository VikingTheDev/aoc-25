use std::fs;

pub fn solve(use_test: bool) -> (String, String) {
    let input = if use_test {
        fs::read_to_string("inputs/day13_test.txt").expect("Failed to read test input")
    } else {
        fs::read_to_string("inputs/day13.txt").expect("Failed to read input")
    };

    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);

    (part1, part2)
}

fn solve_part1(_input: &str) -> String {
    // TODO: Implement part 1
    "Not implemented".to_string()
}

fn solve_part2(_input: &str) -> String {
    // TODO: Implement part 2
    "Not implemented".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("inputs/day13_test.txt").expect("Failed to read test input");
        assert_eq!(solve_part1(&input), "expected_result");
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("inputs/day13_test.txt").expect("Failed to read test input");
        assert_eq!(solve_part2(&input), "expected_result");
    }
}
