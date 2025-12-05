use aoc_25::year2025::day05::*;

const INPUT: &str = include_str!("day05_input.txt");

#[test]
fn part1_test() {
    let parsed = parse(INPUT);
    assert_eq!(part1(&parsed), 3);
}

#[test]
fn part2_test() {
    let parsed = parse(INPUT);
    assert_eq!(part2(&parsed), 14);
}
