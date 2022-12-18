use itertools::Itertools;
use std::char;

pub fn solve(input: String) {
    let mut start: (usize, usize) = (0, 0);
    let mut finish: (usize, usize) = (0, 0);
    let mut map: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let mut map_row: Vec<char> = line.chars().collect();
            if let Some((x, ..)) = map_row.iter().find_position(|c| **c == 'S') {
                start = (x, y);
                map_row[x] = 'a';
            }
            if let Some((x, ..)) = map_row.iter().find_position(|c| **c == 'E') {
                finish = (x, y);
                map_row[x] = 'z';
            }
            map_row
        })
        .collect();

    dbg!(&start, &finish);
    let mut cursors = vec![start];
    let mut route_len = 0;
    loop {
        let mut new_cursors: Vec<(usize, usize)> = vec![];
        for (x, y) in cursors.iter() {
            let me = map[*y][*x] as u8;
            let potential_cursors = get_nearby_coordinates(*x, *y);
            potential_cursors
                .iter()
                .filter(|(x, y)| *y < map.len() && *x < map[0].len() && map[*y][*x] as u8 <= me + 1)
                .for_each(|c| new_cursors.push(c.clone()));
        }
        route_len += 1;
        new_cursors.sort_unstable();
        new_cursors.dedup();

        if new_cursors.contains(&finish) {
            break;
        }
        if new_cursors.is_empty() {
            dbg!(new_cursors.len());
            dbg!(&new_cursors);
            draw(map);
            break;
        }
        cursors.iter().for_each(|(x, y)| {
            map[*y][*x] = 'Z';
        });
        cursors = new_cursors;
        // dbg!(route_len);
    }

    dbg!(route_len);
}

fn draw(map: Vec<Vec<char>>) {
    for row in map {
        let line = row.iter().map(|c| c.to_string()).join("");
        println!("{}", line);
    }
}

fn get_nearby_coordinates(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut coordinates = vec![];
    if y > 0 {
        coordinates.push((x, y - 1));
    }
    if x > 0 {
        coordinates.push((x - 1, y));
    }
    coordinates.push((x, y + 1));
    coordinates.push((x + 1, y));

    coordinates
}
