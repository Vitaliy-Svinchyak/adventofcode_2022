use std::collections::HashSet;

#[derive(Default, Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Position {
    x: isize,
    y: isize,
}

pub fn solve(input: String) {
    let answer_1 = solve_with_amount_of_knots(&input, 1);
    let answer_2 = solve_with_amount_of_knots(&input, 9);

    dbg!(answer_1, answer_2);
}
pub fn solve_with_amount_of_knots(input: &str, amount_of_knots: usize) -> usize {
    let mut rope_parts = vec![Position::default(); amount_of_knots + 1];
    let mut unique_positions = HashSet::new();
    unique_positions.insert(Position::default());

    for mov in input.split('\n') {
        let mut move_parts = mov.split(' ');
        let direction = move_parts.next().unwrap();
        let distance = move_parts.next().unwrap().parse::<isize>().unwrap();
        for _ in 0..distance {
            match direction {
                "R" => rope_parts[0].x += 1,
                "L" => rope_parts[0].x -= 1,
                "U" => rope_parts[0].y += 1,
                "D" => rope_parts[0].y -= 1,
                _ => unreachable!(),
            }

            for i in 1..rope_parts.len() {
                let prev_i = i - 1;
                let head = rope_parts[prev_i];

                let tail = rope_parts[i];
                let x_diff = head.x.abs_diff(tail.x);
                let y_diff = head.y.abs_diff(tail.y);
                if x_diff + y_diff > 2 {
                    rope_parts[i].x += if head.x > tail.x { 1 } else { -1 };
                    rope_parts[i].y += if head.y > tail.y { 1 } else { -1 };
                } else {
                    if x_diff > 1 {
                        rope_parts[i].x += if head.x > tail.x { 1 } else { -1 };
                    }
                    if y_diff > 1 {
                        rope_parts[i].y += if head.y > tail.y { 1 } else { -1 };
                    }
                }

                if i == rope_parts.len() - 1 {
                    unique_positions.insert(rope_parts[i]);
                }
            }
        }
    }

    unique_positions.len()
}
