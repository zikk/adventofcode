use std::fs::read_to_string;
const INPUT_PATH: &str = "./inputs/day15.in";

struct Pos {
    y: usize,
    x: usize,
}

fn main() {
    let content = read_to_string(INPUT_PATH).unwrap();
    let input = content.trim().split("\n\n").collect::<Vec<_>>();

    let mut game = input[0]
        .split("\n")
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let movements = input[1].replace("\n", "").chars().collect::<Vec<_>>();

    let mut pos = Pos { y: 0, x: 0 };

    for y in 0..game.len() {
        for x in 0..game.len() {
            if game[y][x] == '@' {
                pos = Pos { y, x };
            }
        }
    }

    let last_index = game.len() - 2;

    movements.iter().for_each(|m| match m {
        '^' => {
            if pos.y > 1 {
                let mut boxes = 0;
                let mut _skip = false;
                let mut y = pos.y - 1;

                loop {
                    let next_char = game[y][pos.x];

                    if next_char == '#' {
                        _skip = true;
                        break;
                    }

                    if next_char == 'O' {
                        boxes += 1;
                    } else {
                        break;
                    }

                    y = y - 1;
                }

                if !_skip {
                    pos = Pos {
                        y: pos.y - 1,
                        x: pos.x,
                    };
                    game[pos.y][pos.x] = '.';

                    for i in 1..boxes + 1 {
                        game[pos.y - i][pos.x] = 'O';
                    }
                }
            }
        }
        '>' => {
            if pos.x < last_index {
                let mut boxes = 0;
                let mut _skip = false;
                let mut x = pos.x + 1;

                loop {
                    let next_char = game[pos.y][x];

                    if next_char == '#' {
                        _skip = true;
                        break;
                    }

                    if next_char == 'O' {
                        boxes += 1;
                    } else {
                        break;
                    }

                    x = x + 1;
                }

                if !_skip {
                    pos = Pos {
                        y: pos.y,
                        x: pos.x + 1,
                    };
                    game[pos.y][pos.x] = '.';

                    for i in 1..boxes + 1 {
                        game[pos.y][pos.x + i] = 'O';
                    }
                }
            }
        }
        'v' => {
            if pos.y < last_index {
                let mut boxes = 0;
                let mut _skip = false;
                let mut y = pos.y + 1;

                loop {
                    let next_char = game[y][pos.x];

                    if next_char == '#' {
                        _skip = true;
                        break;
                    }

                    if next_char == 'O' {
                        boxes += 1;
                    } else {
                        break;
                    }

                    y = y + 1;
                }

                if !_skip {
                    pos = Pos {
                        y: pos.y + 1,
                        x: pos.x,
                    };
                    game[pos.y][pos.x] = '.';

                    for i in 1..boxes + 1 {
                        game[pos.y + i][pos.x] = 'O';
                    }
                }
            }
        }
        '<' => {
            if pos.x > 1 {
                let mut boxes = 0;
                let mut _skip = false;
                let mut x = pos.x - 1;

                loop {
                    let next_char = game[pos.y][x];

                    if next_char == '#' {
                        _skip = true;
                        break;
                    }

                    if next_char == 'O' {
                        boxes += 1;
                    } else {
                        break;
                    }

                    x = x - 1;
                }

                if !_skip {
                    pos = Pos {
                        y: pos.y,
                        x: pos.x - 1,
                    };
                    game[pos.y][pos.x] = '.';

                    for i in 1..boxes + 1 {
                        game[pos.y][pos.x - i] = 'O';
                    }
                }
            }
        }
        _ => (),
    });

    let mut result = 0;

    for y in 1..game.len() - 1 {
        for x in 1..game.len() - 1 {
            if game[y][x] == 'O' {
                result = result + (y * 100 + x);
            }
        }
    }

    println!("Result : {}", result);
}
