use std::{collections::HashSet, fs::read_to_string};

use anyhow::Result;

fn main() -> Result<()> {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .split("")
        .flat_map(|x| x.chars().next())
        .collect();

    let total = read_to_string("./inputs/day3.prod")?
        .lines()
        .map(|items| {
            let mut shared_items: HashSet<char> = HashSet::new();

            let first_half: Vec<char> = items
                .split("")
                .take(items.len() / 2 + 1)
                .flat_map(|x| x.chars().next())
                .collect();
            let second_half: Vec<char> = items
                .split("")
                .skip(items.len() / 2 + 1)
                .flat_map(|x| x.chars().next())
                .collect();

            first_half.iter().for_each(|&c| {
                if second_half.iter().any(|&c2| c == c2) {
                    shared_items.insert(c);
                }
            });

            return alphabet
                .iter()
                .position(|&c| shared_items.contains(&c))
                .unwrap()
                + 1;
        })
        .sum::<usize>();

    println!("Total: {}", total);
    return Ok(());
}
