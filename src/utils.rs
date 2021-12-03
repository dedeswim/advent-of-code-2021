use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn open_file(filename: &String) -> File {
    File::open(filename).expect(format!("Cannot open {}", filename).as_str())
}

pub fn lines_from_file(filename: &String) -> Lines<BufReader<File>> {
    BufReader::new(open_file(filename))
        .lines()
}