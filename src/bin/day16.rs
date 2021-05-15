const INPUT: &str = "11110010111001001";

fn main() {
    // The first disk you have to fill has length 272. Using the initial state in your puzzle
    // input, what is the correct checksum?
    println!("Part 1: {}", checksum(&generate_data(INPUT, 272)));

    // The second disk you have to fill has length 35651584. Again using the initial state in
    // your puzzle input, what is the correct checksum for this disk?
    // I strongly believed some optimizations would be required, but this takes much less than 1s
    // under release mode. Rust is awesome! :D
    println!("Part 2: {}", checksum(&generate_data(INPUT, 35651584)));
}

fn generate_data(input: &str, size: usize) -> Vec<bool> {
    let mut data: Vec<bool> = input.chars().map(|c| c == '1').collect();

    while data.len() < size {
        let mut d = data.iter().rev().map(std::ops::Not::not).collect();
        data.push(false);
        data.append(&mut d);
    }

    data.resize(size, false);
    data
}

fn checksum(data: &[bool]) -> String {
    let mut checksum: Vec<bool> = data
        .chunks_exact(2)
        .map(|chunk| !(chunk[0] ^ chunk[1]))
        .collect();

    while checksum.len() % 2 == 0 {
        checksum = checksum
            .chunks_exact(2)
            .map(|chunk| !(chunk[0] ^ chunk[1]))
            .collect();
    }

    checksum
        .into_iter()
        .map(|b| if b { '1' } else { '0' })
        .collect()
}
