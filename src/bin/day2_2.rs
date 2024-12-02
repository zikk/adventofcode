use std::fs::read_to_string;

const IS_PROD: bool = true;

const TEST_INPUT_PATH: &str = "./inputs/day2.test.in";
const PROD_INPUT_PATH: &str = "./inputs/day2.prod.in";
const INPUT_PATH: &str = if IS_PROD {
    PROD_INPUT_PATH
} else {
    TEST_INPUT_PATH
};

fn is_level_safe(values: &Vec<usize>) -> bool {
    let mut is_inc = false;
    let mut is_safe = true;

    for i in 0..values.len() - 1 {
        if !is_safe {
            continue;
        }

        let a = values.get(i).unwrap();
        let b = values.get(i + 1).unwrap();
        let diff = a.abs_diff(*b);

        if i == 0 {
            is_inc = a < b;
        }

        if diff < 1 || diff > 3 {
            is_safe = false;
        }

        if i > 0 {
            if is_inc && a > b || !is_inc && a < b {
                is_safe = false;
            }
        }
    }

    return is_safe;
}

fn is_level_safe_deep(values: &Vec<usize>) -> bool {
    if is_level_safe(values) {
        return true;
    }

    for i in 0..values.len() {
        let truncated = values
            .iter()
            .enumerate()
            .filter(|(idx, _)| {
                return *idx != i;
            })
            .map(|(_, v)| {
                return *v;
            })
            .collect::<Vec<usize>>();

        if is_level_safe(&truncated) {
            return true;
        }
    }

    return false;
}

fn main() {
    let levels = read_to_string(INPUT_PATH)
        .unwrap()
        .trim()
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| {
            return line
                .split(" ")
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
        })
        .collect::<Vec<_>>();

    let mut result = 0;

    levels.iter().for_each(|level| {
        if is_level_safe_deep(level) {
            result += 1;
        }
    });

    println!("Result : {}", result);
}
