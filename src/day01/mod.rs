use std::str::FromStr;

fn parse_groups<T: FromStr>(input: &str) -> impl Iterator<Item = impl Iterator<Item = T> + '_> {
    input
        .trim()
        .split("\n\n")
        .map(|group| group.lines().filter_map(|line| line.parse().ok()))
}

pub fn part1(input: &str) -> u32 {
    let groups = parse_groups::<u32>(input);
    groups.map(|line| line.sum()).max().unwrap()
}

pub fn part2(input: &str) -> u32 {
    let groups = parse_groups::<u32>(input);
    let mut group_sums = groups.map(|line| line.sum()).collect::<Vec<u32>>();
    group_sums.sort_unstable();
    group_sums.into_iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST: &str = include_str!("input_test.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT_TEST), 24_000);
        assert_eq!(part1(INPUT), 68_467);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT_TEST), 45_000);
        assert_eq!(part2(INPUT), 203_420);
    }
}
