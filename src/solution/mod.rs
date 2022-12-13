mod day1;
mod day2;

pub fn solve(input: String, day: u8) {
    match day {
        1 => day1::solve(input),
        2 => day2::solve(input),
        _ => panic!("No solution for day {day}"),
    }
}
