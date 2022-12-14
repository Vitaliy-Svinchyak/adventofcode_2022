use crate::solution::day2::Hand::{Paper, Rock, Scissors};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}
#[derive(Copy, Clone, Debug, PartialEq)]
enum GameResult {
    Lose,
    Draw,
    Win,
}

pub fn solve(input: String) {
    let points: u32 = input
        .split('\n')
        .map(|pair| {
            let selections: Vec<&str> = pair.split(' ').collect();
            let enemy_selection = match_selection(selections[0]);
            let desired_result = match_result(selections[1]);
            get_result_points(desired_result) + get_points_of_selection(enemy_selection, desired_result)
        })
        .sum();

    dbg!(points);
}

fn match_result(selection: &str) -> GameResult {
    match selection {
        "X" => GameResult::Lose,
        "Y" => GameResult::Draw,
        "Z" => GameResult::Win,
        _ => panic!("Unknown hand {selection}"),
    }
}

fn match_selection(selection: &str) -> Hand {
    match selection {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => panic!("Unknown hand {selection}"),
    }
}

fn get_result_points(result: GameResult) -> u32 {
    match result {
        GameResult::Lose => 0,
        GameResult::Draw => 3,
        GameResult::Win => 6
    }
}

fn get_points_of_selection(enemy_selection: Hand, desired_result: GameResult) -> u32 {
    let my_selection = match enemy_selection {
        Rock => {
            match desired_result {
                GameResult::Lose => Scissors,
                GameResult::Draw => Rock,
                GameResult::Win => Paper,
            }
        }
        Paper => {
            match desired_result {
                GameResult::Lose => Rock,
                GameResult::Draw => Paper,
                GameResult::Win => Scissors,
            }
        }
        Scissors => {
            match desired_result {
                GameResult::Lose => Paper,
                GameResult::Draw => Scissors,
                GameResult::Win => Rock,
            }
        }
    };

    get_points_for_hand(my_selection)
}

fn get_points_for_hand(selection: Hand) -> u32 {
    match selection {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    }
}