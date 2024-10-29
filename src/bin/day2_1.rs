use std::fs::read_to_string;

const MAX_RED: u16 = 12;
const MAX_GREEN: u16 = 13;
const MAX_BLUE: u16 = 14;

const IS_PROD: bool = true;

const TEST_INPUT_PATH: &str = "./inputs/day2.test.in";
const PROD_INPUT_PATH: &str = "./inputs/day2.prod.in";
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
        .for_each(|line| {
            let game: Vec<&str> = line.split(":").collect();
            let game_number = game
                .get(0)
                .unwrap()
                .split(" ")
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap()
                .parse::<u16>()
                .unwrap();

            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            game.get(1).unwrap().split(";").for_each(|round| {
                round.split(",").for_each(|set| {
                    let split_set = set.split(" ").skip(1).collect::<Vec<&str>>();
                    let value = split_set.get(0).unwrap().parse::<u16>().unwrap();
                    match *split_set.get(1).unwrap() {
                        "red" => {
                            if value >= max_red {
                                max_red = value;
                            }
                        }
                        "green" => {
                            if value >= max_green {
                                max_green = value;
                            }
                        }
                        "blue" => {
                            if value >= max_blue {
                                max_blue = value;
                            }
                        }
                        _ => (),
                    }
                });
            });

            if max_red <= MAX_RED && max_green <= MAX_GREEN && max_blue <= MAX_BLUE {
                result += game_number;
            }
        });

    println!("Result : {:?}", result);
}
