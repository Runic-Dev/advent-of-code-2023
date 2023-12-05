use std::fs;

use aoc_day_01_lib::extract_number;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let answer = extract_number(file);
    println!("{}", answer);
}
