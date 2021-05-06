const INPUT_PATH: &str = "inputs/day7.txt";

pub struct IPv7 {
    supernet_sequences: Vec<String>,
    hypernet_sequences: Vec<String>,
}

impl IPv7 {
    pub fn supports_tls(&self) -> bool {
        self.supernet_sequences
            .iter()
            .any(|s| Self::has_autonomous_bridge_bypass_annotation(s))
            && !self
                .hypernet_sequences
                .iter()
                .any(|s| Self::has_autonomous_bridge_bypass_annotation(s))
    }

    pub fn supports_ssl(&self) -> bool {
        let babs: Vec<&[u8]> = self.byte_allocation_blocks().collect();

        self.area_broadcast_accessors().any(|aba| {
            babs.iter()
                .any(|bab| bab == &Self::area_broadcast_accessor_to_byte_allocation_block(aba))
        })
    }

    fn has_autonomous_bridge_bypass_annotation(s: &str) -> bool {
        s.as_bytes().windows(4).any(|window| {
            window[0] != window[1] && window[0] == window[3] && window[1] == window[2]
        })
    }

    fn area_broadcast_accessors(&self) -> impl Iterator<Item = &[u8]> {
        self.supernet_sequences.iter().flat_map(|address| {
            address
                .as_bytes()
                .windows(3)
                .filter(|window| window[0] != window[1] && window[0] == window[2])
        })
    }

    fn byte_allocation_blocks(&self) -> impl Iterator<Item = &[u8]> {
        self.hypernet_sequences.iter().flat_map(|address| {
            address
                .as_bytes()
                .windows(3)
                .filter(|window| window[0] != window[1] && window[0] == window[2])
        })
    }

    fn area_broadcast_accessor_to_byte_allocation_block(aba: &[u8]) -> Vec<u8> {
        vec![aba[1], aba[0], aba[1]]
    }
}

impl From<&str> for IPv7 {
    fn from(address: &str) -> Self {
        let mut supernet_sequences = Vec::new();
        let mut hypernet_sequences = Vec::new();

        for (idx, s) in address.split(|c| c == '[' || c == ']').enumerate() {
            if !s.is_empty() {
                if idx % 2 == 0 {
                    supernet_sequences.push(s.to_string());
                } else {
                    hypernet_sequences.push(s.to_string());
                }
            }
        }

        IPv7 {
            supernet_sequences,
            hypernet_sequences,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addresses: Vec<IPv7> = std::fs::read_to_string(INPUT_PATH)?
        .lines()
        .map(IPv7::from)
        .collect();

    // How many IPs in your puzzle input support TLS?
    println!(
        "Part 1: {}",
        addresses
            .iter()
            .filter(|address| address.supports_tls())
            .count()
    );

    // How many IPs in your puzzle input support SSL?
    println!(
        "Part 2: {}",
        addresses
            .iter()
            .filter(|address| address.supports_ssl())
            .count()
    );

    Ok(())
}
