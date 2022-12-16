use itertools::Itertools;

#[derive(Debug)]
struct MoveCommand {
    amount: usize,
    from: usize,
    to: usize,
}

impl From<&str> for MoveCommand {
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.split_whitespace().collect();
        let amount = parts[1].parse::<usize>().unwrap();
        let from = parts[3].parse::<usize>().unwrap();
        let to = parts[5].parse::<usize>().unwrap();

        MoveCommand { amount, from, to }
    }
}

pub fn solve(input: String) {
    let mut parts = input.split("\n\n");
    let crates = parse_crates(parts.next().unwrap());
    let moves = parse_moves(parts.next().unwrap());
    let crates_after_move = process_moves_on_crates(moves, crates);
    let result = take_top_crates(crates_after_move);

    dbg!(result);
}

fn parse_moves(moves: &str) -> Vec<MoveCommand> {
    moves.lines().map(MoveCommand::from).collect()
}

fn parse_crates(input: &str) -> Vec<Vec<char>> {
    let mut crates = vec![vec![]; 10];

    input.lines().for_each(|row: &str| {
        row.chars()
            .enumerate()
            .filter(|(.., char)| *char as u8 >= b'A' && *char as u8 <= b'Z')
            .for_each(|(i, char)| {
                let tower = i / 4 + 1;
                crates[tower].insert(0, char);
            });
    });

    crates
}

fn process_moves_on_crates(moves: Vec<MoveCommand>, mut crates: Vec<Vec<char>>) -> Vec<Vec<char>> {
    moves.iter().for_each(|mov: &MoveCommand| {
        let new_len = crates[mov.from].len() - mov.amount;
        let moved_part = crates[mov.from].split_off(new_len);
        crates[mov.to].extend(moved_part);
    });

    crates
}

fn take_top_crates(crates: Vec<Vec<char>>) -> String {
    crates.iter().filter_map(|stock| stock.last()).join("")
}
