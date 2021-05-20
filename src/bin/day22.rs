use regex::Regex;
use std::collections::HashMap;
use std::iter::once;

const INPUT_PATH: &str = "inputs/day22.txt";

thread_local! {
    pub static NODE_REGEX: Regex = {
        Regex::new(r"^/dev/grid/node-x(\d+)-y(\d+)\s*(\d+)T\s*(\d+)T\s*(\d+)T\s*(\d+)%$").unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(u64, u64);

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

#[derive(Debug, Clone, Copy)]
pub struct Node {
    size: u64,
    used: u64,
    available: u64,
}

impl Node {
    pub fn new(size: u64, used: u64, available: u64) -> Self {
        Node {
            size,
            used,
            available,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cluster(HashMap<Position, Node>);

impl Cluster {
    pub fn get(&self, position: &Position) -> Option<&Node> {
        self.0.get(position)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<Position, Node> {
        self.0.iter()
    }
}

impl<T: AsRef<str>> From<T> for Cluster {
    fn from(input: T) -> Self {
        let input = input.as_ref();

        let mut h = HashMap::new();

        for line in input.lines().skip(2) {
            let caps = NODE_REGEX.with(|re| re.captures(line)).unwrap();
            let x = caps.get(1).unwrap().as_str().parse().unwrap();
            let y = caps.get(2).unwrap().as_str().parse().unwrap();
            let total = caps.get(3).unwrap().as_str().parse().unwrap();
            let used = caps.get(4).unwrap().as_str().parse().unwrap();
            let available = caps.get(5).unwrap().as_str().parse().unwrap();

            h.insert(Position::new(x, y), Node::new(total, used, available));
        }

        Cluster(h)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let cluster = Cluster::from(&input);

    // How many viable pairs of nodes are there?
    println!("Part 1: {}", part1(&cluster));

    // What is the fewest number of steps required to move your goal data to node-x0-y0?
    part2(&cluster);

    Ok(())
}

/// To do this, you'd like to count the number of viable pairs of nodes. A viable pair is any two
/// nodes (A,B), regardless of whether they are directly connected, such that:
///
/// * Node A is not empty (its Used is not zero).
/// * Nodes A and B are not the same node.
/// * The data on node A (its Used) would fit on node B (its Avail).
#[allow(clippy::suspicious_operation_groupings)]
fn part1(cluster: &Cluster) -> usize {
    cluster
        .iter()
        .filter(|(_, n)| n.used > 0)
        .flat_map(|(&p, n)| {
            cluster
                .iter()
                .filter(move |(&q, m)| p != q && n.used <= m.available)
        })
        .count()
}

fn part2(cluster: &Cluster) {
    // We represent the massive nodes as '#' (unpassable), the empty node as '_' (only one) and
    // the rest as '.'. Replace the top-right most node with 'G' to see the problem. It becomes
    // relatively easy to do it by hand.
    // Considering that only the empty node has enough available storage, it might be possible to
    // use BFS instead.
    for y in 0..=26 {
        let mut s = String::new();
        for x in 0..=36 {
            let &Node {
                size,
                used,
                available: _,
            } = cluster.get(&Position::new(x, y)).unwrap();

            if size > 100 {
                s.push('#');
            } else if used == 0 {
                s.push('_');
            } else {
                s.push('.');
            }
        }
        println!("{}", s);
    }
}
