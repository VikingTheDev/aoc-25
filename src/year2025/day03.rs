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

        // We have to pick exactly two digits in *order*
        // The two digits have to produce the maximum product when combined (1234 -> 34)
        for (i, &digit1) in line.iter().enumerate() {
            for &digit2 in &line[i + 1..] {
                let product = (digit1 as u32) * 10 + (digit2 as u32);
                if product > max_num {
                    max_num = product;
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
