#[derive(Debug)]
struct MoveInstruction {
    amount: u8,
    from: usize,
    to: usize,
}

pub fn solve(input: String) {
    let mut parts = input.split("\n\n");
    let crates = parts.next().unwrap();
    let moves = parts.next().unwrap();
    let moves = parse_moves(moves);
    let crates = parse_crates(crates);
    let crates_after_move = process_moves_on_crates(moves, crates);
    let result = take_top_crates(crates_after_move);
    dbg!(result);
}

fn parse_moves(moves: &str) -> Vec<MoveInstruction> {
    moves
        .split('\n')
        .map(|mov| {
            let parts: Vec<&str> = mov.split(' ').collect();
            let amount = parts[1].parse::<u8>().unwrap();
            let from = parts[3].parse::<usize>().unwrap();
            let to = parts[5].parse::<usize>().unwrap();

            MoveInstruction { amount, from, to }
        })
        .collect()
}

fn parse_crates(crates: &str) -> Vec<Vec<char>> {
    let mut result = vec![vec![]; 10];
    crates.split('\n').for_each(|row: &str| {
        row.chars().enumerate().for_each(|(i, char)| {
            let unicode = char as u32;
            if unicode > 64 && unicode < 91 {
                let index = i / 4 + 1;
                result[index].insert(0, char);
            }
        })
    });

    result
}

fn process_moves_on_crates(
    moves: Vec<MoveInstruction>,
    mut crates: Vec<Vec<char>>,
) -> Vec<Vec<char>> {
    moves.iter().for_each(|mov: &MoveInstruction| {
        let len_before_change = crates[mov.to].len();
        (0..mov.amount).for_each(|_| {
            let crat = crates[mov.from].pop().unwrap();
            crates[mov.to].insert(len_before_change, crat);
        });
    });

    crates
}

fn take_top_crates(crates: Vec<Vec<char>>) -> String {
    let mut accum = String::default();
    for crat in crates {
        if let Some(char) = crat.last() {
            accum.push(*char);
        }
    }

    accum
}
