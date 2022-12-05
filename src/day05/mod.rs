use itertools::Itertools;

fn parse_input(
    input: &str,
) -> (
    Box<[Vec<char>]>,
    impl Iterator<Item = (usize, usize, usize)> + '_,
) {
    let (stack, instructions) = input.split_once("\n\n").unwrap();

    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(9);

    for row in stack.lines().rev().skip(1) {
        for (i, c) in row.chars().skip(1).step_by(4).enumerate() {
            if stacks.len() <= i {
                stacks.resize_with(i + 1, Default::default);
            }

            if c.is_ascii_uppercase() {
                stacks[i].push(c)
            }
        }
    }

    (
        stacks.into(),
        instructions.lines().map(|instr| {
            instr
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .tuples()
                .map(|(count, from, to)| (count, from - 1, to - 1))
                .exactly_one()
                .unwrap()
        }),
    )
}

pub fn part1(input: &str) -> String {
    let (mut stacks, instructions) = parse_input(input);

    for (count, from, to) in instructions {
        for _ in 0..count {
            let c = stacks[from].pop().unwrap();
            stacks[to].push(c);
        }
    }

    stacks.iter().map(|s| *s.last().unwrap()).collect()
}

pub fn part2(input: &str) -> String {
    let (mut stacks, instructions) = parse_input(input);

    for (count, from, to) in instructions {
        for _ in 0..count {
            let c = stacks[from].pop().unwrap();
            stacks[to].push(c);
        }

        let len = stacks[to].len();
        stacks[to][len - count..].reverse()
    }

    stacks.iter().map(|s| *s.last().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST: &str = include_str!("input_test.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT_TEST), "CMZ");
        assert_eq!(part1(INPUT), "FZCMJCRHZ");
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT_TEST), "MCD");
        assert_eq!(part2(INPUT), "JSDHQMZGF");
    }
}
