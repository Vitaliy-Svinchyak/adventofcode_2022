pub fn solve(input: String) {
    let mut register = 1;
    let mut cycle: isize = 0;
    let cycles_to_check = [20, 60, 100, 140, 180, 220];
    let mut total_signal_strength = 0;

    let mut screen = [[" "; 40]; 6];

    for command in input.split(|b| b == ' ' || b == '\n') {
        cycle += 1;

        // drawing
        let current_pixel = ((cycle - 1) % 40) as usize;
        if current_pixel.abs_diff(register as usize) <= 1 {
            let screen_row = (cycle / 40) as usize;
            screen[screen_row][current_pixel] = "■";
        }

        if cycles_to_check.contains(&cycle) {
            let signal_strength = cycle * register;
            total_signal_strength += signal_strength;
        }

        if let Ok(bonus) = command.parse::<isize>() {
            register += bonus;
        }
    }

    dbg!(total_signal_strength);
    draw(&screen);
}

// FBURHZCH
fn draw(screen: &[[&str; 40]]) {
    for row in screen {
        println!("{}", row.join(""));
    }
}
