use std::array;
use std::ops::RangeInclusive;
use std::str::FromStr;

struct Range(RangeInclusive<u8>);

impl Range {
    fn contains(&self, other: &Self) -> bool {
        self.0.contains(other.0.start()) && self.0.contains(other.0.end())
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.0.contains(other.0.start()) || self.0.contains(other.0.end())
    }
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once('-').unwrap();
        Ok(Self(a.parse().unwrap()..=b.parse().unwrap()))
    }
}

fn parse_ranges<const N: usize>(input: &str) -> impl Iterator<Item = [Range; N]> + '_ {
    input.trim().lines().map(|line| {
        let mut ranges = line.split(',');
        array::from_fn(|_| ranges.next().unwrap().parse().unwrap())
    })
}

pub fn part1(input: &str) -> usize {
    parse_ranges(input)
        .filter(|[a, b]| a.contains(b) || b.contains(a))
        .count()
}

pub fn part2(input: &str) -> usize {
    parse_ranges(input)
        .filter(|[a, b]| a.overlaps(b) || b.overlaps(a))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST: &str = include_str!("input_test.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT_TEST), 2);
        assert_eq!(part1(INPUT), 599);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT_TEST), 4);
        assert_eq!(part2(INPUT), 928);
    }
}
