use std::{collections::HashMap, fs::read_to_string};

const IS_PROD: bool = true;

const TEST_INPUT_PATH: &str = "./inputs/day8.test";
const PROD_INPUT_PATH: &str = "./inputs/day8.in";
const INPUT_PATH: &str = if IS_PROD {
    PROD_INPUT_PATH
} else {
    TEST_INPUT_PATH
};

#[derive(Debug, Clone, Copy)]
struct Pos {
    y: isize,
    x: isize,
}

fn main() {
    let mut antennas: HashMap<char, Vec<Pos>> = HashMap::new();

    let mut input = read_to_string(INPUT_PATH)
        .unwrap()
        .trim()
        .split("\n")
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let size = input.len();

    for y in 0..size {
        for x in 0..size {
            let v = input[y][x];
            if v != '.' {
                let pos = Pos {
                    y: y as isize,
                    x: x as isize,
                };

                if let Some(list) = antennas.get_mut(&v) {
                    list.push(pos);
                } else {
                    antennas.insert(v, vec![pos]);
                }
            }
        }
    }

    for (_, v) in antennas.iter() {
        for i in 0..v.len() {
            let anchor = v[i];

            for j in 0..v.len() {
                if i == j {
                    continue;
                }

                let comp = v[j];

                let antinode_pos = Pos {
                    y: anchor.y + anchor.y - comp.y,
                    x: anchor.x + anchor.x - comp.x,
                };

                if antinode_pos.x >= size as isize
                    || antinode_pos.y >= size as isize
                    || antinode_pos.x < 0
                    || antinode_pos.y < 0
                {
                    continue;
                }

                input[antinode_pos.y as usize][antinode_pos.x as usize] = '#';
            }
        }
    }

    let mut result = 0;

    input.iter().for_each(|l| {
        l.iter().for_each(|c| {
            if *c == '#' {
                result += 1;
            }
        });
    });

    println!("Result : {}", result);
}
