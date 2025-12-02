use aoc_25::year2025::day02::*;

const INPUT: &str = include_str!("day02_input.txt");

#[test]
fn part1_test() {
    let parsed = parse(INPUT);
    assert_eq!(part1(&parsed), 1227775554);
}

#[test]
fn part2_test() {
    let parsed = parse(INPUT);
    assert_eq!(part2(&parsed), 4174379265);
}
