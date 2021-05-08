const INPUT_PATH: &str = "inputs/day9.txt";

#[derive(Debug, Clone, Copy)]
enum Format {
    VersionOne,
    VersionTwo,
}

/// We're assuming that (sub-)markers are contained within a single marker.
fn decompressed_length(s: &str, format: Format) -> usize {
    let mut iter = s.trim().chars();
    let mut length = 0;

    while let Some(c) = iter.next() {
        if c == '(' {
            let (nbr_chars, repetitions) = read_marker(&mut iter);
            let sub_string = iter.by_ref().take(nbr_chars);
            match format {
                Format::VersionOne => {
                    // We could use `nbr_chars * repetitions`, but we need to consume the
                    // iterator to effectively skip the characters.
                    length += sub_string.count() * repetitions;
                }
                Format::VersionTwo => {
                    length +=
                        decompressed_length(&sub_string.collect::<String>(), format) * repetitions;
                }
            }
        } else {
            length += 1;
        }
    }

    length
}

fn read_marker<I: Iterator<Item = char>>(iter: &mut I) -> (usize, usize) {
    let marker: String = iter.take_while(|&c| c != ')').collect();
    let mut m = marker.split('x');

    (
        m.next().unwrap().parse().unwrap(),
        m.next().unwrap().parse().unwrap(),
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT_PATH)?;

    // What is the decompressed length of the file (your puzzle input)? Don't count whitespace.
    println!(
        "Part 1: {}",
        decompressed_length(&input, Format::VersionOne)
    );

    // What is the decompressed length of the file using this improved format?
    println!(
        "Part 2: {}",
        decompressed_length(&input, Format::VersionTwo)
    );

    Ok(())
}
