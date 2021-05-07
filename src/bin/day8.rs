const INPUT_PATH: &str = "inputs/day8.txt";

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Rect(usize, usize),
    RotateColumn(usize, usize),
    RotateRow(usize, usize),
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let mut iter = input.split_whitespace();
        if iter.next() == Some("rect") {
            let mut iter = iter.next().unwrap().split('x');
            let a = iter.next().unwrap().parse().unwrap();
            let b = iter.next().unwrap().parse().unwrap();
            Instruction::Rect(a, b)
        } else {
            let t = iter.next().unwrap();
            let row_column = iter
                .next()
                .unwrap()
                .split('=')
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
            let size = iter.nth(1).unwrap().parse().unwrap();
            if t == "column" {
                Instruction::RotateColumn(row_column, size)
            } else {
                Instruction::RotateRow(row_column, size)
            }
        }
    }
}

#[derive(Debug, Clone)]
struct LittleScreen([bool; Self::WIDTH * Self::HEIGHT]);

impl LittleScreen {
    const WIDTH: usize = 50;
    const HEIGHT: usize = 6;

    pub fn new() -> Self {
        Self([false; Self::WIDTH * Self::HEIGHT])
    }

    pub fn apply_instruction(&mut self, instruction: Instruction) -> &mut Self {
        match instruction {
            Instruction::Rect(a, b) => self.create_rectangle(a, b),
            Instruction::RotateColumn(column, steps) => self.rotate_column(column, steps),
            Instruction::RotateRow(row, steps) => self.rotate_row(row, steps),
        }

        self
    }

    pub fn nbr_pixels_lit(&self) -> usize {
        self.0.iter().filter(|pixel| **pixel).count()
    }

    fn create_rectangle(&mut self, a: usize, b: usize) {
        for y in 0..b {
            for x in 0..a {
                self.0[y * Self::WIDTH + x] = true;
            }
        }
    }

    fn rotate_column(&mut self, column: usize, steps: usize) {
        let mut initial_column = [false; Self::HEIGHT];
        for (i, pixel) in initial_column
            .iter_mut()
            .zip(self.0.iter().skip(column).step_by(Self::WIDTH))
        {
            *i = *pixel;
        }

        for (i, pixel) in initial_column
            .iter()
            .cycle()
            .skip(Self::HEIGHT - (steps % Self::HEIGHT))
            .zip(self.0.iter_mut().skip(column).step_by(Self::WIDTH))
        {
            *pixel = *i;
        }
    }

    fn rotate_row(&mut self, row: usize, steps: usize) {
        let mut initial_row = [false; Self::WIDTH];
        for (i, pixel) in initial_row
            .iter_mut()
            .zip(self.0.iter().skip(row * Self::WIDTH))
        {
            *i = *pixel;
        }

        // Since we are cycling through it (for the offset), the `take` adaptor is absolutely
        // necessary. Otherwise, we will keep cycling and the iterator over the screen will
        // continue as well.
        for (i, pixel) in initial_row
            .iter()
            .cycle()
            .skip(Self::WIDTH - (steps % Self::WIDTH))
            .take(Self::WIDTH)
            .zip(self.0.iter_mut().skip(row * Self::WIDTH))
        {
            *pixel = *i;
        }
    }
}

impl std::iter::FromIterator<Instruction> for LittleScreen {
    fn from_iter<I: IntoIterator<Item = Instruction>>(iter: I) -> Self {
        let mut screen = LittleScreen::new();
        for instruction in iter {
            screen.apply_instruction(instruction);
        }

        screen
    }
}

impl std::fmt::Display for LittleScreen {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s: String = self
            .0
            .chunks_exact(Self::WIDTH)
            .flat_map(|row| {
                row.iter()
                    .map(|pixel| if *pixel { '#' } else { '.' })
                    .chain(std::iter::once('\n'))
            })
            .collect();
        write!(f, "{}", s)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let screen: LittleScreen = std::fs::read_to_string(INPUT_PATH)?
        .lines()
        .map(Instruction::from)
        .collect();

    // There seems to be an intermediate check of the voltage used by the display: after you
    // swipe your card, if the screen did work, how many pixels should be lit?
    println!("Part 1: {}", screen.nbr_pixels_lit());

    // After you swipe your card, what code is the screen trying to display?
    println!("Part 2: \n{}", screen);

    Ok(())
}
