const INPUT_PATH: &str = "inputs/day10.txt";

#[derive(Debug, Clone, Copy)]
enum Destination {
    Bot(usize),
    Output(usize),
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Input(usize, usize),
    Rule(usize, Destination, Destination),
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let mut iter = input.split_whitespace();
        match iter.next().unwrap() {
            "value" => {
                let chip = iter.next().unwrap().parse().unwrap();
                let bot = iter.last().unwrap().parse().unwrap();
                Instruction::Input(bot, chip)
            }
            "bot" => {
                let bot = iter.next().unwrap().parse().unwrap();
                let low_dest = iter.nth(3).unwrap();
                let low = iter.next().unwrap().parse().unwrap();
                let low = match low_dest {
                    "bot" => Destination::Bot(low),
                    "output" => Destination::Output(low),
                    i => unimplemented!("{}", i),
                };

                let high_dest = iter.nth(3).unwrap();
                let high = iter.next().unwrap().parse().unwrap();
                let high = match high_dest {
                    "bot" => Destination::Bot(high),
                    "output" => Destination::Output(high),
                    i => unimplemented!("{}", i),
                };

                Instruction::Rule(bot, low, high)
            }
            i => unimplemented!("{}", i),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Bot {
    pub id: usize,
    pub chips: Vec<usize>,
    pub rule: Option<(Destination, Destination)>,
}

impl Bot {
    pub fn new(id: usize) -> Self {
        Bot {
            id,
            chips: Vec::new(),
            rule: None,
        }
    }
}

fn read_instructions(input: &str) -> Vec<Bot> {
    let mut bots = Vec::new();

    for instruction in input.lines().map(Instruction::from) {
        match instruction {
            Instruction::Input(bot, chip) => {
                while bots.len() <= bot {
                    bots.push(Bot::new(bots.len()));
                }

                bots[bot].chips.push(chip);
            }
            Instruction::Rule(bot, low, high) => {
                while bots.len() <= bot {
                    bots.push(Bot::new(bots.len()));
                }

                bots[bot].rule = Some((low, high))
            }
        }
    }

    bots
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let bots = read_instructions(&input);

    println!("Part 1: {}", part1(&bots));
    println!("Part 2: {}", part2(&bots));

    Ok(())
}

/// Based on your instructions, what is the number of the bot that is
/// responsible for comparing value-61 microchips with value-17 microchips?
fn part1(bots: &[Bot]) -> usize {
    let mut bots = bots.to_vec();

    while let Some(bot) = bots.iter_mut().find(|bot| bot.chips.len() == 2) {
        let mut chips = std::mem::take(&mut bot.chips);
        chips.sort_unstable();

        if chips == [17, 61] {
            return bot.id;
        }

        if let Some((low_dest, high_dest)) = bot.rule {
            if let Destination::Bot(b) = low_dest {
                bots[b].chips.push(chips[0]);
            }
            if let Destination::Bot(b) = high_dest {
                bots[b].chips.push(chips[1]);
            }
        }
    }

    0
}

/// What do you get if you multiply together the values of one chip in each of outputs 0, 1, and 2?
fn part2(bots: &[Bot]) -> usize {
    let mut bots = bots.to_vec();
    let mut result = 1;

    while let Some(bot) = bots.iter_mut().find(|bot| bot.chips.len() == 2) {
        let mut chips = std::mem::take(&mut bot.chips);
        chips.sort_unstable();

        if let Some((low_dest, high_dest)) = bot.rule {
            match low_dest {
                Destination::Bot(b) => bots[b].chips.push(chips[0]),
                Destination::Output(o) if o <= 2 => result *= chips[0],
                Destination::Output(_) => (),
            }

            match high_dest {
                Destination::Bot(b) => bots[b].chips.push(chips[1]),
                Destination::Output(o) if o <= 2 => result *= chips[1],
                Destination::Output(_) => (),
            }
        }
    }

    result
}
