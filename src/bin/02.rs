use std::{
    env,
    error::Error,
    fmt,
    str::FromStr,
};
use advent_of_code_2021::utils::lines_from_file;

struct Position {
    x: usize,
    y: usize,
    aim: Option<usize>,
}

impl Position {
    fn move_by(self: Self, movement: Movement) -> Self {
        match self.aim {
            Some(aim) => match movement {
                Movement::Up(quantity) => Self {
                    x: self.x,
                    y: self.y,
                    aim: Some(aim - quantity),
                },
                Movement::Down(quantity) => Self {
                    x: self.x,
                    y: self.y,
                    aim: Some(aim + quantity),
                },
                Movement::Forward(quantity) => Self {
                    x: self.x + quantity,
                    y: self.y + aim * quantity,
                    aim: Some(aim),
                },
            },
            None => match movement {
                Movement::Up(quantity) => Self {
                    x: self.x,
                    y: self.y - quantity,
                    aim: None,
                },
                Movement::Down(quantity) => Self {
                    x: self.x,
                    y: self.y + quantity,
                    aim: None,
                },
                Movement::Forward(quantity) => Self {
                    x: self.x + quantity,
                    y: self.y,
                    aim: None,
                },
            },
        }
    }
}

enum Movement {
    Up(usize),
    Down(usize),
    Forward(usize),
}

#[derive(Debug)]
struct ParseMovementError {
    movement: String,
}

impl fmt::Display for ParseMovementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid movement: {}", self.movement)
    }
}

impl Error for ParseMovementError {}

impl FromStr for Movement {
    type Err = ParseMovementError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let elements = input.split_whitespace().collect::<Vec<&str>>();
        let quantity = elements[1].parse::<usize>().unwrap();
        match elements[0] {
            "up" => Ok(Movement::Up(quantity)),
            "down" => Ok(Movement::Down(quantity)),
            "forward" => Ok(Movement::Forward(quantity)),
            _ => Err(ParseMovementError {
                movement: elements[0].to_string(),
            }),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Usage: {} input-file exercise", args[0]);
    }
    let filename = &args[1];
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

    let final_position = lines_from_file(filename)
        .map(|line| line.unwrap().parse::<Movement>().unwrap())
        .fold(initial_position, Position::move_by);

    let result = final_position.x * final_position.y;

    println!("Final position: {:?}", result);
}
