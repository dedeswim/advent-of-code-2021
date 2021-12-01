use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // Not the most efficient way to implement this, as the whole stdio stream gets read,
    // but I wanted to use a functional approach. An alternative could of course have
    // been to use a loop.

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Usage: {} input-file window-size", args[0]);
    }

    let filename = &args[1];
    let window_size: usize = args[2].parse().expect("window size must be a number");

    let file = File::open(filename).unwrap();

    let total = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse().unwrap()) // Parse ints
        .collect::<Vec<i32>>()
        .windows(window_size) // Create sliding window
        .map(|w| w.iter().sum::<i32>()) // Sum elements in sliding window
        .collect::<Vec<i32>>()
        .windows(2) // Create sliding window for comparison and compare
        .filter(|line| line[1] > line[0])
        .count(); // Count how many lines are left

    println!("{}", total.to_string());
}
