pub fn part1(input: &str) -> u32 {
    let get_score = |line| match line {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        _ => unreachable!(),
    };
    input.trim().lines().map(get_score).sum()
}

pub fn part2(input: &str) -> u32 {
    let get_score = |line| match line {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
        _ => unreachable!(),
    };
    input.trim().lines().map(get_score).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST: &str = include_str!("input_test.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT_TEST), 15);
        assert_eq!(part1(INPUT), 13_675);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT_TEST), 12);
        assert_eq!(part2(INPUT), 14_184);
    }
}
