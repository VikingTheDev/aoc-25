use std::collections::HashMap;

type ParsedInput = Vec<Vec<u8>>;

pub fn parse(input: &str) -> ParsedInput {
    input.trim()
        .lines()
        .map(|line| {
            line.trim()
                .bytes()
                .filter_map(|b| {
                    if b.is_ascii_digit() {
                        Some(b - b'0')
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect()
}

pub fn part1(_input: &ParsedInput) -> u32 {
    let mut sum = 0;

    // Iterate over all lines
    for line in _input {
        let mut max_num = 0;

        // Pick any two positions i < j and form number line[i]*10 + line[j]
        for i in 0..line.len() {
            for j in (i+1)..line.len() {
                let num = (line[i] as u32) * 10 + (line[j] as u32);
                if num > max_num {
                    max_num = num;
                }
            }
        }

        sum += max_num;
    }

    sum
}

pub fn part2(_input: &ParsedInput) -> u64 {
    // Now we have to pick exactly 12 digits. This is exponentially harder if we do it naively.
    // However, we can use dynamic programming to solve this efficiently.
    let mut sum: u64 = 0;

    // Iterate over all lines
    for line in _input {
        let n = line.len();
        let mut dp = vec![vec![0u64; 13]; n + 1];
        for i in 0..n {
            for j in 0..=12 {
                // Not take the current digit
                if dp[i + 1][j] < dp[i][j] {
                    dp[i + 1][j] = dp[i][j];
                }
                // Take the current digit if we can
                if j < 12 {
                    let new_value = dp[i][j] * 10 + line[i] as u64;
                    if dp[i + 1][j + 1] < new_value {
                        dp[i + 1][j + 1] = new_value;
                    }
                }
            }
        }
        sum += dp[n][12];
    }

    sum
}
