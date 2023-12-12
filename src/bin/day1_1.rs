use clap::Parser;
use std::fs::read_to_string;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Args {
    #[arg(short = 'e', long = "env", value_name = "env", default_value = "prod")]
    env: String,
}

fn main() {
    let args = Args::parse();
    let input_filename = if args.env == "prod" {
        "./inputs/day1.in"
    } else {
        "./inputs/day1_1.test.in"
    };

    let mut total = 0;

    read_to_string(input_filename)
        .unwrap()
        .trim_end()
        .split("\n")
        .for_each(|line| {
            let mut first_digit: isize = -1;
            let mut last_digit: isize = -1;

            line.split("").for_each(|c| {
                let d = match c.parse::<isize>() {
                    Ok(d) => d,
                    Err(_) => -1,
                };

                if d == -1 {
                    return;
                }

                if first_digit == -1 {
                    first_digit = d
                } else {
                    last_digit = d
                }
            });

            if last_digit == -1 {
                last_digit = first_digit;
            }

            let line_total =
                match (first_digit.to_string() + &last_digit.to_string()).parse::<isize>() {
                    Ok(line_total) => line_total,
                    Err(_) => panic!("Failed to parse string to number"),
                };

            total += line_total
        });

    println!("{}", total);
}
