use std::{cell::RefCell, fs::read_to_string, rc::Rc};

#[derive(Debug)]
struct Node {
    value: usize,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn add_child(&self, child: Rc<Node>) {
        self.children.borrow_mut().push(child);
    }

    fn get_paths_ends(node: Rc<Node>) -> Vec<Rc<Node>> {
        let borrowed_node = Rc::clone(&node);
        let children = borrowed_node.children.borrow();
        let mut result = Vec::new();

        if children.len() == 0 && node.value == 9 {
            result.push(node);
        } else {
            children.iter().for_each(|c| {
                result.extend(Node::get_paths_ends(c.clone()));
            });
        }

        return result;
    }

    fn count_ends(node: &Rc<Node>) -> usize {
        let borrowed_node = Rc::clone(&node);
        let paths_ends = Node::get_paths_ends(borrowed_node);

        let mut uniq_ends: Vec<Rc<Node>> = Vec::new();
        paths_ends.iter().for_each(|p_e| {
            if !uniq_ends.iter().any(|e| Rc::ptr_eq(p_e, e)) {
                uniq_ends.push(Rc::clone(p_e));
            }
        });

        return uniq_ends.len();
    }
}

fn main() {
    let input = read_to_string("./inputs/day10.in")
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
        .map(|node| Node::count_ends(node))
        .reduce(|a, b| a + b)
        .unwrap();

    println!("Result : {}", result);
}
