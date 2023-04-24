use std::fs::read_to_string;

use anyhow::{Ok, Result};

fn is_range_in_range(a: &Vec<usize>, b: &Vec<usize>) -> bool {
    if a.get(0) >= b.get(0) && a.get(a.len() - 1) <= b.get(b.len() - 1) {
        return true;
    }

    return false;
}

fn main() -> Result<()> {
    let total: usize = read_to_string("./inputs/day4.prod")?
        .lines()
        .filter_map(|line| {
            let shifts = line.split(",").collect::<Vec<&str>>();
            let shift_1: Vec<usize> = shifts
                .get(0)
                .expect("Should really work wtf!")
                .split("-")
                .map(|s| s.parse::<usize>().expect("Should really work wtf!"))
                .collect();
            let shift_2: Vec<usize> = shifts
                .get(1)
                .expect("Should really work wtf!")
                .split("-")
                .map(|s| s.parse::<usize>().expect("Should really work wtf!"))
                .collect();

            let shift_1_range: Vec<usize> = match (shift_1.get(0), shift_1.get(1)) {
                (Some(start), Some(end)) => (*start..=*end).collect(),
                _ => {
                    panic!("Really shouldn't happen figure it out")
                }
            };

            let shift_2_range: Vec<usize> = match (shift_2.get(0), shift_2.get(1)) {
                (Some(start), Some(end)) => (*start..=*end).collect(),
                _ => {
                    panic!("Really shouldn't happen figure it out")
                }
            };

            if is_range_in_range(&shift_1_range, &shift_2_range)
                || is_range_in_range(&shift_2_range, &shift_1_range)
            {
                return Some(1);
            }

            return Some(0);
        })
        .sum::<usize>();

    println!("Total: {}", total);
    return Ok(());
}
