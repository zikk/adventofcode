use std::fs::read_to_string;

use anyhow::Result;

fn main() -> Result<()> {
    let game_hands = vec!['A', 'B', 'C'];
    let wins = vec![['A', 'B'], ['B', 'C'], ['C', 'A']];
    let loses = vec![['A', 'C'], ['B', 'A'], ['C', 'B']];

    let mut player_score = 0;

    read_to_string("./inputs/day2.prod")?
        .lines()
        .into_iter()
        .for_each(|line| {
            let chars: Vec<char> = line
                .split_whitespace()
                .flat_map(|x| x.chars().next())
                .collect();

            match chars.get(1) {
                Some('X') => {
                    let losing_hand = loses.iter().find(|&x| x.get(0) == chars.get(0)).unwrap();
                    let round_value = game_hands
                        .iter()
                        .position(|&x| x == losing_hand[1])
                        .unwrap()
                        + 1;
                    player_score = player_score + round_value + 0;
                }
                Some('Z') => {
                    let winning_hand = wins.iter().find(|&x| x.get(0) == chars.get(0)).unwrap();
                    let round_value = game_hands
                        .iter()
                        .position(|&x| x == winning_hand[1])
                        .unwrap()
                        + 1;
                    player_score = player_score + round_value + 6;
                }
                Some('Y') => {
                    let round_value = game_hands.iter().position(|&x| x == chars[0]).unwrap() + 1;
                    player_score = player_score + round_value + 3;
                }
                _ => (),
            }
        });

    println!("Result: {:?}", player_score);
    return Ok(());
}
