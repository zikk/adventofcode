use std::fs::read_to_string;

const IS_PROD: bool = true;

const TEST_INPUT_PATH: &str = "./inputs/day9.test";
const PROD_INPUT_PATH: &str = "./inputs/day9.in";
const INPUT_PATH: &str = if IS_PROD {
    PROD_INPUT_PATH
} else {
    TEST_INPUT_PATH
};

fn main() {
    let input = read_to_string(INPUT_PATH)
        .expect("File input to be read")
        .trim()
        .chars()
        .map(|c| {
            return c
                .to_string()
                .parse::<usize>()
                .expect("Character to be parsed to usize");
        })
        .collect::<Vec<_>>();

    let mut disk: Vec<Option<usize>> = vec![];

    let mut id = 0;

    for i in 0..input.len() {
        let value = input[i];
        let is_space = i % 2 == 1;

        for _ in 0..value {
            if is_space {
                disk.push(None);
            } else {
                disk.push(Some(id));
            }
        }

        if is_space {
            id += 1;
        }
    }

    for i in (0..disk.len()).rev() {
        if disk[i].is_some() {
            let (idx, _) = disk.iter().enumerate().find(|(_, v)| v.is_none()).unwrap();

            if idx > i {
                break;
            }

            (disk[i], disk[idx]) = (disk[idx], disk[i]);
        }
    }

    let mut result = 0;
    disk.iter()
        .filter(|v| v.is_some())
        .enumerate()
        .for_each(|(i, v)| {
            result += i * v.unwrap();
        });

    println!("{:?}", result);
}
