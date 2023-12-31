use clap::Parser;
use fancy_regex::Regex;
use std::fs::read_to_string;

#[derive(Parser)]
struct Args {
    #[arg(short = 'e', long = "env", value_name = "env", default_value = "prod")]
    env: String,
}

#[derive(Debug)]
struct DigitWithPos {
    value: isize,
    pos: isize,
}

fn main() {
    let args = Args::parse();
    let input_filename = match args.env.as_str() {
        "test" => "./inputs/day1_2.test.in",
        "prod" => "./inputs/day1.in",
        _ => panic!("No env value provided"),
    };

    let mut total = 0;
    let digits = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let first_num_re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)")
        .expect("first_num_re regex to have been created correctly");
    let last_num_re = Regex::new(
        r"(one|two|three|four|five|six|seven|eight|nine)(?!.*(one|two|three|four|five|six|seven|eight|nine))",
    )
    .expect("last_num_re regex to have been created correctly");

    read_to_string(input_filename)
        .expect("input file to be read")
        .trim_end()
        .split("\n")
        .for_each(|line| {
            let mut first_digit: DigitWithPos = DigitWithPos { value: -1, pos: -1 };
            let mut last_digit: DigitWithPos = DigitWithPos { value: -1, pos: -1 };

            line.split("")
                .skip(1)
                .into_iter()
                .enumerate()
                .for_each(|(i, c)| {
                    let d = match c.parse::<isize>() {
                        Ok(d) => Some(d),
                        Err(_) => None,
                    };

                    if d.is_none() {
                        return;
                    }

                    let value = d.expect("d to have value");

                    if first_digit.value == -1 {
                        first_digit = DigitWithPos {
                            value: value as isize,
                            pos: i as isize,
                        }
                    } else {
                        last_digit = DigitWithPos {
                            value: value as isize,
                            pos: i as isize,
                        }
                    }
                });

            if last_digit.value == -1 {
                last_digit = DigitWithPos {
                    value: first_digit.value,
                    pos: first_digit.pos,
                };
            }

            let first_match = first_num_re
                .find(line)
                .expect("Error running first_num_re regex");
            let last_match = last_num_re
                .find(line)
                .expect("Error running last_num_re regex");

            if first_match.is_some() {
                let first_occurence = first_match.expect("first_match to have value");
                let first_occurent_pos = first_occurence.start() as isize;
                let value = digits
                    .iter()
                    .position(|&d| d == first_occurence.as_str())
                    .expect("digit to be in vector") as isize
                    + 1;

                if first_digit.pos == -1 || first_occurent_pos < first_digit.pos {
                    first_digit = DigitWithPos {
                        value: value as isize,
                        pos: first_occurent_pos,
                    }
                }

                if last_digit.pos == -1 || first_occurent_pos > last_digit.pos {
                    last_digit = DigitWithPos {
                        value: value as isize,
                        pos: first_occurent_pos,
                    }
                }
            }

            if last_match.is_some() {
                let last_occurence = last_match.expect("first_match to have value");
                let last_occurence_pos = last_occurence.start() as isize;
                let value = digits
                    .iter()
                    .position(|&d| d == last_occurence.as_str())
                    .expect("digit to be in vector") as isize
                    + 1;

                if first_digit.pos == -1 || last_occurence_pos < first_digit.pos {
                    first_digit = DigitWithPos {
                        value: value as isize,
                        pos: last_occurence_pos,
                    }
                }

                if last_digit.pos == -1 || last_occurence_pos > last_digit.pos {
                    last_digit = DigitWithPos {
                        value: value as isize,
                        pos: last_occurence_pos,
                    }
                }
            }

            let line_total = match (first_digit.value.to_string() + &last_digit.value.to_string())
                .parse::<isize>()
            {
                Ok(line_total) => line_total,
                Err(_) => panic!("Failed to parse string to number"),
            };

            total += line_total
        });

    println!("{}", total);
}
