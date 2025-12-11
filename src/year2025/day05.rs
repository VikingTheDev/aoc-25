type ParsedInput = (Vec<(u64, u64)>, Vec<u64>);

pub fn parse(input: &str) -> ParsedInput {
    // First part is a list of ranges, two numbers per line separated by '-'
    // Second part is a list of IDs, one per line
    // The sections are separated by a blank line
    let mut sections = input.trim().split("\n\n");
    let ranges_section = sections.next().unwrap();
    let ids_section = sections.next().unwrap();

    let mut ranges: Vec<(u64, u64)> = ranges_section
        .lines()
        .filter_map(|line| {
            let mut parts = line.trim().split('-');
            if let (Some(start_str), Some(end_str)) = (parts.next(), parts.next()) {
                if let (Ok(start), Ok(end)) = (start_str.parse::<u64>(), end_str.parse::<u64>()) {
                    return Some((start, end));
                }
            }
            None
        })
        .collect();

    let ids: Vec<u64> = ids_section
        .lines()
        .filter_map(|line| line.trim().parse::<u64>().ok())
        .collect();

    // Merge overlapping ranges during parsing
    ranges.sort_by_key(|r| r.0);
    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    for range in ranges {
        if let Some(last) = merged_ranges.last_mut() {
            if range.0 <= last.1 + 1 {
                // Overlapping or contiguous ranges, merge them
                if range.1 > last.1 {
                    last.1 = range.1;
                }
            } else {
                // Non-overlapping range, add it to the list
                merged_ranges.push(range);
            }
        } else {
            // First range, just add it
            merged_ranges.push(range);
        }
    }

    (merged_ranges, ids)
}

pub fn part1(_input: &ParsedInput) -> u32 {
    // Check each ID, see if it is in any of the ranges
    let (ranges, ids) = _input;
    let mut valid_count: u32 = 0;

    // Iterate over each ID, and check if it is in any of the ranges
    for id in ids {
        for range in ranges.iter() {
            if id_in_range(*id, range) {
                valid_count += 1;
                break;
            }
        }
    }

    valid_count
}

// Helper to check if an ID is in any of the ranges
pub fn id_in_range(id: u64, range: &(u64, u64)) -> bool {
    id >= range.0 && id <= range.1
}

pub fn part2(_input: &ParsedInput) -> u64 {
    // Now we have to count the total number of valid IDs in the ranges
    // We have to first handle overlapping ranges
    // Then we can sum up the sizes of the non-overlapping ranges
    // Ranges are already merged in parse, so just sum up their sizes
    let (ranges, _ids) = _input;

    let mut total_count: u64 = 0;
    for range in ranges.iter() {
        total_count += (range.1 - range.0 + 1) as u64;
    }
    total_count
}
