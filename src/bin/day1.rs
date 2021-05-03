use std::collections::HashSet;

const INPUT_PATH: &str = "inputs/day1.txt";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Instruction(Turn, i32);

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let (t, s) = input.trim().split_at(1);
        let turn = Turn::from(t);
        let steps = s.parse::<i32>().unwrap();
        Instruction(turn, steps)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Turn {
    Left,
    Right,
}

impl From<&str> for Turn {
    fn from(input: &str) -> Self {
        match input {
            "L" => Turn::Left,
            "R" => Turn::Right,
            _ => panic!("Invalid character: {}", input),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn(self, turn: Turn) -> Self {
        match self {
            Direction::North => match turn {
                Turn::Left => Direction::West,
                Turn::Right => Direction::East,
            },
            Direction::South => match turn {
                Turn::Left => Direction::East,
                Turn::Right => Direction::West,
            },
            Direction::East => match turn {
                Turn::Left => Direction::North,
                Turn::Right => Direction::South,
            },
            Direction::West => match turn {
                Turn::Left => Direction::South,
                Turn::Right => Direction::North,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Position(i32, i32);

impl Position {
    fn update(self, direction: Direction, steps: i32) -> Self {
        match direction {
            Direction::North => Self(self.0, self.1 - steps),
            Direction::South => Self(self.0, self.1 + steps),
            Direction::East => Self(self.0 + steps, self.1),
            Direction::West => Self(self.0 - steps, self.1),
        }
    }

    fn distance_from_origin(self) -> i32 {
        self.0.abs() + self.1.abs()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let instructions: Vec<Instruction> = std::fs::read_to_string(INPUT_PATH)?
        .split(',')
        .map(Instruction::from)
        .collect();

    let part1 = part1(&instructions);
    println!("Part 1: {}", part1);

    let part2 = part2(&instructions);
    println!("Part 2: {}", part2);

    Ok(())
}

/// How many blocks away is Easter Bunny HQ?
fn part1(instructions: &[Instruction]) -> i32 {
    let mut direction = Direction::North;
    let mut position = Position::default();

    for &Instruction(turn, steps) in instructions {
        direction = direction.turn(turn);
        position = position.update(direction, steps);
    }

    position.distance_from_origin()
}

/// How many blocks away is the first location you visit twice?
fn part2(instructions: &[Instruction]) -> i32 {
    let mut visited = HashSet::new();
    let mut direction = Direction::North;
    let mut position = Position::default();

    for &Instruction(turn, steps) in instructions {
        direction = direction.turn(turn);
        // Have to check intermediate positions as well.
        for _ in 0..steps {
            position = position.update(direction, 1);
            if !visited.insert(position) {
                return position.distance_from_origin();
            }
        }
    }

    0
}
