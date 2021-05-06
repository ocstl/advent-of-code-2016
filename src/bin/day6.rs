use advent_of_code_2016::counter::Counter;

const INPUT_PATH: &str = "inputs/day6.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string(INPUT_PATH)?;
    let n = input.lines().next().unwrap().len();
    let mut counters = vec![Counter::<char>::new(); n];

    for line in input.lines() {
        for (idx, c) in line.char_indices() {
            counters[idx].add(c);
        }
    }

    println!("Part 1: {}", part1(&counters));
    println!("Part 2: {}", part2(&counters));

    Ok(())
}

/// Given the recording in your puzzle input, what is the error-corrected
/// version of the message being sent?
fn part1(counters: &[Counter<char>]) -> String {
    counters
        .iter()
        .map(|counter| counter.most_common().unwrap())
        .collect()
}

/// Given the recording in your puzzle input and this new decoding methodology,
/// what is the original message that Santa is trying to send?
fn part2(counters: &[Counter<char>]) -> String {
    counters
        .iter()
        .map(|counter| counter.least_common().unwrap())
        .collect()
}
