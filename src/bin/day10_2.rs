use std::{cell::RefCell, fs::read_to_string, rc::Rc};
const INPUT_PATH: &str = "./inputs/day10.in";

#[derive(Debug)]
struct Node {
    value: usize,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn add_child(&self, child: Rc<Node>) {
        self.children.borrow_mut().push(child);
    }

    fn count_paths(&self) -> usize {
        let children = self.children.borrow();

        if children.len() == 0 {
            if self.value == 9 {
                return 1;
            } else {
                return 0;
            }
        }

        let mut result = 0;

        children.iter().for_each(|c| {
            result += c.count_paths();
        });

        return result;
    }
}

fn main() {
    let input = read_to_string(INPUT_PATH)
        .unwrap()
        .trim()
        .split("\n")
        .map(|l| {
            l.chars()
                .map(|c| {
                    Rc::new(Node {
                        value: c.to_string().parse::<usize>().unwrap(),
                        children: RefCell::new(vec![]),
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let size = input.len();
    let mut starting_points: Vec<Rc<Node>> = vec![];

    for y in 0..size {
        for x in 0..size {
            let node = Rc::clone(&input[y][x]);

            let has_left = x > 0;
            let has_right = x < size - 1;
            let has_top = y > 0;
            let has_bottom = y < size - 1;

            if node.value == 0 {
                starting_points.push(Rc::clone(&node));
            }

            if has_left {
                let left_node = Rc::clone(&input[y][x - 1]);
                let diff = left_node.value as isize - node.value as isize;

                if diff == 1 {
                    node.add_child(left_node);
                }
            }

            if has_right {
                let right_node = Rc::clone(&input[y][x + 1]);
                let diff = right_node.value as isize - node.value as isize;

                if diff == 1 {
                    node.add_child(right_node);
                }
            }

            if has_top {
                let top_node = Rc::clone(&input[y - 1][x]);
                let diff = top_node.value as isize - node.value as isize;

                if diff == 1 {
                    node.add_child(top_node);
                }
            }

            if has_bottom {
                let bottom_node = Rc::clone(&input[y + 1][x]);
                let diff = bottom_node.value as isize - node.value as isize;

                if diff == 1 {
                    node.add_child(bottom_node);
                }
            }
        }
    }

    let result = starting_points
        .iter()
        .map(|node| node.count_paths())
        .reduce(|a, b| a + b)
        .unwrap();

    println!("Result : {}", result);
}
