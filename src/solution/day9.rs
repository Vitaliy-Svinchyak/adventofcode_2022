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

pub fn solve_with_amount_of_knots(input: &str, number_of_knots: usize) -> usize {
    let mut rope_parts = vec![Position::default(); number_of_knots + 1];
    let mut unique_positions = HashSet::new();
    unique_positions.insert(Position::default());

    for movement in input.lines() {
        let (direction, distance) = movement.split_at(2);
        let distance = distance.parse::<isize>().unwrap();

        for _ in 0..distance {
            match direction {
                "R " => rope_parts[0].x += 1,
                "L " => rope_parts[0].x -= 1,
                "U " => rope_parts[0].y += 1,
                "D " => rope_parts[0].y -= 1,
                _ => unreachable!(),
            }

            for knot_i in 1..rope_parts.len() {
                let head = rope_parts[knot_i - 1];
                let tail = rope_parts[knot_i];

                let x_diff = head.x.abs_diff(tail.x);
                let y_diff = head.y.abs_diff(tail.y);

                if x_diff + y_diff > 2 {
                    rope_parts[knot_i].x += get_bonus(head.x, tail.x);
                    rope_parts[knot_i].y += get_bonus(head.y, tail.y);
                } else if x_diff > 1 {
                    rope_parts[knot_i].x += get_bonus(head.x, tail.x);
                } else if y_diff > 1 {
                    rope_parts[knot_i].y += get_bonus(head.y, tail.y);
                }

                if knot_i == number_of_knots {
                    unique_positions.insert(rope_parts[knot_i]);
                }
            }
        }
    }

    unique_positions.len()
}

fn get_bonus(a: isize, b: isize) -> isize {
    a.cmp(&b) as isize
}
