use aoc_25::year2025::day07::*;

const INPUT: &str = include_str!("day07_input.txt");

#[test]
fn part1_test() {
    let parsed = parse(INPUT);
    assert_eq!(part1(&parsed), 21);
}

#[test]
fn part2_test() {
    let parsed = parse(INPUT);
    assert_eq!(part2(&parsed), 40);
}
