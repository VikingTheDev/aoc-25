type ParsedInput = Vec<IdRange>;
pub struct IdRange {
    start: u64,
    end: u64,
}

pub fn parse(input: &str) -> ParsedInput {
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
    // Sum of invalid IDs in all ranges
    // Invalid ID: even digit count, first half = second half
    // E.g., 1212, 1717, 123123 are invalid
    // 
    // Key insight: An "invalid" 2n-digit number is uniquely determined by its first n digits.
    // So there are exactly 9 * 10^(n-1) such numbers for each length 2n.
    // 
    // We'll enumerate all invalid numbers and check if they're in any range.
    // Since invalid numbers are sparse, this is much faster than iterating all IDs.
    
    let mut invalid_sum: u64 = 0;
    
    for range in _input {
        invalid_sum += sum_invalid_ids_in_range_part1(range.start, range.end);
    }
    
    invalid_sum
}

fn sum_invalid_ids_in_range_part1(start: u64, end: u64) -> u64 {
    // Generate all invalid IDs and sum those in [start, end]
    // An invalid ID has even length 2n and first half = second half
    // We iterate over possible half lengths and generate the invalid IDs
    
    let mut sum: u64 = 0;
    
    // For each even total length (2, 4, 6, 8, ...)
    for half_len in 1..=10u32 {
        let total_len = half_len * 2;
        
        // Range of half values: for n digits, half is in [10^(n-1), 10^n - 1]
        // except for n=1 where half is in [1, 9] (single digits)
        let half_min: u64 = if half_len == 1 { 1 } else { 10u64.pow(half_len - 1) };
        let half_max: u64 = 10u64.pow(half_len) - 1;
        
        // The multiplier to create full ID: half * (10^n + 1)
        let multiplier = 10u64.pow(half_len) + 1;
        
        // Range of IDs: [half_min * multiplier, half_max * multiplier]
        let id_min = half_min * multiplier;
        let id_max = half_max * multiplier;
        
        // Skip if no overlap with [start, end]
        if id_max < start || id_min > end {
            continue;
        }
        
        // Find range of half values that produce IDs in [start, end]
        // id = half * multiplier
        // start <= half * multiplier <= end
        // start / multiplier <= half <= end / multiplier
        let half_start = (start + multiplier - 1) / multiplier; // ceil division
        let half_end = end / multiplier;
        
        // Clamp to valid half range
        let half_start = half_start.max(half_min);
        let half_end = half_end.min(half_max);
        
        if half_start <= half_end {
            // Sum of IDs = multiplier * sum of halves
            // Sum of halves from a to b = (b - a + 1) * (a + b) / 2
            let count = half_end - half_start + 1;
            let half_sum = count * (half_start + half_end) / 2;
            sum += multiplier * half_sum;
        }
    }
    
    sum
}

pub fn part2(_input: &ParsedInput) -> u64 {
    // Invalid ID: made up of a repeating pattern at least twice
    // E.g., 1212, 121121, 123123123, 77, 1111
    
    let mut invalid_sum: u64 = 0;
    
    for range in _input {
        invalid_sum += sum_invalid_ids_in_range_part2(range.start, range.end);
    }
    
    invalid_sum
}

fn sum_invalid_ids_in_range_part2(start: u64, end: u64) -> u64 {
    // Use inclusion-exclusion: count IDs that are k-repetitions for various k
    // A number with total length L and pattern length P (L = k*P, k >= 2) is invalid
    //
    // We need to count each invalid ID exactly once. The simplest approach:
    // - A number is invalid if its shortest repeating unit divides the length
    // - Use Möbius function for inclusion-exclusion on divisors
    //
    // Actually, let's enumerate invalid IDs directly since they're relatively sparse.
    // For each pattern length P, for each repetition count K >= 2, generate IDs
    // and use inclusion-exclusion to avoid double-counting.
    
    // Simpler approach: collect all invalid IDs in range and sum them (using a set for dedup)
    // But this might still be slow for huge ranges...
    
    // Even simpler: iterate through pattern lengths and compute directly
    // A pattern of length P repeated K times has total length P*K
    // The pattern itself is in range [10^(P-1), 10^P - 1] (or [0, 9] for P=1)
    // For P=1: pattern is a single digit 1-9 (not 0), ID is digit repeated K times
    
    use std::collections::HashSet;
    let mut counted: HashSet<u64> = HashSet::new();
    let mut sum: u64 = 0;
    
    // Determine max total length we need to consider
    let max_len = if end == 0 { 1 } else { end.ilog10() as usize + 1 };
    
    // For each pattern length
    for pattern_len in 1..=max_len {
        // For each repetition count (at least 2)
        for reps in 2..=20 {
            let total_len = pattern_len * reps;
            if total_len > max_len + 1 {
                break;
            }
            
            // Pattern range
            let pattern_min: u64 = if pattern_len == 1 { 0 } else { 10u64.pow((pattern_len - 1) as u32) };
            let pattern_max: u64 = 10u64.pow(pattern_len as u32) - 1;
            
            // Skip patterns starting with 0 (except pattern = 0 itself for edge cases)
            let pattern_min = pattern_min.max(if pattern_len == 1 { 0 } else { 10u64.pow((pattern_len - 1) as u32) });
            
            for pattern in pattern_min..=pattern_max {
                // Skip if pattern starts with 0 and has length > 1
                if pattern_len > 1 && pattern < 10u64.pow((pattern_len - 1) as u32) {
                    continue;
                }
                
                // Build the full ID by repeating pattern
                let mut id: u64 = 0;
                let multiplier = 10u64.pow(pattern_len as u32);
                for _ in 0..reps {
                    id = id * multiplier + pattern;
                }
                
                // Check if in range and not counted
                if id >= start && id <= end && !counted.contains(&id) {
                    counted.insert(id);
                    sum += id;
                }
            }
        }
    }
    
    sum
}