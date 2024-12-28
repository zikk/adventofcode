use std::{collections::HashMap, fs::read_to_string};
const INPUT_PATH: &str = "./inputs/day11.in";

fn main() {
    let mut stones: HashMap<usize, usize> = HashMap::new();

    read_to_string(INPUT_PATH)
        .unwrap()
        .trim()
        .split(" ")
        .for_each(|v| {
            let value = v.to_string().parse::<usize>().unwrap();
            *stones.entry(value).or_insert(0) += 1;
        });

    for _ in 0..75 {
        let mut new_stones: HashMap<usize, usize> = HashMap::new();

        stones.iter().for_each(|(v, count)| {
            let value_as_chars = v.to_string().chars().collect::<Vec<_>>();

            if *v == 0 {
                *new_stones.entry(1).or_insert(0) += count;
            } else if value_as_chars.len() % 2 == 0 {
                let (start, end) = value_as_chars.split_at(value_as_chars.len() / 2);
                let start_i = start
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .concat()
                    .parse::<usize>()
                    .unwrap();
                let end_i = end
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .concat()
                    .parse::<usize>()
                    .unwrap();

                *new_stones.entry(start_i).or_insert(0) += count;
                *new_stones.entry(end_i).or_insert(0) += count;
            } else {
                *new_stones.entry(v * 2024).or_insert(0) += count;
            }
        });

        stones = new_stones.clone();
    }

    println!(
        "Result : {}",
        stones.values().map(|v| *v).reduce(|a, b| a + b).unwrap(),
    );
}
