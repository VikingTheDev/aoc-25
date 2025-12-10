use aoc_25::year2025::day09::*;

const INPUT: &str = include_str!("day09_input.txt");

#[test]
fn part1_test() {
    let parsed = parse(INPUT);
    assert_eq!(part1(&parsed), 50);
}

#[test]
fn part2_test() {
    let parsed = parse(INPUT);
    assert_eq!(part2(&parsed), 24);
}
