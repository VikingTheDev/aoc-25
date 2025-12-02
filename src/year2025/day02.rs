type ParsedInput = Vec<IdRange>;
pub struct IdRange {
    start: u64,
    end: u64,
}

pub fn parse(input: &str) -> ParsedInput {
    // Input is a single line of comma-separated strings
    input.trim()
        .split(",")
        .filter_map(|s| {
            let s = s.trim();
            let mut parts = s.split('-');
            if let (Some(start_str), Some(end_str)) = (parts.next(), parts.next()) {
                if let (Ok(start), Ok(end)) = (start_str.parse::<u64>(), end_str.parse::<u64>()) {
                    return Some(IdRange { start, end });
                }
            }
            None
        })
        .collect()
}

pub fn part1(_input: &ParsedInput) -> u64 {
    let mut invalid_sum: u64 = 0;

    // Iterate over each range, and add the values of invalid IDs
    for range in _input {
        for id in range.start..=range.end {
            if is_invalid_id(id) {
                invalid_sum += id as u64;
            }
        }
    }

    invalid_sum
}

pub fn part2(_input: &ParsedInput) -> u64 {
    let mut invalid_sum: u64 = 0;

    // Iterate over each range, and add the values of invalid IDs
    for range in _input {
        for id in range.start..=range.end {
            if is_invalid_id_part2(id) {
                invalid_sum += id as u64;
            }
        }
    }

    invalid_sum
}

pub fn is_invalid_id(id: u64) -> bool {
    // Convert ID to string
    let id_str = id.to_string();
    // if length is odd, we can return false immediately
    if id_str.len() % 2 != 0 {
        return false;
    }
    // Split into two equal halves
    let mid = id_str.len() / 2;
    let (first_half, second_half) = id_str.split_at(mid);
    // Check if the two halves are equal
    first_half == second_half
}

pub fn is_invalid_id_part2(id: u64) -> bool {
    // Convert ID to string
    let id_str = id.to_string();
    // ID is invalid if the number is only made up of
    // a sequence of numbers repeated *at least twice*
    let len = id_str.len();
    for sub_len in 1..=(len / 2) {
        if len % sub_len == 0 {
            let sub_str = &id_str[0..sub_len];
            let mut repeated = true;
            for i in (0..len).step_by(sub_len) {
                if &id_str[i..i + sub_len] != sub_str {
                    repeated = false;
                    break;
                }
            }
            if repeated {
                return true;
            }
        }
    }
    false
}