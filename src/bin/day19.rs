const INPUT: u32 = 3005290;

fn main() {
    // With the number of Elves given in your puzzle input, which Elf gets all the presents?
    // See the Josephus problem (or Section 1.3 in Concrete Mathematics 2nd edition) for an
    // explanation.
    println!("Part 1: {}", (INPUT << 1) ^ (INPUT.next_power_of_two() + 1));

    // With the number of Elves given in your puzzle input, which Elf now gets all the presents?
    println!("Part 2: {}", part2(INPUT));
}

fn part2(n: u32) -> u32 {
    // Finding the pattern is relatively easy: f(n) = n - m + min(n - 2m, 0),
    // where m is the next lowest power of 3, and n > 1.
    if n == 1 {
        return 1;
    }

    let m = (0..)
        .map(|p| 3_u32.pow(p))
        .take_while(|&m| m < n)
        .last()
        .unwrap_or_default();

    (n - m) + n.saturating_sub(2 * m)
}
