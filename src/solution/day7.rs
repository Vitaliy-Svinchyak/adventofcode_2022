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

    for row in input.split('\n') {
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
        } else if row.starts_with("$ ls") {
            // do nothing
        } else {
            if row.starts_with("dir") {
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
    }

    hdd
}

fn get_root_dirs_with_size_less_than(hdd: &Vec<Dir>, size_limit: usize) -> usize {
    let mut total_size = 0;
    for (i, ..) in hdd.iter().skip(1).enumerate() {
        let size = get_dir_size(&hdd, i + 1);
        if size <= size_limit {
            total_size += size;
        }
    }

    total_size
}

fn get_dir_size(hdd: &[Dir], needed_i: usize) -> usize {
    let mut needed_indexes = vec![needed_i];
    let mut total_size = hdd[needed_i].files_size;

    for (i, dir) in hdd.iter().enumerate() {
        if needed_indexes.contains(&dir.parent) && i != needed_i {
            needed_indexes.push(i);
            total_size += dir.files_size;
        }
    }

    total_size
}

fn get_total_size_of_removed_dirs(hdd: &Vec<Dir>) -> usize {
    let current_total_size = get_dir_size(hdd, 0);
    let current_free_space = 70000000 - current_total_size;
    let size_to_clean = 30000000 - current_free_space;
    let sizes = dirs_by_size(hdd);
    let biggest_dirs: Vec<usize> = sizes
        .into_iter()
        .filter(|size| *size >= size_to_clean)
        .collect();

    let cleaned_size = biggest_dirs.last().unwrap();
    *cleaned_size
}

fn dirs_by_size(hdd: &Vec<Dir>) -> Vec<usize> {
    let mut result: Vec<usize> = hdd
        .iter()
        .enumerate()
        .map(|(i, ..)| get_dir_size(hdd, i))
        .collect();
    result.sort_by(|a_size, b_size| b_size.cmp(a_size));

    result
}
