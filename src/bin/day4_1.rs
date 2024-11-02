use std::fs::read_to_string;

const IS_PROD: bool = true;

const TEST_INPUT_PATH: &str = "./inputs/day4.test.in";
const PROD_INPUT_PATH: &str = "./inputs/day4.prod.in";
const INPUT_PATH: &str = if IS_PROD {
    PROD_INPUT_PATH
} else {
    TEST_INPUT_PATH
};

fn main() {
    let mut result = 0;

    read_to_string(INPUT_PATH)
        .unwrap()
        .trim_end()
        .split("\n")
        .for_each(|v| {
            let mut game_points = 0;

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

            winning_numbers.into_iter().for_each(|n| {
                if player_numbers.contains(&n) {
                    game_points = if game_points == 0 { 1 } else { game_points * 2 }
                }
            });

            result += game_points;
        });

    println!("Result : {}", result);
}
