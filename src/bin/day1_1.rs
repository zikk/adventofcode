use std::fs::read_to_string;

const IS_PROD: bool = true;

const TEST_INPUT_PATH: &str = "./inputs/day1.test.in";
const PROD_INPUT_PATH: &str = "./inputs/day1.prod.in";
const INPUT_PATH: &str = if IS_PROD {
    PROD_INPUT_PATH
} else {
    TEST_INPUT_PATH
};

fn main() {
    let mut a: Vec<usize> = vec![];
    let mut b: Vec<usize> = vec![];

    read_to_string(INPUT_PATH)
        .unwrap()
        .trim()
        .split("\n")
        .for_each(|l| {
            let values = l
                .split("   ")
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            a.push(*values.get(0).unwrap());
            b.push(*values.get(1).unwrap());
        });

    a.sort();
    b.sort();

    let mut result = 0;

    for n in 0..a.len() {
        let va = a.get(n).unwrap();
        let vb = b.get(n).unwrap();

        result += va.abs_diff(*vb);
    }

    println!("Result : {:?}", result);
}
