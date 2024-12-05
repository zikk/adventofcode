use std::fs::read_to_string;

const IS_PROD: bool = true;

const TEST_INPUT_PATH: &str = "./inputs/day4.test";
const PROD_INPUT_PATH: &str = "./inputs/day4.in";
const INPUT_PATH: &str = if IS_PROD {
    PROD_INPUT_PATH
} else {
    TEST_INPUT_PATH
};

fn horizontal_search(input: &Vec<Vec<char>>) -> usize {
    let mut result = 0;

    input.iter().for_each(|line| {
        line.windows(4).for_each(|x| {
            let mut v = String::from("");
            x.iter().for_each(|c| v.push(*c));
            if v == "XMAS" || v == "SAMX" {
                result += 1;
            }
        });
    });

    return result;
}

fn vertical_search(input: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    let size = input.len();

    for y in 0..size {
        let mut line: Vec<char> = vec![];

        for x in 0..size {
            line.push(input[x][y]);
        }

        line.windows(4).for_each(|x| {
            let mut v = String::from("");
            x.iter().for_each(|c| v.push(*c));
            if v == "XMAS" || v == "SAMX" {
                result += 1;
            }
        });
    }

    return result;
}

fn diagonal_search_ltr(input: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    let size = input.len();

    for anchor in 0..size {
        let mut first_line: Vec<char> = vec![];
        let mut second_line: Vec<char> = vec![];

        for index in 0..size - anchor {
            first_line.push(input[anchor + index][index]);
            if anchor > 0 {
                second_line.push(input[index][anchor + index]);
            }
        }

        if first_line.len() >= 4 {
            first_line.windows(4).for_each(|x| {
                let mut v = String::from("");
                x.iter().for_each(|c| v.push(*c));
                if v == "XMAS" || v == "SAMX" {
                    result += 1;
                }
            });
        }

        if second_line.len() >= 4 {
            second_line.windows(4).for_each(|x| {
                let mut v = String::from("");
                x.iter().for_each(|c| v.push(*c));
                if v == "XMAS" || v == "SAMX" {
                    result += 1;
                }
            });
        }
    }

    return result;
}

fn diagonal_search_rtl(input: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    let size = input.len();
    let last_index = size - 1;

    for anchor in (0..size).rev() {
        let mut first_line: Vec<char> = vec![];
        let mut second_line: Vec<char> = vec![];

        for index in 0..size - (last_index - anchor) {
            first_line.push(input[index][anchor - index]);

            if anchor < last_index {
                let y = last_index - anchor;
                second_line.push(input[y + index][last_index - index]);
            }
        }

        if first_line.len() >= 4 {
            first_line.windows(4).for_each(|x| {
                let mut v = String::from("");
                x.iter().for_each(|c| v.push(*c));
                if v == "XMAS" || v == "SAMX" {
                    result += 1;
                }
            });
        }

        if second_line.len() >= 4 {
            second_line.windows(4).for_each(|x| {
                let mut v = String::from("");
                x.iter().for_each(|c| v.push(*c));
                if v == "XMAS" || v == "SAMX" {
                    result += 1;
                }
            });
        }
    }

    return result;
}

fn main() {
    let input = read_to_string(INPUT_PATH)
        .unwrap()
        .trim()
        .split("\n")
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut result = 0;

    result += horizontal_search(&input);
    result += vertical_search(&input);
    result += diagonal_search_ltr(&input);
    result += diagonal_search_rtl(&input);

    println!("Result : {}", result);
}
