use std::time::Instant;

mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn solve(input: String, day: usize) {
    let solvers = [
        day1::solve,
        day2::solve,
        day3::solve,
        day4::solve,
        day5::solve,
        day6::solve,
        day7::solve,
        day8::solve,
        day9::solve,
        day10::solve,
        day11::solve,
        day12::solve,
    ];
    if day > solvers.len() {
        panic!("No solution for day {day}");
    }

    let now = Instant::now();
    solvers[day - 1](input);
    println!("Used time: {}ms", now.elapsed().as_millis());
}
