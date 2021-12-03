use std::env;
use advent_of_code_2021::utils::lines_from_file;

fn char_to_bit(c: char) -> u32 {
    match c {
        '0' => 0,
        '1' => 1,
        _ => panic!("Invalid char")
    }
}

fn string_to_bits(s: String) -> Vec<u32> {
    s.chars().map(|c| char_to_bit(c)).collect()
}

fn to_u32(slice: &[bool]) -> u32 {
    slice.iter().fold(0, |acc, &b| acc * 2 + b as u32)
}

fn sum_vectors(a: (u32, Vec<u32>), b: (u32, Vec<u32>)) -> (u32, Vec<u32>) {
    let (a_counter, a_vec) = a;
    let (b_counter, b_vec) = b;
    let sum_vec = a_vec.iter().zip(b_vec.iter())
        .map(|(a, b)| a + b)
        .collect();
    let sum_counter = a_counter + b_counter;
    (sum_counter, sum_vec)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let binary_vectors: Vec<Vec<u32>> = lines_from_file(filename)
        .map(|l| string_to_bits(l.unwrap())).collect();

    let (n_rows, vec_sums) = binary_vectors
        .into_iter()
        .map(|v| (1, v))
        .reduce(sum_vectors).unwrap();

    let gamma_rate_vec: Vec<bool> = vec_sums.iter().map(|b| b > &(n_rows / 2)).collect();
    let epsilon_rate_vec: Vec<bool> = gamma_rate_vec.iter().map(|b| !b).collect();
    let gamma_rate = to_u32(&gamma_rate_vec);
    let epsilon_rate = to_u32(&epsilon_rate_vec);
    println!("Power (ex. 1): {:?}", gamma_rate * epsilon_rate)


}