use std::fs::read_to_string;

const IS_PROD: bool = true;

const TEST_INPUT_PATH: &str = "./inputs/day5.test";
const PROD_INPUT_PATH: &str = "./inputs/day5.in";
const INPUT_PATH: &str = if IS_PROD {
    PROD_INPUT_PATH
} else {
    TEST_INPUT_PATH
};

fn main() {
    let input = read_to_string(INPUT_PATH)
        .unwrap()
        .trim()
        .split("\n\n")
        .map(|x| x.to_string())
        .collect::<Vec<_>>();

    let rules = input
        .get(0)
        .unwrap()
        .split("\n")
        .map(|v| {
            let x = v.split("|").take(2).collect::<Vec<_>>();
            return (
                x[0].parse::<usize>().unwrap(),
                x[1].parse::<usize>().unwrap(),
            );
        })
        .collect::<Vec<_>>();

    let game = input.get(1).unwrap().split("\n").collect::<Vec<_>>();
    let mut result = 0;

    game.iter().for_each(|set| {
        let values = set
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let is_in_order = values.windows(2).all(|x| {
            let a = x[0];
            let b = x[1];

            return rules.iter().any(|(v1, v2)| {
                return *v1 == a && *v2 == b;
            });
        });

        if is_in_order {
            let index = values.len() / 2;
            result += values[index];
        }
    });

    println!("Result : {}", result);
}
