type ParsedInput = (Vec<(u64, u64)>, Vec<u64>);

pub fn parse(input: &str) -> ParsedInput {
    // First part is a list of ranges, two numbers per line separated by '-'
    // Second part is a list of IDs, one per line
    // The sections are separated by a blank line
    let mut sections = input.trim().split("\n\n");
    let ranges_section = sections.next().unwrap();
    let ids_section = sections.next().unwrap();

    let ranges: Vec<(u64, u64)> = ranges_section.lines()
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
    let ids: Vec<u64> = ids_section.lines()
        .filter_map(|line| line.trim().parse::<u64>().ok())
        .collect();

    (ranges, ids)
}

pub fn part1(_input: &ParsedInput) -> u32 {
    // Check each ID, see if it is in any of the ranges
    let (ranges, ids) = _input;
    let mut valid_count: u32 = 0;

    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    let mut sorted_ranges = ranges.clone();
    // Sort ranges by start value
    sorted_ranges.sort_by_key(|r| r.0);
    // Merge overlapping ranges
    for range in sorted_ranges {
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

    // Iterate over each ID, and check if it is in any of the ranges
    for id in ids {
        let mut is_valid = false;
        for range in merged_ranges.iter() {
            if id_in_range(*id, range) {
                is_valid = true;
                break;
            }
        }
        if is_valid {
            valid_count += 1;
        }
    }

    valid_count
}

// Helper to check if an ID is in any of the ranges
pub fn id_in_range(id: u64, range: &(u64, u64)) -> bool {
    id >= range.0 && id <= range.1
}

pub fn part2(_input: &ParsedInput) -> u64{
    // Now we have to count the total number of valid IDs in the ranges
    // We have to first handle overlapping ranges
    // Then we can sum up the sizes of the non-overlapping ranges
    let (ranges, _) = _input;
    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by_key(|r| r.0);
    for range in sorted_ranges {
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
    // Now sum up the sizes of the merged ranges
    let mut total_count: u64 = 0;
    for range in merged_ranges {
        total_count += (range.1 - range.0 + 1) as u64;
    }
    total_count
}
