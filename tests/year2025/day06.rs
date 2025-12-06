use aoc_25::year2025::day06::*;

const INPUT: &str = include_str!("day06_input.txt");

#[test]
fn part1_test() {
    let parsed = parse(INPUT);
    assert_eq!(part1(&parsed), 4277556);
}

#[test]
fn part2_test() {
    let parsed = parse(INPUT);
    assert_eq!(part2(&parsed), 3263827);
}
