mod day1;
mod day2;
mod day3;
mod day4;

pub fn solve(input: String, day: u8) {
    match day {
        1 => day1::solve(input),
        2 => day2::solve(input),
        3 => day3::solve(input),
        4 => day4::solve(input),
        _ => panic!("No solution for day {day}"),
    }
}

