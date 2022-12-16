use itertools::Itertools;

pub fn solve(input: String) {
    let answer_1 = solve_1(&input);
    let answer_2 = solve_2(&input);

    dbg!(answer_1, answer_2);
}

fn solve_1(input: &str) -> u32 {
    input
        .lines()
        .map(|rucksack: &str| {
            let items: Vec<char> = rucksack.chars().into_iter().collect();
            let (first_part, second_part) = items.split_at(items.len() / 2);
            let duplicate = first_part
                .iter()
                .find(|ch| second_part.contains(ch))
                .unwrap();

            get_char_priority(*duplicate)
        })
        .sum()
}
fn solve_2(input: &str) -> u32 {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|group| {
            let g: Vec<&str> = group.collect();
            let e1 = g[1];
            let e2 = g[2];
            let duplicate = g[0]
                .chars()
                .into_iter()
                .find(|ch| e1.contains(&ch.to_string()) && e2.contains(&ch.to_string()))
                .unwrap();

            get_char_priority(duplicate)
        })
        .sum()
}

fn get_char_priority(ch: char) -> u32 {
    let unicode_number = ch as u32;
    if unicode_number > 96 {
        unicode_number - 96
    } else {
        unicode_number - 38
    }
}
