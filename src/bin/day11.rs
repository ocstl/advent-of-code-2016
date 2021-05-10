use std::collections::{BTreeSet, HashSet, VecDeque};

const INPUT_PATH: &str = "inputs/day11.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Element {
    Dilithium,
    Elerium,
    Plutonium,
    Promethium,
    Ruthenium,
    Strontium,
    Thulium,
}

impl From<&str> for Element {
    fn from(element: &str) -> Self {
        match element {
            "dilithium" => Element::Dilithium,
            "elerium" => Element::Elerium,
            "plutonium" => Element::Plutonium,
            "promethium" => Element::Promethium,
            "ruthenium" => Element::Ruthenium,
            "strontium" => Element::Strontium,
            "thulium" => Element::Thulium,
            e => unimplemented!("{}", e),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Equipment {
    Generator(Element),
    Microchip(Element),
}

impl Equipment {
    fn is_generator(&self) -> bool {
        std::mem::discriminant(self)
            == std::mem::discriminant(&Equipment::Generator(Element::Dilithium))
    }

    fn is_microchip(&self) -> bool {
        std::mem::discriminant(self)
            == std::mem::discriminant(&Equipment::Microchip(Element::Dilithium))
    }

    fn element(self) -> Element {
        match self {
            Equipment::Generator(element) => element,
            Equipment::Microchip(element) => element,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Floor(BTreeSet<Equipment>);

impl Floor {
    pub fn add(&mut self, equipment: Equipment) -> &mut Self {
        self.0.insert(equipment);
        self
    }

    pub fn remove(&mut self, equipment: &Equipment) -> &mut Self {
        self.0.remove(equipment);
        self
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> std::collections::btree_set::Iter<Equipment> {
        self.0.iter()
    }

    /// If at least one generator is present, a chip will be fried if its corresponding generator
    /// is not present.
    pub fn fried_chip(&self) -> bool {
        let generators: HashSet<Element> = self.generators().collect();
        !generators.is_empty()
            && self
                .microchips()
                .any(|element| !generators.contains(&element))
    }

    pub fn generators(&self) -> impl Iterator<Item = Element> + '_ {
        self.0.iter().filter_map(|e| {
            if e.is_generator() {
                Some(e.element())
            } else {
                None
            }
        })
    }

    pub fn microchips(&self) -> impl Iterator<Item = Element> + '_ {
        self.0.iter().filter_map(|e| {
            if e.is_microchip() {
                Some(e.element())
            } else {
                None
            }
        })
    }
}

impl From<&str> for Floor {
    fn from(line: &str) -> Self {
        let mut floor = Floor::default();
        let mut iter = line
            .trim()
            .trim_end_matches('.')
            .split(&[',', ' '][..])
            .rev();

        while let Some(w) = iter.next() {
            match w {
                "generator" => {
                    let element = Element::from(iter.next().unwrap().split('-').next().unwrap());
                    floor.add(Equipment::Generator(element));
                }
                "microchip" => {
                    let element = Element::from(iter.next().unwrap().split('-').next().unwrap());
                    floor.add(Equipment::Microchip(element));
                }
                _ => (),
            }
        }

        floor
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT_PATH)?;
    let floors: Vec<Floor> = input.lines().map(Floor::from).collect();

    println!("Part 1: {}", part1(&floors));
    println!("Part 2: {}", part2(&floors));

    Ok(())
}

fn neighbouring_floors(elevator: usize) -> impl Iterator<Item = usize> {
    (0..4)
        .filter(move |&f| f > elevator && f - elevator == 1)
        .chain((0..4).filter(move |&f| f < elevator && elevator - f == 1))
}

/// In your situation, what is the minimum number of steps required to bring all of the objects
/// to the fourth floor?
fn part1(floors: &[Floor]) -> usize {
    // If everything (or nothing) is already on the fourth floor, we are done.
    if floors.iter().take(3).all(Floor::is_empty) {
        return 0;
    }

    let mut to_visit = VecDeque::new();
    to_visit.push_back((0, 0, floors.to_vec()));

    let mut visited = HashSet::new();

    while let Some((steps, elevator, state)) = to_visit.pop_front() {
        let steps = steps + 1;

        for (eq1, eq2) in state[elevator]
            .iter()
            .flat_map(|eq1| state[elevator].iter().map(move |eq2| (eq1, eq2)))
        {
            let mut new_state = state.clone();
            new_state[elevator].remove(eq1);
            new_state[elevator].remove(eq2);

            if !new_state[elevator].fried_chip() {
                for new_elevator in neighbouring_floors(elevator) {
                    let mut s = new_state.clone();
                    s[new_elevator].add(*eq1);
                    s[new_elevator].add(*eq2);

                    // Return early if we find the answer.
                    if s.iter().take(3).all(Floor::is_empty) {
                        return steps;
                    }

                    if !s[new_elevator].fried_chip() && visited.insert((new_elevator, s.clone())) {
                        to_visit.push_back((steps, new_elevator, s))
                    }
                }
            }
        }
    }

    0
}

/// What is the minimum number of steps required to bring all of the objects, including these
/// four new ones, to the fourth floor?
fn part2(floors: &[Floor]) -> usize {
    let mut floors = floors.to_vec();
    floors[0].add(Equipment::Generator(Element::Elerium));
    floors[0].add(Equipment::Microchip(Element::Elerium));
    floors[0].add(Equipment::Generator(Element::Dilithium));
    floors[0].add(Equipment::Microchip(Element::Dilithium));
    part1(&floors)
}
