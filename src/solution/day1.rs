pub fn solve(input: String) {
    let mut callories: Vec<u32> = input
        .split("\n\n")
        .map(|elf_inventory| {
            elf_inventory
                .split("\n")
                .map(|calories| calories.parse::<u32>().expect("Cannot parse calories"))
                .sum()
        })
        .collect();
    callories.sort();
    let result: u32 = callories.iter().rev().take(3).sum();

    dbg!(result);
}
