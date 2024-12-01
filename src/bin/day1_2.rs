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

    let mut result = 0;
    let vec_len = a.len();

    for n in 0..vec_len {
        let va = a.get(n).unwrap();
        let mut repeats = 0;

        for x in 0..vec_len {
            let vb = b.get(x).unwrap();
            if va == vb {
                repeats += 1;
            }
        }

        result += va * repeats;
    }

    println!("Result : {:?}", result);
}
