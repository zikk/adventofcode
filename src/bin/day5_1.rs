use std::fs::read_to_string;

const IS_PROD: bool = true;

const TEST_INPUT_PATH: &str = "./inputs/day5.test.in";
const PROD_INPUT_PATH: &str = "./inputs/day5.prod.in";
const INPUT_PATH: &str = if IS_PROD {
    PROD_INPUT_PATH
} else {
    TEST_INPUT_PATH
};

#[derive(Debug)]
struct PositionMap {
    source: usize,
    destination: usize,
    range: usize,
}

fn main() {
    let mut seeds: Vec<usize> = vec![];
    let mut map_groups: Vec<Vec<PositionMap>> = vec![];

    read_to_string(INPUT_PATH)
        .unwrap()
        .trim()
        .split("\n\n")
        .enumerate()
        .for_each(|(i, v)| {
            if i == 0 {
                seeds = v
                    .to_string()
                    .split(":")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .get(1)
                    .unwrap()
                    .trim()
                    .split(" ")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                return;
            }

            let maps = v
                .split("\n")
                .skip(1)
                .map(|l| {
                    let range_values = l
                        .trim()
                        .split(" ")
                        .map(|v| v.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();

                    return PositionMap {
                        destination: range_values.get(0).unwrap().clone(),
                        source: range_values.get(1).unwrap().clone(),
                        range: range_values.get(2).unwrap().clone(),
                    };
                })
                .collect::<Vec<PositionMap>>();

            map_groups.push(maps);
        });

    let seed_results = seeds
        .iter()
        .map(|seed| {
            let mut destination: usize = *seed;

            map_groups.iter().for_each(|group| {
                let map_option = group.iter().find(|map| {
                    let source_start = map.source;
                    let source_end = map.source + map.range - 1;

                    if destination < source_start || destination > source_end {
                        return false;
                    }

                    return true;
                });

                if map_option.is_some() {
                    let map = map_option.unwrap();
                    let destination_start = map.destination;
                    let destination_end = map.destination + map.range - 1;
                    let distance = destination - map.source;

                    destination = if map.destination + distance <= destination_end {
                        destination_start + distance
                    } else {
                        destination
                    };
                }
            });

            return destination;
        })
        .collect::<Vec<usize>>();

    println!("{:?}", seed_results.iter().min().unwrap());
}
