use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_whitespace();

        Ok(match it.next().unwrap() {
            "noop" => Instruction::Noop,
            "addx" => Instruction::Addx(it.next().unwrap().parse().unwrap()),
            _ => unreachable!(),
        })
    }
}

pub fn part1(input: &str) -> i32 {
    let instructions = input.trim().lines().map(|line| line.parse().unwrap());

    let mut cycle_count = 0;
    let mut total_signal_strength = 0;

    let mut increment_cycle = |x_register| {
        cycle_count += 1;

        if (cycle_count + 20) % 40 == 0 {
            total_signal_strength += cycle_count * (x_register + 1);
        }
    };

    let mut x_register = 0;

    for instr in instructions {
        match instr {
            Instruction::Noop => {
                increment_cycle(x_register);
            }
            Instruction::Addx(val) => {
                increment_cycle(x_register);
                increment_cycle(x_register);
                x_register += val;
            }
        }
    }

    total_signal_strength
}

pub fn part2(input: &str) -> String {
    let instructions = input.trim().lines().map(|line| line.parse().unwrap());

    let mut output = String::with_capacity(240);

    let mut cycle_count = 0;

    let mut increment_cycle = |x_register| {
        let col = cycle_count % 40;

        if (x_register..x_register + 3).contains(&col) {
            output.push('#');
        } else {
            output.push('.');
        }

        if col == 39 {
            output.push('\n');
        }

        cycle_count += 1;
    };

    let mut x_register = 0;

    for instr in instructions {
        match instr {
            Instruction::Noop => {
                increment_cycle(x_register);
            }
            Instruction::Addx(val) => {
                increment_cycle(x_register);
                increment_cycle(x_register);
                x_register += val;
            }
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST: &str = include_str!("input_test.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT_TEST), 13_140);
        assert_eq!(part1(INPUT), 14_620);
    }

    #[test]
    fn part2_works() {
        assert_eq!(
            part2(INPUT_TEST),
            concat!(
                "##..##..##..##..##..##..##..##..##..##..\n",
                "###...###...###...###...###...###...###.\n",
                "####....####....####....####....####....\n",
                "#####.....#####.....#####.....#####.....\n",
                "######......######......######......####\n",
                "#######.......#######.......#######.....\n"
            )
        );
        assert_eq!(
            part2(INPUT),
            concat!(
                "###....##.####.###..#..#.###..####.#..#.\n",
                "#..#....#.#....#..#.#..#.#..#.#....#..#.\n",
                "###.....#.###..#..#.####.#..#.###..#..#.\n",
                "#..#....#.#....###..#..#.###..#....#..#.\n",
                "#..#.#..#.#....#.#..#..#.#.#..#....#..#.\n",
                "###...##..#....#..#.#..#.#..#.#.....##..\n"
            )
        );
    }
}
