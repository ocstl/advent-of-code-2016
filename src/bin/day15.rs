use regex::Regex;

const INPUT_PATH: &str = "inputs/day15.txt";

thread_local! {
    pub static DISC: Regex = {
        Regex::new(r"(?x)
            ^Disc\ \#(\d*)\ has\ (\d*)\ positions;
            \ at\ time=0,\ it\ is\ at\ position\ (\d*).$
        ").unwrap()
    };
}

#[derive(Debug, Clone, Copy)]
struct Disc {
    id: i64,
    positions: i64,
    initial_position: i64,
}

impl Disc {
    pub fn new(id: i64, positions: i64, initial_position: i64) -> Self {
        Disc {
            id,
            positions,
            initial_position,
        }
    }
}

impl From<&str> for Disc {
    fn from(input: &str) -> Self {
        let caps = DISC.with(|re| re.captures(input)).unwrap();
        let id = caps.get(1).unwrap().as_str().parse().unwrap();
        let positions = caps.get(2).unwrap().as_str().parse().unwrap();
        let initial_position = caps.get(3).unwrap().as_str().parse().unwrap();

        Disc {
            id,
            positions,
            initial_position,
        }
    }
}

#[allow(clippy::many_single_char_names)]
fn extended_euclid(a: i64, b: i64) -> (i64, i64) {
    let mut old_r = a;
    let mut r = b;
    let mut old_s = 1;
    let mut s = 0;
    let mut old_t = 0;
    let mut t = 1;

    while r != 0 {
        let q = old_r / r;
        std::mem::swap(&mut old_r, &mut r);
        r -= q * old_r;
        std::mem::swap(&mut old_s, &mut s);
        s -= q * old_s;
        std::mem::swap(&mut old_t, &mut t);
        t -= q * old_t;
    }

    (old_s, old_t)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let discs: Vec<Disc> = input.lines().map(Disc::from).collect();

    println!("Part 1: {}", part1(&discs));
    println!("Part 2: {}", part2(&discs));

    Ok(())
}

/// What is the first time you can press the button to get a capsule?
fn part1(discs: &[Disc]) -> i64 {
    // Since the number of positions are all coprime for the discs, we can use the Chinese
    // remainder theorem, updating using one disc at a time.
    let first_disc = discs[0];
    let mut a1 = (-first_disc.id - first_disc.initial_position).rem_euclid(first_disc.positions);
    let mut n1 = first_disc.positions;

    for disc in &discs[1..] {
        let a2 = (-disc.id - disc.initial_position).rem_euclid(disc.positions);
        let n2 = disc.positions;
        let (m1, m2) = extended_euclid(n1, n2);

        a1 = (a1 * m2 * n2 + a2 * m1 * n1).rem_euclid(n1 * n2);
        n1 *= n2;
    }

    a1
}

/// With this new disc, and counting again starting from time=0 with the configuration in your
/// puzzle input, what is the first time you can press the button to get another capsule?
fn part2(discs: &[Disc]) -> i64 {
    // We could extract the modulus from `part1`, but it's simpler to just add one more disc
    // (with 11 positions, starting at position 0 at time 0) and run it again.
    let mut discs = discs.to_vec();
    let id = discs.last().unwrap().id;
    discs.push(Disc::new(id + 1, 11, 0));

    part1(&discs)
}
