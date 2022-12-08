use std::{ops::Index, str::FromStr};

use itertools::Itertools;

#[derive(Debug)]
struct Grid {
    data: Box<[u8]>,
    rows: usize,
}

impl Grid {
    fn rows(&self) -> usize {
        self.rows
    }

    fn cols(&self) -> usize {
        self.data.len() / self.rows
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = 0;
        let mut grid = Vec::new();

        for line in s.trim().lines() {
            rows += 1;
            grid.extend(line.chars().filter_map(|c| c.to_digit(10).map(|d| d as u8)))
        }

        Ok(Grid {
            data: grid.into_boxed_slice(),
            rows,
        })
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        debug_assert!(x <= self.cols());
        debug_assert!(y <= self.rows());
        &self.data[x + y * self.rows]
    }
}

pub fn part1(input: &str) -> usize {
    let grid: Grid = input.parse().unwrap();

    let rows = grid.rows();
    let cols = grid.cols();

    let nodes = (0..rows).cartesian_product(0..cols);

    nodes
        .filter(|&node| {
            let height = grid[node];
            let (x, y) = node;

            (0..x).all(|x| grid[(x, y)] < height)
                || (0..y).all(|y| grid[(x, y)] < height)
                || (x + 1..cols).all(|x| grid[(x, y)] < height)
                || (y + 1..rows).all(|y| grid[(x, y)] < height)
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    let grid: Grid = input.parse().unwrap();

    let rows = grid.rows();
    let cols = grid.cols();

    let nodes = (0..rows).cartesian_product(0..cols);

    nodes
        .map(|node| {
            let height = grid[node];
            let (x, y) = node;

            (0..x)
                .rev()
                .position(|x| height <= grid[(x, y)])
                .map(|pos| pos + 1)
                .unwrap_or(x)
                * (0..y)
                    .rev()
                    .position(|y| height <= grid[(x, y)])
                    .map(|pos| pos + 1)
                    .unwrap_or(y)
                * (x + 1..cols)
                    .position(|x| height <= grid[(x, y)])
                    .map(|pos| pos + 1)
                    .unwrap_or(cols - x - 1)
                * (y + 1..rows)
                    .position(|y| height <= grid[(x, y)])
                    .map(|pos| pos + 1)
                    .unwrap_or(rows - y - 1)
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST: &str = include_str!("input_test.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT_TEST), 21);
        assert_eq!(part1(INPUT), 1543);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT_TEST), 8);
        assert_eq!(part2(INPUT), 595_080);
    }
}
