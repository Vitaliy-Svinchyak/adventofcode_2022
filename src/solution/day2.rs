pub fn solve(input: String) {
    let points: u32 = input
        .split('\n')
        .map(|pair| {
            let selections: Vec<&str> = pair.split(' ').collect();
            let enemy_selection = selections[0].chars().next().unwrap() as u8;
            let desired_result = selections[1].chars().next().unwrap() as u8;
            get_result_points(desired_result)
                + get_points_of_selection(enemy_selection, desired_result)
        })
        .sum();

    dbg!(points);
}

fn get_result_points(result: u8) -> u32 {
    ((result - b'X') * 3) as u32
}

fn get_points_of_selection(enemy_selection: u8, desired_result: u8) -> u32 {
    let my_selection = match desired_result {
        b'X' => {
            if enemy_selection == b'A' {
                b'C'
            } else {
                enemy_selection - 1
            }
        }
        b'Y' => enemy_selection,
        b'Z' => {
            if enemy_selection == b'C' {
                b'A'
            } else {
                enemy_selection + 1
            }
        }
        _ => unreachable!(),
    };

    (my_selection - b'A' + 1) as u32
}
