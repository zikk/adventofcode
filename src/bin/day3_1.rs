use std::fs::read_to_string;

const IS_PROD: bool = true;

const TEST_INPUT_PATH: &str = "./inputs/day3.test.in";
const PROD_INPUT_PATH: &str = "./inputs/day3.prod.in";
const INPUT_PATH: &str = if IS_PROD {
    PROD_INPUT_PATH
} else {
    TEST_INPUT_PATH
};

fn is_character_a_symbol(value: &char) -> bool {
    let value_is_number = value.to_string().parse::<usize>().is_ok();
    return !value_is_number && *value != '.';
}

fn main() {
    let input = read_to_string(INPUT_PATH)
        .expect("To read input file")
        .trim_end()
        .split("\n")
        .map(|line| {
            return line.chars().collect::<Vec<char>>();
        })
        .collect::<Vec<Vec<char>>>();

    let mut result = 0;

    let max_x = input.get(0).unwrap().len() - 1;
    let max_y = input.len() - 1;

    for (y, line) in input.clone().into_iter().enumerate() {
        let mut digits: Vec<char> = vec![];
        let mut number_has_adjacent_symbol = false;

        for (x, value) in line.into_iter().enumerate() {
            let is_value_digit = value.to_string().parse::<usize>().is_ok();

            if is_value_digit {
                digits.push(value);

                let has_top = y > 0;
                let has_bottom = y < max_y;
                let has_left = x > 0;
                let has_right = x < max_x;
                let has_top_left = has_top && has_left;
                let has_top_right = has_top && has_right;
                let has_bottom_left = has_bottom && has_left;
                let has_bottom_right = has_bottom && has_right;

                if has_top {
                    let value = input.get(y - 1).unwrap().get(x).unwrap();
                    if is_character_a_symbol(value) {
                        number_has_adjacent_symbol = true;
                    }
                }

                if has_bottom {
                    let value = input.get(y + 1).unwrap().get(x).unwrap();
                    if is_character_a_symbol(value) {
                        number_has_adjacent_symbol = true;
                    }
                }

                if has_right {
                    let value = input.get(y).unwrap().get(x + 1).unwrap();
                    if is_character_a_symbol(value) {
                        number_has_adjacent_symbol = true;
                    }
                }

                if has_left {
                    let value = input.get(y).unwrap().get(x - 1).unwrap();
                    if is_character_a_symbol(value) {
                        number_has_adjacent_symbol = true
                    }
                }

                if has_top_left {
                    let value = input.get(y - 1).unwrap().get(x - 1).unwrap();
                    if is_character_a_symbol(value) {
                        number_has_adjacent_symbol = true;
                    }
                }

                if has_top_right {
                    let value = input.get(y - 1).unwrap().get(x + 1).unwrap();
                    if is_character_a_symbol(value) {
                        number_has_adjacent_symbol = true;
                    }
                }

                if has_bottom_left {
                    let value = input.get(y + 1).unwrap().get(x - 1).unwrap();
                    if is_character_a_symbol(value) {
                        number_has_adjacent_symbol = true
                    }
                }

                if has_bottom_right {
                    let value = input.get(y + 1).unwrap().get(x + 1).unwrap();
                    if is_character_a_symbol(value) {
                        number_has_adjacent_symbol = true;
                    }
                }
            }

            if !is_value_digit || x == max_x {
                let digits_len = digits.len();
                if digits_len > 0 {
                    let number_as_str = digits
                        .clone()
                        .into_iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<String>>()
                        .concat()
                        .parse::<usize>()
                        .unwrap();

                    digits = vec![];

                    if number_has_adjacent_symbol {
                        result += number_as_str;
                    }

                    number_has_adjacent_symbol = false;
                }
            }
        }
    }

    println!("Result {:?}", result)
}
