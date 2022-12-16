pub fn solve(input: String) {
    let overlaps: u32 = input
        .lines()
        .map(|pair| {
            let elves: Vec<&str> = pair.split(',').collect();

            let elf1: Vec<u8> = elves[0].split('-').flat_map(str::parse).collect();
            let elf1_range = elf1[0]..=elf1[1];

            let elf2: Vec<u8> = elves[1].split('-').flat_map(str::parse).collect();
            let elf2_range = elf2[0]..=elf2[1];

            (elf1_range.contains(elf2_range.start())
                || elf1_range.contains(elf2_range.end())
                || elf2_range.contains(elf1_range.start())
                || elf2_range.contains(elf1_range.end())) as u32
        })
        .sum();

    dbg!(overlaps);
}
