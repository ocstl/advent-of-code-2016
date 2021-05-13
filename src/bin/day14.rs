use md5::compute;
use std::collections::VecDeque;

const INPUT: &[u8] = b"ihaygndm";

#[derive(Debug, Clone)]
struct PotentialKey {
    pub nonce: u32,
    pub hash: [u8; 16],
    pub first_triplet: Option<u8>,
    pub quintuplets: Vec<u8>,
}

impl PotentialKey {
    pub fn new(nonce: u32) -> Self {
        let hash = compute([INPUT, nonce.to_string().as_bytes()].concat()).0;
        let (first_triplet, quintuplets) = Self::triplet_and_quintuplets(&hash);

        PotentialKey {
            nonce,
            hash,
            first_triplet,
            quintuplets,
        }
    }

    pub fn with_key_stretching(nonce: u32, repetitions: usize) -> Self {
        let hash = compute([INPUT, nonce.to_string().as_bytes()].concat()).0;
        let hash = std::iter::successors(Some(hash), |h| Some(compute(Self::hash_to_hex(h)).0))
            .nth(repetitions)
            .unwrap();
        let (first_triplet, quintuplets) = Self::triplet_and_quintuplets(&hash);

        PotentialKey {
            nonce,
            hash,
            first_triplet,
            quintuplets,
        }
    }

    fn hash_to_hex(hash: &[u8; 16]) -> [u8; 32] {
        let mut hex = [0; 32];
        for (a, b) in hex.iter_mut().step_by(2).zip(hash.iter()) {
            *a = Self::hex_value_to_hex_ascii(b >> 4);
        }

        for (a, b) in hex.iter_mut().skip(1).step_by(2).zip(hash.iter()) {
            *a = Self::hex_value_to_hex_ascii(b & 0xf);
        }

        hex
    }

    fn hex_value_to_hex_ascii(value: u8) -> u8 {
        match value {
            0..=9 => 48 + value,
            10..=15 => 97 + value - 10,
            _ => unreachable!(value),
        }
    }

    fn triplet_and_quintuplets(hash: &[u8; 16]) -> (Option<u8>, Vec<u8>) {
        let hex = Self::hash_to_hex(hash);

        let first_triplet = hex.windows(3).find_map(|w| {
            if w[0] == w[1] && w[0] == w[2] {
                Some(w[0])
            } else {
                None
            }
        });

        let quintuplets = hex
            .windows(5)
            .filter_map(|w| {
                if w[0] == w[1] && w[0] == w[2] && w[0] == w[3] && w[0] == w[4] {
                    Some(w[0])
                } else {
                    None
                }
            })
            .collect();

        (first_triplet, quintuplets)
    }
}

fn main() {
    // Given the actual salt in your puzzle input, what index produces your 64th one-time pad key?
    let hash_generator = (0..).map(PotentialKey::new);
    println!("Part 1: {}", nonce_of_nth_hash(64, hash_generator));

    // Given the actual salt in your puzzle input and using 2016 extra MD5 calls of key stretching,
    // what index now produces your 64th one-time pad key?
    let hash_generator = (0..).map(|nonce| PotentialKey::with_key_stretching(nonce, 2016));
    println!("Part 2: {}", nonce_of_nth_hash(64, hash_generator));
}

/// However, not all of these MD5 hashes are keys, and you need 64 new keys for your one-time pad.
/// A hash is a key only if:
/// * It contains three of the same character in a row, like 777. Only consider the first such
/// triplet in a hash.
/// * One of the next 1000 hashes in the stream contains that same character five times in a row,
/// like 77777.
///
/// Considering future hashes for five-of-a-kind sequences does not cause those hashes to be
/// skipped; instead, regardless of whether the current hash is a key, always resume testing for
/// keys starting with the very next hash.
fn nonce_of_nth_hash<I: Iterator<Item = PotentialKey>>(nth: usize, mut hash_generator: I) -> u32 {
    let mut hashes = VecDeque::with_capacity(1000);
    hashes.extend(hash_generator.by_ref().take(1000));

    let mut keys = Vec::with_capacity(nth);

    while keys.len() < nth {
        let key = hashes.pop_front().unwrap();
        hashes.push_back(hash_generator.by_ref().next().unwrap());

        if let Some(c) = key.first_triplet {
            if hashes.iter().any(|h| h.quintuplets.iter().any(|q| q == &c)) {
                keys.push(key);
            }
        }
    }

    keys.last().unwrap().nonce
}
