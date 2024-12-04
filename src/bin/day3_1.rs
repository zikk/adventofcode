use std::fs::read_to_string;

const IS_PROD: bool = true;

const TEST_INPUT_PATH: &str = "./inputs/day3.test.in";
const PROD_INPUT_PATH: &str = "./inputs/day3.prod.in";
const INPUT_PATH: &str = if IS_PROD {
    PROD_INPUT_PATH
} else {
    TEST_INPUT_PATH
};

struct Scanner {
    input: String,
    index: usize,
    result: usize,
    end: bool,
}

impl Scanner {
    fn seek(&mut self, length: usize) -> String {
        let result = self
            .input
            .chars()
            .skip(self.index)
            .take(length)
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("");

        self.index += length;

        if self.index >= self.input.len() {
            self.end = true;
        }

        return result;
    }

    fn scan_mul(&mut self) {
        let keyword = self.seek(2);

        if keyword != "ul" {
            return;
        }

        let open_paren = self.seek(1);
        if open_paren != "(" {
            return;
        }

        let mut a_digits: Vec<String> = vec![];

        loop {
            let d = self.seek(1);
            let is_digit = self.is_digit(&d);

            if !is_digit && d != "," {
                return;
            }

            if !is_digit {
                break;
            }

            a_digits.push(d);
        }

        let mut b_digits: Vec<String> = vec![];

        loop {
            let d = self.seek(1);
            let is_digit = self.is_digit(&d);

            if !is_digit && d != ")" {
                return;
            }

            if !is_digit {
                break;
            }

            b_digits.push(d);
        }

        let a = a_digits.concat().parse::<usize>().unwrap();
        let b = b_digits.concat().parse::<usize>().unwrap();

        self.result += a * b;
    }

    fn is_digit(&self, d: &String) -> bool {
        return d.parse::<usize>().is_ok();
    }

    fn scan(&mut self) -> usize {
        loop {
            let s = self.seek(1).chars().nth(0).unwrap();

            match s {
                'm' => {
                    self.scan_mul();
                }
                _ => (),
            }

            if self.end {
                break;
            }
        }

        return self.result;
    }

    fn new(input: String) -> Scanner {
        return Scanner {
            input,
            index: 0,
            result: 0,
            end: false,
        };
    }
}

fn main() {
    let input = read_to_string(INPUT_PATH)
        .unwrap()
        .trim()
        .split("\n")
        .collect::<Vec<_>>()
        .join("");

    let mut scanner = Scanner::new(input);
    let result = scanner.scan();

    println!("Result : {}", result);
}
