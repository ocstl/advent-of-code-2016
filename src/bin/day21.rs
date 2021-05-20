use permutohedron::Heap;

const INPUT_PATH: &str = "inputs/day21.txt";
const LETTERS: &[u8] = b"abcdefgh";

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    SwapPositions(usize, usize),
    SwapLetters(u8, u8),
    RotateLeft(usize),
    RotateRight(usize),
    RotatePosition(u8),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl<T: AsRef<str>> From<T> for Instruction {
    fn from(instruction: T) -> Self {
        let instruction = instruction.as_ref();
        let mut iter = instruction.trim().split_whitespace();

        match iter.next() {
            Some("swap") => match iter.next() {
                Some("position") => {
                    let first = iter.next().unwrap().parse().unwrap();
                    let second = iter.nth(2).unwrap().parse().unwrap();
                    Instruction::SwapPositions(first, second)
                }
                Some("letter") => {
                    let first = iter.next().unwrap().bytes().next().unwrap();
                    let second = iter.nth(2).unwrap().bytes().next().unwrap();
                    Instruction::SwapLetters(first, second)
                }
                _ => panic!("Invalid instruction: {}", instruction),
            },
            Some("rotate") => match iter.next() {
                Some("left") => Instruction::RotateLeft(iter.next().unwrap().parse().unwrap()),
                Some("right") => Instruction::RotateRight(iter.next().unwrap().parse().unwrap()),
                Some("based") => {
                    Instruction::RotatePosition(iter.nth(4).unwrap().bytes().next().unwrap())
                }
                _ => panic!("Invalid instruction: {}", instruction),
            },
            Some("reverse") => {
                let first = iter.nth(1).unwrap().parse().unwrap();
                let second = iter.nth(1).unwrap().parse().unwrap();
                Instruction::Reverse(first, second)
            }
            Some("move") => {
                let first = iter.nth(1).unwrap().parse().unwrap();
                let second = iter.nth(2).unwrap().parse().unwrap();
                Instruction::Move(first, second)
            }
            _ => panic!("Invalid instruction: {}", instruction),
        }
    }
}

pub struct Password(Vec<u8>);

impl Password {
    pub fn new(s: &[u8]) -> Self {
        Password(s.to_vec())
    }

    pub fn apply_instruction(&mut self, instruction: Instruction) -> &mut Self {
        match instruction {
            Instruction::SwapPositions(a, b) => {
                self.0.swap(a, b);
            }
            Instruction::SwapLetters(a, b) => {
                let a = self.0.iter().position(|&x| x == a).unwrap();
                let b = self.0.iter().position(|&x| x == b).unwrap();
                self.0.swap(a, b);
            }
            Instruction::RotateLeft(a) => {
                self.0.rotate_left(a);
            }
            Instruction::RotateRight(a) => {
                self.0.rotate_right(a);
            }
            Instruction::RotatePosition(a) => {
                let a = self.0.iter().position(|&x| x == a).unwrap();
                let a = (1 + a + if a >= 4 { 1 } else { 0 }) % self.0.len();
                self.0.rotate_right(a);
            }
            Instruction::Reverse(a, b) => self.0[a..=b].reverse(),
            Instruction::Move(a, b) => {
                let x = self.0.remove(a);
                self.0.insert(b, x);
            }
        }

        self
    }

    pub fn apply_instructions(&mut self, instructions: &[Instruction]) -> &mut Self {
        for &instruction in instructions {
            self.apply_instruction(instruction);
        }

        self
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = String::from_utf8_lossy(&self.0);
        write!(f, "{}", s)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let instructions: Vec<Instruction> = std::fs::read_to_string(INPUT_PATH)?
        .lines()
        .map(Instruction::from)
        .collect();

    // Given the list of scrambling operations in your puzzle input, what is the result of
    // scrambling abcdefgh?
    let mut password = Password::new(LETTERS);
    password.apply_instructions(&instructions);
    println!("Part 1: {}", password);

    // What is the un-scrambled version of the scrambled password fbgdceah?
    let password = Heap::new(&mut LETTERS.to_vec())
        .find_map(|h| {
            let mut p = Password::new(&h);
            p.apply_instructions(&instructions);
            if p.to_string() == "fbgdceah" {
                Some(unsafe { String::from_utf8_unchecked(h.to_vec()) })
            } else {
                None
            }
        })
        .unwrap_or_else(|| String::from("No password found."));
    println!("Part 2: {}", password);

    Ok(())
}
