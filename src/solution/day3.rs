use itertools::Itertools;

pub fn solve(input: String) {
    let mut i = 2;

    let result: u32 = input
        .split("\n")
        .group_by(|_| {
            i += 1;
            i / 3 % 2 == 0
        })
        .into_iter()
        .map(|(.., group)| {
            let g: Vec<&str> = group.collect();
            let e0: Vec<char> = g[0].chars().into_iter().collect();
            let e1 = g[1];
            let e2 = g[2];
            let duplicate = e0
                .into_iter()
                .find(|ch| e1.contains(&ch.to_string()) && e2.contains(&ch.to_string()))
                .unwrap();

            get_char_priority(duplicate)
        })
        .sum();

    dbg!(result);
}

pub fn solve_1(input: String) {
    let result: u32 = input
        .split("\n")
        .map(|rucksack: &str| {
            let items: Vec<char> = rucksack.chars().into_iter().collect();
            let (first_part, second_part) = items.split_at(items.len() / 2);
            let duplicate = first_part
                .iter()
                .find(|ch| second_part.contains(ch))
                .unwrap();

            get_char_priority(*duplicate)
        })
        .sum();

    dbg!(result);
}

fn get_char_priority(ch: char) -> u32 {
    let unicode_number = ch as u32;
    if unicode_number > 96 {
        unicode_number - 96
    } else {
        unicode_number - 38
    }
}
