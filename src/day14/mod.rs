use std::{array, collections::HashMap};

use itertools::Itertools;

enum Tile {
    Rock,
    Sand,
}

type Point = [i32; 2];
type Cave = HashMap<Point, Tile>;

fn coordinates(input: &str) -> impl Iterator<Item = impl Iterator<Item = Point> + '_> {
    input.lines().map(|line| {
        line.split(" -> ").map(|pair| {
            let mut it = pair.split(',');
            array::from_fn(|_| it.next().unwrap().parse().unwrap())
        })
    })
}

fn create_cave(coordinates: impl IntoIterator<Item = impl IntoIterator<Item = Point>>) -> Cave {
    let mut cave = Cave::new();

    for line in coordinates.into_iter() {
        for ([x_a, y_a], [x_b, y_b]) in line.into_iter().tuple_windows() {
            if x_a == x_b {
                let dy = y_b - y_a;
                for step in 0..=dy.abs() {
                    cave.insert([x_a, y_a + step * dy.signum()], Tile::Rock);
                }
            } else if y_a == y_b {
                let dx = x_b - x_a;
                for step in 0..=dx.abs() {
                    cave.insert([x_a + step * dx.signum(), y_a], Tile::Rock);
                }
            } else {
                panic!("Not a straight line!")
            }
        }
    }

    cave
}

#[allow(dead_code)]
fn draw_cave(cave: &Cave) {
    let x_min = cave.keys().map(|&[x, _]| x).min().unwrap();
    let x_max = cave.keys().map(|&[x, _]| x).max().unwrap();
    let y_min = cave.keys().map(|&[_, y]| y).min().unwrap();
    let y_max = cave.keys().map(|&[_, y]| y).max().unwrap();

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            match cave.get(&[x, y]) {
                Some(Tile::Rock) => print!("#"),
                Some(Tile::Sand) => print!("o"),
                None => print!("."),
            }
        }
        println!();
    }
    println!();
}

pub fn part1(input: &str) -> usize {
    let mut cave = create_cave(coordinates(input));

    let sand_source = [500, 0];
    let y_max = cave.keys().map(|&[_, y]| y).max().unwrap();

    'outer: loop {
        let [mut x, mut y] = sand_source;

        'inner: loop {
            if y > y_max {
                break 'outer;
            } else if let Some(point) = [[x, y + 1], [x - 1, y + 1], [x + 1, y + 1]]
                .into_iter()
                .find(|point| !cave.contains_key(point))
            {
                [x, y] = point;
            } else {
                cave.insert([x, y], Tile::Sand);
                break 'inner;
            }
        }
    }

    cave.values()
        .filter(|tile| matches!(tile, Tile::Sand))
        .count()
}

pub fn part2(input: &str) -> usize {
    let mut cave = create_cave(coordinates(input));

    let sand_source = [500, 0];
    let y_max = cave.keys().map(|&[_, y]| y).max().unwrap();

    'outer: loop {
        let [mut x, mut y] = sand_source;

        'inner: loop {
            if cave.contains_key(&sand_source) {
                break 'outer;
            } else if y + 1 >= y_max + 2 {
                cave.insert([x, y], Tile::Sand);
                break 'inner;
            } else if let Some(point) = [[x, y + 1], [x - 1, y + 1], [x + 1, y + 1]]
                .into_iter()
                .find(|point| !cave.contains_key(point))
            {
                [x, y] = point;
            } else {
                cave.insert([x, y], Tile::Sand);
                break 'inner;
            }
        }
    }

    cave.values()
        .filter(|tile| matches!(tile, Tile::Sand))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST: &str = include_str!("input_test.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT_TEST), 24);
        assert_eq!(part1(INPUT), 1199);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT_TEST), 93);
        assert_eq!(part2(INPUT), 23_925);
    }
}
