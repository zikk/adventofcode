use std::collections::HashSet;

use anyhow::Result;

fn main() -> Result<()> {
    let file = std::fs::read_to_string("./inputs/day6.prod")?;
    const DISTINCT_MARKER_SIZE: usize = 4;

    let result = file
        .chars()
        .collect::<Vec<char>>()
        .windows(DISTINCT_MARKER_SIZE)
        .position(|chunk| {
            let mut hash: HashSet<char> = HashSet::new();

            for c in chunk {
                hash.insert(*c);
            }

            return hash.len() == DISTINCT_MARKER_SIZE;
        });

    println!("Result: {:?}", result.unwrap() + DISTINCT_MARKER_SIZE);

    return Ok(());
}
