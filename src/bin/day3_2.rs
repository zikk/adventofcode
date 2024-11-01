use std::{collections::HashMap, fs::read_to_string};

const IS_PROD: bool = true;

const TEST_INPUT_PATH: &str = "./inputs/day3.test.in";
const PROD_INPUT_PATH: &str = "./inputs/day3.prod.in";
const INPUT_PATH: &str = if IS_PROD {
    PROD_INPUT_PATH
} else {
    TEST_INPUT_PATH
};

struct Idx {
    y: usize,
    x: usize,
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
    let mut gears: HashMap<String, Vec<usize>> = HashMap::new();

    for (y, line) in input.clone().into_iter().enumerate() {
        let mut digits: Vec<char> = vec![];
        let mut number_has_adjacent_asterisk = false;
        let mut asterisk_idx = Idx { y: 0, x: 0 };

        for (x, value) in line.into_iter().enumerate() {
            let is_value_digit = value.to_string().parse::<isize>().is_ok();

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
                    if *value == '*' {
                        number_has_adjacent_asterisk = true;
                        asterisk_idx = Idx { y: y - 1, x };
                    }
                }

                if has_bottom {
                    let value = input.get(y + 1).unwrap().get(x).unwrap();
                    if *value == '*' {
                        number_has_adjacent_asterisk = true;
                        asterisk_idx = Idx { y: y + 1, x };
                    }
                }

                if has_right {
                    let value = input.get(y).unwrap().get(x + 1).unwrap();
                    if *value == '*' {
                        number_has_adjacent_asterisk = true;
                        asterisk_idx = Idx { y, x: x + 1 };
                    }
                }

                if has_left {
                    let value = input.get(y).unwrap().get(x - 1).unwrap();
                    if *value == '*' {
                        number_has_adjacent_asterisk = true;
                        asterisk_idx = Idx { y, x: x - 1 };
                    }
                }

                if has_top_left {
                    let value = input.get(y - 1).unwrap().get(x - 1).unwrap();
                    if *value == '*' {
                        number_has_adjacent_asterisk = true;
                        asterisk_idx = Idx { y: y - 1, x: x - 1 };
                    }
                }

                if has_top_right {
                    let value = input.get(y - 1).unwrap().get(x + 1).unwrap();
                    if *value == '*' {
                        number_has_adjacent_asterisk = true;
                        asterisk_idx = Idx { y: y - 1, x: x + 1 };
                    }
                }

                if has_bottom_left {
                    let value = input.get(y + 1).unwrap().get(x - 1).unwrap();
                    if *value == '*' {
                        number_has_adjacent_asterisk = true;
                        asterisk_idx = Idx { y: y + 1, x: x - 1 };
                    }
                }

                if has_bottom_right {
                    let value = input.get(y + 1).unwrap().get(x + 1).unwrap();
                    if *value == '*' {
                        number_has_adjacent_asterisk = true;
                        asterisk_idx = Idx { y: y + 1, x: x + 1 };
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

                    if number_has_adjacent_asterisk {
                        let index = format!("{},{}", asterisk_idx.y, asterisk_idx.x);
                        let gear = gears.entry(index).or_insert(vec![]);
                        gear.push(number_as_str);
                        number_has_adjacent_asterisk = false;
                    }
                }
            }
        }
    }

    for (_, values) in gears {
        if values.len() == 2 {
            result += values.get(0).unwrap() * values.get(1).unwrap();
        }
    }

    println!("Result {:?}", result)
}
