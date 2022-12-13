use std::{cmp::Ordering, fmt::Display, str::FromStr};

#[derive(Debug, Eq, PartialEq, Clone)]
enum Packet {
    Node(u8),
    List(Vec<Packet>),
}

impl Packet {
    fn parse_packet(s: &mut &str) -> Option<Packet> {
        Self::parse_list(s).or_else(|| Self::parse_node(s))
    }

    fn parse_node(s: &mut &str) -> Option<Packet> {
        let (i, _) = s.char_indices().find(|(_, c)| !c.is_ascii_digit())?;
        let (prefix, suffix) = s.split_at(i);
        let node = prefix.parse().ok()?;
        *s = suffix;
        Some(Packet::Node(node))
    }

    fn parse_list(s: &mut &str) -> Option<Packet> {
        let mut packets = Vec::new();
        *s = s.strip_prefix('[')?;
        while let Some(packet) = Self::parse_packet(s) {
            packets.push(packet);
            *s = s.strip_prefix(',').unwrap_or(*s);
        }
        *s = s.strip_prefix(']')?;
        Some(Packet::List(packets))
    }
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        Self::parse_packet(&mut s).ok_or(())
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Node(node) => write!(f, "{node}"),
            Self::List(list) => {
                write!(f, "[")?;
                for (i, packet) in list.iter().enumerate() {
                    write!(f, "{packet}")?;
                    if i + 1 < list.len() {
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")?;
                Ok(())
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Node(byte_a), Self::Node(byte_b)) => byte_a.cmp(byte_b),
            (Self::Node(_), Self::List(_)) => Packet::List(vec![self.clone()]).cmp(other),
            (Self::List(_), Self::Node(_)) => self.cmp(&Packet::List(vec![other.clone()])),
            (Self::List(list_a), Self::List(list_b)) => {
                let (mut it_a, mut it_b) = (list_a.iter(), list_b.iter());
                loop {
                    match (it_a.next(), it_b.next()) {
                        (Some(packet_a), Some(packet_b)) => match packet_a.cmp(packet_b) {
                            Ordering::Equal => continue,
                            ordering => break ordering,
                        },
                        (None, Some(_)) => break Ordering::Less,
                        (Some(_), None) => break Ordering::Greater,
                        (None, None) => break Ordering::Equal,
                    }
                }
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .filter_map(|(index, lines)| {
            let mut it = lines.lines();

            let packet_a: Packet = it.next().unwrap().parse().unwrap();
            let packet_b: Packet = it.next().unwrap().parse().unwrap();

            match packet_a.cmp(&packet_b) {
                Ordering::Greater => None,
                _ => Some(index + 1),
            }
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let divider_packets = [
        Packet::from_str("[[2]]").unwrap(),
        Packet::from_str("[[6]]").unwrap(),
    ];

    let mut packets: Vec<Packet> = input.lines().filter_map(|line| line.parse().ok()).collect();

    packets.extend_from_slice(&divider_packets);
    packets.sort_unstable();

    divider_packets
        .into_iter()
        .filter_map(|divider_packet| {
            packets
                .iter()
                .enumerate()
                .find_map(|(index, packet)| (*packet == divider_packet).then_some(index + 1))
        })
        .product()
}

#[cfg(test)]
mod tests {

    use super::*;

    static INPUT_TEST: &str = include_str!("input_test.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT_TEST), 13);
        assert_eq!(part1(INPUT), 5529);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT_TEST), 140);
        assert_eq!(part2(INPUT), 27_690);
    }
}
