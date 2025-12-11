use aoc_25::year2025::day11::*;

const INPUT: &str = include_str!("day11_input.txt");
const INPUT2: &str = include_str!("day11_input2.txt");

#[test]
fn part1_test() {
    let parsed = parse(INPUT);
    assert_eq!(part1(&parsed), 5);
}

#[test]
fn part2_test() {
    let parsed = parse(INPUT2);
    assert_eq!(part2(&parsed), 2);
}
