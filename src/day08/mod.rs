use anyhow::{Context, Error, Result};
use itertools::Itertools;
use std::str::FromStr;

use crate::utils::AocError::*;

#[derive(Debug)]
pub struct Navigation {
    path: Vec<char>,
    nodes: Vec<Node>,
}

impl Navigation {
    pub fn find(&self, node: &str) -> Option<usize> {
        self.nodes
            .iter()
            .find_position(|n| n.id == node)
            .map(|(p, _)| p)
    }

    pub fn follow(&self, start: &str, zzz: bool) -> Option<usize> {
        let mut current = start;
        let mut index = self.find(current)?;
        let mut instructions = self.path.iter().cycle();
        let mut counter = 0;

        loop {
            counter += 1;
            let direction = instructions.next()?;
            let node = self.nodes.get(index)?;
            current = match direction {
                'L' => &node.left,
                'R' => &node.right,
                _ => None?,
            };

            if zzz && current == "ZZZ" {
                break;
            }

            if !zzz && ends_with(current, 'Z') {
                break;
            }

            index = self.find(current)?;
        }

        Some(counter)
    }

    pub fn follow_parallel(&self) -> Option<usize> {
        let all_ends_with_a = self
            .nodes
            .iter()
            .filter_map(|n| {
                if ends_with(&n.id, 'A') {
                    Some(&n.id)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let len = all_ends_with_a
            .iter()
            .map(|n| self.follow(n, false))
            .collect::<Option<Vec<_>>>()?;
        let result = len.iter().fold(1, |acc, el| num::Integer::lcm(&acc, el));

        Some(result)
    }
}

impl FromStr for Navigation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut split = s.split("\n\n");

        let path = split
            .next()
            .ok_or(GenericError)
            .context("Cannot split path")?
            .chars()
            .collect::<Vec<_>>();

        let nodes = split
            .next()
            .ok_or(GenericError)
            .context("Could not parse nodes")?
            .lines()
            .map(Node::from_str)
            .collect::<Result<Vec<_>>>()?;

        Ok(Navigation { path, nodes })
    }
}

pub fn ends_with(s: &str, c: char) -> bool {
    match s.chars().last() {
        Some(v) => v == c,
        _ => false,
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    id: String,
    left: String,
    right: String,
}

impl FromStr for Node {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: regex::Regex =
                regex::Regex::new(r"^(?P<id>\w+?) = \((?P<left>\w+?), (?P<right>\w+?)\)$").unwrap();
        }

        let (id, left, right) = RE
            .captures(s)
            .and_then(|cap| {
                let id = cap.name("id").map(|v| v.as_str())?.to_string();
                let left = cap.name("left").map(|v| v.as_str())?.to_string();
                let right = cap.name("right").map(|v| v.as_str())?.to_string();

                Some((id, left, right))
            })
            .context("Error during parse")?;

        Ok(Node { id, left, right })
    }
}

#[aoc_generator(day08)]
pub fn input_generator(input: &str) -> Result<Navigation> {
    Navigation::from_str(input)
}

#[aoc(day08, part1)]
pub fn solve_part1(input: &Navigation) -> Result<usize> {
    let result = input
        .follow("AAA", true)
        .ok_or(GenericError)
        .context("Follow failed")?;
    Ok(result)
}

#[aoc(day08, part2)]
pub fn solve_part2(input: &Navigation) -> Result<usize> {
    let result = input
        .follow_parallel()
        .ok_or(GenericError)
        .context("Follow failed")?;
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample1() -> &'static str {
        "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
    }

    fn sample2() -> &'static str {
        "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
    }

    fn input(s: &str) -> Result<Navigation> {
        input_generator(s)
    }

    #[test]
    fn part1_sample1() -> Result<()> {
        let data = input(sample1())?;
        Ok(assert_eq!(2, solve_part1(&data)?))
    }

    #[test]
    fn part1_sample2() -> Result<()> {
        let data = input(sample2())?;
        Ok(assert_eq!(6, solve_part1(&data)?))
    }

    fn sample3() -> &'static str {
        "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
    }

    #[test]
    fn part2_sample3() -> Result<()> {
        let data = input(sample3())?;
        Ok(assert_eq!(6, solve_part2(&data)?))
    }
}
