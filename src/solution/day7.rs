pub fn solve(input: String) {
    let hdd = construct_hdd(input);
    let answer_1 = get_root_dirs_with_size_less_than(&hdd, 100000);
    let answer_2 = get_total_size_of_removed_dirs(&hdd);
    dbg!(answer_1, answer_2);
}

#[derive(Debug)]
struct Dir {
    name: String,
    children: Vec<usize>,
    files_size: usize,
    parent: usize,
}
impl Dir {
    fn new(name: String, parent: usize) -> Dir {
        Dir {
            name,
            files_size: 0,
            children: vec![],
            parent,
        }
    }

    fn add_child(&mut self, child: usize) {
        self.children.push(child);
    }
}

fn construct_hdd(input: String) -> Vec<Dir> {
    let mut hdd = vec![Dir::new("/".to_owned(), 0)];
    let mut pointer: usize = 0;

    for row in input.lines().filter(|row| *row != "$ ls") {
        if row.starts_with("$ cd") {
            let place = row.replace("$ cd ", "");
            pointer = match place.as_str() {
                "/" => 0,
                ".." => hdd[pointer].parent,
                new_current_dir => *hdd[pointer]
                    .children
                    .iter()
                    .find(|entry| hdd[**entry].name == new_current_dir)
                    .unwrap(),
            }
        } else if row.starts_with("dir") {
            let dir_name = row.replace("dir ", "");
            let hdd_len = hdd.len();
            hdd.push(Dir::new(dir_name, pointer));
            hdd[pointer].add_child(hdd_len);
        } else {
            let mut file_parts = row.split(' ');
            let file_size = file_parts.next().unwrap().parse::<usize>().unwrap();
            hdd[pointer].files_size += file_size;
        }
    }

    hdd
}

fn get_root_dirs_with_size_less_than(hdd: &[Dir], size_limit: usize) -> usize {
    hdd.iter()
        .map(|dir| get_dir_size(hdd, dir))
        .filter(|size| *size <= size_limit)
        .sum()
}

fn get_dir_size(hdd: &[Dir], dir: &Dir) -> usize {
    dir.children.iter().fold(dir.files_size, |acc, child| {
        acc + get_dir_size(hdd, &hdd[*child])
    })
}

fn get_total_size_of_removed_dirs(hdd: &[Dir]) -> usize {
    let current_total_size = get_dir_size(hdd, &hdd[0]);
    let current_free_space = 70000000 - current_total_size;
    let size_to_clean = 30000000 - current_free_space;
    let sizes = dirs_by_size(hdd);

    sizes
        .into_iter()
        .filter(|size| *size >= size_to_clean)
        .last()
        .unwrap()
}

fn dirs_by_size(hdd: &[Dir]) -> Vec<usize> {
    let mut result: Vec<usize> = hdd.iter().map(|dir| get_dir_size(hdd, dir)).collect();
    result.sort_by(|a_size, b_size| b_size.cmp(a_size));

    result
}