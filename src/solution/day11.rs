#[derive(Debug, Copy, Clone)]
enum Operation {
    Multiply(Argument),
    Add(Argument),
}
#[derive(Debug, Copy, Clone)]
enum Argument {
    Old,
    Number(usize),
}
#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test_divisible: usize,
    if_true_throw_to: usize,
    if_false_throw_to: usize,
    amount_of_inspections: usize,
}

impl Monkey {
    fn new(
        items: Vec<usize>,
        operation: Operation,
        test_divisible: usize,
        if_true_throw_to: usize,
        if_false_throw_to: usize,
    ) -> Monkey {
        Monkey {
            items,
            operation,
            test_divisible,
            if_true_throw_to,
            if_false_throw_to,
            amount_of_inspections: 0,
        }
    }
    fn add_items(&mut self, new_items: Vec<usize>) {
        self.items.extend(new_items);
    }

    fn inspect_items(&mut self) {
        for item in &mut self.items {
            *item = match self.operation {
                Operation::Multiply(argument) => match argument {
                    Argument::Old => *item * *item,
                    Argument::Number(n) => *item * n,
                },
                Operation::Add(argument) => match argument {
                    Argument::Old => *item + *item,
                    Argument::Number(n) => *item + n,
                },
            };
            // monkey gets bored
            *item = *item / 3;
            self.amount_of_inspections += 1;
        }
    }

    fn get_recipient_of_item(&self, item: usize) -> usize {
        if item % self.test_divisible == 0 {
            self.if_true_throw_to
        } else {
            self.if_false_throw_to
        }
    }

    fn clear_items(&mut self) {
        self.items.clear();
    }
}

pub fn solve(input: String) {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(parse_monkey).collect();

    let mut buffer = vec![vec![]; monkeys.len()];
    for _round in 0..20 {
        for (index, monkey) in monkeys.iter_mut().enumerate() {
            let new_items = std::mem::replace(&mut buffer[index], vec![]);
            monkey.add_items(new_items);
            monkey.inspect_items();
            for item in &monkey.items {
                let new_owner = monkey.get_recipient_of_item(*item);
                buffer[new_owner].push(*item);
            }
            monkey.clear_items();
        }

        for (index, monkey) in monkeys.iter_mut().enumerate() {
            let new_items = std::mem::replace(&mut buffer[index], vec![]);
            monkey.add_items(new_items);
        }
    }
    let mut inspections: Vec<usize> = monkeys
        .iter()
        .map(|monkey| monkey.amount_of_inspections)
        .collect();
    inspections.sort_unstable();
    let answer_1 = inspections.pop().unwrap() * inspections.pop().unwrap();
    dbg!(answer_1);
}

fn parse_monkey(monkey: &str) -> Monkey {
    let mut description_rows = monkey.split("\n").skip(1);
    let items_raw = description_rows.next().unwrap();
    let operation_raw = description_rows.next().unwrap();
    let test_raw = description_rows.next().unwrap();
    let if_true = description_rows.next().unwrap();
    let if_false = description_rows.next().unwrap();

    let items: Vec<usize> = items_raw
        .replace("Starting items: ", "")
        .trim()
        .split(", ")
        .map(|item| item.parse::<usize>().unwrap())
        .collect();

    let a = operation_raw.replace("Operation: new = old ", "");
    let mut operation_parts = a.trim().split(" ");
    let sign = operation_parts.next().unwrap();
    let argument = operation_parts.next().unwrap();
    let argument = if argument == "old" {
        Argument::Old
    } else {
        Argument::Number(argument.parse().unwrap())
    };
    let operation = if sign == "*" {
        Operation::Multiply(argument)
    } else {
        Operation::Add(argument)
    };

    let test_divisible = test_raw
        .replace("Test: divisible by ", "")
        .trim()
        .parse::<usize>()
        .unwrap();

    let if_true_throw_to = if_true
        .replace("If true: throw to monkey ", "")
        .trim()
        .parse::<usize>()
        .unwrap();
    let if_false_throw_to = if_false
        .replace("If false: throw to monkey ", "")
        .trim()
        .parse::<usize>()
        .unwrap();

    Monkey::new(
        items,
        operation,
        test_divisible,
        if_true_throw_to,
        if_false_throw_to,
    )
}
