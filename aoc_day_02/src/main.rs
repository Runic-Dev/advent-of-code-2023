use std::fs;

use aoc_day_02_lib::{parse_color_amounts, GameRow, add_qualifying_ids};

fn main() {
    let file = fs::read_to_string("input.txt")
        .expect("Error reading file")
        .lines()
        .map(|line| parse_color_amounts(line.to_string()))
        .collect::<Vec<GameRow>>();

    let answer = add_qualifying_ids(file);

    println!("{}", answer);
}
