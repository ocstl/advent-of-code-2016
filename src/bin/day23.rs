use advent_of_code_2016::computer::Computer;

const INPUT_PATH: &str = "inputs/day23.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT_PATH)?;

    // The rest of the electronics seem to place the keypad entry (the number of eggs, 7) in
    // register a, run the code, and then send the value left in register a to the safe.
    // What value should be sent to the safe?
    let register_a = Computer::new(&input)?
        .set_registers([7, 0, 0, 0])
        .run()?
        .registers()[0];
    println!("Part 1: {}", register_a);

    // You're quite sure your logic is working correctly, so the only other thing is... you check
    // the painting again. As it turns out, colored eggs are still eggs. Now you count 12.
    // Anyway, what value should actually be sent to the safe?
    let register_a = Computer::new(&input)?
        .set_registers([12, 0, 0, 0])
        .run()?
        .registers()[0];
    println!("Part 2: {}", register_a);

    Ok(())
}
