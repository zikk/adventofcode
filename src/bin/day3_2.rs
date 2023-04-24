use std::fs::read_to_string;

use anyhow::Result;

fn main() -> Result<()> {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .split("")
        .flat_map(|x| x.chars().next())
        .collect();

    let file = read_to_string("./inputs/day3.prod")?;
    let lines: Vec<&str> = file.as_str().split("\n").collect();

    let total: usize = lines
        .chunks(3)
        .filter_map(|group| {
            let first_sack: Vec<char> = group.get(0).unwrap().chars().collect();
            let second_sack: Vec<char> = group.get(1).unwrap().chars().collect();
            let third_sack: Vec<char> = group.get(2).unwrap().chars().collect();

            for c in &first_sack {
                if second_sack.contains(&c) && third_sack.contains(&c) {
                    return Some(*c);
                }
            }

            return None;
        })
        .map(|c| alphabet.iter().position(|&c2| c == c2).unwrap() + 1)
        .sum::<usize>();

    println!("Total: {:?}", total);
    return Ok(());
}
