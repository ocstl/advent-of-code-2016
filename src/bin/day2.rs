const INPUT_PATH: &str = "inputs/day2.txt";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!("Invalid character: {}", c),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Key {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Key {
    fn update(self, direction: Direction) -> Self {
        match self {
            Key::One => match direction {
                Direction::Up => Key::One,
                Direction::Down => Key::Four,
                Direction::Right => Key::Two,
                Direction::Left => Key::One,
            },
            Key::Two => match direction {
                Direction::Up => Key::Two,
                Direction::Down => Key::Five,
                Direction::Right => Key::Three,
                Direction::Left => Key::One,
            },
            Key::Three => match direction {
                Direction::Up => Key::Three,
                Direction::Down => Key::Six,
                Direction::Right => Key::Three,
                Direction::Left => Key::Two,
            },
            Key::Four => match direction {
                Direction::Up => Key::One,
                Direction::Down => Key::Seven,
                Direction::Right => Key::Five,
                Direction::Left => Key::Four,
            },
            Key::Five => match direction {
                Direction::Up => Key::Two,
                Direction::Down => Key::Eight,
                Direction::Right => Key::Six,
                Direction::Left => Key::Four,
            },
            Key::Six => match direction {
                Direction::Up => Key::Three,
                Direction::Down => Key::Nine,
                Direction::Right => Key::Six,
                Direction::Left => Key::Five,
            },
            Key::Seven => match direction {
                Direction::Up => Key::Four,
                Direction::Down => Key::Seven,
                Direction::Right => Key::Eight,
                Direction::Left => Key::Seven,
            },
            Key::Eight => match direction {
                Direction::Up => Key::Five,
                Direction::Down => Key::Eight,
                Direction::Right => Key::Nine,
                Direction::Left => Key::Seven,
            },
            Key::Nine => match direction {
                Direction::Up => Key::Six,
                Direction::Down => Key::Nine,
                Direction::Right => Key::Nine,
                Direction::Left => Key::Eight,
            },
        }
    }
}

impl Default for Key {
    fn default() -> Self {
        Key::Five
    }
}

impl From<Key> for char {
    fn from(key: Key) -> Self {
        match key {
            Key::One => '1',
            Key::Two => '2',
            Key::Three => '3',
            Key::Four => '4',
            Key::Five => '5',
            Key::Six => '6',
            Key::Seven => '7',
            Key::Eight => '8',
            Key::Nine => '9',
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Key2 {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
}

impl Key2 {
    fn update(self, direction: Direction) -> Self {
        match self {
            Key2::One => match direction {
                Direction::Up => Key2::One,
                Direction::Down => Key2::Three,
                Direction::Right => Key2::One,
                Direction::Left => Key2::One,
            },
            Key2::Two => match direction {
                Direction::Up => Key2::Two,
                Direction::Down => Key2::Six,
                Direction::Right => Key2::Three,
                Direction::Left => Key2::Two,
            },
            Key2::Three => match direction {
                Direction::Up => Key2::One,
                Direction::Down => Key2::Seven,
                Direction::Right => Key2::Four,
                Direction::Left => Key2::Two,
            },
            Key2::Four => match direction {
                Direction::Up => Key2::Four,
                Direction::Down => Key2::Eight,
                Direction::Right => Key2::Four,
                Direction::Left => Key2::Three,
            },
            Key2::Five => match direction {
                Direction::Up => Key2::Five,
                Direction::Down => Key2::Five,
                Direction::Right => Key2::Six,
                Direction::Left => Key2::Five,
            },
            Key2::Six => match direction {
                Direction::Up => Key2::Two,
                Direction::Down => Key2::A,
                Direction::Right => Key2::Seven,
                Direction::Left => Key2::Five,
            },
            Key2::Seven => match direction {
                Direction::Up => Key2::Three,
                Direction::Down => Key2::B,
                Direction::Right => Key2::Eight,
                Direction::Left => Key2::Six,
            },
            Key2::Eight => match direction {
                Direction::Up => Key2::Four,
                Direction::Down => Key2::C,
                Direction::Right => Key2::Nine,
                Direction::Left => Key2::Seven,
            },
            Key2::Nine => match direction {
                Direction::Up => Key2::Nine,
                Direction::Down => Key2::Nine,
                Direction::Right => Key2::Nine,
                Direction::Left => Key2::Eight,
            },
            Key2::A => match direction {
                Direction::Up => Key2::Six,
                Direction::Down => Key2::A,
                Direction::Right => Key2::B,
                Direction::Left => Key2::A,
            },
            Key2::B => match direction {
                Direction::Up => Key2::Seven,
                Direction::Down => Key2::D,
                Direction::Right => Key2::C,
                Direction::Left => Key2::A,
            },
            Key2::C => match direction {
                Direction::Up => Key2::Eight,
                Direction::Down => Key2::C,
                Direction::Right => Key2::C,
                Direction::Left => Key2::B,
            },
            Key2::D => match direction {
                Direction::Up => Key2::B,
                Direction::Down => Key2::D,
                Direction::Right => Key2::D,
                Direction::Left => Key2::D,
            },
        }
    }
}

impl Default for Key2 {
    fn default() -> Self {
        Key2::Five
    }
}

impl From<Key2> for char {
    fn from(key: Key2) -> Self {
        match key {
            Key2::One => '1',
            Key2::Two => '2',
            Key2::Three => '3',
            Key2::Four => '4',
            Key2::Five => '5',
            Key2::Six => '6',
            Key2::Seven => '7',
            Key2::Eight => '8',
            Key2::Nine => '9',
            Key2::A => 'A',
            Key2::B => 'B',
            Key2::C => 'C',
            Key2::D => 'D',
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let instructions: Vec<Vec<Direction>> = std::fs::read_to_string(INPUT_PATH)?
        .lines()
        .map(|line| line.chars().map(Direction::from).collect())
        .collect();

    let part1 = part1(&instructions);
    println!("Part 1: {}", part1);

    let part2 = part2(&instructions);
    println!("Part 1: {}", part2);

    Ok(())
}

/// What is the bathroom code?
fn part1(instructions: &[Vec<Direction>]) -> String {
    let mut key = Key::default();
    let mut code = String::new();
    for steps in instructions {
        key = steps
            .iter()
            .fold(key, |current, direction| current.update(*direction));
        code.push(key.into());
    }

    code
}

/// what is the correct bathroom code?
fn part2(instructions: &[Vec<Direction>]) -> String {
    let mut key = Key2::default();
    let mut code = String::new();
    for steps in instructions {
        key = steps
            .iter()
            .fold(key, |current, direction| current.update(*direction));
        code.push(key.into());
    }

    code
}
