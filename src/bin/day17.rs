use std::collections::VecDeque;

const INPUT: &[u8] = b"hhhxzeay";
const DIRECTIONS: [u8; 4] = [b'U', b'D', b'L', b'R'];
const MIN_X: i64 = 0;
const MAX_X: i64 = 3;
const MIN_Y: i64 = 0;
const MAX_Y: i64 = 3;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<u8> for Direction {
    fn from(c: u8) -> Self {
        match c {
            b'U' => Direction::Up,
            b'D' => Direction::Down,
            b'L' => Direction::Left,
            b'R' => Direction::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Position(i64, i64);

impl Position {
    pub fn new(x: i64, y: i64) -> Self {
        Position(x, y)
    }

    fn update(self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Position(self.0, self.1 - 1),
            Direction::Down => Position(self.0, self.1 + 1),
            Direction::Left => Position(self.0 - 1, self.1),
            Direction::Right => Position(self.0 + 1, self.1),
        }
    }
}

fn main() {
    let start_position = Position::new(0, 0);
    let end_position = Position::new(3, 3);
    println!("Part 1: {}", part1(start_position, end_position));
    println!("Part 2: {}", part2(start_position, end_position));
}

/// Given your vault's passcode, what is the shortest path (the actual path, not just the length)
/// to reach the vault?
fn part1(start_position: Position, end_position: Position) -> String {
    let mut to_visit = VecDeque::new();
    to_visit.push_back((Vec::new(), start_position));

    while let Some((steps, position)) = to_visit.pop_front() {
        if position == end_position {
            //  The only `u8`s are those in `DIRECTIONS`, so this is fine.
            return unsafe { String::from_utf8_unchecked(steps) };
        }

        for d in open_directions(&steps) {
            let new_p = position.update(Direction::from(d));
            if new_p.0 >= MIN_X && new_p.0 <= MAX_X && new_p.1 >= MIN_Y && new_p.1 <= MAX_Y {
                let mut new_s = steps.clone();
                new_s.push(d);
                to_visit.push_back((new_s, new_p));
            }
        }
    }

    String::new()
}

/// What is the length of the longest path that reaches the vault?
fn part2(start_position: Position, end_position: Position) -> usize {
    // Use a `Vec` instead of a `VecDeque`, as we cannot end early, and exploring all the way
    // through as soon as possible should limit the required size.
    let mut to_visit = vec![(Vec::new(), start_position)];
    let mut max_steps = 0;

    while let Some((steps, position)) = to_visit.pop() {
        if position == end_position {
            max_steps = max_steps.max(steps.len());
        } else {
            for d in open_directions(&steps) {
                let new_p = position.update(Direction::from(d));
                if new_p.0 >= MIN_X && new_p.0 <= MAX_X && new_p.1 >= MIN_Y && new_p.1 <= MAX_Y {
                    let mut new_s = steps.clone();
                    new_s.push(d);
                    to_visit.push((new_s, new_p));
                }
            }
        }
    }

    max_steps
}

fn open_directions(steps: &[u8]) -> Vec<u8> {
    md5::compute(&[INPUT, steps].concat())
        .0
        .iter()
        .take(2)
        .flat_map(|c| std::iter::once(*c >= 0xb0).chain(std::iter::once((c & 0xf) >= 0xb)))
        .zip(DIRECTIONS.iter())
        .filter_map(|(b, d)| if b { Some(d) } else { None })
        .copied()
        .collect()
}
