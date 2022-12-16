pub fn solve(input: String) {
    let tree_matrix: Vec<Vec<u8>> = input
        .lines()
        .map(|row| {
            row.chars()
                .map(|char| char.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect();
    let max_y = tree_matrix.len() - 1;
    let max_x = tree_matrix[0].len() - 1;
    let mut answer_1 = max_x * 2 + max_y * 2;
    let mut answer_2 = 0;

    for (y, row) in tree_matrix.iter().enumerate() {
        if y == 0 || y == max_y {
            continue;
        }

        for (x, tree_height) in row.iter().enumerate() {
            if x == 0 || x == max_x {
                continue;
            }
            let x_i = x as isize;
            let y_i = y as isize;
            let is_visible = is_tree_visible_from(&tree_matrix, *tree_height, x_i, y_i, -1, 0)
                || is_tree_visible_from(&tree_matrix, *tree_height, x_i, y_i, 1, 0)
                || is_tree_visible_from(&tree_matrix, *tree_height, x_i, y_i, 0, -1)
                || is_tree_visible_from(&tree_matrix, *tree_height, x_i, y_i, 0, 1);
            let view_distance =
                get_tree_view_distance_from(&tree_matrix, *tree_height, x_i, y_i, -1, 0)
                    * get_tree_view_distance_from(&tree_matrix, *tree_height, x_i, y_i, 1, 0)
                    * get_tree_view_distance_from(&tree_matrix, *tree_height, x_i, y_i, 0, -1)
                    * get_tree_view_distance_from(&tree_matrix, *tree_height, x_i, y_i, 0, 1);

            if view_distance > answer_2 {
                answer_2 = view_distance;
            }

            if is_visible {
                answer_1 += 1;
            }
        }
    }

    dbg!(answer_1, answer_2);
}

fn is_tree_visible_from(
    tree_matrix: &Vec<Vec<u8>>,
    tree_height: u8,
    mut x: isize,
    mut y: isize,
    x_bonus: isize,
    y_bonus: isize,
) -> bool {
    let max_y = (tree_matrix.len() - 1) as isize;
    let max_x = (tree_matrix[0].len() - 1) as isize;

    while x > 0 && y > 0 && x < max_x && y < max_y {
        x += x_bonus;
        y += y_bonus;
        if tree_matrix[y as usize][x as usize] >= tree_height {
            return false;
        }
    }

    true
}
fn get_tree_view_distance_from(
    tree_matrix: &Vec<Vec<u8>>,
    tree_height: u8,
    mut x: isize,
    mut y: isize,
    x_bonus: isize,
    y_bonus: isize,
) -> u32 {
    let mut distance = 0;
    let max_y = (tree_matrix.len() - 1) as isize;
    let max_x = (tree_matrix[0].len() - 1) as isize;

    while x > 0 && y > 0 && x < max_x && y < max_y {
        x += x_bonus;
        y += y_bonus;
        distance += 1;
        if tree_matrix[y as usize][x as usize] >= tree_height {
            break;
        }
    }

    distance
}
