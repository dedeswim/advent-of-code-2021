use std::{env, fs::File, io::{BufReader, BufRead}, str::FromStr};

struct PositionEx1 {
    x: usize,
    y: usize,
}

impl Default for PositionEx1 {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

struct PositionEx2 {
    x: usize,
    y: usize,
    aim: usize,
}

impl Default for PositionEx2 {
    fn default() -> Self {
        Self { x: 0, y: 0, aim: 0 }
    }
}

trait Movable {
    fn move_by(self: Self, movement: Movement) -> Self;
}

impl Movable for PositionEx1 {
    fn move_by(
        self: Self,
        Movement {
            direction,
            quantity,
        }: Movement,
    ) -> Self {
        match direction {
            Direction::Up => Self {
                x: self.x,
                y: self.y - quantity,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + quantity,
            },
            Direction::Forward => Self {
                x: self.x + quantity,
                y: self.y,
            },
        }
    }
}

impl Movable for PositionEx2 {
    fn move_by(
        self: Self,
        Movement {
            direction,
            quantity,
        }: Movement,
    ) -> Self {
        match direction {
            Direction::Up => Self {
                x: self.x,
                y: self.y,
                aim: self.aim - quantity,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y,
                aim: self.aim + quantity,
            },
            Direction::Forward => Self {
                x: self.x + quantity,
                y: self.y + self.aim * quantity,
                aim: self.aim,
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

fn compute_final_position<T>(input : std::io::Lines<BufReader<File>>) -> T
where
    T: Movable, T: Default
{
    let initial_position = T::default();
    let movements = input.map(|line| line.unwrap().parse::<Movement>().unwrap());
    movements.fold(initial_position, |position, movement| {
        position.move_by(movement)
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Usage: {} input-file exercise", args[0]);
    }
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();
    let exercise = &args[2];

    let result = if exercise == "1" {
        let final_position = compute_final_position::<PositionEx1>(lines);
        final_position.x * final_position.y
    } else {
        let final_position = compute_final_position::<PositionEx2>(lines);
        final_position.x * final_position.y
    };
    println!("Final position: {:?}", result);
}
