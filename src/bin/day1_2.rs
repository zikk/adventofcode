use std::fs::read_to_string;

use anyhow::Result;

fn main() -> Result<()> {
    let mut total: Vec<usize> = read_to_string("./inputs/day1.prod")?
        .split("\n\n")
        .map(|x| x.lines().flat_map(str::parse::<usize>).sum())
        .collect();

    total.sort_by(|a, b| b.cmp(a));

    println!("Result: {:?}", total.into_iter().take(3).sum::<usize>());
    return Ok(());
}
