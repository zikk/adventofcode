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
        let mut is_safe = true;
        let mut is_inc = false;

        for i in 0..level.len() - 1 {
            let a = level.get(i).unwrap();
            let b = level.get(i + 1).unwrap();
            let diff = a.abs_diff(*b);

            if i == 0 {
                is_inc = a < b;
            }

            if diff < 1 || diff > 3 {
                is_safe = false;
            }

            if i > 0 {
                if is_inc && a > b {
                    is_safe = false
                }

                if !is_inc && a < b {
                    is_safe = false;
                }
            }
        }

        if is_safe {
            result += 1;
        }
    });

    println!("Result : {}", result);
}
