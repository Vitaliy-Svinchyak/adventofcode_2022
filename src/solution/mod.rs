mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

pub fn solve(input: String, day: usize) {
    let solvers = [
        day1::solve,
        day2::solve,
        day3::solve,
        day4::solve,
        day5::solve,
        day6::solve,
        day7::solve,
    ];
    if day > solvers.len() {
        panic!("No solution for day {day}");
    }

    solvers[day - 1](input);
}
