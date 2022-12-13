pub fn solve(input: String) {
    let result: u32 = input
        .split("\n\n")
        .map(|elf_inventory| {
            elf_inventory
                .split("\n")
                .map(|calories| calories.parse::<u32>().expect("Cannot parse calories"))
                .sum()
        })
        .max()
        .expect("There is no winner");
    dbg!(result);
}
