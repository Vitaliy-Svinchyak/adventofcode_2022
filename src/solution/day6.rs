use itertools::Itertools;

pub fn solve(input: String) {
    let needed_length = 14;
    let mut buffer = vec![];
    let mut result = 0;
    for (i, char) in input.chars().enumerate() {
        if buffer.len() == needed_length {
            buffer.remove(0);
        }
        buffer.push(char);

        if buffer.len() == needed_length && all_unique(&buffer) {
            result = i + 1;
            break;
        }
    }

    dbg!(result);
}

fn all_unique(buffer: &[char]) -> bool {
    for (i, char) in buffer.iter().enumerate() {
        let found = buffer.iter().find_position(|c| *c == char);
        if let Some((index, ..)) = found {
            if index != i {
                return false;
            }
        }
    }

    true
}
