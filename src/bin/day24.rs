use std::collections::{BTreeSet, HashSet, VecDeque};

const INPUT_PATH: &str = "inputs/day24.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(usize, usize);

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position(x, y)
    }

    pub fn neighbouring_positions_unchecked(self) -> [Position; 4] {
        [
            Position::new(self.0 + 1, self.1),
            Position::new(self.0 - 1, self.1),
            Position::new(self.0, self.1 + 1),
            Position::new(self.0, self.1 - 1),
        ]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Wall,
    Open,
    ExposedWire(u32),
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Wall,
            '.' => Tile::Open,
            _ if c.is_ascii_digit() => Tile::ExposedWire(c.to_digit(10).unwrap()),
            _ => panic!("Unexpected character: {}", c),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HVACSystem(Vec<Vec<Tile>>);

impl HVACSystem {
    pub fn iter(&self) -> std::slice::Iter<Vec<Tile>> {
        self.0.iter()
    }

    pub fn get(&self, position: Position) -> Option<&Tile> {
        self.0.get(position.1).and_then(|row| row.get(position.0))
    }
}

impl<T: AsRef<str>> From<T> for HVACSystem {
    fn from(input: T) -> Self {
        let s = input
            .as_ref()
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();
        HVACSystem(s)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let system = HVACSystem::from(input);

    println!("Part 1: {}", part1(&system));
    println!("Part 2: {}", part2(&system));

    Ok(())
}

/// Given your actual map, and starting from location 0, what is the fewest number of steps
/// required to visit every non-0 number marked on the map at least once?
fn part1(system: &HVACSystem) -> u32 {
    let mut start_position = Position::new(0, 0);
    let mut exposed_wires = BTreeSet::new();

    for (y, row) in system.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            match tile {
                Tile::ExposedWire(0) => {
                    start_position = Position::new(x, y);
                    exposed_wires.insert(0);
                }
                Tile::ExposedWire(n) => {
                    exposed_wires.insert(*n);
                }
                _ => (),
            }
        }
    }

    let mut to_visit = VecDeque::new();
    to_visit.push_back((0, start_position, BTreeSet::new()));

    let mut visited = HashSet::new();
    visited.insert((start_position, BTreeSet::new()));

    while let Some((steps, position, wires)) = to_visit.pop_front() {
        if wires == exposed_wires {
            return steps;
        }

        to_visit.extend(
            position
                .neighbouring_positions_unchecked()
                .iter()
                .filter_map(|&p| match system.get(p) {
                    Some(Tile::Open) => {
                        if visited.insert((p, wires.clone())) {
                            Some((steps + 1, p, wires.clone()))
                        } else {
                            None
                        }
                    }
                    Some(Tile::ExposedWire(n)) => {
                        let mut new_wires = wires.clone();
                        new_wires.insert(*n);
                        if visited.insert((p, new_wires.clone())) {
                            Some((steps + 1, p, new_wires))
                        } else {
                            None
                        }
                    }
                    _ => None,
                }),
        );
    }

    0
}

/// What is the fewest number of steps required to start at 0, visit every non-0 number marked on
/// the map at least once, and then return to 0?
fn part2(system: &HVACSystem) -> u32 {
    let mut start_position = Position::new(0, 0);
    let mut exposed_wires = BTreeSet::new();

    for (y, row) in system.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            match tile {
                Tile::ExposedWire(0) => {
                    start_position = Position::new(x, y);
                    exposed_wires.insert(0);
                }
                Tile::ExposedWire(n) => {
                    exposed_wires.insert(*n);
                }
                _ => (),
            }
        }
    }

    let mut to_visit = VecDeque::new();
    to_visit.push_back((0, start_position, BTreeSet::new()));

    let mut visited = HashSet::new();
    visited.insert((start_position, BTreeSet::new()));

    while let Some((steps, position, wires)) = to_visit.pop_front() {
        if wires == exposed_wires && position == start_position {
            return steps;
        }

        to_visit.extend(
            position
                .neighbouring_positions_unchecked()
                .iter()
                .filter_map(|&p| match system.get(p) {
                    Some(Tile::Open) => {
                        if visited.insert((p, wires.clone())) {
                            Some((steps + 1, p, wires.clone()))
                        } else {
                            None
                        }
                    }
                    Some(Tile::ExposedWire(n)) => {
                        let mut new_wires = wires.clone();
                        new_wires.insert(*n);
                        if visited.insert((p, new_wires.clone())) {
                            Some((steps + 1, p, new_wires))
                        } else {
                            None
                        }
                    }
                    _ => None,
                }),
        );
    }

    0
}
