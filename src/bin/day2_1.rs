use std::fs::read_to_string;

use anyhow::Result;

fn main() -> Result<()> {
    let _opponent_hands = vec!['A', 'B', 'C'];
    let player_hands = vec!['X', 'Y', 'Z'];
    let wins = vec![['X', 'C'], ['Y', 'A'], ['Z', 'B']];
    let loses = vec![['X', 'B'], ['Y', 'C'], ['Z', 'A']];

    let file = read_to_string("./inputs/day2.prod")?;

    let x: Vec<_> = file
        .lines()
        .into_iter()
        .map(|line| {
            let chars: Vec<char> = line
                .split_whitespace()
                .flat_map(|x| x.chars().next())
                .collect();

            let player_score: usize = player_hands.iter().position(|&x| x == chars[1]).unwrap() + 1;

            let is_win = wins
                .iter()
                .any(|t| t.get(1) == chars.get(0) && t.get(0) == chars.get(1));

            if is_win {
                return player_score + 6;
            }

            let is_lose = loses
                .iter()
                .any(|t| t.get(1) == chars.get(0) && t.get(0) == chars.get(1));

            if is_lose {
                return player_score;
            }

            return player_score + 3;
        })
        .collect();

    println!("Result: {:?}", x.iter().sum::<usize>());
    return Ok(());
}
