use std::{array, ops::RangeInclusive};

use lazy_static::lazy_static;
use regex::Regex;

type Point = [i32; 2];

fn bounds<T: Ord + Copy, const N: usize>(
    points: impl IntoIterator<Item = [T; N]>,
) -> Option<[RangeInclusive<T>; N]> {
    points.into_iter().fold(None, |bounds, point| match bounds {
        None => Some(point.map(|x| x..=x)),
        Some(ranges) => Some(array::from_fn(|i| {
            let x = unsafe { *point.get_unchecked(i) };
            let range = unsafe { ranges.get_unchecked(i) };
            x.min(*range.start())..=x.max(*range.end())
        })),
    })
}

fn parse_input(input: &str) -> impl Iterator<Item = (Point, Point)> + '_ {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
        )
        .unwrap();
    }

    PATTERN.captures_iter(input).map(|cap| {
        let sensor = [cap[1].parse().unwrap(), cap[2].parse().unwrap()];
        let beacon = [cap[3].parse().unwrap(), cap[4].parse().unwrap()];
        (sensor, beacon)
    })
}

#[inline]
fn manhattan_distance(point_a: Point, point_b: Point) -> i32 {
    point_a
        .into_iter()
        .zip(point_b.into_iter())
        .map(|(a, b)| (b - a).abs())
        .sum()
}

pub fn part1(input: &str, y: i32) -> usize {
    let distances = parse_input(input)
        .map(|(sensor, beacon)| (sensor, beacon, manhattan_distance(sensor, beacon)))
        .collect::<Vec<_>>();

    let [x_range, _] = bounds(distances.iter().copied().flat_map(|([x, y], _, dist)| {
        [[x - dist, y], [x + dist, y], [x, y - dist], [x, y + dist]]
    }))
    .unwrap();

    x_range
        .filter(|&x| {
            let point = [x, y];
            distances.iter().any(|(sensor, beacon, dist)| {
                point != *beacon && manhattan_distance(point, *sensor) <= *dist
            })
        })
        .count()
}

pub fn part2(input: &str, range: RangeInclusive<i32>) -> i64 {
    let distances = parse_input(input)
        .map(|(sensor, beacon)| (sensor, manhattan_distance(sensor, beacon)))
        .collect::<Vec<_>>();

    distances
        .iter()
        .flat_map(|([sensor_x, sensor_y], dist)| {
            let dist = dist + 1;
            (-dist..=dist).flat_map(move |dx| {
                let x = sensor_x + dx;
                let dy = dist - dx.abs();
                [[x, sensor_y + dy], [x, sensor_y - dy]]
            })
        })
        .filter(|[x, y]| range.contains(x) && range.contains(y))
        .find_map(|point| {
            distances
                .iter()
                .all(|(sensor, dist)| manhattan_distance(point, *sensor) > *dist)
                .then(|| {
                    let [x, y] = point;
                    (x as i64) * 4_000_000 + (y as i64)
                })
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST: &str = include_str!("input_test.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT_TEST, 10), 26);
        assert_eq!(part1(INPUT, 2_000_000), 5_461_729);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT_TEST, 0..=20), 56_000_011);
        assert_eq!(part2(INPUT, 0..=4_000_000), 10_621_647_166_538);
    }
}
