pub fn solve(input: String) {
    let mut register = 1;
    let mut cycle = 0;
    let cycles_to_check = [20, 60, 100, 140, 180, 220];
    let mut total_signal_strength = 0;

    let mut screen = vec![Vec::with_capacity(40); 6];
    let mut screen_row = 0;

    for command in input.replace(' ', "\n").lines() {
        cycle += 1;

        // drawing
        let current_pixel = screen[screen_row].len();
        if current_pixel.abs_diff(register as usize) <= 1 {
            screen[screen_row].push("■");
        } else {
            screen[screen_row].push(" ");
        }
        if cycle % 40 == 0 {
            screen_row += 1;
        }

        // registry
        if cycles_to_check.contains(&cycle) {
            let signal_strength = cycle * register;
            println!("{cycle} * {register} = {signal_strength}");
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
fn draw(screen: &[Vec<&str>]) {
    for row in screen {
        println!("{}", row.join(""));
    }
}
