use std::fs::read_to_string;

const IS_PROD: bool = true;

const TEST_INPUT_PATH: &str = "./inputs/day4.test";
const PROD_INPUT_PATH: &str = "./inputs/day4.in";
const INPUT_PATH: &str = if IS_PROD {
    PROD_INPUT_PATH
} else {
    TEST_INPUT_PATH
};

fn main() {
    let input = read_to_string(INPUT_PATH)
        .unwrap()
        .trim()
        .split("\n")
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut result = 0;

    let size = input.len();
    for y in 1..size - 1 {
        for x in 1..size - 1 {
            if input[y][x] == 'A' {
                let first_line = vec![input[y - 1][x - 1], input[y][x], input[y + 1][x + 1]]
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .concat();
                let second_line = vec![input[y - 1][x + 1], input[y][x], input[y + 1][x - 1]]
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .concat();

                if (first_line == "MAS" || first_line == "SAM")
                    && (second_line == "MAS" || second_line == "SAM")
                {
                    result += 1;
                }
            }
        }
    }

    println!("Result : {}", result);
}
