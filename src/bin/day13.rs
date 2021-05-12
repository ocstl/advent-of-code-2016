use std::collections::{HashSet, VecDeque};
use std::iter::once;

const INPUT: u64 = 1362;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(u64, u64);

impl Position {
    pub fn new(x: u64, y: u64) -> Self {
        Position(x, y)
    }

    pub fn neighbouring_positions(self) -> impl Iterator<Item = Self> {
        let xs = once(self.0 + 1)
            .chain(self.0.checked_sub(1).into_iter())
            .map(move |x| Position(x, self.1));
        let ys = once(self.1 + 1)
            .chain(self.1.checked_sub(1).into_iter())
            .map(move |y| Position(self.0, y));

        xs.chain(ys)
    }
}

fn main() {
    // What is the fewest number of steps required for you to reach 31,39?
    println!(
        "Part 1: {}",
        part1(Position::new(1, 1), Position::new(31, 39)).unwrap()
    );

    // How many locations (distinct x,y coordinates, including your starting location) can you reach in at most 50 steps?
    println!("Part 2: {}", part2(Position::new(1, 1), 50))
}

fn is_open_space(position: Position) -> bool {
    let x = position.0;
    let y = position.1;

    (x * (x + 3 + 2 * y) + y * (1 + y) + INPUT).count_ones() % 2 == 0
}

fn part1(start_position: Position, end_position: Position) -> Option<u64> {
    let mut to_visit = VecDeque::new();
    to_visit.push_back((0, start_position));

    let mut visited = HashSet::new();
    visited.insert(start_position);

    while let Some((steps, position)) = to_visit.pop_front() {
        if position == end_position {
            return Some(steps);
        }

        to_visit.extend(position.neighbouring_positions().filter_map(|p| {
            if visited.insert(p) && is_open_space(p) {
                Some((steps + 1, p))
            } else {
                None
            }
        }));
    }

    None
}

fn part2(start_position: Position, max_steps: u64) -> usize {
    let mut to_visit = VecDeque::new();
    to_visit.push_back((0, start_position));

    let mut visited = HashSet::new();
    visited.insert(start_position);

    while let Some((steps, position)) = to_visit.pop_front() {
        // Stop once we hit the max number of steps. `visited` will contain the correct number
        // since we inserted this position into it before getting here.
        if steps == max_steps {
            break;
        }

        to_visit.extend(position.neighbouring_positions().filter_map(|p| {
            if is_open_space(p) && visited.insert(p) {
                Some((steps + 1, p))
            } else {
                None
            }
        }));
    }

    visited.len()
}
