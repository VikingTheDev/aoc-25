type ParsedInput = Vec<(String, Vec<String>)>;

pub fn parse(input: &str) -> ParsedInput {
    // Each line is structured as "key: val val val"
    // We parse it into a vector of (key, [vals])
    input.lines().map(|line| {
        let mut parts = line.split(':');
        let key = parts.next().unwrap().trim().to_string();
        let vals = parts.next().unwrap_or("").trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        (key, vals)
    }).collect()
}

pub fn part1(_input: &ParsedInput) -> u32 {
    // The strings represent machines (or nodes) and their connections.
    // We have two named nodes, "you" and "out".
    // In part 1, we need to find the total number of paths from "you" to "out".
    // We'll use BFS (breadth-first search) to explore all paths.
    use std::collections::{HashMap, VecDeque};
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for (key, vals) in _input.iter() {
        graph.insert(key.as_str(), vals.iter().map(|s| s.as_str()).collect());
    }
    let mut path_count: u32 = 0;
    let mut queue: VecDeque<&str> = VecDeque::new();
    queue.push_back("you");
    while let Some(current) = queue.pop_front() {
        if current == "out" {
            path_count += 1;
            continue;
        }
        if let Some(neighbors) = graph.get(current) {
            for &neighbor in neighbors {
                queue.push_back(neighbor);
            }
        }
    }
    path_count
}

pub fn part2(_input: &ParsedInput) -> u64 {
    // In part 2, we need to find the paths from "svr" to "out"
    // However, we should only count paths that pass through both "dac" AND "fft"
    // Use memoization: count paths from (node, seen_dac, seen_fft) to "out"
    // Attempting to explore all paths is veeeeeery slow without memoization
    use std::collections::HashMap;
    
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for (key, vals) in _input.iter() {
        graph.insert(key.as_str(), vals.iter().map(|s| s.as_str()).collect());
    }
    
    // Memoization cache: (node, seen_dac, seen_fft) -> path count
    let mut memo: HashMap<(&str, bool, bool), u64> = HashMap::new();
    
    fn count_paths<'a>(
        node: &'a str,
        seen_dac: bool,
        seen_fft: bool,
        graph: &HashMap<&'a str, Vec<&'a str>>,
        memo: &mut HashMap<(&'a str, bool, bool), u64>,
    ) -> u64 {
        // Update flags for current node
        let seen_dac = seen_dac || node == "dac";
        let seen_fft = seen_fft || node == "fft";
        
        if node == "out" {
            return if seen_dac && seen_fft { 1 } else { 0 };
        }
        
        let key = (node, seen_dac, seen_fft);
        if let Some(&cached) = memo.get(&key) {
            return cached;
        }
        
        let mut total = 0u64;
        if let Some(neighbors) = graph.get(node) {
            for &neighbor in neighbors {
                total += count_paths(neighbor, seen_dac, seen_fft, graph, memo);
            }
        }
        
        memo.insert(key, total);
        total
    }
    
    count_paths("svr", false, false, &graph, &mut memo)
}