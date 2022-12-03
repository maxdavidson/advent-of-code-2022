use itertools::Itertools;

const fn priority(item: u8) -> u8 {
    match item {
        b'a'..=b'z' => item - b'a' + 1,
        b'A'..=b'Z' => item - b'A' + 27,
        _ => unreachable!(),
    }
}

fn bitset(sack: &[u8]) -> u64 {
    sack.iter().fold(0, |set, &item| set | 1 << priority(item))
}

pub fn part1(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| {
            let line = line.as_bytes();
            let (a, b) = line.split_at(line.len() / 2);

            let a = bitset(a);
            let b = bitset(b);

            (a & b).trailing_zeros()
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| line.as_bytes())
        .tuples()
        .map(|(a, b, c)| {
            let a = bitset(a);
            let b = bitset(b);
            let c = bitset(c);

            (a & b & c).trailing_zeros()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST: &str = include_str!("input_test.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT_TEST), 157);
        assert_eq!(part1(INPUT), 8233);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT_TEST), 70);
        assert_eq!(part2(INPUT), 2821);
    }
}
