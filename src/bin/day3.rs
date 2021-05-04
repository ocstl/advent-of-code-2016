use std::num::ParseIntError;

const INPUT_PATH: &str = "inputs/day3.txt";

fn valid_triangle(a: u32, b: u32, c: u32) -> bool {
    a + b + c > 2 * a.max(b).max(c)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Vec<Vec<u32>> = std::fs::read_to_string(INPUT_PATH)?
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u32>())
                .collect::<Result<Vec<u32>, ParseIntError>>()
        })
        .collect::<Result<Vec<Vec<u32>>, ParseIntError>>()?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

/// In your puzzle input, how many of the listed triangles are possible?
fn part1(triangles: &[Vec<u32>]) -> usize {
    triangles
        .iter()
        .filter(|triangle| valid_triangle(triangle[0], triangle[1], triangle[2]))
        .count()
}

/// In your puzzle input, and instead reading by columns, how many of the
/// listed triangles are possible?
fn part2(triangles: &[Vec<u32>]) -> usize {
    let l = triangles.len();
    let mut transposed = [
        Vec::with_capacity(l),
        Vec::with_capacity(l),
        Vec::with_capacity(l),
    ];

    for line in triangles {
        transposed[0].push(line[0]);
        transposed[1].push(line[1]);
        transposed[2].push(line[2]);
    }

    transposed
        .iter()
        .flat_map(|line| {
            line.chunks_exact(3)
                .filter(|triangle| valid_triangle(triangle[0], triangle[1], triangle[2]))
        })
        .count()
}
