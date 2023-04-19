use std::fs::read_to_string;

use anyhow::Result;

fn main() -> Result<()> {
    let total: Vec<usize> = read_to_string("./inputs/day1.prod")?
        .split("\n\n")
        .map(|x| x.lines().flat_map(str::parse::<usize>).sum())
        .collect();

    println!("Result: {:?}", total.iter().max().unwrap());
    return Ok(());
}
