use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let file = std::fs::read_to_string("./inputs/day5.prod").expect("Unable to read input file");
    let number_of_lanes = (file.lines().next().unwrap().len() + 1) / 4;
    let crates_str = file.split("\n\n").next().unwrap();

    let mut lanes: Vec<Vec<Option<char>>> = vec![vec![]; number_of_lanes];

    for line in crates_str.lines().rev().skip(1) {
        let chars = line.chars().collect::<Vec<char>>();

        for (index, chars_chunk) in chars.chunks(4).enumerate() {
            let c = chars_chunk.get(1).copied();
            if c != Some(' ') {
                lanes[index].push(chars_chunk.get(1).copied());
            }
        }
    }

    let instructions = file.split("\n\n").last().unwrap();

    for instruction in instructions.lines() {
        let mut instruction_digits = vec![];
        let nums_regex = Regex::new(r"(\d+)").unwrap();

        for d in nums_regex.captures_iter(instruction) {
            let dig = d.get(1).unwrap().as_str().parse::<usize>().unwrap();
            instruction_digits.push(dig)
        }

        let lane_idx = instruction_digits[1] - 1;
        let lane_len = lanes[lane_idx].len();
        let to_move = lanes[lane_idx]
            .drain((lane_len - instruction_digits[0])..)
            .collect::<Vec<_>>();
        let next_lane_idx = instruction_digits[2] - 1;
        lanes[next_lane_idx].append(&mut to_move.clone().iter().cloned().collect());
    }

    let result: Vec<String> = lanes
        .into_iter()
        .map(|x| {
            return x
                .last()
                .expect("Value should exist")
                .expect("Value should exist")
                .to_string();
        })
        .collect();

    println!("Result : {:?}", result.join(""));
    return Ok(());
}
