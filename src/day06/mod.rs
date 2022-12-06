use itertools::Itertools;

fn solve(input: &str, len: usize) -> usize {
    let mut it = input.char_indices();

    while let Some((i, _)) = it.next() {
        if it.clone().take(len).map(|(_, c)| c).all_unique() {
            return i + len + 1;
        }
    }

    unreachable!()
}

pub fn part1(input: &str) -> usize {
    solve(input, 4)
}

pub fn part2(input: &str) -> usize {
    solve(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);

        assert_eq!(part1(INPUT), 1300);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);

        assert_eq!(part2(INPUT), 3986);
    }
}
