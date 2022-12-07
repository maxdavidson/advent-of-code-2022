use std::{collections::HashMap as Map, iter};

#[derive(Debug)]
enum Input<'a> {
    List,
    ChangeDirectory { name: &'a str },
}

#[derive(Debug)]
enum Output<'a> {
    File { name: &'a str, size: u32 },
    Directory { name: &'a str },
}

#[derive(Debug)]
enum IO<'a> {
    Input(Input<'a>),
    Output(Output<'a>),
}

impl<'a> From<&'a str> for IO<'a> {
    fn from(s: &'a str) -> Self {
        let mut terms = s.split_whitespace();

        match terms.next().unwrap() {
            "$" => match terms.next().unwrap() {
                "ls" => IO::Input(Input::List),
                "cd" => IO::Input(Input::ChangeDirectory {
                    name: terms.next().unwrap(),
                }),
                _ => panic!(),
            },
            "dir" => IO::Output(Output::Directory {
                name: terms.next().unwrap(),
            }),
            rem => IO::Output(Output::File {
                size: rem.parse().unwrap(),
                name: terms.next().unwrap(),
            }),
        }
    }
}

#[derive(Debug)]
enum Filesystem<'a> {
    File { size: u32 },
    Directory(Map<&'a str, Filesystem<'a>>),
}

impl<'a> Filesystem<'a> {
    fn dir() -> Self {
        Self::Directory(Map::new())
    }

    fn file(size: u32) -> Self {
        Self::File { size }
    }

    fn insert(&mut self, path: &[&'a str], output: Output<'a>) {
        if let Filesystem::Directory(content) = self {
            if let Some((name, path)) = path.split_first() {
                let fs = content.entry(name).or_insert_with(Filesystem::dir);
                fs.insert(path, output)
            } else {
                match output {
                    Output::File { name, size } => {
                        content.insert(name, Filesystem::file(size));
                    }
                    Output::Directory { name } => {
                        content.insert(name, Filesystem::dir());
                    }
                }
            }
        }
    }

    fn size(&self) -> u32 {
        match self {
            Self::File { size } => *size,
            Self::Directory(content) => content.values().map(Self::size).sum(),
        }
    }

    fn nodes(&'a self) -> impl Iterator<Item = &'a Filesystem<'a>> {
        let mut nodes = vec![self];

        iter::from_fn(move || {
            let node = nodes.pop();
            if let Some(Self::Directory(content)) = node {
                nodes.extend(content.values());
            }
            node
        })
    }
}

enum IOState {
    Reading,
    Writing,
}

fn parse_input(input: &str) -> Filesystem {
    let mut node = Filesystem::dir();
    let mut path = Vec::new();
    let mut state = IOState::Reading;

    for line in input.trim().lines() {
        match (&state, line.into()) {
            (_, IO::Input(command)) => match command {
                Input::ChangeDirectory { name } => {
                    match name {
                        "/" => {
                            path.clear();
                        }
                        ".." => {
                            path.pop();
                        }
                        name => {
                            path.push(name);
                        }
                    };
                    state = IOState::Reading;
                }
                Input::List => {
                    state = IOState::Writing;
                }
            },
            (IOState::Writing, IO::Output(output)) => {
                node.insert(&path, output);
            }
            (IOState::Reading, IO::Output(_)) => {
                // Ignore output
            }
        }
    }

    node
}

pub fn part1(input: &str) -> u32 {
    let fs = parse_input(input);

    let mut total_size = 0;

    for node in fs.nodes() {
        if let Filesystem::Directory(_) = node {
            let size = node.size();
            if size <= 100_000 {
                total_size += size;
            }
        }
    }

    total_size
}

pub fn part2(input: &str) -> u32 {
    let fs = parse_input(input);

    let total_size = fs.size();
    let free_size = 70_000_000 - total_size;
    let needed_size = 30_000_000 - free_size;

    let mut best_size: Option<u32> = None;

    for node in fs.nodes() {
        if let Filesystem::Directory(_) = node {
            let size = node.size();
            if size >= needed_size && size <= best_size.unwrap_or(size) {
                best_size = Some(size)
            }
        }
    }

    best_size.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_TEST: &str = include_str!("input_test.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT_TEST), 95_437);
        assert_eq!(part1(INPUT), 1_297_683);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT_TEST), 24_933_642);
        assert_eq!(part2(INPUT), 5_756_764);
    }
}
