const INPUT_PATH: &str = "inputs/day18.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Safe,
    Trap,
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            '.' => Tile::Safe,
            '^' => Tile::Trap,
            _ => panic!("Not a valid tile: {}", c),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row(Vec<Tile>);

impl Row {
    pub fn count_safe_tiles(&self) -> usize {
        self.0.iter().filter(|&&t| t == Tile::Safe).count()
    }

    pub fn generate_next_row(&self) -> Self {
        // We can simplify the rules to XORing the right and left tiles.
        let t = [&[Tile::Safe] as &[Tile], &self.0, &[Tile::Safe] as &[Tile]].concat();
        let tiles = t
            .windows(3)
            .map(|w| {
                if (w[0] == Tile::Trap) ^ (w[2] == Tile::Trap) {
                    Tile::Trap
                } else {
                    Tile::Safe
                }
            })
            .collect();

        Row(tiles)
    }
}

impl<T: AsRef<str>> From<T> for Row {
    fn from(tiles: T) -> Row {
        Row(tiles.as_ref().trim().chars().map(Tile::from).collect())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let initial_row = Row::from(&input);

    // Starting with the map in your puzzle input, in a total of 40 rows (including the starting
    // row), how many safe tiles are there?
    println!("Part 1: {}", safe_tiles_over_n_rows(&initial_row, 40));

    // How many safe tiles are there in a total of 400000 rows?
    println!("Part 2: {}", safe_tiles_over_n_rows(&initial_row, 400000));

    Ok(())
}

fn safe_tiles_over_n_rows(initial_row: &Row, nbr_rows: usize) -> usize {
    std::iter::successors(Some(initial_row.clone()), |row| {
        Some(row.generate_next_row())
    })
    .map(|row| row.count_safe_tiles())
    .take(nbr_rows)
    .sum()
}
