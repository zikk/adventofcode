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
    let mut disk_values: Vec<usize> = vec![];

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
        } else {
            disk_values.push(id);
        }
    }

    for value_to_move in disk_values.iter().rev() {
        let mut found = false;
        let mut start_idx = 0;
        let mut end_idx = 0;

        for (i, v) in disk.iter().enumerate().rev() {
            if v.is_some() && v.unwrap() == *value_to_move {
                if !found {
                    found = true;
                    end_idx = i;
                    start_idx = i;
                } else {
                    start_idx = i;
                }
            } else {
                if found {
                    break;
                }
            }
        }

        let size = end_idx - start_idx + 1;
        if let Some(values) = disk
            .windows(size)
            .enumerate()
            .find(|(_, vs)| vs.iter().all(|v| v.is_none()))
        {
            let (values_idx, _) = values;

            if values_idx < start_idx {
                for i in 0..size {
                    (disk[values_idx + i], disk[start_idx + i]) =
                        (disk[start_idx + i], disk[values_idx + i]);
                }
            }
        }
    }

    let mut result = 0;
    disk.iter().enumerate().for_each(|(i, v)| {
        if let Some(x) = v {
            result += i * x;
        }
    });

    println!("{:?}", result);
}
