use std::collections::HashSet;

fn walk_snake<const N: usize>(input: &str) -> usize {
    let commands = input.trim().lines().map(|line| {
        let (dir, steps) = line.split_once(' ').unwrap();
        (dir.chars().next().unwrap(), steps.parse().unwrap())
    });

    let mut snake = [[0i32; 2]; N];
    let mut visited = HashSet::new();

    for (dir, steps) in commands {
        for _ in 0..steps {
            snake[0] = {
                let [hx, hy] = snake[0];
                match dir {
                    'R' => [hx + 1, hy],
                    'L' => [hx - 1, hy],
                    'U' => [hx, hy + 1],
                    'D' => [hx, hy - 1],
                    _ => unreachable!(),
                }
            };

            for i in 0..(N - 1) {
                let [hx, hy] = snake[i];
                let [tx, ty] = snake[i + 1];
                let [dx, dy] = [hx - tx, hy - ty];

                if dx.abs() > 1 || dy.abs() > 1 {
                    snake[i + 1] = [tx + dx.signum(), ty + dy.signum()];
                }
            }

            if let Some(&tail_pos) = snake.last() {
                visited.insert(tail_pos);
            }
        }
    }

    visited.len()
}

pub fn part1(input: &str) -> usize {
    walk_snake::<2>(input)
}

pub fn part2(input: &str) -> usize {
    walk_snake::<10>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2"), 13);
        assert_eq!(part1(INPUT), 6376);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2"), 1);
        assert_eq!(part2("R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20"), 36);
        assert_eq!(part2(INPUT), 2607);
    }
}
