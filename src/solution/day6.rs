use itertools::Itertools;

pub fn solve(input: String) {
    let chars = input.chars().collect::<Vec<char>>();
    let needed_length = 14;

    let (index, ..) = chars
        .windows(needed_length)
        .enumerate()
        .find(|(.., buffer)| buffer.iter().all_unique())
        .unwrap();
    println!("{}", index + needed_length);
}
