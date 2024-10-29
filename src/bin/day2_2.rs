use std::fs::read_to_string;

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
        .expect("To read input file to string")
        .trim_end()
        .split("\n")
        .for_each(|line| {
            let game: Vec<&str> = line.split(":").collect();
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            game.get(1)
                .expect("To get the game rounds")
                .split(";")
                .for_each(|round| {
                    round.split(",").for_each(|set| {
                        let split_set = set.split(" ").skip(1).collect::<Vec<&str>>();
                        let value = split_set
                            .get(0)
                            .expect("Value to be extracted")
                            .parse::<u16>()
                            .expect("Value to be parse to integer");
                        let color = *split_set.get(1).expect("Color to be extracted");

                        match color {
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

            if max_red > 0 && max_green > 0 && max_blue > 0 {
                result += max_red * max_green * max_blue;
            }
        });

    println!("Result : {:?}", result);
}
