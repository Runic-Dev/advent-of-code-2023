use std::fs;

use aoc_day_02_lib::play_game;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Error reading file");
    play_game(file);
}
