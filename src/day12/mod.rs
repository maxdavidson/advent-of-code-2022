use std::{
    collections::{HashSet, VecDeque},
    ops::Index,
};

type Point = [i32; 2];

#[derive(Debug)]
struct Grid<T> {
    data: Box<[T]>,
    rows: usize,
    cols: usize,
}

impl<T> Grid<T> {
    fn rows(&self) -> usize {
        self.rows
    }

    fn cols(&self) -> usize {
        self.cols
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn iter(&self) -> impl Iterator<Item = (Point, &T)> + '_ {
        self.data
            .iter()
            .enumerate()
            .map(|(i, val)| ([(i % self.cols) as i32, (i / self.cols) as i32], val))
    }

    fn get(&self, &[x, y]: &Point) -> Option<&T> {
        let x = x as usize;
        let y = y as usize;
        if (0..self.cols()).contains(&x) && (0..self.rows()).contains(&y) {
            self.data.get(x + y * self.cols)
        } else {
            None
        }
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, point: Point) -> &Self::Output {
        self.get(&point).unwrap()
    }
}

fn parse_input(input: &str) -> (Grid<u8>, Point, Point) {
    let mut data = Vec::new();
    let mut start_point = None;
    let mut end_point = None;
    let mut rows = 0;
    let mut cols = 0;

    for (y, line) in input.trim().lines().enumerate() {
        rows += 1;

        for (x, c) in line.chars().enumerate() {
            if y == 0 {
                cols += 1;
            }

            let point = [x as i32, y as i32];
            let val = match c {
                'S' => {
                    start_point = Some(point);
                    b'a'
                }
                'E' => {
                    end_point = Some(point);
                    b'z'
                }
                'a'..='z' => c as u8,
                _ => {
                    unreachable!();
                }
            };

            data.push(val);
        }
    }

    debug_assert_eq!(data.len(), rows * cols);

    let grid = Grid {
        data: data.into(),
        rows,
        cols,
    };

    (grid, start_point.unwrap(), end_point.unwrap())
}

fn find_closest_distance(
    height_map: &Grid<u8>,
    start_point: Point,
    end_point: Point,
) -> Option<usize> {
    let mut queue = VecDeque::with_capacity(height_map.len());
    let mut visited = HashSet::with_capacity(height_map.len());

    queue.push_back((start_point, 0));

    while let Some((point, dist)) = queue.pop_front() {
        if point == end_point {
            return Some(dist);
        }

        let height = height_map[point];
        let [x, y] = point;

        for neighbor_point in [[x - 1, y], [x + 1, y], [x, y - 1], [x, y + 1]] {
            if let Some(&neighbor_height) = height_map.get(&neighbor_point) {
                if neighbor_height <= height + 1 && visited.insert(neighbor_point) {
                    queue.push_back((neighbor_point, dist + 1));
                }
            }
        }
    }

    None
}

pub fn part1(input: &str) -> usize {
    let (height_map, start_point, end_point) = parse_input(input);

    find_closest_distance(&height_map, start_point, end_point).unwrap()
}

pub fn part2(input: &str) -> usize {
    let (height_map, _, end_point) = parse_input(input);

    height_map
        .iter()
        .filter_map(|(point, height)| {
            if *height == b'a' {
                find_closest_distance(&height_map, point, end_point)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST: &str = include_str!("input_test.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT_TEST), 31);
        assert_eq!(part1(INPUT), 361);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT_TEST), 29);
        assert_eq!(part2(INPUT), 354);
    }
}
