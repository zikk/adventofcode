use std::fs::read_to_string;

fn main() {
    let mut input = read_to_string("./inputs/day11.in")
        .unwrap()
        .trim()
        .split(" ")
        .map(|v| v.to_string().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..25 {
        let mut intermedia_result: Vec<usize> = vec![];

        input.iter().for_each(|v| {
            let value_as_chars = v.to_string().chars().collect::<Vec<_>>();

            if *v == 0 {
                intermedia_result.push(1);
            } else if value_as_chars.len() % 2 == 0 {
                let (start, end) = value_as_chars.split_at(value_as_chars.len() / 2);
                let start_i = start
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .concat()
                    .parse::<usize>()
                    .unwrap();
                let end_i = end
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .concat()
                    .parse::<usize>()
                    .unwrap();

                intermedia_result.push(start_i);
                intermedia_result.push(end_i);
            } else {
                intermedia_result.push(v * 2024);
            }
        });

        input = intermedia_result;
    }

    println!("Result : {}", input.len());
}
