use std::fs::read_to_string;

const IS_PROD: bool = true;

const TEST_INPUT_PATH: &str = "./inputs/day7.test";
const PROD_INPUT_PATH: &str = "./inputs/day7.in";
const INPUT_PATH: &str = if IS_PROD {
    PROD_INPUT_PATH
} else {
    TEST_INPUT_PATH
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Add,
    Mult,
}

fn calc_operators(size: usize) -> Vec<Vec<Operator>> {
    let mut operators_combinasions: Vec<Vec<Operator>> = vec![];

    let possitibitiles_count = (2 as usize).pow(size as u32);
    operators_combinasions.push(vec![Operator::Add; size]);

    for y in 1..possitibitiles_count {
        let mut next = operators_combinasions[y - 1].clone();

        for x in 0..size {
            if next[x] == Operator::Add {
                next[x] = Operator::Mult;
                break;
            } else {
                next[x] = Operator::Add;
            }
        }

        operators_combinasions.push(next);
    }

    return operators_combinasions;
}

fn main() {
    let operations = read_to_string(INPUT_PATH)
        .unwrap()
        .trim()
        .split("\n")
        .map(|l| {
            let operation = l.split(":").collect::<Vec<_>>();
            let result = operation.get(0).unwrap().trim().parse::<usize>().unwrap();
            let operands = operation
                .get(1)
                .unwrap()
                .trim()
                .split(" ")
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            return (result, operands);
        })
        .collect::<Vec<_>>();

    let possible_operations = operations
        .iter()
        .filter(|operation| {
            let (result, operands) = operation;
            let operators_list = calc_operators(operands.len() - 1);

            return operators_list.iter().any(|operators| {
                let mut r = 0;
                operands.iter().enumerate().for_each(|(i, operand)| {
                    if i == 0 {
                        r = *operand;
                    } else {
                        let operator = operators[i - 1];
                        if operator == Operator::Add {
                            r += operand;
                        } else {
                            r *= operand;
                        }
                    }
                });
                return *result == r;
            });
        })
        .collect::<Vec<_>>();

    let mut result = 0;
    possible_operations.iter().for_each(|op| result += op.0);
    println!("Result : {}", result);
}
