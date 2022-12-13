mod day1;

pub fn solve(input: String, day: u8) {
    match day {
        1 => day1::solve(input),
        _ => panic!("No solution for day {day}"),
    }
}
