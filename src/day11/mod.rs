use std::mem;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: Test,
    total_inspections: usize,
}

#[derive(Debug, Clone)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

#[derive(Debug, Clone)]
struct Test {
    divisible_by: u64,
    monkey_if_true: usize,
    monkey_if_false: usize,
}

fn iter_monkeys(input: &str) -> impl Iterator<Item = Monkey> + '_ {
    input.split("\n\n").map(|chunk| {
        let mut lines = chunk.lines();

        lines.next().unwrap();

        let items = {
            let line = lines.next().unwrap();
            line.trim_start()
                .strip_prefix("Starting items: ")
                .unwrap()
                .split(", ")
                .map(|item| item.parse().unwrap())
                .collect()
        };

        let operation = {
            let line = lines.next().unwrap();
            let op = line.trim_start().strip_prefix("Operation:").unwrap().trim();

            if op == "new = old * old" {
                Operation::Square
            } else if let Some(amount) = op.strip_prefix("new = old * ") {
                Operation::Multiply(amount.parse().unwrap())
            } else if let Some(amount) = op.strip_prefix("new = old + ") {
                Operation::Add(amount.parse().unwrap())
            } else {
                unreachable!()
            }
        };

        let test = {
            let divisible_by = {
                let line = lines.next().unwrap();
                line.trim_start()
                    .strip_prefix("Test: divisible by ")
                    .unwrap()
                    .trim()
                    .parse()
                    .unwrap()
            };

            let monkey_if_true = {
                let line = lines.next().unwrap();
                line.trim_start()
                    .strip_prefix("If true: throw to monkey ")
                    .unwrap()
                    .trim()
                    .parse()
                    .unwrap()
            };

            let monkey_if_false = {
                let line = lines.next().unwrap();
                line.trim_start()
                    .strip_prefix("If false: throw to monkey ")
                    .unwrap()
                    .trim()
                    .parse()
                    .unwrap()
            };

            Test {
                divisible_by,
                monkey_if_true,
                monkey_if_false,
            }
        };

        Monkey {
            items,
            operation,
            test,
            total_inspections: 0,
        }
    })
}

pub fn part1(input: &str) -> usize {
    let monkeys = iter_monkeys(input);

    let mut monkeys = monkeys.collect::<Vec<_>>();

    for _ in 0..20 {
        for monkey_index in 0..monkeys.len() {
            let items = mem::take(&mut monkeys[monkey_index].items);

            monkeys[monkey_index].total_inspections += items.len();

            for item in items {
                let worry_level = match monkeys[monkey_index].operation {
                    Operation::Add(val) => item + val,
                    Operation::Multiply(val) => item * val,
                    Operation::Square => item * item,
                };

                let worry_level = worry_level / 3;

                let Test {
                    divisible_by,
                    monkey_if_false,
                    monkey_if_true,
                } = monkeys[monkey_index].test;

                let target_monkey_index = if worry_level % divisible_by == 0 {
                    monkey_if_true
                } else {
                    monkey_if_false
                };

                monkeys[target_monkey_index].items.push(worry_level)
            }
        }
    }

    monkeys.sort_unstable_by_key(|monkey| monkey.total_inspections);

    monkeys
        .into_iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.total_inspections)
        .product()
}

pub fn part2(input: &str) -> usize {
    let monkeys = iter_monkeys(input);

    let mut monkeys = monkeys.collect::<Vec<_>>();

    let lowest_common_multiple = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible_by)
        .product::<u64>();

    let mut items = Vec::new();

    for _ in 0..10_000 {
        for monkey_index in 0..monkeys.len() {
            items.clear();
            mem::swap(&mut items, &mut monkeys[monkey_index].items);

            monkeys[monkey_index].total_inspections += items.len();

            for item in items.iter() {
                let worry_level = match monkeys[monkey_index].operation {
                    Operation::Add(val) => item + val,
                    Operation::Multiply(val) => item * val,
                    Operation::Square => item * item,
                };

                let worry_level = worry_level % lowest_common_multiple;

                let Test {
                    divisible_by,
                    monkey_if_false,
                    monkey_if_true,
                } = monkeys[monkey_index].test;

                let target_monkey_index = if worry_level % divisible_by == 0 {
                    monkey_if_true
                } else {
                    monkey_if_false
                };

                monkeys[target_monkey_index].items.push(worry_level)
            }
        }
    }

    monkeys.sort_unstable_by_key(|monkey| monkey.total_inspections);

    monkeys
        .into_iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.total_inspections)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST: &str = include_str!("input_test.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT_TEST), 10_605);
        assert_eq!(part1(INPUT), 98_280);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT_TEST), 2_713_310_158);
        assert_eq!(part2(INPUT), 17_673_687_232);
    }
}
