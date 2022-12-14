pub fn solve(input: String) {
    let overlaps: u32 = input
        .split('\n')
        .map(|pair| {
            let a: Vec<&str> = pair.split(',').collect();
            let mut elf1 = a[0].split('-');
            let mut elf2 = a[1].split('-');
            let s1 = elf1.next().unwrap().parse::<u8>().unwrap();
            let s2 = elf1.next().unwrap().parse::<u8>().unwrap();
            let elf1_range = s1..=s2;

            let s1 = elf2.next().unwrap().parse::<u8>().unwrap();
            let s2 = elf2.next().unwrap().parse::<u8>().unwrap();
            let elf2_range = s1..=s2;
            if elf1_range.contains(elf2_range.start())
                || elf1_range.contains(elf2_range.end())
                || elf2_range.contains(elf1_range.start())
                || elf2_range.contains(elf1_range.end())
            {
                1
            } else {
                0
            }
        })
        .sum();

    dbg!(overlaps);
}
