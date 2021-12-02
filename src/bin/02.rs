use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

struct Position {
    x: usize,
    y: usize,
    aim: usize
}

impl Position {
    pub fn move_by_ex_1(self: Self, Movement { direction, quantity }: Movement) -> Position {
        match direction {
            Direction::Up => Position {
                x: self.x,
                y: self.y - quantity,
                aim: 0
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + quantity,
                aim: 0
            },
            Direction::Forward => Position {
                x: self.x + quantity,
                y: self.y,
                aim: 0
            },
        }
    }

    pub fn move_by_ex_2(self: Self, Movement { direction, quantity }: Movement) -> Position {
        match direction {
            Direction::Up => Position {
                x: self.x,
                y: self.y,
                aim: self.aim - quantity
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y,
                aim: self.aim + quantity
            },
            Direction::Forward => Position {
                x: self.x + quantity,
                y: self.y + self.aim * quantity,
                aim: self.aim
            },
        }
    }
}

enum Direction {
    Up,
    Down,
    Forward,
}

struct Movement {
    direction: Direction,
    quantity: usize,
}

impl FromStr for Movement {
    type Err = std::io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let elements = input.split_whitespace().collect::<Vec<&str>>();
        let direction = match elements[0] {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "forward" => Direction::Forward,
            _ => panic!("Invalid direction"),
        };
        let quantity = elements[1].parse::<usize>().unwrap();
        Ok(Movement {
            direction,
            quantity,
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Usage: {} input-file exercise", args[0]);
    }
    let filename = &args[1];
    let file = File::open(filename).unwrap();

    let exercise = &args[2];
    let movement_function = if exercise == "1" {
        Position::move_by_ex_1
    } else if exercise == "2" {
        Position::move_by_ex_2
    } else {
        panic!("Invalid exercise");
    };

    let final_position = BufReader::new(file)
        .lines()
        .map(|l| Movement::from_str(l.unwrap().as_str()).unwrap())
        .fold(
            Position { x: 0, y: 0, aim: 0 },
            movement_function,
        );
    println!("Final position: {:?}", final_position.x * final_position.y);
}
