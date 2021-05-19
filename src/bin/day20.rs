const INPUT_PATH: &str = "inputs/day20.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct AddressFilter {
    start_value: u32,
    end_value: u32,
}

impl AddressFilter {
    pub fn blocked_address(self, address: u32) -> bool {
        self.start_value <= address && address <= self.end_value
    }
}

impl<T: AsRef<str>> From<T> for AddressFilter {
    fn from(input: T) -> Self {
        let (start_value, end_value) = input.as_ref().split_once('-').unwrap();

        AddressFilter {
            start_value: start_value.parse().unwrap(),
            end_value: end_value.parse().unwrap(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut filters: Vec<AddressFilter> = std::fs::read_to_string(INPUT_PATH)?
        .lines()
        .map(AddressFilter::from)
        .collect();

    filters.sort_unstable();

    // Given the list of blocked IPs you retrieved from the firewall (your puzzle input), what is
    // the lowest-valued IP that is not blocked?
    let first_address = filters.iter().fold(0, |current, filter| {
        if filter.blocked_address(current) {
            filter.end_value + 1
        } else {
            current
        }
    });

    println!("Part 1: {}", first_address);

    // How many IPs are allowed by the blacklist?
    let (count, final_address) = filters.iter().fold((0, 0), |(count, current), filter| {
        match (filter.start_value <= current, current <= filter.end_value) {
            (true, true) => (count, filter.end_value),
            (true, false) => (count, current),
            (false, true) => ((filter.start_value - current - 1) + count, filter.end_value),
            (false, false) => unreachable!(),
        }
    });

    // In case the last filter doesn't cover all the way to the last address.
    let allowed_addresses = count + (std::u32::MAX - final_address);

    println!("Part 2: {}", allowed_addresses);

    Ok(())
}
