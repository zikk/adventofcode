use clap::Parser;
use std::fs::read_to_string;

#[derive(Parser)]
struct Args {
    #[arg(short = 'e', long = "env", value_name = "env", default_value = "prod")]
    env: String,
}

fn main() {
    let args = Args::parse();
    let input_filename = match args.env.as_str() {
        "test" => "./inputs/day1_2.test.in",
        "prod" => "./inputs/day1.in",
        _ => panic!("No env value provided"),
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
                    Ok(d) => Some(d),
                    Err(_) => None,
                };

                if d.is_none() {
                    return;
                }

                let value = d.unwrap();

                if first_digit == -1 {
                    first_digit = value
                } else {
                    last_digit = value
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
