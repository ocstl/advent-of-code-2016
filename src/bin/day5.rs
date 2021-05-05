use md5::compute;

const DOOR_ID: &[u8] = b"abbhdwsy";
const PASSWORD_LENGTH: usize = 8;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

/// Given the actual Door ID, what is the password?
fn part1() -> String {
    (0_u32..)
        .filter_map(|nonce| {
            let digest = compute([DOOR_ID, nonce.to_string().as_bytes()].concat());
            if begins_with_five_zeroes(&digest.0) {
                format!("{:x}", digest).chars().nth(5)
            } else {
                None
            }
        })
        .take(PASSWORD_LENGTH)
        .collect()
}

/// Given the actual Door ID and this new method, what is the password?
fn part2() -> String {
    let mut password: [Option<char>; PASSWORD_LENGTH] = [None; PASSWORD_LENGTH];

    for nonce in 0_u32.. {
        let digest = compute([DOOR_ID, nonce.to_string().as_bytes()].concat());
        if begins_with_five_zeroes(&digest.0) {
            let idx = digest.0[2] as usize;
            if idx < password.len() && password[idx].is_none() {
                password[idx].replace(format!("{:x}", digest).chars().nth(6).unwrap());

                if password.iter().all(|c| c.is_some()) {
                    break;
                }
            }
        }
    }

    password.iter().flatten().collect()
}

fn begins_with_five_zeroes(digest: &[u8; 16]) -> bool {
    digest[0..2] == [0, 0] && digest[2] < 0b10000
}
