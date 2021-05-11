use advent_of_code_2016::computer::Computer;

const INPUT_PATH: &str = "inputs/day12.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let mut computer = Computer::new(&input)?;

    // After executing the assembunny code in your puzzle input, what value is left in register a?
    let register_a = computer.run()?.registers()[0];
    println!("Part 1: {}", register_a);

    // If you instead initialize register c to be 1, what value is now left in register a?
    let register_a = computer
        .reset()
        .set_registers([0, 0, 1, 0])
        .run()?
        .registers()[0];
    println!("Part 2: {}", register_a);

    Ok(())
}
