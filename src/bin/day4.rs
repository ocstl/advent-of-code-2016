use advent_of_code_2016::counter::Counter;
use std::cmp::Reverse;

const INPUT_PATH: &str = "inputs/day4.txt";

#[derive(Clone, Debug)]
struct Room {
    name: String,
    sector_id: u32,
    checksum: String,
}

impl From<&str> for Room {
    fn from(input: &str) -> Self {
        let mut iter = input
            .trim_end_matches(']')
            .rsplitn(3, |c| c == '[' || c == '-');

        let checksum = iter.next().unwrap().to_string();
        let sector_id = iter.next().unwrap().parse().unwrap();
        let name = iter.next().unwrap().to_string();

        Self {
            checksum,
            sector_id,
            name,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rooms: Vec<Room> = std::fs::read_to_string(INPUT_PATH)?
        .lines()
        .map(Room::from)
        .collect();

    let real_rooms: Vec<Room> = rooms
        .into_iter()
        .filter(|room| {
            let mut sorted_name: Vec<(char, usize)> = room
                .name
                .chars()
                .collect::<Counter<char>>()
                .into_iter()
                .filter(|(c, _)| *c != '-')
                .collect();
            sorted_name.sort_by_key(|(c, n)| (Reverse(*n), *c));

            let checksum = sorted_name
                .iter()
                .take(room.checksum.len())
                .map(|(c, _)| c)
                .collect::<String>();

            checksum == room.checksum
        })
        .collect();

    println!(
        "Part 1: {}",
        real_rooms.iter().map(|room| room.sector_id).sum::<u32>()
    );

    println!("Part 2: {}", part2(&real_rooms));

    Ok(())
}

fn part2(rooms: &[Room]) -> u32 {
    rooms
        .iter()
        .find_map(|room| {
            let shift = (room.sector_id % 26) as u8;
            let plain: String = room
                .name
                .chars()
                .map(|c| match c {
                    '-' => ' ',
                    _ => char::from(((c as u8 + shift - b'a') % 26) + b'a'),
                })
                .collect();

            if plain == "northpole object storage" {
                Some(room.sector_id)
            } else {
                None
            }
        })
        .unwrap()
}
