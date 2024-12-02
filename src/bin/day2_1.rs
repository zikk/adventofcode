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
    read_to_string(INPUT_PATH).unwrap().trim().split("\n");
}
