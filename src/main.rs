mod solution;

use std::fs;

fn main() {
    let day = std::env::args()
        .nth(1)
        .expect("Day should be passed as first argument")
        .parse::<u8>()
        .expect("Cannot parse day as number");
    let input_bytes = fs::read(format!("./input/day{day}.txt"))
        .unwrap_or_else(|_| panic!("No input for day {day}"));
    let input = String::from_utf8(input_bytes).expect("Cannot parse input as string");

    solution::solve(input, day);
}
