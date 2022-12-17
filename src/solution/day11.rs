use itertools::Itertools;

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

    fn inspect_items(&mut self, common_factor: usize) -> Vec<usize> {
        self.items
            .drain(0..)
            .map(|mut item| {
                item = match self.operation {
                    Operation::Multiply(argument) => match argument {
                        Argument::Old => item * item,
                        Argument::Number(n) => item * n,
                    },
                    Operation::Add(argument) => match argument {
                        Argument::Old => item + item,
                        Argument::Number(n) => item + n,
                    },
                };
                item = item % common_factor;
                self.amount_of_inspections += 1;
                item
            })
            .collect()
    }

    fn get_recipient_of_item(&self, item: usize) -> usize {
        if item % self.test_divisible == 0 {
            self.if_true_throw_to
        } else {
            self.if_false_throw_to
        }
    }
}

pub fn solve(input: String) {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(parse_monkey).collect();
    let common_factor: usize = monkeys.iter().map(|m| m.test_divisible).product();

    let mut buffer = vec![vec![]; monkeys.len()];
    for _round in 0..10000 {
        for (index, monkey) in monkeys.iter_mut().enumerate() {
            let new_items = std::mem::take(&mut buffer[index]);
            monkey.add_items(new_items);
            for item in monkey.inspect_items(common_factor) {
                let new_owner = monkey.get_recipient_of_item(item);
                buffer[new_owner].push(item);
            }
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
    let description_rows: Vec<&str> = monkey.lines().skip(1).collect();

    let items: Vec<usize> = description_rows[0]
        .replace("Starting items: ", "")
        .trim()
        .split(", ")
        .map(|item| item.parse::<usize>().unwrap())
        .collect();

    let clean_operation = description_rows[1].replace("Operation: new = old ", "");
    let (sign, argument): (&str, &str) =
        clean_operation.split_whitespace().collect_tuple().unwrap();
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

    let test_divisible = description_rows[2]
        .rsplit_once(' ')
        .unwrap()
        .1
        .parse()
        .unwrap();
    let if_true_throw_to = description_rows[3]
        .rsplit_once(' ')
        .unwrap()
        .1
        .parse()
        .unwrap();
    let if_false_throw_to = description_rows[4]
        .rsplit_once(' ')
        .unwrap()
        .1
        .parse()
        .unwrap();

    Monkey::new(
        items,
        operation,
        test_divisible,
        if_true_throw_to,
        if_false_throw_to,
    )
}
