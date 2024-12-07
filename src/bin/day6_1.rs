use std::{
    collections::HashSet,
    fs::read_to_string,
    thread::{self},
    time,
};

const IS_PROD: bool = true;

const TEST_INPUT_PATH: &str = "./inputs/day6.test";
const PROD_INPUT_PATH: &str = "./inputs/day6.in";
const INPUT_PATH: &str = if IS_PROD {
    PROD_INPUT_PATH
} else {
    TEST_INPUT_PATH
};

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

struct Game {
    input: Vec<Vec<String>>,
    dir: Direction,
    size: usize,
    x: usize,
    y: usize,
    visited: HashSet<String>,
    end: bool,
    total: usize,
}

impl Game {
    fn try_up(&mut self) {
        let is_guard_leaving = self.y == 0;

        if is_guard_leaving {
            self.end = true;
            return;
        }

        let is_up_obstacle = self.input[self.y - 1][self.x] == "#";
        if !is_up_obstacle {
            self.y = self.y - 1;
            self.dir = Direction::UP;
        } else {
            self.try_right();
        }
    }

    fn try_down(&mut self) {
        let is_guard_leaving = self.y + 1 == self.size;

        if is_guard_leaving {
            self.end = true;
            return;
        }

        let is_down_obstacle = self.input[self.y + 1][self.x] == "#";

        if !is_down_obstacle {
            self.y = self.y + 1;
            self.dir = Direction::DOWN;
        } else {
            self.try_left();
        }
    }

    fn try_right(&mut self) {
        let is_guard_leaving = self.x + 1 == self.size;

        if is_guard_leaving {
            self.end = true;
            return;
        }

        let is_right_obstacle = self.input[self.y][self.x + 1] == "#";

        if !is_right_obstacle {
            self.x = self.x + 1;
            self.dir = Direction::RIGHT;
        } else {
            self.try_down();
        }
    }

    fn try_left(&mut self) {
        let is_guard_leaving = self.x == 0;

        if is_guard_leaving {
            self.end = true;
            return;
        }

        let is_left_obstacle = self.input[self.y][self.x - 1] == "#";

        if !is_left_obstacle {
            self.x = self.x - 1;
            self.dir = Direction::LEFT;
        } else {
            self.try_up();
        }
    }

    fn new(input: &Vec<Vec<String>>) -> Game {
        let mut dir: Direction = Direction::UP;
        let size = input.len();
        let mut x = 0;
        let mut y = 0;

        for y_i in 0..size {
            for x_i in 0..size {
                let c = input[y_i][x_i].chars().nth(0).unwrap();

                if ["^", "v", ">", "<"]
                    .iter()
                    .any(|s| s.chars().nth(0).unwrap() == c)
                {
                    x = x_i;
                    y = y_i;

                    match c {
                        '^' => dir = Direction::UP,
                        'v' => dir = Direction::DOWN,
                        '>' => dir = Direction::RIGHT,
                        '<' => dir = Direction::LEFT,
                        _ => (),
                    }
                }
            }
        }

        return Game {
            input: input.clone(),
            size,
            dir,
            x,
            y,
            visited: HashSet::new(),
            end: false,
            total: 0,
        };
    }

    fn _print(&mut self) -> () {
        thread::sleep(time::Duration::from_millis(50));
        print!("\x1B[2J\x1B[1;1H");
        println!("Total run so far : {}", self.total);

        let size = self.input.len();
        for y in 0..size {
            for x in 0..size {
                if y == self.y && x == self.x {
                    match self.dir {
                        Direction::UP => print!("^"),
                        Direction::DOWN => print!("v"),
                        Direction::LEFT => print!("<"),
                        Direction::RIGHT => print!(">"),
                    }
                } else {
                    print!("{}", self.input[y][x]);
                }
            }

            println!("");
        }
    }

    fn start(&mut self) -> usize {
        loop {
            self.total += 1;
            self.visited.insert(format!("{},{}", self.y, self.x));
            // println!("y: {}, x: {}", self.y, self.x);

            // self._print();

            match self.dir {
                Direction::UP => self.try_up(),
                Direction::DOWN => self.try_down(),
                Direction::RIGHT => self.try_right(),
                Direction::LEFT => self.try_left(),
            }

            if self.end {
                return self.visited.len();
            }
        }
    }
}

fn main() {
    let input = read_to_string(INPUT_PATH)
        .unwrap()
        .trim()
        .split("\n")
        .map(|l| l.chars().map(|c| c.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut game = Game::new(&input);
    println!("Result : {:?}", game.start());
}
