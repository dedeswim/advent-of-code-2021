use std::io::{self, BufRead};

const WINDOW: usize = 3;

fn main() {
    // Not the most efficient way to implement this, as the whole stdio stream gets read,
    // but I wanted to use a functional approach. An alternative could of course have
    // been to use a loop.

    let total = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse().unwrap()) // Parse ints
        .collect::<Vec<i32>>()
        .windows(WINDOW) // Create sliding window
        .map(|w| w.iter().sum::<i32>()) // Sum elements in sliding window
        .collect::<Vec<i32>>()
        .windows(2) // Create sliding window for comparison and compare
        .filter(|line| line[1] > line[0])
        .count(); // Count how many lines are left

    println!("{}", total.to_string());
}
