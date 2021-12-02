use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

struct Position {
    x: usize,
    y: usize,
    aim: Option<usize>,
}

impl Position {
    fn move_by(
        self: Self,
        Movement {
            direction,
            quantity,
        }: Movement,
    ) -> Self {
        match self.aim {
            Some(aim) => match direction {
                Direction::Up => Self {
                    x: self.x,
                    y: self.y,
                    aim: Some(aim - quantity),
                },
                Direction::Down => Self {
                    x: self.x,
                    y: self.y,
                    aim: Some(aim + quantity),
                },
                Direction::Forward => Self {
                    x: self.x + quantity,
                    y: self.y + aim * quantity,
                    aim: Some(aim),
                },
            },
            None => match direction {
                Direction::Up => Self {
                    x: self.x,
                    y: self.y - quantity,
                    aim: None,
                },
                Direction::Down => Self {
                    x: self.x,
                    y: self.y + quantity,
                    aim: None,
                },
                Direction::Forward => Self {
                    x: self.x + quantity,
                    y: self.y,
                    aim: None,
                },
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

    let initial_position = if exercise == "1" {
        Position {
            x: 0,
            y: 0,
            aim: None,
        }
    } else if exercise == "2" {
        Position {
            x: 0,
            y: 0,
            aim: Some(0),
        }
    } else {
        panic!("Invalid exercise");
    };

    let final_position = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse::<Movement>().unwrap())
        .fold(initial_position, Position::move_by);

    let result = final_position.x * final_position.y;

    println!("Final position: {:?}", result);
}
