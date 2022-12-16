pub fn solve(input: String) {
    let mut callories: Vec<u32> = input
        .split("\n\n")
        .map(|elf_inventory| {
            elf_inventory
                .lines()
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum()
        })
        .collect();
    callories.sort_unstable();
    let result: u32 = callories.iter().rev().take(3).sum();

    dbg!(result);
}
