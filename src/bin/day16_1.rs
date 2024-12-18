use std::fs;
const INPUT_PATH: &str = "./inputs/day16.in";

fn main() {
    let content = fs::read_to_string(INPUT_PATH).unwrap();

    let input = content.split("\n\n").collect::<Vec<_>>();

    let mut registers = input[0]
        .split("\n")
        .map(|l| {
            l.split(":").collect::<Vec<_>>()[1]
                .trim()
                .parse::<usize>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let instructions = input[1].split(":").collect::<Vec<_>>()[1]
        .trim()
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut pos = 0;
    let mut outputs: Vec<usize> = Vec::new();

    loop {
        if pos >= instructions.len() {
            break;
        }

        let op = instructions[pos];
        let literal_operand = instructions[pos + 1];

        let combo_operand = match literal_operand {
            4 => registers[0],
            5 => registers[1],
            6 => registers[2],
            _ => literal_operand,
        };

        match op {
            0 => {
                let divider = (2 as usize).pow(combo_operand as u32);
                registers[0] = registers[0] / divider;
            }
            1 => {
                registers[1] = registers[1] ^ literal_operand;
            }
            2 => {
                registers[1] = combo_operand % 8;
            }
            3 => {
                if registers[0] != 0 {
                    pos = literal_operand;
                    continue;
                }
            }
            4 => registers[1] = registers[1] ^ registers[2],
            5 => {
                outputs.push(combo_operand % 8);
            }
            6 => {
                let divider = (2 as usize).pow(combo_operand as u32);
                registers[1] = registers[0] / divider;
            }
            7 => {
                let divider = (2 as usize).pow(combo_operand as u32);
                registers[2] = registers[0] / divider;
            }
            _ => (),
        }

        pos = pos + 2;
    }

    println!(
        "Result : {}",
        outputs
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
}
