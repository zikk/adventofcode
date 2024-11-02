use std::{collections::HashMap, fs::read_to_string};

const IS_PROD: bool = true;

const TEST_INPUT_PATH: &str = "./inputs/day4.test.in";
const PROD_INPUT_PATH: &str = "./inputs/day4.prod.in";
const INPUT_PATH: &str = if IS_PROD {
    PROD_INPUT_PATH
} else {
    TEST_INPUT_PATH
};

#[derive(Debug, Clone)]
struct Card {
    winning_numbers: Vec<usize>,
    player_numbers: Vec<usize>,
}

#[derive(Debug, Clone)]
struct CardStack {
    card: Card,
    size: usize,
}

fn main() {
    let mut cards: HashMap<usize, CardStack> = HashMap::new();

    read_to_string(INPUT_PATH)
        .unwrap()
        .trim_end()
        .split("\n")
        .for_each(|v| {
            let card_number = v
                .split(":")
                .collect::<Vec<&str>>()
                .get(0)
                .unwrap()
                .to_string()
                .trim()
                .split(" ")
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let card = v
                .split(":")
                .skip(1)
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .concat()
                .split("|")
                .map(|v| v.to_string().trim().to_string())
                .collect::<Vec<String>>();

            let winning_numbers = card
                .get(0)
                .unwrap()
                .trim()
                .split(" ")
                .filter(|v| v.trim().parse::<usize>().is_ok())
                .map(|v| v.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let player_numbers = card
                .get(1)
                .unwrap()
                .trim()
                .split(" ")
                .filter(|v| v.trim().parse::<usize>().is_ok())
                .map(|v| v.trim().to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let stack = CardStack {
                card: Card {
                    winning_numbers,
                    player_numbers,
                },
                size: 1,
            };

            cards.insert(card_number, stack);
        });

    for i in 1..(cards.len()) {
        let mut game_points = 0;
        let game = cards.get(&i).unwrap().clone();
        let winning_numbers = game.card.winning_numbers;
        let player_numbers = game.card.player_numbers;

        winning_numbers.into_iter().for_each(|n| {
            if player_numbers.contains(&n) {
                game_points += 1;
            }
        });

        for p in i + 1..i + 1 + game_points {
            if let Some(game_to_inc) = cards.get_mut(&p) {
                game_to_inc.size += game.size;
            }
        }
    }

    let mut result = 0;
    cards.values().for_each(|v| result += v.size);
    println!("Result : {}", result);
}
